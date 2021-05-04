#[derive(Debug)]
pub struct User {
    id: u32,
    email: Option<String>,
    name: Option<String>,
}

impl User {
    pub fn new(id: u32, email: Option<String>, name: Option<String>) -> User {
        User {
            id,
            email,
            name,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn with_id(self, id: u32) -> User {
        User {
            id,
            ..self
        }
    }

    pub fn with_email(self, email: String) -> User {
        User {
            email: Some(email),
            ..self
        }
    }

    pub fn with_name(self, name: String) -> User {
        User {
            name: Some(name),
            ..self
        }
    }
}