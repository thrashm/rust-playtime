use std::{
    thread::{
        spawn,
        sleep,
    },
    time::Duration,
    sync::mpsc::channel,
};

fn main() {
    let (sender, receiver) = channel();

    spawn(move || {
        let val = String::from("!!!");
        sleep(Duration::from_secs(2));
        sender.send(Some(String::from("hi"))).unwrap();
        sleep(Duration::from_secs(2));
        sender.send(Some(String::from("nanners"))).unwrap();
        sleep(Duration::from_secs(2));
        sender.send(Some(val)).unwrap();
        sender.send(None);
    });

    'loopin: loop {
        let receiver_result = receiver.recv();
    
        match receiver_result {
            Ok(received) => match received {
                Some(value) => println!("Get: {}", value),
                None => break 'loopin,
            },
            Err(err) => eprint!("Error: {}", err),
        }
    }
}
