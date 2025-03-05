mod processes;
#[macro_use] mod utilities;
use processes::{gametools_handler::GameToolsHandler, keys_handler::KeysHandler};
use utilities::system_utilities::SystemUtilities;
use std::{fs::File, sync::mpsc, thread};

#[macro_use] extern crate log;

extern crate simplelog;
use simplelog::*;


fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create("debug.log").unwrap()),
            ]
        ).unwrap();
    SystemUtilities::clear_console();
    info!("Initializing GameTools...");

    let (tx, rx) = mpsc::channel::<String>();
    let keys_handler_tx = tx.clone();


    let keys_handler = KeysHandler::new(keys_handler_tx);
    let mut gametools_handler = GameToolsHandler::new(rx);


    // Loops Started in Threads
    let keys_thread = thread::spawn(move || keys_handler.handler_loop());
    let anti_afk_thread = thread::spawn(move || gametools_handler.handler_loop());

    keys_thread.join().unwrap();
    anti_afk_thread.join().unwrap();

}
