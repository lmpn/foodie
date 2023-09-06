#[derive(Debug, Clone)]
pub struct Ingredient {
    id: i64,
    name: String,
    amount: f64,
    unit: String,
}

impl Ingredient {
    pub fn new(id: i64, name: String, amount: f64, unit: String) -> Self {
        Self {
            id,
            name,
            amount,
            unit,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
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
