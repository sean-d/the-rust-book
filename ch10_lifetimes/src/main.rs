/*
dangling references for everyone!!


fn main() {
    let dang = get_dangle();
    println!("{dang}");

    let r;

    {
        let x = 5;
        r = &x;
    }
    println!("{r}");
}

fn get_dangle() -> &u8 {
    let x: u8 = 5;
    return &x;
}
*/

fn main() {
    // we give main_string the lifetime of the main function
    let main_string = String::from("main-string");

    {
        // these have the lifetime of this scoped block...so a shorter lifetime
        //let string1 = String::from("abcd");
        let string2 = String::from("xyzasdfasdfasdf");

        let result = longest(main_string.as_str(), string2.as_str());
        println!("the longest string is: {}", result);
    }
}

/*
    longest gets a reference to x and y. we dont know what they are, we just know they are coming.

    it returns a reference to either x or y, and we have no idea what the lifetime of that reference is.

    based on the if statement, we are returning either x or y.
    x could have a different lifetime than y.

    we also have no idea what the lifetime of x or y could be. this function could be called from many different places and thus different lifetimes.

    from the perspective of the compiler/borrow checker, it has no idea how to handle this ambiguity.

    because of that, we get an error with the following code: missing lifetime specifier

    ```
    fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
    ```

*/

/*
we fix the longest problems with the use of generic lifetime annotations (or lifetimes).

to specify litimes:
&i32: a reference
&'a i32: a reference with an explicit lifetime
&'a mut i32: a mutable reference with an explicit lifetime

we add <'a> to longest to declare that we have a lifetime relationship. all items to be part of that relationship will have the same lifetime.

note: we can name it whatever we want. rust convention is a-> lowercase.

so by giving the function, x, y, and the returned reference membership into the lifetime relationship, it is saying:
the returned reference, which is assigned to result in main, is gauranteed to live at least as long as the shortest living item in the relationship...
so if x has a lifetime of one itereation, and y has a lifetime that ends when the program exists, the returned reference will have the lifetime of x.

note: the returned value's lifetime must be bound to one of the passed in parameters and not something created in the function; dangling references are not allowed.


*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
lifetimes of variables being passed in are called input lifetimes
lifetimes of returned values are called output lifetimes

lifetime elision rules: (the first rule applies to the input lifetimes, the other two apply to the output lifetimes)

note: if the compiler gets to the end of the three rules and there are still references where lifetimes are not reconsiliable, the compiler gunna be angry. these rules apply to fn and impl blocks.

1. a lifetime parameter assigned is assigned to each parameter is always a reference. in other words, a function with one param gets one lifetime param

```
fn functionthing<'a>(x: &'a i32);
```

a function with two params gets two separate lifetime params:

```fn anotherfunction<'a, 'b>(x: &'a i32, y: &'b i32);
```

and so on...

2. if there is exactly one input lifetime param, that lifetime is assigned to all output lifetime params:
```
fn stuff<'a>(x: &'a i32) -> &'a i32{}
```

3. if there are multiple input lifetime params, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime params. this rule makes methods much nier to read and write...fewer symbols.



*/
fn first_word(a: &str) -> &str {}

/*
the first rule gets applied. it states that each param gets its own lifetime.
*/
fn first_word<'a>(a: &'a str) -> &str {}

/*
the second rule applies because there is exactly one lifetime. the lifetime of the one input param gets assigned to the output lifetime.
*/

fn first_word<'a>(a: &'a str) -> &'a str {}

/*
all references in this function sig have lifetimes now...the compiler can continue...
*/

fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &str {}

/*
the first rule gets applied so each param gets its own lifetime.
the second rule does not apply since there is more than one param
the third rule does apply either as longest is a function rather than a method...none of the params are self.

after working through all the rules, the compiler will error out...why?
we have no way of telling what the lifetime of the returned result will be.
*/

/*
the following:

the struct has a single field, part, that holds a string slice, which is a reference.

we declare a generic lifetime parameter in <> after the name of the struct so we can use this lifetime parameter in the body of the struct definition.

this annotation means that any instance of ImportantExcerpt cannot outlive the reference it holds in its part field.

lots of words...

the important_except function creates an instance of the struct and that struct holds a reference to the first sentence of the string owned by novel.

the data in novel exists prior to the instantiation of the struct instance. additionally, novel does not go out of scope until after the struct does...the reference in the struct is valid because of this.


yep, confusing.
*/
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn important_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);
}

/*
another look at this confusing matter

thare are two input lifetimes (&self, announcement: &str).
rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
then, because one of the params is &self, the return type gets the lifetime of &self. the third rule in action.
all lifetimes have been accounted form.
*/

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("attention please: {announcement}");
        self.part
    }
}

/*
static lifetime

'static denotes that the affected reference CAN live for the entire duration of the program.

note: all string literals have the 'static' lifetime, which is annotated as such:

let s: &'static str = "I'm in your pool....yaaaay";

the text of this string is stored directly in the program's binary, which is always available.

because of this, the lifetime of all string literals is 'static.

...full circle.

another important note:
the compiler will suggest using the 'static lifetime in error messages.

before doing so, decide if the reference you have actually lives through the duration of your program or not. not all things need a lifetime from beginning->end of an excution.

most of the time when the error message suggests 'static, it's attempting to resolve a dangling reference or a mismatch of the available lifetimes.

*/

/*

let's tie it all together for one very confusing and not-a-chance-at-getting-it-wrong example

same longest function as we have been using...

now it has an extra param, announcement of generic type T.

this param can be filled by any type that implements the Display trait as specified by the where clause.

the extra param will be printed using {} which is why the Display trait bound is necessary.

because lifetimes are of type generic, they go in the same brackets as generic lifetime paramenters... <'a, T> in this case.

ffs...
 */

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {announcement}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
