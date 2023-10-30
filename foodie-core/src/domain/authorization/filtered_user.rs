#[derive(Debug, Clone, Default)]
pub struct FilteredUser {
    id: uuid::Uuid,
    name: String,
    email: String,
    role: String,
    photo: String,
    verified: bool,
}

impl FilteredUser {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        email: String,
        role: String,
        photo: String,
        verified: bool,
    ) -> Self {
        Self {
            id,
            name,
            email,
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
