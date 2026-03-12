struct User<'a> {
    //using references inside struct
    name: &'a str // if the lifetype of nam is 'a then lifetime of struct is also 'a
    // this means , if name goes out of scope User also goes out of scope
}
// if name goes out of scope, the user also goes out of scope
fn main () {
    let user;
    {
        let name = String::from("harkirat"); 
        let user = User {name: &name};
    }
    println!("{}", user.name);// this does not compile because , the lifetime of name is finihed
    //this causes dangling error
}


