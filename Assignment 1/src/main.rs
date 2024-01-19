use std::thread;
use std::time::Duration;

fn is_prime(n:u64) -> bool {
    // we don't count 0 or 1 as prime
    if n <= 1{
        return false;
    }
    let n = n as f64;

    let sqrt_n = n.sqrt();

    let truncated_sqrt = sqrt_n as u64;

    let adjusted = truncated_sqrt + 1;

    for i in 2..sqrt_n {
        // if a number divides evenly within it's range, it's not prime
        if n % i == 0 {
            return false;
        }
    }
    return true;
}


fn main()
{
    let mut handles = vec![];

    for i in 0..5{
        let handle = thread::spawn(move || {
            println!("Hello from thread number {}", i);
            thread::sleep(Duration::from_millis(500))
        });
        handles.push(handle)
    }

    for handle in handles{
        handle.join().unwrap()
    }
    println!("All threads have finished.")
}