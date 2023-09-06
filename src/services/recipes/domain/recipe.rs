use super::ingredient::Ingredient;

#[derive(Debug, Clone)]
pub struct Recipe {
    id: i64,
    name: String,
    image: String,
    method: String,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(
        id: i64,
        name: String,
        image: String,
        method: String,
        ingredients: Vec<Ingredient>,
    ) -> Self {
        Self {
            id,
            name,
            image,
            method,
            ingredients,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
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
}
