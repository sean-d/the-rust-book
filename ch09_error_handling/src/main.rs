use std::{fs::File, io::ErrorKind};
fn main() {
    /*

    // panic causes rust to unwind the stack...it walked up the stack and cleans up...can take a while.
    // can add to Cargo.toml to simply crash the app and not unwind...
    // [profile.release]
    // panic = 'abort'

    //panic!("crash");

    // will cause OOB error. let's you know about RUST_BACKTRACE
    // $ RUST_BACKTRACE=1 cargo run ... gives full stack trace
    // let v = vec![1];
    // v[2];
    */

    /*
    File::open returns a Result<T,E>. T is filled in by the implementation of File::open with the type of the successful value: std::fs::File, which is a file handle. Phew.

    The type of E used in the error value is std::io::Error. This return type means that the call to File::open might succeed and return a file handle...or the call might fail (file doesn't exist, we dont have permissions to access it, etc).

    The File::open function needs a way to alert success or failure and at the same time, give us either the file handle or the error information. This information is what the Result enum conveys.

    If File::open succeeds, greeting_from_file will be an instance of Ok that contains a file handle.

    if File::open failes, greeting_from_file will be an instance of Err that contains more infomration about the error that occurred.

    We match on greetin_from_file since we dont know what it holds and we use the two arms (Ok, Err) to test which it actually is. the result is stored in greeting_file.

    Because there can be different errors from what we are trying, we need to check if certain errors happen and mitigate if possible.

    So we check if the error is simply that the file does not exist; if not, we create it and return the file handle to that new file.  If that also fails, we panic.

    Any other error we panic.
     */

    let greeting_from_file = File::open("hello.txt");

    let greeting_file = match greeting_from_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening file: {other_error:?}");
            }
        },
    };

    println!("{greeting_file:?}");
}
