#[derive(Debug)]
pub enum UserEmail {
    UserEmailEmpty {
        is_email_activated: bool,
    },

    UserEmailUnactivated {
        email: String,
        is_email_activated: bool,
    },

    UserEmailActivated {
        email: String,
        is_email_activated: bool,
    },
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub user_email: UserEmail,
}

impl User {
    fn new(id: &str, user_email: UserEmail) -> Self {
        User {
            id: id.to_string(),
            user_email,
        }
    }

    pub fn new_empty(id: &str) -> Self {
        Self::new(
            id,
            UserEmail::UserEmailEmpty {
                is_email_activated: false,
            },
        )
    }

    pub fn new_unactivated(id: &str, email: &str) -> Self {
        Self::new(
            id,
            UserEmail::UserEmailUnactivated {
                email: email.to_string(),
                is_email_activated: false,
            },
        )
    }

    pub fn new_activated(id: &str, email: &str) -> Self {
        Self::new(
            id,
            UserEmail::UserEmailActivated {
                email: email.to_string(),
                is_email_activated: true,
            },
        )
    }
}

fn main() {
    let user1 = User::new_empty("1");
    let user2 = User::new_unactivated("2", "user2@example.com");
    let user3 = User::new_activated("3", "user3@example.com");
    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", user3);
}
