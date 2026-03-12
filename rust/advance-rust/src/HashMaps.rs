/*
Hashmaps 
stores a key value pair in rust 
insert 
get 
remove 
clear 


*/

// use std::collections:Hashmaps

// fn main() {
//     let mut users: HashMap<String,u32> = HashMap::new(); 
//     users.insert(k:String::from("harkirat"),v:22); //k is key and // v means: value
//     users.insert(k:String::from("raman",v:32)); 



//     let first_user_age: = user.get("harkirat");// should return 22 // but it return option enum
//     match first_user_age {
//         Some(age) => println!("age is {}", age), 
//         None => println!("User is not found in db")
//     }
// }

// use std::collections::HashMap;
// // you are using vector to creagte tuple with small bracket
// fn group_values_by_key(vec:Vec<(String,i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new(); 
//     for (key,value) in vec {// destrucuting the value
//         hm.insert(k:key,v:value);
//     }
//     return hm;
// }

// struct User {
//     name: String, 
//     age: i32,
// } // you can use this as generic type in vector as well
// but accessing the name and age will be different


// fn main () {
//     let input_vec = vec![(String::from("harkirat"), 22),(String::from("raman"),32)];
//     let hm = group_values_by_key(input_vec); 
    
//     println!("{:?}", hm);
// }