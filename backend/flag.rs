#![allow(unused)] 
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn main(file: &str) -> Option<Result<String>> {
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();
    watcher.watch(file, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
           Ok(event.to_string()),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
