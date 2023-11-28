use std::{
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};

struct RustConcurrency;

impl RustConcurrency {
    fn using_join_handlers() {
        let mut threads = vec![];

        for i in 0..10 {
            let handle = spawn(move || {
                println!("Printing counter {} from new thread", i);
            });
            threads.push(handle);
        }

        // join() awaits for each thread to finish before continuing
        for handle in threads {
            handle.join().unwrap()
        }

        // this is going to be printed after the handlers closures
        println!("After thread handlers finished printing.");
    }

    fn using_channels_with_threads() {
        let (tx, rx) = channel();

        for i in 0..10 {
            // clone a sender to send to other threads
            let tx_clone = tx.clone();

            spawn(move || tx_clone.send(format!("Message sent No. {}", i)));
        }

        for message in rx {
            println!("{}", message);
        }
    }
}

fn main() {
    RustConcurrency::using_join_handlers();
    sleep(Duration::from_millis(1000));
    RustConcurrency::using_channels_with_threads();
}
