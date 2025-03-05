use core::time;
use std::sync::mpsc::{self};
use std::thread::{self, sleep};
use crossbeam::channel::Receiver;
use crossbeam::channel;

pub struct GameToolsHandler {
    rx: mpsc::Receiver<String>,
}

impl GameToolsHandler {
    pub fn new(rx: mpsc::Receiver<String>) -> GameToolsHandler {
        info!("GameToolsHandler Created.");

        
        GameToolsHandler {
            rx,
        }
    }

    fn start_anti_afk_loop(_rx: Receiver<String>) {
        
        info!("Starting Anti-AFK Loop.");
        let __rx = _rx;
        
        thread::spawn(move || {
            loop {
                info!("Anti-AFK Loop Running.");
                sleep(time::Duration::from_secs(1));

                let to_execute = __rx.try_recv();
            
                let to_execute = match to_execute {
                    Ok(to_execute) => to_execute,
                    Err(_) => {
                        //println!("Error: {:?}", e);
                        "nothing".to_string()
                    },
                };

                if to_execute == "stop" {
                    info!("Stopping Anti-AFK Loop.");
                    break;
                }

            }
        });

    }

    pub fn handler_loop(&mut self) -> () {
        
        
        let (tx, _rx) = channel::unbounded();
        let _tx = tx.clone();

        let mut run_afk: bool = false;

        loop {
            let to_execute = self.rx.try_recv();
            let to_execute = match to_execute {
                Ok(to_execute) => to_execute,
                Err(_) => {
                    //println!("Error: {:?}", e);
                    sleep(time::Duration::from_secs(2));
                    "nothing".to_string()
                },
            };
            

            if to_execute == "start_afk" {
                if !run_afk {
                    GameToolsHandler::start_anti_afk_loop(_rx.clone());
                    run_afk = true;
                } else {
                    info!("Anti-AFK Loop Already Running.");
                }
            } else if to_execute == "stop_afk" {
                if run_afk {
                    _tx.send("stop".to_string()).unwrap();
                    run_afk = false;
                } else {
                    info!("Anti-AFK Loop Already Stopped.");
                }

            }
        }
    }
}