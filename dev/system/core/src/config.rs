//------------------------------------------------------------------------------
//! Configuration module.
//------------------------------------------------------------------------------

use crate::LoadToStringMap;

use std::collections::HashMap;
use std::env;
use std::path::Path;


//------------------------------------------------------------------------------
/// Configuration.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Config
{
    map: HashMap<String, String>,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        let mut config = Self
        {
            map: HashMap::new(),
        };

        let config_path = env::var("CONFIG_PATH").unwrap_or("./".to_string());
        config.map = config.load(Path::new(&config_path));
        config
    }

    //--------------------------------------------------------------------------
    /// Gets a configuration value as a String.
    //--------------------------------------------------------------------------
    pub fn get( &self, key: &str ) -> String
    {
        self.map.get(key).unwrap_or(&"".to_string()).to_string()
    }

    //--------------------------------------------------------------------------
    /// Gets a configuration value as a bool.
    //--------------------------------------------------------------------------
    pub fn get_as_bool( &self, key: &str ) -> bool
    {
        self.get(key).parse::<bool>().unwrap_or(false)
    }

    //--------------------------------------------------------------------------
    /// Gets a configuration value as a number.
    //--------------------------------------------------------------------------
    pub fn get_as_i64( &self, key: &str ) -> i64
    {
        self.get(key).parse::<i64>().unwrap_or(0)
    }

    //--------------------------------------------------------------------------
    /// Gets a configuration value as a vector.
    //--------------------------------------------------------------------------
    pub fn get_as_vec( &self, key: &str ) -> Vec<String>
    {
        self.get(key).split(",").map(|s| s.to_string()).collect()
    }
}

impl LoadToStringMap for Config {}
