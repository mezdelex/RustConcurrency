use std::{
    cell::RefCell,
    rc::Rc,
    sync::{mpsc::channel, Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

struct RustOwnership;

impl RustOwnership {
    fn another_owner(message: Rc<RefCell<String>>) {
        // https://github.com/rust-lang/rust/issues/39232 <- careful with this issue;
        // autoimports may cause unwanted behavior if std::borrow::BorrowMut gets imported while using borrow_mut().
        // we want RefCell.borrow_mut(), not std::borrow::BorrowMut
        let mut mutable_message = message.borrow_mut();
        *mutable_message += "!!!";
    }

    fn allow_mutability_of_immutable_elements_by_different_owners() {
        let message = Rc::new(RefCell::new(String::from("I shall mutate this")));
        let message_clone = Rc::clone(&message);

        Self::another_owner(message_clone);

        println!("{}", message.borrow());
    }
}

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

    fn concurrent_thread_safety_with_arc() {
        // Arc -> Atomically Reference Counted
        let counter = Arc::new(Mutex::new(0));

        for i in 0..10 {
            let counter_clone = Arc::clone(&counter);

            spawn(move || {
                let mut counter_guard = counter_clone.lock().unwrap();
                *counter_guard = i;

                println!("Counter: {}", counter_guard);
            });
        }
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
    RustOwnership::allow_mutability_of_immutable_elements_by_different_owners();
    sleep(Duration::from_millis(1000));
    RustConcurrency::using_join_handlers();
    sleep(Duration::from_millis(1000));
    RustConcurrency::concurrent_thread_safety_with_arc();
    sleep(Duration::from_millis(1000));
    RustConcurrency::using_channels_with_threads();
}
