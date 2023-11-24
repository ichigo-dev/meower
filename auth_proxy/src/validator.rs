//------------------------------------------------------------------------------
//! Validator.
//------------------------------------------------------------------------------

use regex::Regex;


//------------------------------------------------------------------------------
/// Validator.
//------------------------------------------------------------------------------
pub(crate) struct Validator
{
    errors: Vec<String>,
}

impl Validator
{
    //--------------------------------------------------------------------------
    /// Creates a new validator.
    //--------------------------------------------------------------------------
    pub(crate) fn new( value: &str ) -> ValidationBuilder
    {
        ValidationBuilder::new(value)
    }

    //--------------------------------------------------------------------------
    /// Returns the errors.
    //--------------------------------------------------------------------------
    pub(crate) fn errors( &self ) -> &Vec<String>
    {
        &self.errors
    }

    //--------------------------------------------------------------------------
    /// Validate.
    //--------------------------------------------------------------------------
    pub(crate) fn validate( &mut self ) -> bool
    {
        self.errors.is_empty()
    }
}


//------------------------------------------------------------------------------
/// Validation builder.
//------------------------------------------------------------------------------
pub(crate) struct ValidationBuilder
{
    value: String,
    errors: Vec<String>,
}

impl ValidationBuilder
{
    //--------------------------------------------------------------------------
    /// Creates a new validation builder.
    //--------------------------------------------------------------------------
    pub(crate) fn new( value: &str ) -> Self
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
    pub(crate) fn not_empty( &mut self, msg: &str ) -> &mut Self
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
    pub(crate) fn min_len( &mut self, min: usize, msg: &str ) -> &mut Self
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
    pub(crate) fn max_len( &mut self, max: usize, msg: &str ) -> &mut Self
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
    pub(crate) fn same( &mut self, other: &str, msg: &str ) -> &mut Self
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
    pub(crate) fn is_email( &mut self, msg: &str ) -> &mut Self
    {
        let regex = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
        self.regex(regex, msg)
    }

    //--------------------------------------------------------------------------
    /// Checks if the value matches the regex.
    //--------------------------------------------------------------------------
    pub(crate) fn regex( &mut self, regex: &str, msg: &str ) -> &mut Self
    {
        let regex = Regex::new(regex).unwrap();
        if !regex.is_match(&self.value)
        {
            self.errors.push(msg.to_string());
        }
        self
    }

    //--------------------------------------------------------------------------
    /// Ends the validation.
    //--------------------------------------------------------------------------
    pub(crate) fn validate( &self ) -> Validator
    {
        Validator
        {
            errors: self.errors.to_vec(),
        }
    }
}
