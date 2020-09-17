mod generic_update;

pub fn update_request_handler(update_request: UpdateRequest) {
    let user = read_user(&update_request.email);
    if let Some(mut user) = user {
        update(&mut user, update_request);
        store(user)
    }
}

fn update(user: &mut User, update_request: UpdateRequest) {
    user.email = update_request.email;
    user.name = update_request.name;
}

fn store(user: User) {
    // TODO call DB
}

fn read_user(email: &str) -> Option<User> {
    // TODO call DB
    match email {
        "awu@novatec-gmbh.de" => Some(User {
            email: email.to_string(),
            name: "Andre".to_string(),
        }),
        _ => None,
    }
}

pub struct UpdateRequest {
    email: String,
    name: String,
}
struct User {
    email: String,
    name: String,
}
