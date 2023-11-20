use leptos::{
    server_fn::{self, server},
    ServerFnError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    pub uuid: Uuid,
    pub name: String,
    pub image: String,
}

#[server(GetRecipes, "/api")]
pub async fn get_recipes(count: u8, page: u8) -> Result<Vec<Recipe>, ServerFnError> {
    use crate::server::context_recipe_service;
    use foodie_core::ports::incoming::recipe::recipe_page_query::RecipesPageQuery;

    let service = &context_recipe_service()? as &dyn RecipesPageQuery;
    let recipes = service
        .recipes_page_query(count, page)
        .await?
        .into_iter()
        .map(|value| Recipe {
            uuid: value.uuid(),
            name: value.name().to_string(),
            image: value.image().to_string(),
        })
        .collect::<Vec<_>>();
    Ok(recipes)
}



#[server(GetRecipe, "/api")]
pub async fn get_recipe(uuid : Uuid) -> Result<Recipe, ServerFnError> {
    use crate::server::context_recipe_service;
    use foodie_core::ports::incoming::recipe::recipe_query::RecipeQuery;

    let service = &context_recipe_service()? as &dyn RecipeQuery;
    let recipe = service
        .recipe_query(uuid)
        .await
        .map(|value| Recipe {
            uuid: value.uuid(),
            name: value.name().to_string(),
            image: value.image().to_string(),
        })
        ?;
    Ok(recipe)
}
