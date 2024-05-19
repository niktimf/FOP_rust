

/// 1 блок

/*
pub fn some_fn(val: Email) {
    // ...
}

 */



/// 2 блок
#[derive(Debug)]
pub struct ManagerEmail(String);

#[derive(Debug)]
pub struct AdminEmail(String);

#[derive(Debug)]
pub struct UserEmail(String);

#[derive(Debug)]
pub enum Email {
    ManagerEmail(ManagerEmail),
    AdminEmail(AdminEmail),
    UserEmail(UserEmail),
    NotEmail
}

pub fn of_string(val: String) -> Email {
    match (val.contains("@"), val.contains("%"), val.contains("*")) {
        (true, false, false) => Email::ManagerEmail(ManagerEmail(val)),
        (false, true, false) => Email::AdminEmail(AdminEmail(val)),
        (false, false, true) => Email::UserEmail(UserEmail(val)),
        _ => Email::NotEmail
    }
}



fn main() {
    let manager_email= "HelloWorld_@mail.com".to_string();
    let admin_email = "HelloWorld_%mail.com".to_string();
    let user_email = "HelloWorld_*mail.com".to_string();
    let invalid_email = "HelloWorld_mail.com".to_string();

    let check_manager_email = of_string(manager_email);
    let check_admin_email = of_string(admin_email);
    let check_user_email = of_string(user_email);
    let check_invalid_email = of_string(invalid_email);


    match check_manager_email {
        Email::ManagerEmail(email) => println!("ManagerEmail: {}", email.0),
        _ => println!("{:?}", check_manager_email),
    }

    match check_admin_email {
        Email::AdminEmail(admin_email) => println!("AdminEmail: {}", admin_email.0),
        _ => println!("{:?}", check_admin_email),
    }

    match check_user_email {
        Email::UserEmail(user_email) => println!("UserEmail: {}", user_email.0),
        _ => println!("{:?}", check_user_email),
    }

    println!("Invalid email: {:?}", check_invalid_email);


    /*
    match check_invalid_email {
        Email::NotEmail => println!("NotEmail"),
        _ => println!("{:?}", check_invalid_email),
    }

     */


    /*
    println!("{:?}", check_manager_email);
    println!("{:?}", check_admin_email);
    println!("{:?}", check_user_email);
    println!("{:?}", check_invalid_email);

     */

}
