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
/// # use mutability_showcase_rust::generic_update::fill_nones;
/// let mut sparse = vec![
///     Some("foo"),
///     None,
///     None,
///     Some("bar"),
///     Some("baz"),
///     None,
/// ];
///
/// fill_nones(&mut sparse);
/// assert_eq!(sparse, vec![
///     Some("foo"),
///     Some("foo"),
///     Some("foo"),
///     Some("bar"),
///     Some("baz"),
///     Some("baz"),
/// ]);
/// ```
pub fn fill_nones<'a, I, T>(iter: I)
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

    #[test]
    fn fill_ranking() {
        #[derive(Eq, PartialEq, Debug)]
        struct LeaderboardEntry {
            name: String,
            ranking: Option<usize>,
        }

        let mut leaderboard = vec![
            LeaderboardEntry {
                name: "Woop".into(),
                ranking: Some(1),
            },
            LeaderboardEntry {
                name: "Dee".into(),
                ranking: Some(2),
            },
            LeaderboardEntry {
                name: "Doo".into(),
                ranking: None,
            },
            LeaderboardEntry {
                name: "Doodle".into(),
                ranking: Some(3),
            },
        ];

        fill_nones(leaderboard.iter_mut().map(|it| &mut it.ranking));

        assert_eq!(
            leaderboard,
            vec![
                LeaderboardEntry {
                    name: "Woop".into(),
                    ranking: Some(1),
                },
                LeaderboardEntry {
                    name: "Dee".into(),
                    ranking: Some(2),
                },
                LeaderboardEntry {
                    name: "Doo".into(),
                    ranking: Some(2),
                },
                LeaderboardEntry {
                    name: "Doodle".into(),
                    ranking: Some(3),
                },
            ]
        );
    }
}
