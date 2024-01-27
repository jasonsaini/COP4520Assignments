use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::Arc;

const TARGET: usize = 100_000_000;
const NUM_THREADS: usize = 8;

fn is_prime(n:usize, memo:&Arc<Mutex<HashMap<usize,bool>>>) -> bool {
    let mut memo = memo.lock().unwrap();

    if let Some(&result) = memo.get(&n){
        return result;
    }

    let upper_lim = (n as f64).sqrt() as usize;

    for i in 2..=upper_lim{
       if n % i == 0{
        memo.insert(n, false);
        return false;
       }
    }
    memo.insert(n,true);
    return true;
}



fn main()
{
    // start recording exec time
    let start_time = Instant::now();

    let mut threads = Vec::new();

    let count = Arc::new(Mutex::new(2));

    let thread_sums = Arc::new(Mutex::new(vec![0; NUM_THREADS]));
    let memo = Arc::new(Mutex::new(HashMap::new()));
    for i in 0..NUM_THREADS{
       let count = Arc::clone(&count);
       let thread_sums = Arc::clone(&thread_sums);
       let memo = Arc::clone(&memo);
       
       let thread = thread::spawn(move || {
        loop {
            let mut num = count.lock().unwrap();

            let val = *num;
            *num += 1;

            drop(num);

            if val > TARGET { break; }

            if is_prime(val, &memo) {
                let mut total = thread_sums.lock().unwrap();
                total[val % NUM_THREADS] += val;
            }

            if val % 10000 == 0 { println!("{}", val); }
        }
    });

        threads.push(thread);
    }

    for t in threads{
       t.join().unwrap();
    }

    // end execution timer
    let exec_time = start_time.elapsed();
    let total:usize = thread_sums.lock().unwrap().iter().sum();
    println!("Total: {:?}", total);
    println!("Execution Time: {} seconds", exec_time.as_secs());

}