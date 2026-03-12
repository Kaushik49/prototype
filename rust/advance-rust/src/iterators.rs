//iterators 
/*
the iterator pattern allows you to perform some taks
on the sequence of items in turn. An interator is reponsible for 
logic of iterating over each item and determining when the sequence 
has finished. When you use iterator you don't have to reimplemtnt 
the logic yourself 

in rust iterators are lazy , meaning they have no effet until you call 
methods that consume the iterator  to use it up.

let v1 = vec![1,2,3]; 
let v1_iter = v1.iter();
 */

//  fn main () {
//     let mut nums = vec![1,2,3]; 
//     // you are borrowing the values from the num using nums.iter()
//     // these are immutable borrowing
//     let iter = nums.iter_mut(); //returns of type iterator
// // for loop to iterate over the iterator

//     let first_number:Option<&mut i32> = iter.next()
//     let second_number:None = iter.next(); 

//     while let Some(val) = iter.next() {
//         println!("{}", val);
//     }// this another way to iterate in the vector
// // you are borrowing the values from the original vector
//     for value in iter {
//         *value = *value +1 
//         println!("Got: {value}")
//     }
//     println!("{:?}, v1");
// }

// //intoiter

// fn main () {
//     let nums = vec![1,2,3]; 
//     let iter = nums.into_iter() //takes the ownership of the collection 
//     // rather than borrowing like the other
//     for value in iter {
//         println!("{}", value)
//     }
// }

//iterators give you somthing extra 
/*

consuming adaptors and iterator adaptors

*/


// fn main () {
//     let v1 = vec![1,2,3];
//     let v1_iter = v1.iter(); 


//     let sum:i32 = v1_iter.sum(); // takes the ownership of self
//     // the sum function endup consuming iterator
//     // it takes self as input , if took &self as input , it would be referencing
//     // you can no longer use the iterator

//     println!("sum is {}", sum); 
//     // for i in v1_iter{} // cannot use the iterator anymore
//     println!("{:?}", v1); // cannot use iterator anymore
// }
/*

iterator adaptors are methods defined on the iterators 
trait that don't consume the iterator. Instead they produce 
different iterators by changing some aspect of the original iterator

map 
filter 

*/
// fn main () {
//     let v1: Vec<i32> = vec![1,2,3]; 
//     let iter = v1.iter();// you are defining iterator
//     let iter2 = iter.map(|x| x+1); // returns another iterator
//     // it is an adaptor that returns another adapter
//     for x in iter2 {
//         println!("{}", x)
//     } 

// }

// fn main () {
//     let v1: Vec<i32> = vec![1,2,3]; 
//     let iter = v1.iter();// you are defining iterator
//     let iter2 = iter.filter(|x| *x % 2 ==0); // returns another iterator
//     // it is an adaptor that returns another adapter
//     for x in iter2 {
//         println!("{}", x)
//     } 

// }

// convert vector into a new iterator

// fn main () {
//     let v1 = vec![1,2,3,4]; 
//     let iter = v1.iter().filter(|x| *x %2 ==1).map(|x| x*2);
    
//     let v2: Vec<i32> = iter.collect();// collects all the values from iterator and puts it into the vector
//     println!("{:?}",v2);
// }



//iterators in HashMaps
// use std::collections::HashMap
// fn main () {
//     //create a hashMap and populate it with some key value pair
//     let mut scores = HashMap::new(); 
//     scores.insert("alice",50); 
//     scores.insert("Bob", 40); 
//     scores.insert("Charlie", 30); 

//     //example 1 : Iterating over references to key-value pairs
//     println!("Iterating over key-value pair:"); 
//     for (key,value) in scores.iter(){
//         println!("{}:{}", key, value);
//     }

//     //example 2 iterating over mutable rerference to key value pair
//     println!("\n Iterating over mutable key-value pair"); 
//     for (key,value) in scores.iter_mut() {
//         *value +=10; // increment each score by 10 
//         println!("{}:{}", key, value);
//     }

// }