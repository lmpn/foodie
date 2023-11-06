use crate::api::authorization_api;
use leptos::*;
use leptos_router::*;

#[component]
pub fn LogoutForm(
    action: Action<authorization_api::Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}
