use std::sync::{Arc, LockResult, Mutex};
use std::thread;

// rust 中, 使用 move 可以将线程外的变量移动到线程内
// 如果直接 使用 String::from("Hello"); 这种 在堆上的数据会有所有权的问题
// 使用 move 移动在线程内上所有权
// 但是如果在 main 线程再次使用 这个数据，就会禁止编译
// 所以需要 Arc 来保证
#[test]
fn example_1_un_mut() {
    let s = Arc::new(String::from("Hello"));
    let s1 = s.clone();
    let handler = thread::spawn( move || {
        // 当你需要在线程内对变量进行修改的时候
        println!("example_1 thread: {}", s1);
    });
    handler.join();
    println!("example_1 thread: {}", s);
}

#[test]
fn example_1_mut() {
    let s = Arc::new(Mutex::new(String::from("Hello")));
    let s1 = s.clone();
    let handler = thread::spawn( move || {
        match s1.lock() {
            Ok(mut guarded ) => {
                guarded.push_str(" thread");
            }
            Err(_) => {
                println!("example_1 thread locked error");
            }
        }
    });

    let _ = handler.join();

    match s.lock() {
        Ok(guarded) => {
            println!("example_1 thread locked: {}", guarded);
        }
        Err(_) => {
            println!("example_1 thread unlocked error");
        }
    }
}