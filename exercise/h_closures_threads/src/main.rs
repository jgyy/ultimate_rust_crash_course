use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let (tx, rx) = channel::unbounded();
    let rx1 = rx.clone();
    let rx2 = rx.clone();

    let handle1 = thread::spawn(move || {
        for msg in rx1.try_iter() {
            println!("Child thread 1 received: {}", msg);
            pause_ms(100); // Simulate work
        }
    });

    let handle2 = thread::spawn(move || {
        for msg in rx2.try_iter() {
            println!("Child thread 2 received: {}", msg);
            pause_ms(100);
        }
    });

    let values = vec![1, 2, 3, 4, 5];
    for value in values {
        println!("Main thread: Sending {}", value);
        tx.send(value).unwrap();
        pause_ms(50);
    }

    drop(tx);

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread: Exiting.");
}
