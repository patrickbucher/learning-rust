use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn beavis_and_butthead(n: usize, sleep_millis: u64) {
    let beavis = thread::spawn(move || {
        for _ in 0..n {
            println!("I am the almighty Cornholio!");
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    });

    let butthead = thread::spawn(move || {
        for _ in 0..n {
            println!("This sucks, hehehe.");
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    });

    beavis.join().unwrap();
    butthead.join().unwrap();

    println!("OK, Beavis and Butt-Head...");
}

pub fn pythagoras(catheti: Vec<(f32, f32)>) -> Vec<(f32, f32, f32)> {
    let (tx_req, rx_req) = mpsc::channel();
    let (tx_res, rx_res) = mpsc::channel();

    let producer = thread::spawn(move || {
        for ab in catheti {
            let (a, b): (f32, f32) = ab;
            tx_req.send((a, b)).unwrap();
        }
    });

    let worker = thread::spawn(move || {
        for ab in rx_req {
            let (a, b): (f32, f32) = ab;
            let c = (a.powf(2.0) + b.powf(2.0)).sqrt();
            tx_res.send((a, b, c)).unwrap();
        }
    });

    let mut results: Vec<(f32, f32, f32)> = Vec::new();
    for result in rx_res {
        results.push(result);
    }

    producer.join().unwrap();
    worker.join().unwrap();

    results
}

pub fn waste_money(balance: f32, expenses: HashMap<u64, f32>) {
    let account = Arc::new(Mutex::new(balance));
    let mut handles = Vec::new();
    for (ms, amount) in expenses {
        println!("spend {amount:.2} every {ms}ms");
        let account = Arc::clone(&account);
        let handle = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(ms));
            let mut b = account.lock().unwrap();
            if amount > *b {
                println!("bust");
                return;
            }
            *b -= amount;
            println!("spent {amount:.2} after {ms}ms, new balance: {b:.2}");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
