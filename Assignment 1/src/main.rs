use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::Arc;

fn is_prime(n:u64) -> bool {
    // we don't count 0 or 1 as prime
    if n <= 1{
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 2..=sqrt_n {
        // if a number divides evenly within it's range, it's not prime
        if n % i == 0 {
            return false;
        }
    }
    return true;
}


fn find_primes_in_range(start:u64, end:u64, primes: &Arc<Mutex<Vec<u64>>>, num_primes :  &Arc<Mutex<u64>>, sum_primes: &Arc<Mutex<u64>>){
    for number in start..=end{
        if is_prime(number)
        {
            // push to primes array from within thread
            let mut primes_lock = primes.lock().unwrap();
            primes_lock.push(number);
           
            let mut num_primes_lock = num_primes.lock().unwrap();
            *num_primes_lock += 1;

            let mut sum_primes_lock = sum_primes.lock().unwrap();
            *sum_primes_lock += number;
        }
    }
}

fn main()
{
    // start recording exec time
    let start_time = Instant::now();

    let num_threads = 8;
    let mut threads = Vec::new();

    let base = 10 as u64;
    let target = base.pow(8);
    let segment = target / num_threads;

    let primes = Arc::new(Mutex::new(Vec::new()));
    let num_primes = Arc::new(Mutex::new(0));
    let sum_primes =  Arc::new(Mutex::new(0));
    for i in 1..=num_threads{
        let primes = Arc::clone(&primes);
        let num_primes = Arc::clone(&num_primes);
        let sum_primes = Arc::clone(&sum_primes);
        let start = i * segment;
        let end = if i == num_threads { target } else { (i + 1) * segment};
        // move protects data manipulation from main thread
        let t = thread::spawn(move || {
            find_primes_in_range(start,end,&primes, &num_primes, &sum_primes)
        });
        threads.push(t);
    }

    for t in threads{
       let _ =  t.join();
    }

    // end execution timer
    let exec_time = start_time.elapsed();
    let prime_lock = primes.lock().unwrap();
    println!("State of primes: {:?}", *prime_lock);
    println!("Execution Time: {} ms", exec_time.as_millis());
    //println!("{} primes found", num_primes)
    //println!("Sum of all primes is {}.", sum_primes)


}