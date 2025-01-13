// generic types in a struct...
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

/*
 we declare <T, U> after impl so rust can identify that the types in the angled brackets in Point are generic and not concrete.
*/
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }

    // create one instance of point. create a second.
    // call this from the first instance calling the second instance and return to a third instance.
    // third instance will have x of the first instance and y of the second.
    // so mixup<A,B> is so it can determine the types from the accompanying type that is binding to those same generic types....Point<A,B> in this case.
    fn mixup<A, B>(self, other: Point<A, B>) -> Point<T, B> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// specify constraint on generic type...implement a method only for the Point<f64, f64) type
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

// generic stuff in enums work just fine. look at Result
// enum Result<T, E> {
//  Ok(T),
//  Err(E),
//}

fn main() {
    // get a list of ints. set the first element as largest.
    // iterate over list comparing each element to the previous
    // whichever is largest becomes/remains the largest.
    // when done, we have the largest and print that out.
    let number_list = vec![11, 12, 100, 13, 14];
    let number_list2 = vec![1, 300, 25, 566, 2513245, 8];
    let char_list = vec!['y', 'Z', 'a', 'q', '1', 'z'];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("the largest number is: {largest}");

    // now if we wish to do this with a second list of numbers, we have to duplicate it all...so let's make a function to remove duplication

    let largest_number = largest_num_in_list(&number_list);
    println!("largest: {largest_number}");

    let another_largest = largest_num_in_list(&number_list2);
    println!("largest2: {another_largest}");

    let largest_char = largest_char_in_list(&char_list);
    println!("largest char:{largest_char}");

    let generic_largest = the_largest(&char_list);
    println!("generic largest with chars: {generic_largest}");

    let integer = Point { x: 5, y: 10 };
    let floaty = Point { x: 1.0, y: 2.0 };
    let two_types = Point { x: 1, y: 2.0 };

    println!("int: {integer:?}, float: {floaty:?}, two types: {two_types:?}");
    println!("distance: {}", floaty.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// function to remove largest element in list duplication
fn largest_num_in_list(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char_in_list(list: &[char]) -> &char {
    let mut largest = &list[0];

    for ch in list {
        if ch > largest {
            largest = ch
        }
    }
    largest
}

/*
we have a list of i32 and another list of chars. we wish to find the largest in each...as it stands, we have to create two functions, one for each type.

with generics, we can reduce this to one function.

 */

// this function is generic over some type T.
// it has one param named list that is a slice value of type T.
// it returns a refernce to a value of the same type T.
// note: PartialOrd has to be usd so only types that can be ordered will be allowed
fn the_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
