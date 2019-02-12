// --- std ---
use std::{
    fs::{File, create_dir},
    path::Path,
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Conf {
    pub cosplayjav_bypass_retry: Option<u32>,
    pub proxy: Option<String>,
    pub database: Option<String>,
}

impl Conf {
    pub fn path() -> String {
        // --- std ---
        use std::env::current_exe;

        let dir = format!(
            "{}/.sexy",
            current_exe().unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        );

        if !Path::new(&dir).is_dir() { create_dir(&dir).unwrap(); }

        format!("{}/conf.json", dir)
    }

    pub fn load_from_json_file() -> Conf {
        let path = Conf::path();
        if Path::new(&path).is_file() { serde_json::from_reader(File::open(&path).unwrap()).unwrap() } else { Conf::default() }
    }

    pub fn save_to_json_file(&self) { serde_json::to_writer_pretty(&mut File::create(&Conf::path()).unwrap(), self).unwrap() }
}

lazy_static! { pub static ref CONF: Conf = Conf::load_from_json_file(); }
