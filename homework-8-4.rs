use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{RngExt};

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();

    let rx_shared = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        });
        producer_handles.push(handle);
    }

    let mut consumer_handles = vec![];
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx_shared);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        consumer_handles.push(handle);
    }

    for handle in producer_handles {
        handle.join().unwrap();
    }

    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::rng(); 
    
    for _ in 0..item_count {
        let val = rng.random_range(1..100); 
        
        println!("Producer {id:?} sent: {val:?}");
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {id:?} done.");
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let received = {
            let lock = rx.lock().unwrap();
            lock.recv().ok()
        };

        match received {
            Some(TERMINATION_SIGNAL) => {
                println!("Consumer {id:?} done. Exiting.");
                break;
            }
            Some(val) => {
                println!("Consumer {id:?}: {val:?}");
                thread::sleep(Duration::from_millis(150));
            }
            None => break, // channel disconnected
        }
    }
}
