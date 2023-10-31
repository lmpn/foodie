#[derive(Clone, Debug)]
pub struct AuthorizationService<Storage>
where
    Storage: Send + Sync,
{
    pub(crate) storage: Storage,
    pub(crate) secret: String,
    pub(crate) maxage: i64,
}

impl<Storage> AuthorizationService<Storage>
where
    Storage: Send + Sync,
{
    pub fn new(storage: Storage, secret: String, maxage: i64) -> Self {
        Self {
            storage,
            secret,
            maxage,
        }
    }
}
