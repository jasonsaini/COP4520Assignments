use std::thread;
use std::time::Duration;

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


fn main()
{
    for i in 1..28{
        if is_prime(i)
        {
            println!("{} is a prime number!", i);
        }
    }
}