fn main() {
    struct  User {
        username : String,
        email : String,
        sing_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        email: String::from("algumero@gmail.com"),
        username : String::from("cavalo"),
        sing_in_count: 203,
        active: true,
    };
    println!("{}",user1.email);
    let user2  = build_user(String::from("Koloss"), "meu niom".to_string());
    let user3 = User {
        username: String::from("Joao dosla"),
        ..user2
    };
}

fn  build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sing_in_count: 1 ,
    }
}
