use leptos::{
    server_fn::{self, server},
    ServerFnError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    uuid: Uuid,
    name: String,
    image: String,
}

#[server(GetRecipes, "/api")]
pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
    Ok(vec![
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe 2".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fkentrollins.com%2Fwp-content%2Fuploads%2F2018%2F07%2Frefried-bean-recipe.jpg&f=1&nofb=1&ipt=9dfc97d5a1921d6f0df42ec8284553c76eae2c7ecb6fe059e1b89082c1ae405b&ipo=images".to_string(),
    },
        Recipe {
        uuid: Uuid::new_v4(),
        name: "a recipe".to_string(),
        image: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.zojirushi.com%2Fuser%2Fimages%2Frecipe%2F254.1.jpg&f=1&nofb=1&ipt=8495ffd4b855b3892cc44c4d43d8bb195542def320db043b3c276c663e150a94&ipo=images".to_string(),
    }])
}
