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
