use std::thread;

struct RustConcurrency;

impl RustConcurrency {
    fn using_join_handlers() {
        let mut threads = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || {
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
}

fn main() {
    RustConcurrency::using_join_handlers();
}
