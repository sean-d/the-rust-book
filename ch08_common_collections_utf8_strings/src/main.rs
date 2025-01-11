fn main() {
    // creating a string, mutable, we can add to it.
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{s}");

    // push_str takes a slice of strings and not a string; there is no borrowing so we are able to do this and still print s2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut z = String::from("lo");
    z.push('l'); // push takes a single char and adds to z
    println!("z is: {z}");

    // concat

    /* the + operator uses the add() method. the add signature looks like: fn add(self, s: &str) -> String {}

        Because of this, s1 ownership is moved into the add method (as self) due to the use of the + concatting and thus is gone after s3 is created.

        s2 is referred to and can still be printed due to it being referenced during add().
    */
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    //println!("s1: {s1}"); // fails
    println!("s2: {s2}");
    println!("s3: {s3}");

    // another + concat example
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {s}");

    // format macro is cleaner and returns a string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    // string indexing does not work as expected. String does not support indexing. move along.

    // a string is simply a wrapper over a Vec<u8>. and some elements in that Vec may span beyond a single placement....
    // be mindful of slicing strings as you may not know where each character will actually end

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s: {s}"); // prints s: Зд

    // iterate over each character
    for c in "Зд".chars() {
        println!("{c}");
    }

    // iterate over each byte
    // note: be sure to remember that valid Unicode scalar values may be made up of more than one byte.
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
