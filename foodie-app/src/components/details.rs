use crate::error_template::ErrorTemplate;
use leptos::*;
use leptos_router::*;
use uuid::Uuid;

#[derive(Clone, Params, PartialEq)]
struct DetailsParams {
    id: uuid::Uuid,
}

async fn details(uuid: uuid::Uuid) -> String {
    uuid.to_string()
}

#[component]
pub fn RecipeDetails() -> impl IntoView {
    let params = use_params::<DetailsParams>();
    let uuid = move || params.with(|params| params.as_ref().map(|params| params.id)
                           .unwrap_or_default());
    let recipe = create_resource(move || (uuid()), move |id| details(id));
    let render_recipe = move || {};
    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p>}>
            <ErrorBoundary
                fallback=|errors|view!{<ErrorTemplate errors=errors/>}
            >
                <div class="w3-main w3-content w3-padding"
                     style="max-width:1200px;margin-top:50px">
                     {move || recipe.get()}
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
