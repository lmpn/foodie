pub mod ingredient;

use ingredient::Ingredient;

#[derive(Debug, Clone)]
pub struct Recipe {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    details: Option<RecipeDetails>,
}
#[derive(Debug, Clone)]
pub struct RecipeDetails {
    method: String,
    ingredients: Vec<Ingredient>,
}

impl RecipeDetails {
    pub fn new(method: String, ingredients: Vec<Ingredient>) -> Self {
        Self {
            method,
            ingredients,
        }
    }

    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    pub fn ingredients(&self) -> &[Ingredient] {
        self.ingredients.as_ref()
    }
}

impl Recipe {
    pub fn new(
        uuid: uuid::Uuid,
        name: String,
        image: String,
        details: Option<RecipeDetails>,
    ) -> Self {
        Self {
            uuid,
            name,
            image,
            details,
        }
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn image(&self) -> &str {
        self.image.as_ref()
    }

    pub fn details(&self) -> Option<&RecipeDetails> {
        self.details.as_ref()
    }
}
