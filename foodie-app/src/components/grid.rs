use crate::{
    api::recipe_api::{get_recipes, Recipe},
    error_template::ErrorTemplate,
};
use leptos::*;

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    view! {
        <div class="grid-item">
            <p>{recipe.name.clone()}</p>
            <img src={recipe.image}/>
        </div>
    }
}

#[component]
pub fn RecipeGrid() -> impl IntoView {
    //TODO pass via argument
    let count = 5;
    let page = 0;
    let recipes = create_resource(move || (), move |_| get_recipes(count, page));
    let render_recipes = move || {
        recipes
            .get()
            .map(move |recipes| match recipes {
                Ok(recipes) => recipes
                    .into_iter()
                    .map(|recipe| view! {<RecipeCard recipe/>})
                    .collect_view(),
                Err(error) => {
                    view! { <pre class="error">"Server Error: " {error.to_string()}</pre>}
                        .into_view()
                }
            })
            .unwrap_or_default()
    };
    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p>}>
            <ErrorBoundary fallback=|errors|view!{<ErrorTemplate errors=errors/>}>
                <div class="grid-container">
                    { render_recipes() }
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
