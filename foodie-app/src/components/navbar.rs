use crate::client_app_state::ClientAppState;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ProperNavBar(toggle_menu: RwSignal<bool>) -> impl IntoView {
    let state = expect_context::<RwSignal<ClientAppState>>();
    let (user, _) = create_slice(
        state,
        |state| state.user.clone(),
        |state, value| state.user = value,
    );
    let on_click = move |_| {
        toggle_menu.update(|value| *value = !*value);
    };
    /*
           <a href="javascript:void(0)" onclick="w3_close()"
               class="w3-bar-item w3-button">Close Menu</a>
           <a href="#food" onclick="w3_close()" class="w3-bar-item w3-button">Food</a>
           <a href="#about" onclick="w3_close()" class="w3-bar-item w3-button">About</a>
    * */

    let options = move || match user.get() {
        None => view! {
            <A on:click=on_click href="/signup" class="w3-bar-item w3-button">"Signup"</A>
            <A on:click=on_click href="/login" class="w3-bar-item w3-button">"Login"</A>
            <A on:click=on_click href="" class="w3-bar-item w3-button">"Close Menu"</A>
        }
        .into_view(),
        Some(_) => view! {
            <A on:click=on_click href="/settings" class="w3-bar-item w3-button">"Setings"</A>
            <A on:click=on_click href="" class="w3-bar-item w3-button">"Close Menu"</A>
        }
        .into_view(),
    };

    view! {
        {
            move||{
                if !toggle_menu.get() {
                    return view!{
                        <nav class="navbar" >
                            {move || options()}
                        </nav>
                 }   }.into_view();
                }
                view!{
                    <nav class="navbar" style="display:block">
                        {move || options()}
                    </nav>
                }.into_view()
            }
        }
    }
}
