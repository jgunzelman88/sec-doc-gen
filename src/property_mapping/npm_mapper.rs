use crate::models::application::{Application, Dependency};
use crate::models::config::Config;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
struct PackageJson {
    pub name: String,
    pub dependecies: HashMap<String, String>,
    pub version: String,
}

pub fn map_application(config: &Config) -> Result<Application, Box<dyn std::error::Error>> {
    let mut npm_file = File::open(format!("{}/package.json", config.base_dir))?;
    let mut json = String::new();
    npm_file.read_to_string(&mut json)?;
    let npm_content : PackageJson = serde_json::from_str(&json)?;
    
    let inter_deps : Vec::<Dependency> = npm_content.dependecies
    .into_iter()
    .map(|dep| {
        return
            Dependency {
                name: dep.0,
                version: dep.1,
                port: None,
                property_mappings: None,
                protocol: None
            }
        }
    ).collect();
    
    Ok(Application {
        name : npm_content.name,
        parent: None,
        subcomponents: None,
        internal_dependencies: inter_deps,
        external_dependencies: Vec::new()
    })
}