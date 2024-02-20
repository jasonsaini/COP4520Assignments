use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

const TARGET: usize = 100_000_000;
const NUM_THREADS: usize = 8;

fn is_prime(n: usize, memo: &Arc<Mutex<HashMap<usize, bool>>>) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut memo = memo.lock().unwrap();

    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let upper_lim = (n as f64).sqrt() as usize;
    for i in (3..=upper_lim).step_by(2) {
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
        let start = i * chunk_size + 1;
        let end = if i == NUM_THREADS - 1 { TARGET } else { (i + 1) * chunk_size };
        let thread_sums = Arc::clone(&thread_sums);
        let memo = Arc::clone(&memo);

        let thread = thread::spawn(move || {
            let mut local_sum = 0;
            for num in (start..=end).step_by(2) { // Only check odd numbers
                if is_prime(num, &memo) {
                    local_sum += num;
                }
            }
            let mut total = thread_sums.lock().unwrap();
            total[i] += local_sum;
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let exec_time = start_time.elapsed();
    let total: usize = thread_sums.lock().unwrap().iter().sum();
    let mut file = File::create("output.txt").expect("Unable to create file");
    writeln!(file, "Total: {:?}", total).expect("Unable to write to file");
    writeln!(file, "Execution Time: {} seconds", exec_time.as_secs()).expect("Unable to write to file");
}
