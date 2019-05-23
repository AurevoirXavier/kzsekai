// --- std ---
use std::{
    fs::File,
    path::Path,
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Conf {
    pub proxy: Option<String>,
    pub database: Option<String>,
    pub cosplayjav_bypass_retry: Option<u32>,
}

impl Conf {
    pub fn path() -> String {
        format!("{}/conf.json", std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned())
    }

    pub fn load_from_json_file() -> Conf {
        let path = Conf::path();
        if Path::new(&path).is_file() { serde_json::from_reader(File::open(&path).unwrap()).unwrap() } else { Conf::default() }
    }

    pub fn save_to_json_file(&self) { serde_json::to_writer_pretty(&mut File::create(&Conf::path()).unwrap(), self).unwrap() }
}

lazy_static! { pub static ref CONF: Conf = Conf::load_from_json_file(); }
