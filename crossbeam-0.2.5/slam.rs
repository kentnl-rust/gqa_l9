use std::thread;
use std::process::{self, Command};
use std::io::prelude::*;

fn one_loop() {
    loop {
        // let res = Command::new("cargo")
        //               .arg("test")
        //               .arg("ms_queue::test::push_try_pop_many_spmc")
        //               .arg("--release")
        //               .status()
        //               .unwrap()
        //               .success();
        let res = Command::new("target/release/stress-msq2")
                      .status()
                      .unwrap();
        if !res.success() {
            println!("Bad exit: {:?}", res);
            std::io::stdout().flush();
            process::exit(1);
        }
    }
}

fn main() {
    println!("Slamming...");
    for _i in 0..3 {
        thread::spawn(one_loop);
    }
    one_loop();
}
