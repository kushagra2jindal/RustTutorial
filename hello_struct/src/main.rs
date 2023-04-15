// Similiar to tuples, just defining eactly what each value means
// Don't need to parse based on index, linear manner!!
#[derive(Debug)]
struct User {
    username: String,
    active: bool
}

struct Point(i32, i32, i32);

fn main() {
    
    // Constructing a struct
    let mut user1 = User {
        username: String::from("kushagra2jindal"),
        active: true
    };

    println!("User is => {:?}", user1);

    dbg!(&user1);

    println!("Username ==> {}, status ==> {}", user1.username, user1.active);
    user1.username = String::from("kj");
    println!("Username ==> {}, status ==> {}", user1.username, user1.active);

    let user2 = build_user(String::from("jindalkushagra"));
    println!("Username ==> {}, status ==> {}", user2.username, user2.active);


    let user3 = User {
        username: String::from("another@example.com"),
        ..user2
    };
    println!("Username ==> {}, status ==> {}", user3.username, user3.active);
    println!("Username ==> {}, status ==> {}", user2.username, user2.active);

    let user4 = User {
        active: false,
        ..user2
    };
    println!("Username ==> {}, status ==> {}", user4.username, user4.active);
    // This will not work now, as user2.username is moved to user4!!
    // println!("Username ==> {}, status ==> {}", user2.username, user2.active);

    let point_a = Point(1, 3, 0);
    println!("Point ==> {}", point_a.1);
}

fn build_user(username : String) -> User {
    User {
        username,
        active: false
    }
}
