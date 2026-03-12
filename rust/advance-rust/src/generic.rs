fn main () {
    let bigger = largest(1,2); 
    let bigger_char = largest('a','b');
    println!("{}", bigger); 
    println!("{}",bigger_char);

    // th cmp package gives partial order 
    fn largest<T: std::cmp::PartialOrd>(a:T, b:T) -> T {
        if a>b {
            a
        }
        else {
            b
        }
    }
}