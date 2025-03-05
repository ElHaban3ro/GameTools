use core::time;
use std::sync::mpsc::{self};
use std::thread::{self, sleep};
use crossbeam::channel::Receiver;
use crossbeam::channel;
use crate::utilities::system_utilities::SystemUtilities;

use rdev::{EventType, Key, simulate};

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
                sleep(time::Duration::from_millis(100));

                let to_execute = __rx.try_recv();
            
                let to_execute = match to_execute {
                    Ok(to_execute) => to_execute,
                    Err(_) => {
                        "nothing".to_string()
                    },
                };

                if to_execute == "stop" {
                    break;
                }

                
                // press w by 5 seconds
                GameToolsHandler::press_key_by_miliseconds(Key::KeyW, 200);
                GameToolsHandler::press_key_by_miliseconds(Key::KeyS, 200);

                GameToolsHandler::press_key_by_miliseconds(Key::Space, 1000);

                sleep(time::Duration::from_secs(5));

            }
        });

    }

    fn strat_auto_run_loop(_rx: Receiver<String>) {
        let __rx = _rx;
        info!("Starting Auto Run Loop.");

        let configs = SystemUtilities::read_configs_json();
        let run_key = SystemUtilities::str_to_key(configs["run_key"].as_str().unwrap());

        thread::spawn(move || {
            simulate(&EventType::KeyPress(Key::KeyW)).unwrap();
            simulate(&EventType::KeyPress(run_key)).unwrap();
            
            loop {
                let to_execute = __rx.try_recv();
                let to_execute = match to_execute {
                    Ok(to_execute) => to_execute,
                    Err(_) => {
                        "nothing".to_string()
                    },
                };

                sleep(time::Duration::from_millis(200));
                if to_execute == "stop" {
                    simulate(&EventType::KeyRelease(Key::KeyW)).unwrap();
                    simulate(&EventType::KeyRelease(run_key)).unwrap();
                    break;
                }
            }
        });
    }

    fn press_key_by_miliseconds(key: Key, miliseconds: u64) {
        simulate(&EventType::KeyPress(key)).unwrap();
        sleep(time::Duration::from_millis(miliseconds));
        simulate(&EventType::KeyRelease(key)).unwrap();
    }

    pub fn handler_loop(&mut self) -> () {
        
        
        let (tx, _rx) = channel::unbounded();
        let _tx = tx.clone();

        let mut run_afk: bool = false;
        let mut run_auto_run: bool = false;

        loop {
            let to_execute = self.rx.try_recv();
            let to_execute = match to_execute {
                Ok(to_execute) => to_execute,
                Err(_) => {
                    sleep(time::Duration::from_millis(300));
                    "nothing".to_string()
                },
            };
            
            if to_execute == "start_afk" {
                if !run_afk {
                    GameToolsHandler::start_anti_afk_loop(_rx.clone());
                    run_afk = true;
                } else {
                    _tx.send("stop".to_string()).unwrap();
                    run_afk = false;
                    info!("Anti-AFK Stoped.");
                }
            } else if to_execute == "stop_afk" {
                if run_afk {
                    _tx.send("stop".to_string()).unwrap();
                    run_afk = false;
                } else {
                    info!("Anti-AFK Loop Already Stopped.");
                }

            } else if to_execute == "start_auto_run" {
                if !run_auto_run {
                    GameToolsHandler::strat_auto_run_loop(_rx.clone());
                    run_auto_run = true;
                } else {
                    _tx.send("stop".to_string()).unwrap();
                    run_auto_run = false;
                    info!("Auto Run Stoped.");
                }
            } else if to_execute == "stop_auto_run" {
                if run_auto_run {
                    _tx.send("stop".to_string()).unwrap();
                    info!("Auto Run Stoped.");
                    run_auto_run = false;
                } else {
                    info!("Auto Run Loop Already Stopped.");
                }
            }
        }
    }
}