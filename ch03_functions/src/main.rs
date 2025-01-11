fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    evaluate_something();

    let x = return_five();
    println!("The value of x is: {}", x);

    let blah = return_more_stuff(1);
    println!("The value of blah is: {}", blah);

    let thing = return_with_return(22);
    println!("The value of thing is: {}", thing);
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    println!("the measurement again is: {value}{unit_label}")
}

/*
statements declare stuff/perform some action but do not return stuff
expressions evaluate stuff and yield a result, return stuff

NOTE: if you add a semicolon to the end of an expression, you turn it into a statement
    and nothing will be evaluated or returned.
 */

// the below function evaluates to 4 and that evaluation gets bound to y.

fn evaluate_something() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn return_five() -> i32 {
     5  // returns 5
}

fn return_more_stuff(x: i32) -> i32 {
    x + 1 
}

fn return_with_return(mut x: i32) -> i32 {
    x += 5;
    return x;
}