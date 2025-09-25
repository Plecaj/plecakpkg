use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub enum BuildSystem {
    Cmake,
}
#[derive(Serialize, Deserialize)]
pub struct Database{
    name: String,
    build_system: BuildSystem,
    dependencies: Option<Vec<(String, String)>>
}

impl Database {
    pub fn new (name: String, build_system: BuildSystem) -> Database {
        Database{name, build_system, dependencies: None}
    }
}
