use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const TARGET: usize = 100_000_000;
const NUM_THREADS: usize = 8;
const BATCH_SIZE: usize = 10000; // New constant for batch allocation

fn is_prime(n: usize, memo: &Arc<Mutex<HashMap<usize, bool>>>) -> bool {
    // Prime checking remains largely unchanged
    // Consider limiting the size of memoization
}

fn main() {
    let start_time = Instant::now();

    let count = Arc::new(Mutex::new(2));
    let thread_sums = Arc::new(Mutex::new(vec![0; NUM_THREADS]));
    let memo = Arc::new(Mutex::new(HashMap::new()));

    // Create a file for output
    let mut file = File::create("output.txt").expect("Unable to create file");

    let threads: Vec<_> = (0..NUM_THREADS)
        .map(|i| {
            let count = Arc::clone(&count);
            let thread_sums = Arc::clone(&thread_sums);
            let memo = Arc::clone(&memo);

            thread::spawn(move || {
                loop {
                    // Allocate a batch of numbers to reduce lock contention
                    let mut batch = Vec::new();
                    {
                        let mut num = count.lock().unwrap();
                        for _ in 0..BATCH_SIZE {
                            if *num > TARGET {
                                break;
                            }
                            batch.push(*num);
                            *num += 1;
                        }
                    }

                    // Break if the batch is empty (TARGET reached)
                    if batch.is_empty() {
                        break;
                    }

                    for val in batch {
                        if is_prime(val, &memo) {
                            let mut total = thread_sums.lock().unwrap();
                            total[i] += val;
                        }

                        if val % 10000 == 0 {
                            writeln!(file, "{}", val).expect("Unable to write to file");
                        }
                    }
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    let exec_time = start_time.elapsed();
    let total: usize = thread_sums.lock().unwrap().iter().sum();
    writeln!(file, "Total: {:?}", total).expect("Unable to write to file");
    writeln!(file, "Execution Time: {} seconds", exec_time.as_secs()).expect("Unable to write to file");
}
