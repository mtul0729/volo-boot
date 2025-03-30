use serde::de::DeserializeOwned;

pub trait LoadConfig: Sized + DeserializeOwned{
    fn load_toml(path: &str) -> Self;
}