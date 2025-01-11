use std::io;

fn main() {
    println!("please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("unable to read line");

    let index: usize = index.trim().parse().expect("index enteed not a number");

    let element = get_value_by_index(index);

    println!("the value of the element at index {index} is: {element}");


    let y = {
        let x = 5;
        x + 1
    };
    print!("{y}");
}
fn get_value_by_index(n: usize) -> usize {
    let a = [1,2,3,4,5];

    let element = a[n];

    return element
}
