use std::{
    thread::{
        spawn,
        sleep
    },
    time::Duration
};

fn main() {
    let mut blah = "blah";
    let handle1 = spawn(move || {
        sleep(Duration::from_millis(2000));
        println!("{}", blah);
    });
    blah = "fire";

    let handle2 = spawn(|| {
        sleep(Duration::from_millis(1000));
        println!("Borg");
    });

    let handles = vec![handle1, handle2];

    println!("{}",blah);

    for handle in handles {
       handle.join().unwrap(); 
    }

    println!("Nya!");
}
