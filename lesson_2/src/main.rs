struct User {
    username: String,
    status: StatusUser,
}

enum StatusUser {
    One,
}

fn main() {
    let user = User {
        username: String::from("Username"),
        status: StatusUser::One,
    };

    if let StatusUser::One = user.status {
        println!("{}", user.username)
    }
}
