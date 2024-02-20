use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const TARGET: usize = 100_000_000;
const NUM_THREADS: usize = 8;

fn is_prime(n: usize, memo: &Arc<Mutex<HashMap<usize, bool>>>) -> bool {
    let mut memo = memo.lock().unwrap();

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    if n < 2 {
        return false;
    }

    let upper_lim = (n as f64).sqrt() as usize;
    for i in 2..=upper_lim {
        if n % i == 0 {
            memo.insert(n, false);
            return false;
        }
    }

    memo.insert(n, true);
    true
}

fn main() {
    let start_time = Instant::now();

    let thread_sums = Arc::new(Mutex::new(vec![0; NUM_THREADS]));
    let memo = Arc::new(Mutex::new(HashMap::new()));

    let mut threads = Vec::new();
    let chunk_size = TARGET / NUM_THREADS;

    for i in 0..NUM_THREADS {
        let thread_sums = Arc::clone(&thread_sums);
        let memo = Arc::clone(&memo);
        let start = i * chunk_size;
        let end = if i == NUM_THREADS - 1 { TARGET } else { start + chunk_size };

        let thread = thread::spawn(move || {
            let mut sum = 0;
            for val in start..end {
                if is_prime(val, &memo) {
                    sum += val;
                }
            }

            let mut total = thread_sums.lock().unwrap();
            total[i] += sum;
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let exec_time = start_time.elapsed();
    let total: usize = thread_sums.lock().unwrap().iter().sum();

    // Write output to a file
    let mut file = File::create("output.txt").expect("Unable to create file");
    writeln!(file, "Total: {:?}", total).expect("Unable to write to file");
    writeln!(file, "Execution Time: {} seconds", exec_time.as_secs()).expect("Unable to write to file");
}
