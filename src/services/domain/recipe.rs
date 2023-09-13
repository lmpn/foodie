use super::ingredient::Ingredient;

#[derive(Debug, Clone)]
pub struct Recipe {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        image: String,
        method: String,
        ingredients: Vec<Ingredient>,
    ) -> Self {
        Self {
            uuid: id,
            name,
            image,
            method,
            ingredients,
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

    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    pub fn ingredients(&self) -> &[Ingredient] {
        self.ingredients.as_ref()
    }
}
