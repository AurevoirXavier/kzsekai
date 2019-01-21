#[derive(Serialize, Deserialize)]
pub struct Conf { proxy: Option<String> }

impl Conf {
    pub fn path() -> String {
        // --- std ---
        use std::env::current_exe;

        format!(
            "{}/sexy_conf.json",
            current_exe().unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        )
    }

    pub fn load_from_json_file() -> Conf {
        // --- std ---
        use std::{
            fs::File,
            path::Path,
        };

        let path = Conf::path();
        if Path::new(&path).is_file() { serde_json::from_reader(File::open(&path).unwrap()).unwrap() } else { Conf::default() }
    }

    pub fn save_to_json_file(&self) {
        // --- std ---
        use std::fs::File;

        serde_json::to_writer_pretty(&mut File::create(&Conf::path()).unwrap(), self).unwrap()
    }
}

impl Default for Conf { fn default() -> Conf { Conf { proxy: None } } }