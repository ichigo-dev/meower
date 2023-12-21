//------------------------------------------------------------------------------
//! Code generation.
//------------------------------------------------------------------------------

use rand::{ thread_rng, Rng };


//------------------------------------------------------------------------------
/// Generates a random code.
//------------------------------------------------------------------------------
const DEFAULT_CODE_LENGTH: usize = 6;
pub fn generate_code( length: Option<usize> ) -> String
{
    let length = length.unwrap_or(DEFAULT_CODE_LENGTH);
    (0..length).map(|_| thread_rng().gen_range(0..10).to_string()).collect()
}
