use std::fmt::Display; 
// the goal is to understand the given syntax
fn longest_with_an_announcement<'a, T> (// generic lifetime annotation
    x: &'a str, 
    y: &'a str, 
    ann:T, 
) -> &'a str
where 
    T: Display, // just doing the types as Display
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            x
        }else {
            y
        }
    }
    /*
    
    
    
    
    
    
     */

fn main () {
    let ans;
    let str1 = String::from("small"); 

    // you are making str2 go out of scope
    {
        let str2 = String::from("longer"); 
        // dangling pointer
        ans = longest(a:&str1, b:&str2);

    }
    println("{}", ans);
}
// the code doesn't work because , b is borrowing the
//value and as soon as str2 goes out of scope , b is a drangling pointer
// the a is valid for 8 lines, b is valid for 4 lines so str should be valid for 3 lines, 
// the lifespan of the answer will be intersection of lifespan of a and b 
fn longest(a:&str, b:&str) -> &str {
    if a.len() > b.len() {
        return a
    }else{
        return b;
    }
}
// introducing generic lifetime 
// whatever is the intersection of both the references is what is lifetime
fn longest<'a>(first:&'a str,second:&'a str) -> &'a str{
    if first.len() > second.len() {
        return first
    }else{
        return second;
    }
}