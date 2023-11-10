use leptos::*;

use crate::components::navbar::ProperNavBar;

#[component]
pub fn Header() -> impl IntoView {
    let (toggle_menu, toggle_menu_set) = create_signal(false);
    view! {
    <ProperNavBar toggle_menu/>
    <div class="w3-top">
      <div class="w3-white w3-xlarge" style="max-width:1200px;margin:auto">
        <div class="w3-button w3-padding-16 w3-left" on:click=move|_|{toggle_menu_set.update(|current| *current= !*current);}>"☰"</div>
        <div class="w3-center w3-padding-16">Foodie</div>
      </div>
    </div>
    }
}
