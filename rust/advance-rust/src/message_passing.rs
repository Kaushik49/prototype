/*

how do you pass message from one thread to another 
how do you make two thread to communicate to each other


you also need to approch thi with safe concorruancy 

Do not communicate by sharing memory 
share memory by communicating


channels library let us transfer data from one thread to another



channel has two halfes , transmitter and receiver . the transmitter half
is the upstream location where you put rubber ducks 




*/


use std::{
    sync::mpsc,// multiple producer single consumer
    thread::{self, spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();// use the channel to create transmitter and reciever

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 10000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();// unwrap , no problem with thread panicking
        });
    }
    drop(tx);// the reciever will keep waiting for transmitter unitl it is dropped

    let mut final_sum: u64 = 0;
    // let reciever = rx.recv.unwrap();
    // for multiple tranmitter there is only one consumer
    for val in rx {
        println!("recv value from thread");
        final_sum = final_sum + val;
    }
    println!("{}", final_sum);
}