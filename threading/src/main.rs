use std::{
    thread::{
        spawn,
        sleep,
    },
    time::Duration,
    sync::mpsc::channel,
};

struct Thing {
    value: String,
}

impl Drop for Thing {
    fn drop(&mut self) { 
        println!("{}", self.value);
     }
}


const cleanUpCount: u8 = 15;

fn main() {
    let (sender, receiver) = channel();

    spawn(move || {
        for n in 1..=20 {
            sleep(Duration::from_millis(250));
            println!("Pushing item");
            sender.send(Some(Thing { value: String::from(format!("Item {}", n)) })).unwrap();
        }
        sender.send(None).unwrap();
    });

    let mut count = 0;
    let mut dump: Vec<Thing> = Vec::new();
    'loopin: loop {
        let receiver_result = receiver.recv();
    
        match receiver_result {
            Ok(received) => match received {
                Some(thing) => dump.push(thing),
                None => break 'loopin,
            },
            Err(err) => eprint!("Error: {}", err),
        }
        count = count + 1;
        if count >= cleanUpCount {
            println!("Cleaning up resources");
            count = 0;
            dump = Vec::new();
            println!("Resources cleaned up");
        }
    }
}
