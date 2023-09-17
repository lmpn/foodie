#[derive(Debug, Clone)]
pub struct User {
    id: uuid::Uuid,
    name: String,
    email: String,
    password: String,
    role: String,
    photo: String,
    verified: bool,
}

impl User {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        email: String,
        password: String,
        role: String,
        photo: String,
        verified: bool,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password,
            role,
            photo,
            verified,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn role(&self) -> &str {
        self.role.as_ref()
    }

    pub fn photo(&self) -> &str {
        self.photo.as_ref()
    }

    pub fn verified(&self) -> bool {
        self.verified
    }
}
