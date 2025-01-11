// hasmaps are not in the prelude so we have to pull them in
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");

    // get will find the value of the supplied key OR None (since get returns an Option<&V)
    // copied() takes the Option<&i32> get returns and turns it into a copy of it as an Option<i32>. unwrap_or takes that and returns the value if present of the supplied default (0 in this case) if not found.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {score}");

    // iteraete over a hashnap

    let mut scores = HashMap::new();

    scores.insert("blue", 11);
    scores.insert("yellow", 134);

    for (k, v) in &scores {
        println!("{k}: {v} ");
    }

    // ownershp: for types that implement the Copy trait (i32), those values are copied into a hashnap. for types that do not implement Copy (String), those are moved and the hashmap takes ownership of those values.

    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are no longer valid as they are now assimilated into map...resistence is futile.

    // if we pass references in while creating entries into the map, we have to make sure those values that are referenced remain active/alive (have a lifetime...hint hint) for at least as long as map is valid.

    // updating an entry is trivial...
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    println!("{scores:?}");
    scores.insert("blue", 2000);
    println!("{scores:?}");

    // adding a k/v only if the key is not present
    // we use entry. it will return an enum called Entry that represents a value that might or might not be present.

    // or_insert happens after Entry is sent to it. if the key exists it will return a mutable reference to the value the key is associated with. if it doesn't exist, it inserts the supplied parameter as the new value and returns a mutable reference with this new value.

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);

    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    println!("{scores:?}");

    // word count...updating a value based on the old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split(" ") and split_whitespace() are the same...both return an interator over subslices, sep by whitespace, of the value in text.

    // or_insert returns a mutable reference (&mut V) to the value for the specified key.

    // the mutable reference is tored in the variable count. in order to assign to that value, count has to be dereferenced (*count).

    // the mutable ref goes out of scope at the end of the for loop so all of these changes are safe and allowed by the borrow checker.

    for word in text.split(" ") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println! {"{map:?}"};
}
