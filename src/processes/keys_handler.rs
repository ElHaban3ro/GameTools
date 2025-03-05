use std::thread;
use std::sync::mpsc;

pub struct KeysHandler {
    tx: mpsc::Sender<String>
}

impl KeysHandler {

    pub fn new(tx: mpsc::Sender<String>) -> KeysHandler {
        info!("KeysHandler Created.");
        KeysHandler {
            tx,
        }
    }

    pub fn handler_loop(self, sleep_time: u64) -> () {
        loop {
            thread::sleep(std::time::Duration::from_secs(sleep_time));
            self.tx.send("minecraft_afk".to_string()).unwrap();
        }
    }

}