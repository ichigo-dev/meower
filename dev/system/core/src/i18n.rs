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

use crate::{ Config, LoadToStringMap };

use std::collections::HashMap;
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

        let mut locale_path = PathBuf::from(&config.get("locale_path"));
        locale_path.push(&self.locale);
        self.map = self.load(&locale_path);

        let mut locale_path = PathBuf::from(&config.get("locale_path"));
        locale_path.push(&config.get("fallback_locale"));
        self.fallback = self.load(&locale_path);
        true
    }

    //--------------------------------------------------------------------------
    /// Finds the locale path.
    //--------------------------------------------------------------------------
    fn find_locale_path( &mut self, config: &Config ) -> bool
    {
        let mut locale_path = PathBuf::from(&config.get("locale_path"));
        locale_path.push(&self.locale);
        if locale_path.exists() == false
        {
            let mut fallback_path = PathBuf::from(&config.get("locale_path"));
            fallback_path.push(&config.get("fallback_locale"));
            if fallback_path.exists() == false
            {
                return false;
            }
            self.locale = config.get("fallback_locale").to_string();
        }
        true
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

impl LoadToStringMap for I18n {}
