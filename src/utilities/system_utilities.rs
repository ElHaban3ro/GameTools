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
}