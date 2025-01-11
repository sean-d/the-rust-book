fn main() {

    let sentence = "hello world";
    let another_sentence = String::from("another sentence");

    let fword = first_word(&sentence);
    println!("{fword}");

    let fword = first_word(&another_sentence);
    println!("{fword}");

    let mut a = [1,2,3,4,5];
    let slice1 = &a[..2];
    println!("{:?}", slice1);
    a[0] = 99;
    let slice2 = &a[..2];
    println!("{:?}", slice2);
    println!("{:?}", slice1);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}