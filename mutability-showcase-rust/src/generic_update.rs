use crate::User;
use std::collections::HashMap;

fn update_map(user: &mut User) -> HashMap<&'static str, &mut String> {
    let mut result = HashMap::new();
    result.insert("name", &mut user.name);
    result.insert("email", &mut user.email);
    result
}

/// Sets Nones to the previous Some
/// ```
/// # use mutability_showcase_rust::generic_update::normalize_fields;
/// let mut sparse = vec![
///     Some("foo"),
///     None,
///     None,
///     Some("bar"),
///     Some("baz"),
///     None,
/// ];
///
/// normalize_fields(&mut sparse);
/// assert_eq!(sparse, vec![
///     Some("foo"),
///     Some("foo"),
///     Some("foo"),
///     Some("bar"),
///     Some("baz"),
///     Some("baz"),
/// ]);
/// ```
pub fn normalize_fields<'a, I, T>(iter: I)
where
    I: IntoIterator<Item = &'a mut Option<T>> + 'a,
    T: 'a + Clone,
{
    let mut last_f = None;
    for t in iter {
        match t {
            Some(f) => {
                last_f = Some(f.clone());
            }
            None => {
                *t = last_f.clone();
            }
        }
    }
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
