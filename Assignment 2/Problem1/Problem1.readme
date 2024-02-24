# Problem 1
## Overview
This Java program simulates a party scenario hosted by a Minotaur. Guests enter a labyrinth one at a time, with the objective of determining when all guests have visited at least once.

## Features
- **Concurrent Guests Handling**: Each guest is represented by a thread, simulating simultaneous actions.
- **Thread Safety**: Utilizes atomic variables and synchronized blocks to manage shared resources in a multithreaded environment.
- **Efficient Synchronization**: Minimizes the scope of synchronized blocks to reduce thread contention.

## Correctness
- **Atomic Variables**: `AtomicInteger` and `AtomicBoolean` ensure accurate counts and state management.
- **Synchronization**: The `synchronized` block with a lock object prevents race conditions.
- **Specific Guest Logic**: Non-leader guests enter once, while the leader guest tracks the number of unique guests.
- **Termination Condition**: The game concludes when all guests have visited the labyrinth.

## Efficiency
- **Thread Management**: Creates and manages multiple threads efficiently for parallel execution.
- **Resource Locking**: Optimizes performance by reducing the potential for thread contention.
- **Early Exit**: Threads terminate early when the game is finished to avoid unnecessary computations.

## Experimental Evaluation
- **Functional Testing**: Validate by running the program with various guest counts.
- **Performance Analysis**: Measure execution time for scalability assessment.
- **Concurrency Testing**: Ensure correct management of shared resources without race conditions or deadlocks.

# Problem 2
## Features
- **Sequential Access**: Ensures that only one guest views the vase at any given time.
- **Concurrency Control**: Utilizes a semaphore to manage access to the showroom.
- **Thread Management**: Represents each guest as an individual thread, allowing for simultaneous actions outside the critical section.

## Correctness
- **Semaphore**: The `Semaphore` with a single permit ensures that only one thread (guest) accesses the showroom at a time.
- **Thread Safety**: Ensures safe access to the critical section (viewing the vase) in a concurrent environment.
- **Consistent Viewing Experience**: Each guest views the vase for a fixed amount of time (simulated by `Thread.sleep(1000)`), ensuring consistency.

## Efficiency
- **Fair Queueing**: The semaphore ensures that guest threads access the showroom in a first-come, first-served manner.
- **Optimized Waiting**: Threads wait efficiently while the showroom is occupied, minimizing resource contention.
- **Resource Utilization**: Efficient use of the CPU as threads are not actively waiting but are instead put to sleep or blocked by the semaphore.

## Experimental Evaluation
- **Functional Testing**: The program can be tested with varying numbers of guests to ensure proper sequential access.
- **Concurrency Testing**: Evaluating the program under high concurrency to ensure no two guests view the vase simultaneously.
- **Performance Metrics**: Measuring the time taken for all guests to view the vase can be used to assess scalability.
