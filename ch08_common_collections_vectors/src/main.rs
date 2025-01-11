fn main() {
    // instantiate a new vector that will contain i32 elements
    //let v: Vec<i32> = Vec::new();

    // using the vec! macro and letting rust determine type based on supplied elements
    //let v = vec![1, 2, 3];

    // create a vec and add stuff to it. note the mut
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let third = v[2];
    // let third = &[2]; // get a reference rather than a copy
    println!("i am the third element: {third}");

    println!("{:#?}", v);

    // can use get() to test if an element is there. as you can see, we match and test and act accordingly.
    let second = v.get(1);
    match second {
        Some(second) => println!("the second element is: {second}"),
        None => println!("no 2nd element"),
    }

    let impossible = v.get(1000);
    match impossible {
        Some(impossible) => println!("tried to get imp: {impossible}"),
        None => println!("does not exist"),
    }

    /*
    // we borrow the first element...then change the vector.
    // the borrow checker is not pleased because we are trying to change a thing actively borrowed elsewhere and this can cause all kinds of problems if allosed.
    // if we were to push() after the printing at the bottom, this would work as the borrowing of the element is out of scope and done so the vector change can take place.
    let mut thing = vec![1, 2, 3];
    let first = &thing[0];
    thing.push(4);
    println!("the first element in thing is: {first}");
    */

    // iterate over vector elements
    let x = vec![1, 2, 3];
    for i in &x {
        println!("{i}");
    }

    // iterate over mutable vector and mutate it
    // note the deref of i in order to get to the value of i. the vec needs to be referenced and so does the element being mutated.
    let mut z = vec![1, 2, 3];
    println!("starter z: {z:?}");
    for i in &mut z {
        *i *= 2;
    }

    println!("mutated z: {z:?}");

    // vectors can only hold data of the same time. but we can use enums to sidestep this little limitation

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        FLoat(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int((42)),
        SpreadsheetCell::FLoat((100.0)),
        SpreadsheetCell::Text((String::from("hello from a cell"))),
    ];

    for r in &row {
        println!("row: {r:?}");
    }
}
