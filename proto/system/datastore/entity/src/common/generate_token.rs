//------------------------------------------------------------------------------
//! Generate token.
//------------------------------------------------------------------------------

use rand::thread_rng;
use rand::distributions::{ DistString, Alphanumeric };


//------------------------------------------------------------------------------
/// FieldHash.
//------------------------------------------------------------------------------
pub trait GenerateToken
{
    fn generate_token(&self) -> String
    {
        let mut rng = thread_rng();
        Alphanumeric.sample_string(&mut rng, 32)
    }
}
