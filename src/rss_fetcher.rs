// use std::thread;
// use std::time::Duration;

// fn fetch_rss(url: &str) {
//     thread::spawn(|| {
//         for i in 1..20 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1100));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(4000));
//     }
// }
