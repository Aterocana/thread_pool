use std::time::{Duration, SystemTime};
use thread_pool::ThreadPool;

const WORKERS: usize = 55;
const LIMIT: u32 = 50;

fn main() {
    assert!(WORKERS > 0);
    assert!(LIMIT > 0);

    for i in 1usize..WORKERS + 1 {
        elapsing_sleep(i, LIMIT);
    }
}

fn elapsing_sleep(workers: usize, limit: u32) -> () {
    let now = SystemTime::now();
    let pool = ThreadPool::new(workers).unwrap();
    for i in 0..limit {
        pool.execute(move || {
            std::thread::sleep(Duration::new(0, i * 1000000));
            // println!("slept for {} ms", i);
        })
        .unwrap();
    }
    drop(pool);
    match now.elapsed() {
        Ok(elapsed) => println!(
            "With {} workers the elapsed time is {} ms (total sleep time: {})",
            workers,
            elapsed.as_millis(),
            (0..limit).sum::<u32>()
        ),
        Err(err) => eprintln!("error: {:?}", err),
    }
}
