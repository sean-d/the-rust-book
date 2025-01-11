#[derive(Debug)] // so we can print
                 // a struct is a struct is a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct...for things like...plot points, colors, etc.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // create a user as one expects....nothing fancy here.
    // make it mut so we can change it
    let mut user1 = User {
        active: true,
        username: String::from("me"),
        email: String::from("not@home.com"),
        sign_in_count: 1,
    };

    println!("before email change");
    println!("{:#?}", user1);

    // change fields as expected
    user1.email = String::from("another@example.com");
    println!("after email change");
    println!("{:#?}", user1);

    let new_user = build_user(
        String::from("me@example.com"),
        String::from("another@example.com"),
    );
    println!("after user building new user");
    println!("{:#?}", new_user);

    // you can take on values from one instance and apply it to another
    let mut user2 = User {
        active: true,
        username: String::from("me2"),
        email: String::from("not2@home.com"),
        sign_in_count: user1.sign_in_count,
    };

    // you can even take all the things you aren't being specific about
    // NOTE: this is going to cause a borrow check error due to the fact we
    // moved username, which is a String, from user1 into user3.
    let mut user3 = User {
        email: String::from("not3@home.com"),
        ..user1
    };

    println!("user2 borrowed sign_in_count from user1");
    println!(
        "user1 signing count:{}  user2 sign_in_count:{}",
        user1.sign_in_count, user2.sign_in_count
    );
    println!("user3 was just lazy...");
    //println!("user 1:{:#?}  user3:{:#?}", user1, user3);
    /*
    the borrow checker issue above happens ^^
     */

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// classic builder pattern
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // you can omit the value if it's the same name as the key.
        email: email,       // but being explicit is desirable so pfffft to that
        sign_in_count: 1,
    }
}
