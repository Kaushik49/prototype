/*
a trait defines the functionality a particular type
has can can share with other types. We can use trait to define
shared behaviour in an abstract way. We can use trait bounds to specify 
the generic type can be any type that has certain behaviour


*/


// defining  a  trait 
pub trait Summery {
    fn summerize(&self) -> String {
        return String::from("hi there")
    }

}

struct Fix;
impl Summery for fix {}
struct User {
    name: String, 
    age: u32, 
}

// you are implementing on trait on the struct
impl Summery for User {
    fn summerize(&self) -> String {
        return format! ("User {} is {} years old ", self.name, self.age);
    }
}

fn main () {
    let user = User {
        name: String::from("Harkirat"), 
        age:21,
    };
    // you are calling the trait on the user 
    println!("{}", user.summerize());
    notify(User); 
    notify(1); // this errors out because it doesn not implement Summery trait
}
// implementing trait has parameters

// trait syntac works for straight forward cases but is acutally syntax sugar for sugar for a 
// longer form known as trait bound, it looks like
fn notify(u: impl Summery) {
    println!("{}", u.username());
}
//you are defiining the generics as summery
pub fn notify <T: Summery + Fix > (item:T) {
    println!("breaking news{}", item.summerize())
}