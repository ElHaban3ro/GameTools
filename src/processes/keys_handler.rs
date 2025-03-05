use std::sync::mpsc;
use rdev::{listen, EventType, Key};
use crate::utilities::system_utilities::SystemUtilities;

pub struct KeysHandler {
    macro_start_afk: Vec<Key>,
    macro_stop_afk: Vec<Key>,
    macro_start_auto_run: Vec<Key>,
    macro_stop_auto_run: Vec<Key>,
    _tx: mpsc::Sender<String>
}

impl KeysHandler {

    pub fn new(_tx: mpsc::Sender<String>) -> KeysHandler {
        info!("KeysHandler Created.");
        SystemUtilities::create_configs_json();
        

        let mut obj = KeysHandler {
            // En caso de que no pueda cargar los macros, asigna una configuración predeterminada.
            // ¿Arreglar? ¿Hay una situación donde no puedan ser cargados?
            macro_start_afk: KeysHandler::macro_vec_to_keys(vec!["F1".to_string()]),
            macro_stop_afk: KeysHandler::macro_vec_to_keys(vec!["F2".to_string()]),
            macro_start_auto_run: KeysHandler::macro_vec_to_keys(vec!["F3".to_string()]),
            macro_stop_auto_run: KeysHandler::macro_vec_to_keys(vec!["F4".to_string()]),
            _tx,
        };
        obj.update_macros();
        return obj;
    }

    fn update_macros(&mut self) -> () {
        let configs_json = SystemUtilities::read_configs_json();
        self.macro_start_afk = KeysHandler::macro_vec_to_keys(configs_json["macro_start_afk"].members().map(|x| x.as_str().unwrap().to_string()).collect());
        self.macro_stop_afk = KeysHandler::macro_vec_to_keys(configs_json["macro_stop_afk"].members().map(|x| x.as_str().unwrap().to_string()).collect());

        self.macro_start_auto_run = KeysHandler::macro_vec_to_keys(configs_json["macro_start_auto_run"].members().map(|x| x.as_str().unwrap().to_string()).collect());
        self.macro_stop_auto_run = KeysHandler::macro_vec_to_keys(configs_json["macro_stop_auto_run"].members().map(|x| x.as_str().unwrap().to_string()).collect());
    }

    fn macro_vec_to_keys(macro_vec: Vec<String>) -> Vec<Key> {
        let mut keys: Vec<Key> = vec![];
        for key in macro_vec {
            keys.push(SystemUtilities::str_to_key(&key));
        }
        return keys;
    }

    pub fn handler_loop(self) -> () {
        let pressed_keys: Vec<Key> = vec![];
        
        loop {
            let mut _pressed = pressed_keys.clone();
            
            let _macro_start_afk = self.macro_start_afk.clone();
            let _macro_stop_afk = self.macro_stop_afk.clone();

            let _macro_start_auto_run = self.macro_start_auto_run.clone();
            let _macro_stop_auto_run = self.macro_stop_auto_run.clone();

            let _tx = self._tx.clone();
            
            if let Err(error) = listen(move |e| {
                //println!("{:?}", _pressed);
                
                match e.event_type {
                    EventType::KeyPress(key) => {
                        if !_pressed.contains(&key) {
                            let configs_json = SystemUtilities::read_configs_json();

                            let macro_start_afk = KeysHandler::macro_vec_to_keys(configs_json["macro_start_afk"].members().map(|x| x.as_str().unwrap().to_string()).collect());
                            let macro_stop_afk = KeysHandler::macro_vec_to_keys(configs_json["macro_stop_afk"].members().map(|x| x.as_str().unwrap().to_string()).collect());

                            let macro_start_auto_run = KeysHandler::macro_vec_to_keys(configs_json["macro_start_auto_run"].members().map(|x| x.as_str().unwrap().to_string()).collect());
                            let macro_stop_auto_run = KeysHandler::macro_vec_to_keys(configs_json["macro_stop_auto_run"].members().map(|x| x.as_str().unwrap().to_string()).collect());

                            //? Si la tecla oprimida está registrada como parte de alguno de 
                            //? los macros, la agrega a la lista de teclas oprimidas.
                            if macro_start_afk.contains(&key) || macro_stop_afk.contains(&key) || macro_start_auto_run.contains(&key) || macro_stop_auto_run.contains(&key) {
                                _pressed.push(key);
                            }
                        }
                    },
                    EventType::KeyRelease(key) => {
                        // Cuando deja de ser presionada, la elimina de la lista.
                        _pressed.retain(|&x| x != key); 
                    },
                    _ => {}
                }

                if _pressed == _macro_start_afk {
                    _tx.send("start_afk".to_string()).unwrap();
                } else if _pressed == _macro_stop_afk {
                    _tx.send("stop_afk".to_string()).unwrap();
                } else if _pressed == _macro_start_auto_run {
                    _tx.send("start_auto_run".to_string()).unwrap();
                } else if _pressed == _macro_stop_auto_run {
                    _tx.send("stop_auto_run".to_string()).unwrap();
                }


            }) {
                println!("Error: {:?}", error)
            }
        }
    }

}