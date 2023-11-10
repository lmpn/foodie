use crate::client_app_state::ClientAppState;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let state = expect_context::<RwSignal<ClientAppState>>();
    let (user, _) = create_slice(
        state,
        |state| state.user.clone(),
        |state, value| state.user = value,
    );

    match user.get() {
        None => view! {
            <A href="/signup">"Signup"</A>", "
            <A href="/login">"Login"</A>", "
        }
        .into_view(),
        Some(user) => view! {
            <A href="/settings">"Settings"</A>", "
            <span>{format!("Logged in as: {} ({})", user.name, user.id)}</span>
        }
        .into_view(),
    }
}

#[component]
pub fn ProperNavBar(toggle_menu: ReadSignal<bool>) -> impl IntoView {
    let state = expect_context::<RwSignal<ClientAppState>>();
    let (user, _) = create_slice(
        state,
        |state| state.user.clone(),
        |state, value| state.user = value,
    );

    let options = move || {
        match user.get() {
        None => view! {
            <A href="/signup" class="w3-bar-item w3-button">"Signup"</A>
            <A href="/login" class="w3-bar-item w3-button">"Login"</A>
            <A href="" on:click=move|_|{log!("close")} class="w3-bar-item w3-button">"Close Menu"</A>
        }
        .into_view(),
        Some(_) => view! {
            <A href="/settings" class="w3-bar-item w3-button">"Setings"</A>
            <A href="" on:click=move|_|{log!("close")} class="w3-bar-item w3-button">"Close Menu"</A>
        }
        .into_view(),
    }
    };

    view! {
        {
            move||{
                if !toggle_menu.get() {
                    log!("navbar false");
                    return view! {}.into_view();
                }
                log!("navbar true");
                view!{<nav class="navbar" >
                    {move || {options()}}
                </nav>}.into_view()
            }
        }
    }
}
