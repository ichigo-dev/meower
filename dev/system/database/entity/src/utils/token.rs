//------------------------------------------------------------------------------
//! Token generation.
//------------------------------------------------------------------------------

use rand::thread_rng;
use rand::distributions::{ DistString, Alphanumeric };


//------------------------------------------------------------------------------
/// Generates a random token.
//------------------------------------------------------------------------------
const DEFAULT_TOKEN_LENGTH: usize = 32;
pub fn generate_token( length: Option<usize> ) -> String
{
    let length = length.unwrap_or(DEFAULT_TOKEN_LENGTH);
    let mut rng = thread_rng();
    Alphanumeric.sample_string(&mut rng, length)
}
