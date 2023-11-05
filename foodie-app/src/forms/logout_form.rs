use crate::auth;
use leptos::*;
use leptos_router::*;

#[component]
pub fn LogoutForm(action: Action<auth::Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}
