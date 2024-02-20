use std::cmp;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

const TARGET: usize = 100_000_000;
const NUM_THREADS: usize = 8;
const PRECOMPUTED_PRIMES: [u32; 4] = [2, 3, 5, 7];

fn sieve_primes(limit: usize) -> HashSet<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=(limit as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime.iter().enumerate().filter(|&(i, &is_prime)| is_prime)
        .map(|(i, _)| i).collect() // Ensure the function returns a HashSet<usize>
}

fn is_prime_threaded(n: usize, precomputed_primes: &[u32], sieve: &HashSet<usize>) -> bool {
    for &prime in precomputed_primes {
        let prime_usize = prime as usize;
        if n == prime_usize {
            return true;
        } else if n % prime_usize == 0 {
            return false;
        }
    }
    sieve.contains(&n) // Ensure the function returns a bool
}
fn main() {
    let start_time = Instant::now();

    let limit = (PRECOMPUTED_PRIMES.last().unwrap() * PRECOMPUTED_PRIMES.last().unwrap()) as usize;
    let precomputed_primes = PRECOMPUTED_PRIMES.to_vec();
    let sieve = sieve_primes(limit);

    let thread_sums = Arc::new(Mutex::new(vec![0; NUM_THREADS]));
    let mut threads = Vec::new();
    
    let chunk_size = ((TARGET - limit) + NUM_THREADS - 1) / NUM_THREADS;

    for i in 0..NUM_THREADS {
        let start = cmp::max(2, limit + i * chunk_size);
        let end = if i == NUM_THREADS - 1 { TARGET } else { start + chunk_size };
        let thread_sums = Arc::clone(&thread_sums);
        let precomputed_primes = precomputed_primes.clone();
        let sieve = sieve.clone();

        println!("Thread {}: Processing range {} to {}", i, start, end); // Debugging print

        let thread = thread::spawn(move || {
            let mut local_sum = 0;
            for num in (start..=end).step_by(2) {
                if is_prime_threaded(num, &precomputed_primes, &sieve) {
                    println!("Thread {}: Prime {}", i, num); // Debugging print
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

    println!("Execution completed. Total sum: {}", total);
    println!("Execution Time: {} seconds", exec_time.as_secs());
}
