//------------------------------------------------------------------------------
//! Validator.
//!
//!
//! # Usage
//!
//! ```
//! use validator::{ Validator, ValidationError };
//!
//! let mut validator = Validator::new()
//!    .required()
//!    .min_length(3)
//!    .max_length(10)
//!    .is_alphanumeric();
//! if let Err(err) = validator.validate("abc")
//! {
//!     println!("Error: {:?}", err);
//! }
//! ```
//------------------------------------------------------------------------------

use regex::Regex;
use rust_i18n::t;
use thiserror::Error;

rust_i18n::i18n!("locales");


//------------------------------------------------------------------------------
/// ValidationError.
//------------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum ValidationError
{
    // Required
    #[error("Required")]
    Required,

    // Length
    #[error("Too short: {0}")]
    TooShort(usize),
    #[error("Too long: {0}")]
    TooLong(usize),

    // Format
    #[error("Invalid ASCII")]
    InvalidAscii,
    #[error("Invalid alphanumeric")]
    InvalidAlphanumeric,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid URL")]
    InvalidUrl,

    // Custom
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Custom: {0}")]
    Custom(String),
}

impl ValidationError
{
    //--------------------------------------------------------------------------
    /// Gets an error message.
    //--------------------------------------------------------------------------
    pub fn get_error_message( &self, column: &str ) -> String
    {
        match self
        {
            // Required
            Self::Required =>
            {
                return t!("validator.error.required", column = column);
            },

            // Length
            Self::TooShort(min) =>
            {
                return t!
                (
                    "validator.error.too_short",
                    column = column, min = min
                );
            },
            Self::TooLong(max) =>
            {
                return t!
                (
                    "validator.error.too_long",
                    column = column, max = max
                );
            },

            // Format
            Self::InvalidAscii =>
            {
                return t!("validator.error.invalid_ascii", column = column);
            },
            Self::InvalidAlphanumeric =>
            {
                return t!
                (
                    "validator.error.invalid_alphanumeric",
                    column = column,
                );
            },
            Self::InvalidEmail =>
            {
                return t!("validator.error.invalid_email", column = column);
            },
            Self::InvalidUrl =>
            {
                return t!("validator.error.invalid_url", column = column);
            },

            // Custom
            Self::InvalidFormat =>
            {
                return t!("validator.error.invalid_format", column = column);
            },
            Self::Custom(message) =>
            {
                return message.clone();
            },
        }
    }
}


//------------------------------------------------------------------------------
/// ValidationRule.
//------------------------------------------------------------------------------
pub enum ValidationRule
{
    // Required
    Required,

    // Length
    MinLength(usize),
    MaxLength(usize),

    // Format
    Ascii,
    Alphanumeric,
    Email,
    Url,

    // Custom
    Regex(Regex),
    Custom(Box<dyn Fn(&str) -> Result<(), ValidationError>>),
}

impl ValidationRule
{
    //--------------------------------------------------------------------------
    /// Validates a value.
    //--------------------------------------------------------------------------
    fn validate( &self, value: &str ) -> Result<(), ValidationError>
    {
        match self
        {
            // Required
            ValidationRule::Required =>
            {
                if value.is_empty()
                {
                    return Err(ValidationError::Required);
                }
            },

            // Length
            ValidationRule::MinLength(min) =>
            {
                if value.len() < *min
                {
                    return Err(ValidationError::TooShort(*min));
                }
            },
            ValidationRule::MaxLength(max) =>
            {
                if value.len() > *max
                {
                    return Err(ValidationError::TooLong(*max));
                }
            },

            // Format
            ValidationRule::Ascii =>
            {
                if !value.is_ascii()
                {
                    return Err(ValidationError::InvalidAscii);
                }
            },
            ValidationRule::Alphanumeric =>
            {
                let chars = value.chars();
                for c in chars
                {
                    if !c.is_alphanumeric()
                    {
                        return Err(ValidationError::InvalidAlphanumeric);
                    }
                }
            },
            ValidationRule::Email =>
            {
                let regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
                if !regex.is_match(value)
                {
                    return Err(ValidationError::InvalidEmail);
                }
            },
            ValidationRule::Url =>
            {
                let regex = Regex::new(r"^(https?)://[^\s/$.?#].[^\s]*$").unwrap();
                if !regex.is_match(value)
                {
                    return Err(ValidationError::InvalidUrl);
                }
            },

            // Custom
            ValidationRule::Regex(regex) =>
            {
                if !regex.is_match(value)
                {
                    return Err(ValidationError::InvalidFormat);
                }
            },
            ValidationRule::Custom(func) =>
            {
                func(value)?;
            },
        }

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Validator.
//------------------------------------------------------------------------------
pub struct Validator
{
    rules: Vec<ValidationRule>,
}

impl Validator
{
    //--------------------------------------------------------------------------
    /// Creates a new Validator.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self
        {
            rules: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Adds a rule.
    //--------------------------------------------------------------------------
    pub fn add_rule( &mut self, rule: ValidationRule ) -> &mut Self
    {
        self.rules.push(rule);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Required rule.
    //--------------------------------------------------------------------------
    pub fn required( &mut self ) -> &mut Self
    {
        self.add_rule(ValidationRule::Required);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds MaxLength rule.
    //--------------------------------------------------------------------------
    pub fn max_length( &mut self, max: usize ) -> &mut Self
    {
        self.add_rule(ValidationRule::MaxLength(max));
        self
    }

    //--------------------------------------------------------------------------
    /// Adds MinLength rule.
    //--------------------------------------------------------------------------
    pub fn min_length( &mut self, min: usize ) -> &mut Self
    {
        self.add_rule(ValidationRule::MinLength(min));
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Ascii rule.
    //--------------------------------------------------------------------------
    pub fn is_ascii( &mut self ) -> &mut Self
    {
        self.add_rule(ValidationRule::Ascii);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Alphanumeric rule.
    //--------------------------------------------------------------------------
    pub fn is_alphanumeric( &mut self ) -> &mut Self
    {
        self.add_rule(ValidationRule::Alphanumeric);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Email rule.
    //--------------------------------------------------------------------------
    pub fn is_email( &mut self ) -> &mut Self
    {
        self.add_rule(ValidationRule::Email);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Url rule.
    //--------------------------------------------------------------------------
    pub fn is_url( &mut self ) -> &mut Self
    {
        self.add_rule(ValidationRule::Url);
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Regex rule.
    //--------------------------------------------------------------------------
    pub fn regex( &mut self, regex: &str ) -> &mut Self
    {
        let regex = Regex::new(regex).unwrap();
        self.add_rule(ValidationRule::Regex(regex));
        self
    }

    //--------------------------------------------------------------------------
    /// Adds Custom rule.
    //--------------------------------------------------------------------------
    pub fn custom
    (
        &mut self,
        func: Box<dyn Fn(&str) -> Result<(), ValidationError>>,
    ) -> &mut Self
    {
        self.add_rule(ValidationRule::Custom(func));
        self
    }

    //--------------------------------------------------------------------------
    /// Validates a value.
    //--------------------------------------------------------------------------
    pub fn validate( &self, value: &str ) -> Result<(), ValidationError>
    {
        for rule in &self.rules
        {
            rule.validate(value)?;
        }
        Ok(())
    }
}
