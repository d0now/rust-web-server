use configparser::ini::Ini;

use crate::Result;

#[derive(Clone, Debug)]
pub struct Config {
    filepath: String,
    parser: Ini,
}

pub fn parse(filepath: &str) -> Result<Config> {
    
    let mut config = Ini::new();
    let _map = config.load(filepath)?;
    
    let ret = Config {
        filepath: String::from(filepath),
        parser: config,
    };

    return Ok(ret);
}

impl Config {

    pub fn check(&self) -> Result<()> {

        let category = "Server";
        let keys = ["Host", "Port"];

        for key in keys.iter() {
            if self.parser.get(category, key).is_none() {
                return Err(format!("{}:{} not found", category, key).into());
            }
        }

        return Ok(());
    }

}