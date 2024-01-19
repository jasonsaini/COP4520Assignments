use std::thread;
use std::time::Duration;


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