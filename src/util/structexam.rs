struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn build_user_style_1(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1
        }
    }

    fn build_user_style_2(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }
}

#[cfg(test)]
mod tests {

    use super::User;

    #[test]
    fn test_new_user() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        println!("The newly created user name is {} and user email is {}", user1.username, user1.email);
    }

    #[test]
    fn test_build_user_style_1() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123"); 
        let user1 = User::build_user_style_1(email, username);

        println!("The newly created user name is {} and user email is {}", user1.username, user1.email);
        // println!("The local user name is {} and user email is {}", username, email);
    }

    #[test]
    fn test_build_user_style_2() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123"); 
        let user1 = User::build_user_style_2(email, username);

        println!("The newly created user name is {} and user email is {}", user1.username, user1.email);
    }
}