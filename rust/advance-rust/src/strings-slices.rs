/*

Strings vs Slices

The string Type is provided by Rusts standard library rahter than coded 
into hard core language is a growable , mutable owned UTF-8 encoded string type
when Rustaceans refer to "strings" in rust they might be rerferring to either the String
or the string slice &str type not just one of those types. Although this 
section is largely about string, both types are heavily in rust standard libaray
and both string adn string slices are UTF-8 encoded


slices let you reference a contiguous sequence of elements in collection 
rather than the whole collection. a slice is kind of reference , so it does not have 
ownership.

a string slice represent certain portion of that string
 things stored in stack
ptr -> to the heap element on the top 
len -> 11 
and capacity -> 11 



*/

// fn main () {
//     let mut name = String::from("harkirat"); 
//     name.push_str(" Singh"); 
//     println!("name is {}", name); 
//     name.replace_range(8..name.len(),""); 
//     println!("name is {}", name);
// }

// slice is if you want to view certain portion of the string


// fn main() {
//     let name = String::from("hello world"); 
//     let ans = first_word(name); 
//     println!("ans is {}", ans);
// }
// you dublicate the first sequence of character in different heap frame in heap 
// consuming twice the amount of memory
// also there are two points in stack that points to different memory
// if the original variable goes out of scope , the ans variable still points to the heap address
// that is problematic in such case, string slice views some portion of original string not to copy over
// fn first_word(str:String) -> String {
//     let mut ans = String::from(""); 
//     for i in str.chars() {
//         if i == ' '  {
//               break;  
//         }
//         ans.push_str(&i.to_string());
//     }
//     return ans;
// }
// given a string return the view of the first word

// if the main pointer goes out of scope , the string slicer 
// goes out of scope as well 

// fn main () {
//     let word = String::from("Hello World"); 
//     let word2 = &word[0..5]; 
//     // this is called string slicing
//     // there is reference that points to the original string 
//     //ownership rule: you can either have one mutable borrowing or many immutable borrowing
//     println!("{}",word2);
// }


// fn main() {
//     let word = String::from("hello world"); 
//     let word2 = find_first_word(&word);
//     println!("{}", word2);

// }

// fn find_first_word(word:&String) -> &str {
//     let mut index  =0 ; 
//     for (_,i) in word.chars().enumerate() {
//         if i == ' ' {
//             break; 
//         }
//         index = index +1;
//     }
//     return &word[0..index];
// }