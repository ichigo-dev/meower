//------------------------------------------------------------------------------
//! Internationalization.
//!
//! # Usage
//!
//! ```rust
//! use meower_utility::{ Config, I18n };
//!
//! fn main()
//! {
//!    let config = Config::new();
//!    let mut i18n = I18n::new();
//!    i18n.init("en", &config);
//!
//!    // Gets the message.
//!    println!("{}", i18n.get("message_key"));
//!
//!    // Gets the message and replace the placeholders.
//!    let replace = HashMap::from_iter(vec![("key", "value")]);
//!    println!("{}", i18n.get_with("message_key", replace));
//! }
//! ```
//------------------------------------------------------------------------------

use crate::Config;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


//------------------------------------------------------------------------------
/// Internationalization.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct I18n
{
    locale: String,
    map: HashMap<String, String>,
    fallback: HashMap<String, String>,
}

impl I18n
{
    //--------------------------------------------------------------------------
    /// Creates a new i18n.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self
        {
            locale: "".to_string(),
            map: HashMap::new(),
            fallback: HashMap::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Initializes the i18n.
    //--------------------------------------------------------------------------
    pub fn init( &mut self, locale: &str, config: &Config ) -> bool
    {
        self.clear();
        if self.find_locale_path(config) == false
        {
            return false;
        }
        self.locale = locale.to_string();
        self.map = self.load(&self.locale, config);
        self.fallback = self.load(config.fallback_locale(), config);
        true
    }

    //--------------------------------------------------------------------------
    /// Finds the locale path.
    //--------------------------------------------------------------------------
    fn find_locale_path( &mut self, config: &Config ) -> bool
    {
        let mut locale_path = PathBuf::from(&config.locale_path());
        locale_path.push(&self.locale);
        if locale_path.exists() == false
        {
            let mut fallback_path = PathBuf::from(&config.locale_path());
            fallback_path.push(config.fallback_locale());
            if fallback_path.exists() == false
            {
                return false;
            }
            self.locale = config.fallback_locale().to_string();
        }
        true
    }

    //--------------------------------------------------------------------------
    /// Loads the i18n map.
    //--------------------------------------------------------------------------
    fn load( &self, locale: &str, config: &Config ) -> HashMap<String, String>
    {
        let mut locale_path = PathBuf::from(&config.locale_path());
        locale_path.push(locale);
        let path = format!
        (
            "{}/**/*.{{yml,yaml,json,toml}}",
            locale_path.to_str().unwrap()
        );

        let glob = globwalk::glob(&path).unwrap();
        let mut map = HashMap::new();
        for entry in glob
        {
            let path = entry.unwrap().into_path();
            if path.is_dir() { continue; }
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap();
            let path = path.to_str().unwrap();
            map.extend(self.load_from_file(path, ext));
        }
        map
    }

    //--------------------------------------------------------------------------
    /// Loads the i18n map.
    //--------------------------------------------------------------------------
    fn load_from_file( &self, path: &str, ext: &str ) -> HashMap<String, String>
    {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        match ext
        {
            "yml" | "yaml" =>
            {
                serde_yaml::from_str(&content).unwrap()
            },
            "json" =>
            {
                serde_json::from_str(&content).unwrap()
            },
            "toml" =>
            {
                toml::from_str(&content).unwrap()
            },
            _ => HashMap::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Clears the i18n map.
    //--------------------------------------------------------------------------
    pub fn clear( &mut self )
    {
        self.map.clear();
    }

    //--------------------------------------------------------------------------
    /// Returns the locale.
    //--------------------------------------------------------------------------
    pub fn locale( &self ) -> &str
    {
        &self.locale
    }

    //--------------------------------------------------------------------------
    /// Sets the locale.
    //--------------------------------------------------------------------------
    pub fn set_locale( &mut self, locale: &str )
    {
        self.locale = locale.to_string();
    }

    //--------------------------------------------------------------------------
    /// Returns the map.
    //--------------------------------------------------------------------------
    pub fn map( &self ) -> &HashMap<String, String>
    {
        &self.map
    }

    //--------------------------------------------------------------------------
    /// Gets the message.
    //--------------------------------------------------------------------------
    pub fn get( &self, key: &str ) -> String
    {
        match self.map.get(key)
        {
            Some(value) => value.to_string(),
            None => match self.fallback.get(key)
            {
                Some(value) => value.to_string(),
                None => format!("{} not found", key),
            },
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the message and replace the placeholders.
    //--------------------------------------------------------------------------
    pub fn get_with
    (
        &self,
        key: &str,
        replace: HashMap<&str, &str>,
    ) -> String
    {
        let mut message = self.get(key);
        for (key, value) in replace
        {
            message = message.replace(&format!("%{}%", key), &value);
        }
        message
    }
}
