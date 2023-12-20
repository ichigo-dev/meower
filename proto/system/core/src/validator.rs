//------------------------------------------------------------------------------
//! Validator.
//!
//! # Usage
//!
//! ```rust
//! use meower_utility::Validator;
//!
//! fn main()
//! {
//!    let mut validator = Validator::new("value")
//!        .not_empty("The value cannot be empty.")
//!        .min_len(5, "The value must be at least 5 characters.")
//!        .max_len(10, "The value must be at most 10 characters.")
//!        .same("value", "The value must be the same as the other.")
//!        .is_email("The value must be an email.")
//!        .validate();
//!
//!     if validator.has_err()
//!     {
//!         println!("Errors: {:?}", validator.errors());
//!     }
//! }
//! ```
//------------------------------------------------------------------------------

use regex::Regex;


//------------------------------------------------------------------------------
/// Validator.
//------------------------------------------------------------------------------
pub struct Validator
{
    errors: Vec<String>,
}

impl Validator
{
    //--------------------------------------------------------------------------
    /// Creates a new validator.
    //--------------------------------------------------------------------------
    pub fn new( value: &str ) -> ValidationBuilder
    {
        ValidationBuilder::new(value)
    }

    //--------------------------------------------------------------------------
    /// Returns the errors.
    //--------------------------------------------------------------------------
    pub fn errors( &self ) -> &Vec<String>
    {
        &self.errors
    }

    //--------------------------------------------------------------------------
    /// Returns the first error.
    //--------------------------------------------------------------------------
    pub fn get_first_error( &mut self ) -> String
    {
        self.errors.first().unwrap_or(&"".to_string()).to_string()
    }

    //--------------------------------------------------------------------------
    /// Returns the last error.
    //--------------------------------------------------------------------------
    pub fn get_last_error( &mut self ) -> String
    {
        self.errors.last().unwrap_or(&"".to_string()).to_string()
    }

    //--------------------------------------------------------------------------
    /// Validate.
    //--------------------------------------------------------------------------
    pub fn has_err( &mut self ) -> bool
    {
        !self.errors.is_empty()
    }
}


//------------------------------------------------------------------------------
/// Validation builder.
//------------------------------------------------------------------------------
pub struct ValidationBuilder
{
    value: String,
    errors: Vec<String>,
}

impl ValidationBuilder
{
    //--------------------------------------------------------------------------
    /// Creates a new validation builder.
    //--------------------------------------------------------------------------
    pub fn new( value: &str ) -> Self
    {
        Self
        {
            value: value.to_string(),
            errors: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Checks if the value is empty.
    //--------------------------------------------------------------------------
    pub fn not_empty( &mut self, msg: &str ) -> &mut Self
    {
        if self.value.is_empty()
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Checks if the value is greater than the min length.
    //--------------------------------------------------------------------------
    pub fn min_len( &mut self, min: usize, msg: &str ) -> &mut Self
    {
        if self.value.len() < min
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Checks if the value is less than the max length.
    //--------------------------------------------------------------------------
    pub fn max_len( &mut self, max: usize, msg: &str ) -> &mut Self
    {
        if self.value.len() > max
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Checks if the value is the same as the other.
    //--------------------------------------------------------------------------
    pub fn same( &mut self, other: &str, msg: &str ) -> &mut Self
    {
        if self.value != other
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Checks if the value is an email format.
    //--------------------------------------------------------------------------
    pub fn is_email( &mut self, msg: &str ) -> &mut Self
    {
        let regex = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
        self.regex(regex, msg)
    }

    //--------------------------------------------------------------------------
    /// Checks if the value matches the regex.
    //--------------------------------------------------------------------------
    pub fn regex( &mut self, regex: &str, msg: &str ) -> &mut Self
    {
        let regex = Regex::new(regex).unwrap();
        if !regex.is_match(&self.value)
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Custom validation.
    //--------------------------------------------------------------------------
    pub fn custom
    (
        &mut self,
        f: impl FnOnce(&str) -> bool,
        msg: &str,
    ) -> &mut Self
    {
        if !f(&self.value)
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Ends the validation.
    //--------------------------------------------------------------------------
    pub fn validate( &self ) -> Validator
    {
        Validator
        {
            errors: self.errors.to_vec(),
        }
    }
}
