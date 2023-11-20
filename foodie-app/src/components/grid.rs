use crate::{
    api::recipe_api::{get_recipes, Recipe},
    error_template::ErrorTemplate,
};
use leptos::*;
use leptos_router::A;
use itertools::Itertools;

#[derive(Clone, PartialEq)]
struct Pagination {
    pub page_size: u8,
    pub page_number: u8,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { page_size: 1, page_number: 0 }
    }
}

impl Pagination{
    pub fn next_page(& mut self){
        self.page_number = self.page_number.saturating_add(1);
    }

    pub fn previous_page(& mut self){
        self.page_number = self.page_number.saturating_sub(1);
    }
}

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    let details = format!("/{}", recipe.uuid);
    view! {
        <A href=details>
            <div class="w3-quarter">
            <img src={recipe.image} style="width:100%"/>
            <h3>{recipe.name.clone()}</h3>
            </div>
        </A>
    }
}

#[component]
pub fn RecipeGrid() -> impl IntoView {
    //TODO pass via argument
    //let (count, set_count) = create_signal(5);
    let (pagination, set_page) = create_signal(Pagination::default());
    let recipes = create_resource(
        move || (pagination.get()), 
        move |pagination| get_recipes(pagination.page_size, 
                                      pagination.page_number)
    );
    let next_page = move || {
        set_page.update(|value|{value.next_page();});
    };
    let previous_page = move || {
        set_page.update(|value|{value.previous_page();});
    };
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
                            <div class="w3-row-padding w3-padding-16 w3-center">
                                {recipe_row}
                            </div>
                        }
                    })
                    .collect_view(),
                Err(error) => {
                    view! { 
                        <pre class="error">
                            "Server Error: " {error.to_string()}
                        </pre>
                    }.into_view()
                }
            })
            .unwrap_or_default()
    };
    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p>}>
            <ErrorBoundary 
                fallback=|errors|view!{<ErrorTemplate errors=errors/>}
            >
                <div class="w3-main w3-content w3-padding" 
                     style="max-width:1200px;margin-top:50px">
                    { move || render_recipes() }
                </div>

                <hr/>
                <div class="w3-center w3-padding-32">
                    <div class="w3-bar">
                        <button class="w3-bar-item w3-button w3-hover-black" 
                            on:click=move|_|{previous_page();}>"<"</button>
                        <button class="w3-bar-item w3-button w3-hover-black" 
                            on:click=move|_|{next_page();}>">"</button>
                    </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
