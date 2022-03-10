pub mod get_OS {
    pub fn determine_default_path() -> &'static str {
        if cfg!(windows) {
            "C:\\Users\\*user name*\\Downloads"
        } else {
            "/home/*user name*/Downloads/"
        }
    }
}
