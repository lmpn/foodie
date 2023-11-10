use crate::{
    api::authorization_api::{self, get_user},
    client_app_state::ClientAppState,
    components::{grid::RecipeGrid, header::Header, navbar::NavBar},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Landing() -> impl IntoView {
    let state = create_rw_signal(ClientAppState::default());
    provide_context(state);

    let login = create_server_action::<authorization_api::Login>();
    let logout = create_server_action::<authorization_api::Logout>();
    let signup = create_server_action::<authorization_api::Signup>();

    let user: Resource<
        (usize, usize, usize),
        Result<Option<authorization_api::AuthenticatedUser>, ServerFnError>,
    > = create_resource(
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

    let is_user_logged_in = move || {
        if let Some(Ok(Some(_))) = user.get() {
            true
        } else {
            false
        }
    };

    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/foodie-app.css"/>
        <Router>
            <main>
                <Header/>
                <Routes>
                    <Route path="/"
                        view=move || {
                            view! {
                                <Transition fallback=move || view! {<span>"Loading..."</span>} >
                                    <Show when=move || {is_user_logged_in()}>
                                        <RecipeGrid/>
                                    </Show>
                                </Transition>
                            }}
                    />
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
