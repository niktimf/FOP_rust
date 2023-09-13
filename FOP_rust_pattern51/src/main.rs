
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


pub fn of_string(val: &Email) -> Result<Email, String> {
    match val.0.contains("@") {
        true => {
            Ok(Email(val.0.to_string()))
        }
        false => {
            Err("Not correct email".to_string())
        }
    }
}


pub fn some_fn(val: Email) {
    // ...
}


/*
pub fn some_other_fn(val: String) {
    match Email::of_string(val) {
        Ok(email) => {
            some_fn(email);
        }
        
        Err(error) => {
            println!("{}", error);
        }
    }
}

 */




fn main() {

    let valid_email = Email("HelloWorld@mail.com".to_string());
    let invalid_email = Email("HelloWorldmail.com".to_string());

    let check_valid_email = of_string(&valid_email);
    let check_invalid_email = of_string(&invalid_email);

    match check_valid_email {
        Ok(email) => {
            println!("{}", email.0);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    match check_invalid_email {
        Ok(email) => {
            println!("{}", email.0);
        }
        Err(err) => {
            println!("{}", err);
        }
    }



    /*
        /// :? указывает на то, что значение должно быть отформатировано с использованием реализации трейта
        /// Debug, то есть Rust выводит значение используя реализацию Debug
        println!("{:?}", check_valid_email);
        println!("{:?}", check_invalid_email);

     */

}
