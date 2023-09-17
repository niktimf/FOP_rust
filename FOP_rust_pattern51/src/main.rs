

/// 1 блок

/*
pub fn some_fn(val: Email) {
    // ...
}

 */



/// 2 блок
/// Макрос, который автоматически реализует trait Debug для нашего типа, чтобы мы смогли вывести
/// значение с помощью println!
#[derive(Debug)]
pub struct Email(String);

#[derive(Debug)]
pub struct AdminEmail(String);

#[derive(Debug)]
pub struct UserEmail(String);

#[derive(Debug)]
pub enum EmailCategory {
    Email(Email),
    AdminEmail(AdminEmail),
    UserEmail(UserEmail),
    NotEmail
}

pub fn of_string(val: String) -> EmailCategory {
    match (val.contains("@"), val.contains("%"), val.contains("*")) {
        (true, false, false) => EmailCategory::Email(Email(val)),
        (false, true, false) => EmailCategory::AdminEmail(AdminEmail(val)),
        (false, false, true) => EmailCategory::UserEmail(UserEmail(val)),
        _ => EmailCategory::NotEmail
    }

    /*
    if val.contains("@") {
        EmailCategory::Email(Email(val))
    } else if val.contains("%") {
        EmailCategory::AdminEmail(AdminEmail(val))
    } else if val.contains("*") {
        EmailCategory::UserEmail(UserEmail(val))
    } else {
        EmailCategory::NotEmail
    }
     */

}



fn main() {
    let email= "HelloWorld_@mail.com".to_string();
    let admin_email = "HelloWorld_%mail.com".to_string();
    let user_email = "HelloWorld_*mail.com".to_string();
    let invalid_email = "HelloWorld_mail.com".to_string();

    let check_email = of_string(email);
    let check_admin_email = of_string(admin_email);
    let check_user_email = of_string(user_email);
    let check_invalid_email = of_string(invalid_email);


    match check_email {
        EmailCategory::Email(email) => println!("Email: {}", email.0),
        _ => println!("{:?}", check_email),
    }

    match check_admin_email {
        EmailCategory::AdminEmail(admin_email) => println!("AdminEmail: {}", admin_email.0),
        _ => println!("{:?}", check_admin_email),
    }

    match check_user_email {
        EmailCategory::UserEmail(user_email) => println!("UserEmail: {}", user_email.0),
        _ => println!("{:?}", check_user_email),
    }

    println!("Invalid email: {:?}", check_invalid_email);



    /*
    match check_invalid_email {
        EmailCategory::NotEmail => println!("NotEmail"),
        _ => println!("{:?}", check_invalid_email),
    }

     */


    /*
    println!("{:?}", check_email);
    println!("{:?}", check_admin_email);
    println!("{:?}", check_user_email);
    println!("{:?}", check_invalid_email);

     */

}
