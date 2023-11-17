use crate::{
    api::recipe_api::{get_recipes, Recipe},
    error_template::ErrorTemplate,
};
use itertools::Itertools;
use leptos::*;

struct Pagination {
    pub page_size: u64,
    pub page_number: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { page_size: 16, page_number: 0 }
    }
}

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    view! {
        <div class="w3-quarter">
            <img src={recipe.image} style="width:100%"/>
            <h3>{recipe.name.clone()}</h3>
        </div>
    }
}

#[component]
pub fn RecipeGrid() -> impl IntoView {
    //TODO pass via argument
    //let (count, set_count) = create_signal(5);
    //let (page, set_page) = create_signal(0);
    let count = 5;
    let page = 0;
    let recipes = create_resource(move || (), move |_| get_recipes(count, page));
    let render_recipes = move || {
        recipes
            .get()
            .map(move |recipes| match recipes {
                Ok(recipes) => recipes
                    .into_iter()
                    .chunks(4)
                    .into_iter()
                    .map(|row| {
                        let recipe_row = row
                            .into_iter()
                            .map(|recipe| {
                                view! {<RecipeCard recipe/>}
                            })
                            .collect_view();
                        view! {
                            <div class="w3-row-padding w3-padding-16 w3-center" id="food">
                                {recipe_row}
                            </div>
                        }
                    })
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
                    { move || render_recipes() }
                </div>

                <div class="w3-center w3-padding-32">
                    <div class="w3-bar">
                        <a href="#" class="w3-bar-item w3-button w3-hover-black">"<"</a>
                        <a href="#" class="w3-bar-item w3-button w3-hover-black">">"</a>
                    </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
