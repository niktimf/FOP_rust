

#[derive(Debug)]
pub enum UserEmail {
    UserEmailEmpty {
        is_email_activated: bool
    },

    UserEmailUnactivated {
        email: String,
        is_email_activated: bool
    },

    UserEmailActivated {
        email: String,
        is_email_activated: bool
    }
}


#[derive(Debug)]
pub struct User {
    pub id: String,
    pub user_email: UserEmail
}



fn main() {

    let user_with_empty_email = User {
        id: String::from("1"),
        user_email: UserEmail::UserEmailEmpty  {
            is_email_activated: false,
        },
    };

    let user_with_unactivated_email = User {
        id: String::from("2"),
        user_email: UserEmail::UserEmailUnactivated {
            email: String::from("unactivated_emal@example.com"),
            is_email_activated: false,
        },
    };

    let user_with_activated_email = User {
        id: String::from("3"),
        user_email: UserEmail::UserEmailActivated {
            email: String::from("activated_emal@example.com"),
            is_email_activated: true,
        },
    };
    
    println!("{:?}", user_with_empty_email);
    println!("{:?}", user_with_unactivated_email);
    println!("{:?}", user_with_activated_email);
    
}
