use toml;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::Write;

#[allow(dead_code)]
pub fn read_toml<T>(path : &str) -> Result<T, Box<dyn std::error::Error>>
where
    T : DeserializeOwned + Default,
{
    let file_string = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };
    let toml_struct =  match toml::from_str::<T>(&file_string) {
        Ok(t) => t,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(toml_struct)
}

#[allow(dead_code)]
pub fn write_toml<T>(path : &str, ser : T) -> Result<(), Box<dyn std::error::Error>>
where
    T : Serialize,
{
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => return Err(Box::new(e)),
    };
    let toml_string = match toml::to_string(&ser) {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };
    match write!(file, "{}", &toml_string) {
        Ok(()) => (),
        Err(e) => return Err(Box::new(e)),
    };

    Ok(())

}