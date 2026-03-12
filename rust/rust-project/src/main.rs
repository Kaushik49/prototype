// fn main() {
//     let ans: bool = is_even(12);
//     println!("{}", ans);
// }

// // do not write camel case. Also i32 means , signed integer of 32 bits that means its should be - 2^31-1<n < 2^31-1 
// // if you want positive numbers use than you write u32 
// fn is_even(num:i32) -> bool {
//     if num % 2 == 0 {
//         return true; 
//     }
//     else{
//         return false;
//     }
// }

// writing the fibonacci algorithm 
// fn main () {
//     let x : u32 = 10; 
//     println!("{}",fib(x));
// }

// fn fib(num: u32) -> u32 {
//     let mut first =  0 ; 
//     let mut second = 1; 
//     // is there else if in rust ???
//     if(num==0){
//         return first;
//     }
//     else if(num ==1){
//         return 1;
//     }
//     // writing the login this is how you actually write the loop in rust 
//     for i in 1..num -2 {
//         let temp = second; 
//         second = second + first; 
//         first = temp
//     }
//     return second;
// }


// string passing
// fn main(){
//     let my_string = String::from("Hello world!");
//     let length = get_string_length(&my_string);// this i think strictly passes the memory , its ampersent represent memory
//     println!("The number of character in the string is :{}",length);
// }

// fn get_string_length(s:&str) -> usize { //usize represents the unsigned size which  is positive and can be of the huge bits
//     s.chars().count() // implicit return , implicitly returned from the function like using return and not semicolon
// // returns the character and the count returns the total character.
// }


// struct in rust 

// struct User {
//     active: bool, 
//     username: String,
//     email: String, 
//     sign_in_count: u64
// }

// fn main () {

//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"), 
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,

//     };
//     println!("User 1 username : {:?}", user1.username); // : 

// }

// implementing function in struct 

// struct Rect {
//     length: u32, 
//     breadth: u32,
// }




// impl Rect {// &self variable is the current struct , that means it takes the variables from the struct 
//     fn area(&self) -> u32 {
//         self.length * self.breadth
//     }
//     fn perimeter(&self, num:u32) -> u32 {
//         2 * (self.length+self.breadth)
//     }
//     fn debug() -> i32 {
//         return 1;
//     }
// }


// fn main () {
//     let rect = Rect {
//         length: 10, 
//         breadth: 30,
//     };
//     println!("area is {}", rect.area());
//     println!("area is {}", rect.perimeter(1));
//     // println!("area is {}", rect.debug());// you cannot do this since it does not belong to the struct, &self has not been passed
//      println!("area is {}", Rect::debug());// this is similar to static function in the javascript class
// }


// enums


// enum Direction {
//     North, 
//     East,
//     South, 
//     West,
// }

// fn main () {
//     let my_direction = Direction::North;
//     let new_direction = my_direction; // No error , because Direction is copy 
//     move_around(new_direction);
// }

// fn move_around(direction:Direction){ // the type is Direction which is an enum
//     // implements logic to move character around
// }

// another implementation of enums


// enum Shape {
//     Circle (f64), //Variant associated with data (radius)
//     Square(f64), // variant associated with data (side length)
//     Rectangle(f64,f64) // variant with associated with data (width, height)
// }

// fn calculate_area(shape: Shape) -> f64 {
//     // this is called pattern matching because you are matching the 
//     // calculate the area of the shape
//     let area = match shape {
//         Shape::Rectangle(a,b ) => a*b,
//         Shape::Circle(r) => 3.14 * r * r,
//         Shape::Square(l) => l *l,
//     }; 
//     return area;
// }

// fn main() {
//     // create instances of different enum shape
//     let circle = Shape::Circle(5.0); 
//     println!("{}",calculate_area(circle));
//     let square = Shape::Rectangle(3.0,6.0);
//     calculate_area(square);
//     let rectangle = Shape::Square(4.0);
//     calculate_area(rectangle);
    
// }

//Options result 

// lets you return null or none value from the function 


// fn find_first_a(s:String) -> Option<i32>  { // it gives option to return a null value it return option of type i32
//     for (index,char) in s.chars().enumerate() { // the iterates over the character and returns the index over the characters
//         if char =='a' {
//             return Some(index as i32);// Some is an enum value of the Option
//         }
//     }
//     return None;// none is enum value of the option
// } 
// // pub enum Option<T>

// fn main (){
//     let index = find_first_a(String::from("preet"));
//     match index {
//         Option::Some(value) => println!("index is {}", value), 
//         Option::None => println!("a not found")
//     }

//     // if (index ==None) {
//     //     println!("a not found"); 
//     // }
//     // else {
//     //     println!("index is {}", index)
//     // }
// }


// results 
// function does not return just return number , it returns of type result
// first variant is the ok variant and the second is the error variant during the execution of the file


// use std::fs::read_to_string;

// fn main (){
//     // the return type of this is return
//     let greeting_file_result: Result<String, std::io::Error> = read_to_string("hello.txt"); // i am not returning a string but a result

//     match greeting_file_result {// pattern matching on the enum Result
//         Ok(file_content)=>{
//             println!("file read successfully:{:?}", file_content); 

//         }, 
//         Err(error) =>{
//             println!("Failed to read file: {:?}", error)
//         }
//     }

// }
// fn read_file_from_hello(file_path:String)-> Result<String, String>{
//     let result = read_to_string("hello.txt"); // result

//     match result {
//         Ok(data)=> Ok(data), // yo actually returing the result variant of the data
//         Err(err) => Err(String::from("-1")), // rather than return hard coded strin you return the result vairant
//     }
// }


// package management 
// you can use cargo install cargo_package_name

// you can bring crate into the rust library

// some popular libraries of cryptography, chrono, or 
// cargo add chrono

// use chrono::{Local, Utc}; 
// fn main(){
//     // get the current data and time in UTC
//     let now = Utc::now(); // get the static method from utc class
//     println!("Current data and time in UTC : {}", now);

//     // formant the data and time
//     let formatted = now.format("%Y-%m-%d %H:%M:%S");
//     println!("formatted date and time :{}", formatted); 

//     //get the local time

//     let local = Local::now(); 
//     println!("Current date and time in local :{}",local)
// }

// Memory management, Heap and stack 
// stack is last in first out , stack one on top of the other
// dynamically allocating memory ,statically allocating memory, 
//dynamically allocating means asking or giving back memory
// Heap is first in first out 
// fn main(){
//     let a:i32 = 1; //32 bit takes // reserving 32 bits 
//     let b:i32 = 2; // takes 32 bit 
//     let sum: i32 = find_sum(a,b);// takes 32 bit total : 96 bit of memory
//     println!("sum is {}", sum); // after the function is executed the memory is freed
// }


// when another function is created , another stack frame is created
// when the function is called , the stack frame gets popped up 

// fn find_sum(a:i32,b:i32) -> i32{
//     let ans: i32 = a+b; 
//     return ans;

// }
// stack is where you store static data, or where you know size of the variable at compile time
// but the amount space that string takes it changes as time goes by
// the main function is pushed to stack frame


// when do we store data on the heap 

/*
the memory is dynamically allocated in runtime in the heap 
stack is allocated at compile time
much larger in size
slower due to dynamic allocation 
use for dynamic and large data structure
// used for vec , hashmap , box, large arrays, structs that can't fit in stack
// stack is used for numbers, boolean, fixed size array, struct etc
*/

// fn main () {
//     // right now the variable is pushed to stack, but as it string becomes larger , 
//     // you cannot store it and it gets pushed in heap 
//     // what is stored in the stack , an address/reference to the specific pointer , pointed to specific string
//     let name: String = String::from("hello"); 
//     // stack holds the variable, the heap holds the content
//     println!("Name is {}", name);
// }



// fn main () { // default you cannot change the value of variable
//     let mut name: String = String::from("hello"); // to make it mutable
//     name.push_str("hello");
// }

// even if we know Hello can be stored in stack by 
//by checking it in compile time
//drangling can be high





// ownership , moving and borrowing and reference


// garbage collector, if the variable looses its existence after the 
// the program is executed, the garbage collector removes the that space from heap
// heap is first in and first out
// buffer overflow and double free error


// rust is developed to keep in mind that developers are dump so that they don't run to memory errors

// fn create_string() {
//     let s: String = String::from("Hello heap memory leak ");
//     //print the string
//     println!("{}",s);
// }

// fn main () {
//     //call the function
//     create_string();
// }
// does cleanup the heap after the function gets popped up from stack
// the answer is they do it through it through ownership
// the stack owns the heap, there is an ownership is owned by the stack
// whenever the owner goes out of scope , the memory is freed

// moving 

// fn create_string() {
//     // ownership only one owner at one time
//     let s1: String = String::from("Hello"); 
//     let s2: String = s1; // moving the variable
//     // print the string
//     println!("{}",s1);
//     println!("{}",s2) // this will show error because , now 
//     // the stack that points to  the originl content is a2 rather than a1
//     // 
// }

// fn main() {
//     create_string();
//     //create_string();
// }

// fn main() {
//     let s1 = String::from("harkirat"); 
//     // let s2 = s1
//     do_something(s1.clone());
//     // this shows error because now the ownership has been transferered to s2
//     println!("number is {}",s1);

    
// }

// fn do_something(s2: String){
//     println!("{}", s2)
// }


// fn main(){
//     let mut s1:String = String::from("Hello"); 
//     s1 = do_something(s1); // you are returning ownership to s1
//     println!("number is {}",s1);
// }

// fn do_something(s2:String) -> String {
//     println!("{}",s2); 
//     return s2;
// }
// harkirat => s1, ownership moved to s2, then again s1 becomes owner again

// browing and references
// if you don't want to transfer ownership you pass the reference
// this means there is a single ownership, you are borrowing the value
// fn create_string(){
//     let  s1: String = String::from("Hello");//s1 owns the value 
//     print_str(&s1);// you are borrowing the variable
//     println!("{}",s1);
// }
// fn print_str(s2:&String){
//     println!("{}",s2);// s2 does not own the value
// }

// fn main() {
//     // call the function 
//     create_string();
// }
// rust and solana have a lot of parralles, they enforce
// a lot of rules enforce so that the final code is optimal

// if you want to borrow and mutate the vlaue


// fn main() {
//     let mut s1 = String::from("harkirat"); 
//     do_something(&mut s1); // you are passing as mutable reference
//     println!("{}",s1);
// }
// fn do_something(s2:&mut String){
//     s2.push_str("singh");// you are chaning the value 
//     // with variable that has now ownership , to do so you need to mut 
//     // the owner variable
//     println!("{}",s2);
// }
fn main () {
    let mut s1 = String::from("harkirat"); 
    let s2 = &mut s1; // if you are changing one even one reference 
    // there is another immutable reference also
    // at any give time either you can just have one mutable reference or many immutable reference
    let s3 = &s1;
    println!("number is  {}, {}, {}",s1,s2,s3)
}
// you cannot have more than one mutable borrow