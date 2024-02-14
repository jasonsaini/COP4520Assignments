Prime Number Finder
To run: 
Open a Terminal: First, open your terminal or command line interface.

Navigate to Your Project Directory: Use the cd command to navigate to your Rust project directory that contains the Cargo.toml file.

Build the Project: Run the command cargo build. This compiles your project and generates an executable in the target/debug directory.

Run the Project: Execute the command cargo run. This will compile (if not already compiled) and then run your Rust application.

Overview
This program finds all prime numbers between 1 and 10^8 using a parallel computing approach with 8 concurrent threads. The goal is to ensure that the computational execution time is approximately equivalent among the threads, optimizing both efficiency and cost-effectiveness.

Summary of Approach:

- The program uses a shared Arc<Mutex> structure for thread-safe access to a hashmap (memo) for memoization and a counter (count).
- Each thread checks numbers for primality in a loop, incrementing the shared counter.
- A shared thread_sums vector keeps track of the sum of prime numbers found by each thread.
- Efficiency is sought by dividing the workload evenly among threads and using memoization to avoid recalculating the primality of numbers.
- Reasoning and Experimental Evaluation:

- The design aims for equal workload distribution among threads to optimize execution time.
- Memoization reduces redundant computations, enhancing efficiency.
- The use of mutexes ensures data consistency but may introduce some overhead due to locking and unlocking.
- Experimental evaluation would focus on execution time, workload balance among threads, and the impact of memoization on performance.
- The program's cost-effectiveness is a consideration, given the pay-per-minute model for machine usage.
