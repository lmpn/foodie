use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    SinkExt, StreamExt,
};
use ini::Ini;
use notify::{
    event::{DataChange, ModifyKind},
    Config, Event, RecommendedWatcher, RecursiveMode, Watcher,
};

pub struct Configuration {
    watcher: UnboundedReceiver<notify::Result<Event>>,
    _w: Box<dyn Watcher>,
    database_url: String,
    migrations_path: String,
    address: (Vec<u8>, u16),
    jwt_secret: String,
    jwt_maxage: i64,
}

impl Configuration {
    pub fn new() -> Self {
        let path =
            std::env::var("INI_CONFIGURATION").expect("env variable INI_CONFIGURATION not set");
        let (mut tx, rx) = unbounded();

        let mut w: Box<dyn Watcher> = Box::new(
            RecommendedWatcher::new(
                move |res| {
                    futures::executor::block_on(async {
                        tx.send(res).await.unwrap();
                    })
                },
                Config::default(),
            )
            .expect("Error creating watcher"),
        );
        w.watch(path.as_ref(), RecursiveMode::NonRecursive)
            .expect("Error starting watcher");
        let configuration = Ini::load_from_file(&path).expect("Error loading configuration file");

        let database_url = configuration
            .get_from(Some("DATABASE"), "url")
            .expect("Invalid url");

        let migrations_path = configuration
            .get_from(Some("DATABASE"), "migrations_path")
            .expect("Invalid migrations path");

        let address: Vec<u8> = configuration
            .get_from(Some("SERVER"), "address")
            .unwrap()
            .split('.')
            .map(|e| e.parse::<u8>().unwrap())
            .collect();
        let port = configuration
            .get_from(Some("SERVER"), "port")
            .expect("Invalid port value")
            .parse::<u16>()
            .expect("Invalid port value");
        let jwt_secret = configuration
            .get_from(Some("AUTHENTICATION"), "JWT_SECRET")
            .expect("invalid JWT secret value")
            .to_string();
        let jwt_maxage = configuration
            .get_from(Some("AUTHENTICATION"), "JWT_MAXAGE")
            .expect("invalid JWT maxage value")
            .parse::<i64>()
            .expect("JWT maxage value must be u32");

        Self {
            watcher: rx,
            _w: w,
            database_url: database_url.to_string(),
            migrations_path: migrations_path.to_string(),
            address: (address, port),
            jwt_secret,
            jwt_maxage,
        }
    }

    pub async fn has_change(&mut self) -> Option<()> {
        loop {
            if let Some(Ok(Event {
                kind: notify::EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                ..
            })) = self.watcher.next().await
            {
                return Some(());
            }
        }
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_ref()
    }

    pub fn jwt_secret(&self) -> &str {
        self.jwt_secret.as_ref()
    }

    pub(crate) fn address(&self) -> ([u8; 4], u16) {
        (
            self.address.0.as_slice().try_into().unwrap(),
            self.address.1,
        )
    }

    pub fn jwt_maxage(&self) -> i64 {
        self.jwt_maxage
    }

    pub fn migrations_path(&self) -> &str {
        self.migrations_path.as_ref()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
