use std::{fs, fs::File, io, io::ErrorKind, io::Read};
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
            all_other_errors => {
                panic!("Problem opening file: {all_other_errors:?}");
            }
        },
    };

    println!("{greeting_file:?}");

    // you can simplify with the unwrap() function....first let's create a file opener using match and handling a Result<>

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem opening file: {error:?}"),
    };

    println!("f: {f:?}");

    let result = read_username_from_file();
    println!("result: {result:?}");

    let shorter_result = shorter_read_username_from_file();
    println!("shorter: {shorter_result:?}");

    /*
    // now let's do the same with .unwrap(). this will return the file or panic...

    // the following will error with: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    let f = File::open("hello2.txt").unwrap();
    println!("unwrap f with error: {f:?}");
    */

    /*
    // can use .expect() that allows us to specify the error message that gets passed along to the panic macro


    // the following will error: failed to open hello2.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    let f = File::open("hello2.txt").expect("failed to open hello2.txt");
    println!("expect error for f: {f:?}");
    */

    /*
    error prop.
     */

    // try to open hello.txt and f is the Result<> with the file or error
    // create a mutable variable, file, that will hold the Result<> that will either be the file handler or an error
    // create a mutable string, s
    // read_to_string attempts to read the contents via the file handler and appends them to the mutable string, s.
    // Result<> is either the contents of the file or an error and that is returned by the function.
    // result will be: result: Ok("sean")

    /*
    the ? will coerce the error type received into the error type defined in the return type of the current function. this means a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

    this also means that ? can only be used in a function that can return things...? cannot be used in main.

    note: ? can be used with Option<> as well...the same rules apply...it will return a Some if there's a value, or None if not...in the same way Result<> returned an Ok and an Err.
     */

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;
        Ok(s)
        /*
        // can replace all of this with the above...notice the ?
        let f = File::open("hello.txt");

        let mut file = match f {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(error) => return Err(error),
        }
        */
    }

    // we can simplify the above even more. since reading files is so commonplace, the stdlib provides a convenient fs::read_to_string.

    fn shorter_read_username_from_file() -> Result<String, io::Error> {
        let filename = String::from("hello.txt");
        fs::read_to_string(filename)
    }

    /*
    the supplied text is taken in and passed to lines()
    -lines() returns an iterator that goes line by line
    -the line gets sent to next()
    -next will iterate over the lines. it returns an Option with the next line or None if there are no more lines.
    -the line that next iterates to is sent to chars()
    -chars() returns an interator over the chars of a string slice
    -the char iterator is passed to last()
    -last() consumes the chars iterator and returns the last element.
     */
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let last_char = match last_char_of_first_line("sean") {
        Some(e) => e,
        None => '_',
    };

    println!("last char: {last_char}");
}
