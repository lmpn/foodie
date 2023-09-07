#[derive(Debug, Clone)]
pub struct Ingredient {
    uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
}

impl Ingredient {
    pub fn new(id: uuid::Uuid, name: String, amount: f64, unit: String) -> Self {
        Self {
            uuid: id,
            name,
            amount,
            unit,
        }
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn unit(&self) -> &str {
        self.unit.as_ref()
    }
}
