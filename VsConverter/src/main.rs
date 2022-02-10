#![feature(path_file_prefix)]

use std::env;
use std::fs;
use std::path;
use std::path::Path;
use walkdir::WalkDir;
use async_process::Command;
use std::time;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let argv = env::args().len();
    if argv <= 1 {
        println!("no input!");
        return
    }
    let mut count = 0;
    for arg in env::args() {
        for entry in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()){
            let p = entry.path();
            count += 1;
            if count >= 100 {
                sleep(Duration::from_secs(3));
                count = 0;
            }
            // println!("{}", p.display());
            let p_path = p.to_str().unwrap();
            if p_path.to_lowercase().ends_with(".jpg") || p_path.to_lowercase().ends_with(".jpeg"){
                let file_name = p.file_name().unwrap().to_str().unwrap();
                let b = &file_name[..file_name.len() - {if p_path.to_lowercase().ends_with(".jpg") {4} else {5}}];
                let mut new_file_name = String::from(b);
                new_file_name.push_str(".png");
                let result_file_path = p.parent().unwrap().join(new_file_name);
                let result_file_path = result_file_path.to_str().unwrap();
                Command::new("magick").args(&[p_path, "-fuzz", "0.8%", "-fill", "none", "-draw","alpha 0,0 floodfill",&result_file_path]).spawn();
                println!("转换中... {}, {}", p.display(), result_file_path);
            }

        }
    }
    println!("任务发布完成，内容多的话可能还有异步任务在执行");
}