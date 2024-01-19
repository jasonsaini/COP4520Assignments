use std::thread;
use std::time::Duration;
use std::time::Instant;


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
            let mut primes_lock = primes.lock();
            primes_lock.push(number);
            *num_primes += 1
            *sum_primes += number
        }
    }
}

fn main()
{
    // start recording exec time
    let start_time = Instant::now();

    let num_threads = 8;
    let mut threads = Vec::new();

    let target = 10 ^ 8;
    let segment = target / num_threads;

    let primes = Arc::new(Mutex::new(Vec::new()));
    let num_primes = 0;
    let sum_primes = 0;

    for i n 1..=num_threads{
        let primes = Arc::clone(&primes)
        let start = i * segment;
        let end = if i == num_threads { target } else { i * segment}
        // move protects data manipulation from main thread
        let t = thread::spawn(move || {
            find_primes_in_range(start,end,&primes)
        });
        threads.push(t);
    }

    for t in threads{
        t.join();
    }

    // end execution timer
    let exec_time = start_time.elapsed();
    let prime_lock = primes.lock()
    println!("Execution Time: {} ms", exec_time.as_millis())
    println!("{} primes found", num_primes)
    println!("Sum of all primes is {}.", sum_primes)


}