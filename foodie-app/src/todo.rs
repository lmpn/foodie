use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    user: Option<AuthenticatedUser>,
    title: String,
    created_at: String,
    completed: bool,
}
use crate::auth::{self, get_user, AuthenticatedUser};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::{auth::{AuthSession}};
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
       use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlTodo {
        id: u32,
        title: String,
        created_at: String,
        completed: bool,
    }

    impl SqlTodo {
        pub async fn into_todo(self, _pool: &SqlitePool, user : Option<AuthenticatedUser>) -> Todo {
            Todo {
                id: self.id,
                user,
                title: self.title,
                created_at: self.created_at,
                completed: self.completed,
            }
        }
    }
}
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use futures::TryStreamExt;

    let pool = pool()?;
    let session = auth()?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos").fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }

    // why can't we just have async closures?
    // let mut rows: Vec<Todo> = rows.iter().map(|t| async { t }).collect();

    let mut converted_todos = Vec::with_capacity(todos.len());

    let current_user = session.current_user;
    for t in todos {
        let todo = t.into_todo(&pool, current_user.clone()).await;
        converted_todos.push(todo);
    }

    let todos: Vec<Todo> = converted_todos;

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => "".to_string(),
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)")
        .bind(title)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

// The struct name and path prefix arguments are optional.
#[server]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[component]
pub fn NavBar(user: Result<Option<AuthenticatedUser>, ServerFnError>) -> impl IntoView {
    match user {
        Err(e) => view! {
            <A href="/signup">"Signup"</A>", "
            <A href="/login">"Login"</A>", "
            <span>{format!("Login error: {}", e)}</span>
        }
        .into_view(),
        Ok(None) => view! {
            <A href="/signup">"Signup"</A>", "
            <A href="/login">"Login"</A>", "
            <span>"Logged out."</span>
        }
        .into_view(),
        Ok(Some(user)) => view! {
            <A href="/settings">"Settings"</A>", "
            <span>{format!("Logged in as: {} ({})", user.name, user.id)}</span>
        }
        .into_view(),
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let login = create_server_action::<auth::Login>();
    let logout = create_server_action::<auth::Logout>();
    let signup = create_server_action::<auth::Signup>();

    let user = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );

    provide_meta_context();

    view! {

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/session_auth_axum.css"/>
        <Router>
            <header>
                <A href="/"><h1>"Foodie"</h1></A>
                <Transition fallback=move || view! {<span>"Loading..."</span>} >
                    {move || {
                        user.get().map(|user|{
                            view!{ <NavBar user=user/> }
                        })
                    }}
                </Transition>
            </header>
            <hr/>
            <main>
                <Routes>
                <Route path="/" view=move || {
                    view! {
                        // only show the outlet if data have loaded
                        <Show when=move || { if let Some(Ok(Some(_))) = user.get(){true}else{false}} fallback=|| view! { <p>"Loading"</p> }>
                        <Outlet/>
                        </Show>
                    }
                }>
                    // nested child route
                    <Route path="/" view=move || view! { }/>
                </Route>
                <Route path="signup" view=move || view! { < crate::forms::signup_form::SignupForm action=signup/> }/>
                <Route path="login" view=move || view! { < crate::forms::login_form::LoginForm action=login /> }/>
                <Route path="settings" view=move || view! {
                    <h1>"Settings"</h1>
                    <crate::forms::logout_form::LogoutForm action=logout />
                }/>
                </Routes>
            </main>
        </Router>
    }
}
