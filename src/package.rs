use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub enum BuildSystem {
    Cmake,
}
#[derive(Serialize, Deserialize)]
pub struct Package {
    name: String,
    build_system: BuildSystem,
    dependencies: Option<Vec<(String, String)>>
}

impl Package {
    pub fn new (name: String, build_system: BuildSystem) -> Package {
        Package {name, build_system, dependencies: None}
    }
}
