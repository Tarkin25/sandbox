use uuid::Uuid;
use crate::Observable;

pub struct CreateUser {
    pub username: String,
    pub api_key: String,
}

#[derive(Debug, Clone)]
pub struct UserCreated {
    pub id: Uuid,
    pub username: String,
    pub api_key: String,
}

#[derive(Default)]
pub struct UserManagement<'a> {
    pub user_created: Observable<'a, UserCreated>,
}

impl UserManagement<'_> {
    pub fn create_user(&mut self, create_user: CreateUser) {
        let created = self.persist_user(create_user);

        self.user_created.emit_event(created);
    }

    fn persist_user(&mut self, create_user: CreateUser) -> UserCreated {
        let CreateUser {
            username,
            api_key,
        } = create_user;

        let created = UserCreated {
            id: Uuid::new_v4(),
            username,
            api_key,
        };

        println!("{:?}", &created);

        created
    }
}