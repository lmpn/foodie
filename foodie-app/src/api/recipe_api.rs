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
    // Ok(vec![
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe 2".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    // },
    //     Recipe {
    //     uuid: Uuid::new_v4(),
    //     name: "a recipe".to_string(),
    //     image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.zojirushi.com%2Fuser%2Fimages%2Frecipe%2F254.1.jpg&f=1&nofb=1&ipt=8495ffd4b855b3892cc44c4d43d8bb195542def320db043b3c276c663e150a94&ipo=images".to_string(),
    // }])
}
