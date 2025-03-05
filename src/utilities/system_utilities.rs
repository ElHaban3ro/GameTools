use json::{object, stringify};
use rdev::Key;
use std::path::Path;

pub struct SystemUtilities {

}

impl SystemUtilities {
    pub fn clear_console() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn _get_os() -> String {
        let os = std::env::consts::OS;
        return os.to_string();
    }

    pub fn read_configs_json() -> json::JsonValue {
        let config_path = Path::new("configs.json");
        if !config_path.exists() {
            SystemUtilities::create_configs_json();
        }
        let config_file = std::fs::read_to_string("configs.json").unwrap();
        let config_json = json::parse(&config_file).unwrap();
        return config_json;
    }

    pub fn create_configs_json() {
        let config_path = Path::new("configs.json");
        if !config_path.exists() {
            let config_object = object! {
                "macro_start_afk": ["ControlLeft", "KeyP"],
                "macro_stop_afk": ["ControlLeft", "KeyP"],
                "macro_start_auto_run": ["ControlLeft", "KeyL"],
                "macro_stop_auto_run": ["ControlLeft", "KeyL"],
            };
            

            let writing = std::fs::write("configs.json", stringify(config_object));
            match writing {
                Ok(_) => info!("Config File Created."),
                Err(error) => println!("Error: {:?}", error)
                
            }
        }
    }

    pub fn str_to_key(key: &str) -> Key {
        match key {
            "Alt" => Key::Alt,
            "AltGr" => Key::AltGr,
            "Backspace" => Key::Backspace,
            "CapsLock" => Key::CapsLock,
            "ControlLeft" => Key::ControlLeft,
            "ControlRight" => Key::ControlRight,
            "Delete" => Key::Delete,
            "DownArrow" => Key::DownArrow,
            "End" => Key::End,
            "Escape" => Key::Escape,
            "F1" => Key::F1,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "Home" => Key::Home,
            "LeftArrow" => Key::LeftArrow,
            "MetaLeft" => Key::MetaLeft,
            "PageDown" => Key::PageDown,
            "PageUp" => Key::PageUp,
            "Return" => Key::Return,
            "RightArrow" => Key::RightArrow,
            "ShiftLeft" => Key::ShiftLeft,
            "Space" => Key::Space,
            "Tab" => Key::Tab,
            "UpArrow" => Key::UpArrow,
            "PrintScreen" => Key::PrintScreen,
            "ScrollLock" => Key::ScrollLock,
            "Pause" => Key::Pause,
            "NumLock" => Key::NumLock,
            "BackQuote" => Key::BackQuote,
            "Num1" => Key::Num1,
            "Num2" => Key::Num2,
            "Num3" => Key::Num3,
            "Num4" => Key::Num4,
            "Num5" => Key::Num5,
            "Num6" => Key::Num6,
            "Num7" => Key::Num7,
            "Num8" => Key::Num8,
            "Num9" => Key::Num9,
            "Num0" => Key::Num0,
            "Minus" => Key::Minus,
            "Equal" => Key::Equal,
            "KeyA" => Key::KeyA,
            "KeyB" => Key::KeyB,
            "KeyC" => Key::KeyC,
            "KeyD" => Key::KeyD,
            "KeyE" => Key::KeyE,
            "KeyF" => Key::KeyF,
            "KeyG" => Key::KeyG,
            "KeyH" => Key::KeyH,
            "KeyI" => Key::KeyI,
            "KeyJ" => Key::KeyJ,
            "KeyK" => Key::KeyK,
            "KeyL" => Key::KeyL,
            "KeyM" => Key::KeyM,
            "KeyN" => Key::KeyN,
            "KeyO" => Key::KeyO,
            "KeyP" => Key::KeyP,
            "KeyQ" => Key::KeyQ,
            "KeyR" => Key::KeyR,
            "KeyS" => Key::KeyS,
            "KeyT" => Key::KeyT,
            "KeyU" => Key::KeyU,
            "KeyV" => Key::KeyV,
            "KeyW" => Key::KeyW,
            "KeyX" => Key::KeyX,
            "KeyY" => Key::KeyY,
            "KeyZ" => Key::KeyZ,
            "LeftBracket" => Key::LeftBracket,
            "RightBracket" => Key::RightBracket,
            "SemiColon" => Key::SemiColon,
            _ => {
                warn!("Unknown Key: {}", key);
                Key::Unknown(0)
            },
        }
    }
}