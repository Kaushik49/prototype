/*
if you are writing code that generates more code , 
that i called meta programming , it means it spits out more more code
a macros is a keyword given , where it has been compressed in to a small keyword
but under the hood it has more code inside the keyword
it enables the generation of code at compile time

Declarative macro : these are the most common type of macros in rust
they genereate code during the compile time


*/

// fn main () {
// println!("{}")
// }

// these are declarative macro 

macro_rules! vec {
    () => (
        // you ware using library from the crate
        // you are using the function vec from the creaete
        // you are using Vec function from vec
        // and you are using new from the Vec
        $crate::vec::Vec::new()

    ); 
    // if there is any expression passed 
    // regex or regular expression is sequence of regular text used to mach the pattern
    // it can be anything , so regex helps to match patter, n means the number of elements
    ($elem:exprl $n:expr) => (
        $crate::vec::from_elem($elem,$n)
    );
}

macro_rules! print {
    () => { // you are using print library from the crate
        $crate::print("\n")
    }
    ($($arg:tt)*) => {{ // $ represents regex for matter matching the input string
        // * means it includes all the arguments passsed 
        $crate::io::_print($crate::format_args_nl!($($arg))*)
    }}
}



macro_rules!say_hello {
    () => {
        println!("Hello, world!")
    }
}
fn main () {
    say_hello();// expands to println!("Hello world!")
}

/*
Procedural macros are complex macros that allows you to define 
custom behaviour for code generation through Rust code itself.They run on 
Rust abstract syntax tree 


*/

use std::fnt::{write,Debug,Formatter};
// a struct common implementation will be debug 
// that is why there is debug macro , mind it is different from debug trait
#[derive(Debug)]

struct User {
    username: String, 
    password: String, 
    age:u32
}
// impl Debug for User{} this is been handled under the hood in macro
// impl Display for User{
// you are implementing the user trait in User
// impl Debug for User {
//     fn fmt(&self,f:&mut Formatter<'_>) -> std::fmt::Result{
//         // this formatter write this line of code
//         write!(f, "username is {}", self.username)
//     }
// }
// }

impl Display for User {
    fn fmt (&self,f:&mut std::fmt::formatter) -> std::fmt::Result{
        write!(f, "this is the user struct with age{}", self.age)
    }
}
fn main() {
    let u:User = User {
        username:String::from("harkirat"),
        password: String::from("harkirat"),
        age:32
    };
    // print!("{}", u);// doesn't display trait
    print!("{:?}",u) // this is debug trait
}

// you want to display trait in a slightly pretty fashion 
// you want to debug trait 

/*

custom like macro

attribute like macro 
the actual macro that exist in library
#[route["GET"]]

the other one is function like macro 


*/