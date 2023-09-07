

/// 1 блок
// По умолчанию поля в rust приватные, поэтому используем pub

pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: std::time::SystemTime
}

// Используется 'static из-за относительной простоты возврата ошибки.
// В реальных приложениях обычно определяются кастомные ошибки с помощью библиотеки thiserror

fn get_user_from_db(db: &DB, user_id: &str) -> Result<User, &'static str> {
    if let Some(result) = db.find_user_by_id(user_id) {
        Ok(User {
            // предположим, что функция возвращает Result
            id: check_is_string(&result.id)?,
            email: check_is_email(&result.email)?,
            password: check_is_string(&result.password)?,
            created_at: check_is_date(&result.created_at)?,
        })
    } else {
        Err("User not found")
    }
}

fn change_user_email(user: &mut User, new_email: &str) -> Result<(), &'static str> {
    match  user.email == new_email {
        true => Err("New email must be different"),
        false => {
            user.email = check_is_email(new_email)?;
            Ok(())
        }
    }
}

fn register_user(email: &str, password: &str) -> Result<User, &'static str> {
    Ok(User {
        id: generate_uuid(),
        email: check_is_email(email)?,
        password: check_is_string(password)?,
        created_at: std::time::SystemTime::now(),
    })
}




/// 2 блок

struct User {
    id: String,
    email: String,
    password: Option<String>,
    created_at: std::time::SystemTime
}

impl User {
    // Наши методы статические по умолчанию, так как они не принимают self, &self или &mut self в качестве
    // первого аргумента
    // Это когда пользователь сам регистрируется
    pub fn register_user(email: &str, password: &str) -> User {
        User {
            id: UUID::new(),
            email: check_is_email(email.to_string()),
            password: Some(check_is_string(password.to_string())),
            created_at: std::time::SystemTime::now(),
        }
    }

    // А вот в случае, если пользователя создает Админ, то он не имеет право
    // присваивать его пароль
    pub fn register_user_by_admin(email: &str) -> User {
        User {
            id: UUID::new(),
            email: check_is_email(email.to_string()),
            password: Some(generate_random_password()),
            created_at: std::time::SystemTime::now(),
        }
    }
}

/// 3 блок

struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    // Конструктор
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    // Функция-фабрика для создания вектора с одинаковыми значениями
    pub fn identical(val: f64) -> Vector2 {
        Vector2::new(val, val)
    }
}
    // impl для клонирования вектора
    impl Clone for Vector2 {
        fn clone(&self) -> Self {
            Vector2::new(self.x, self.y)
        }
    }





fn main() {



    println!("Hello, world!");
}
