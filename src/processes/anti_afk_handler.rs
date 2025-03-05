use std::sync::mpsc;

pub struct AntiAfk {
    rx: mpsc::Receiver<String>,
}

impl AntiAfk {
    pub fn new(rx: mpsc::Receiver<String>) -> AntiAfk {
        info!("AntiAfk Created");

        AntiAfk {
            rx,
        }
    }

    pub fn handler_loop(self) -> () {
        loop {
            let to_execute = self.rx.recv().unwrap();
            info!("AntiAFK Order Recieved: {}", to_execute);
        }
    }
}