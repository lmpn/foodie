use crate::{
    api::authorization_api::{self, get_user},
    client_app_state::ClientAppState,
    components::{details::RecipeDetails, grid::RecipeGrid, header::Header},
};

use leptos::tracing;
use leptos::IntoView;
use leptos::{
    component, create_resource, create_rw_signal, create_server_action, logging::log,
    provide_context, view, Show, Transition,
};
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_reactive::signal_prelude::SignalUpdate;
use leptos_reactive::SignalGet;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn Landing() -> impl IntoView {
    let state = create_rw_signal(ClientAppState::default());
    provide_context(state);
    let login = create_server_action::<authorization_api::Login>();
    let logout = create_server_action::<authorization_api::Logout>();
    let signup = create_server_action::<authorization_api::Signup>();
    let user_resource = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| async move {
            let user = get_user().await;
            if let Ok(inner) = &user {
                state.update(|value| value.user = inner.clone());
            }
            user
        },
    );
    let is_user_logged_in = move || matches!(user_resource.get(), Some(Ok(Some(_))));
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="public/favicon.ico"/>
            <Stylesheet id="leptos" href="/pkg/foodie-app.css"/>
            <Router>
                <Header/>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <Transition fallback=move || view! {<span>"Loading..."</span>} >
                                    <Show when=move || {log!("here {:?}", user_resource.get()); is_user_logged_in()}>
                                        <RecipeGrid/>
                                    </Show>
                            </Transition> }}
                    />
                    <Route path="/:id" view=move || view! { < RecipeDetails /> }/>
                    <Route path="signup" view=move || view! { < crate::forms::signup_form::SignupForm action=signup/> }/>
                    <Route path="login" view=move || view! { < crate::forms::login_form::LoginForm action=login /> }/>
                    <Route path="settings" view=move || view! { <h1>"Settings"</h1> <crate::forms::logout_form::LogoutForm action=logout /> }/>
                </Routes>
            </Router>
    }
}
