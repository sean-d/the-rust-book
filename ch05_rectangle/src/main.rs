#[derive(Debug)] // printing
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation for the Rectangle struct
// we create an associated function and pass in a reference to the object
// this implements on...
// we then use the fields on that referenced object...

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // can name a method the same as a field. rust is smart enough to know which you are after
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.area() > other.area()
    }
    // create a square from the Rectangle struct using one of its associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*
    //let sq_area = area(30, 30);
//     let sq_area = area((30, 30));
//
//     println!("{}", sq_area);
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// refactor with tuples

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
    */

    // typical object based on struct
    let rect = Rectangle {
      width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    /*
    println!("the area of a rect is: {}", area(&rect));
    println!("rect is {:?}", rect);

    //dbg!(rect); // takes ownership...make sure to pass reference
    //println!("rect is {:?}", rect); // will fail due to above thievery
    dbg!(&rect);
    println!("rect is {:?}", rect); // will work
*/

    println!("rect area: {}", rect.area());
    println!("width for rect: {}", rect.width());
    println!("rect1 can hold rect2: {}", rect.can_hold(&rect2));
}


/*
refactor as a method

// passing in a ref to the rectangle and getting the area of it
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
*/
