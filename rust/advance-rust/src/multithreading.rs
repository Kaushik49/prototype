


/*

Using threads to Run code Simultaneously 

Using message Passing to transfer data between threads




*/

use std:: thread; 

use std::time::Duration; 

fn main() {
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {i} from spawned the thread"); 
            thread::sleep (Duration::from_millis(1));
        }
    });
// both the threads will run parallelly 
// both will be displayed 
    for i in 1..5 {
        println!("hi number {i} from the main thread!"); 
        thread::sleep(Duration::from_millis(1));
    }
}


fn main () {
    let sum = 0; 
    let hadle = thread::spawn(||{// closure is very similar to to function 
        //spawning the first thread
        for i in 1..10 {
            println!("hi number is {i} from spwaned thread");
            thread::sleep(Duration::from_millis(1))
        }
    });
    // running the thread to finish before the running the iteration on the main thread
    handle.join().unwrap(); 
    for i in 1..5{
        println!("hi nubmer {i} from the main thread");
        thread::sleep(Duration::from_millis(1))
    }
}

fn main () {
    let x = 1;
    {
        let v = vec![1,2,3]; 
        thread::spawn(move || { // since v goes out of scope you need to move the ownership to closure
            println!("{:?}", v)
        })
    }
}