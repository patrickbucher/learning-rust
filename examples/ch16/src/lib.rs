use std::sync::mpsc;
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
