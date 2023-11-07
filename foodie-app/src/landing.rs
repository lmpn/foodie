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
use crate::{
    api::authorization_api::{self, get_user, AuthenticatedUser},
    components::{grid::RecipeGrid, navbar::NavBar},
};

#[component]
pub fn Landing() -> impl IntoView {
    let login = create_server_action::<authorization_api::Login>();
    let logout = create_server_action::<authorization_api::Logout>();
    let signup = create_server_action::<authorization_api::Signup>();

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
                    <Route path="/" view=RecipeGrid/>
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
