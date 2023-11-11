use leptos::*;

use crate::components::navbar::ProperNavBar;

#[component]
pub fn Header() -> impl IntoView {
    let toggle_menu = create_rw_signal(false);
    let on_click = move || toggle_menu.update(|current| *current = !*current);

    view! {
        <ProperNavBar toggle_menu/>
        <div class="w3-top">
            <div class="w3-white w3-xlarge" style="max-width:1200px;margin:auto">
                <div class="w3-button w3-padding-16 w3-left"
                    on:click=move|_|{on_click();}>"☰"</div>
                <div class="w3-center w3-padding-16">Foodie</div>
            </div>
        </div>
    }
}
