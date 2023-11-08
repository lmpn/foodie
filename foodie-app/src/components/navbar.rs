use crate::api::authorization_api::AuthenticatedUser;
use leptos::*;
use leptos_router::*;

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
        }
        .into_view(),
        Ok(Some(user)) => view! {
            <A href="/settings">"Settings"</A>", "
            <span>{format!("Logged in as: {} ({})", user.name, user.id)}</span>
        }
        .into_view(),
    }
}
