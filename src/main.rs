use thread_pool::ThreadPool;
fn main() {
    let pool = ThreadPool::new(10).unwrap();
    for i in 0..100 {
        pool.execute(move || {
            println!("ciao {}", i);
        })
        .unwrap();
    }
    // std::thread::sleep(std::time::Duration::new(2, 0));
}
