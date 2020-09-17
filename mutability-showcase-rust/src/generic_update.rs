use crate::User;
use std::collections::HashMap;

fn update_map(user: &mut User) -> HashMap<&'static str, &mut String> {
    let mut result = HashMap::new();
    result.insert("name", &mut user.name);
    result.insert("email", &mut user.email);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_user_by_key() {
        let mut user = User {
            name: "Axel".to_string(),
            email: "ash@novatec-gmbh.de".to_string(),
        };

        let mut update_map = update_map(&mut user);

        let mut_ptr = update_map.get_mut("name").unwrap();
        **mut_ptr = "aXeL".to_string();
    }
}
