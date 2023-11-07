use crate::{
    api::recipe_api::{get_recipes, Recipe},
    error_template::ErrorTemplate,
};
use leptos::*;
use leptos_router::*;

#[component]
fn RecipeCard(recipe: &Recipe) -> impl IntoView {}

#[component]
pub fn RecipeGrid() -> impl IntoView {
    let recipes = create_resource(move || (), move |_| get_recipes());
    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p>}>
        <ErrorBoundary fallback=|errors|view!{<ErrorTemplate errors=errors/>}>
        <div>
            {
                move || {

                    recipes.get().into_iter().map().unwrap_or_default()
                }
            }
        </div>
        </ErrorBoundary>
        </Transition>
    }
}
