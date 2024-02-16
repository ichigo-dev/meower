//------------------------------------------------------------------------------
//! JWT Claims utility. 
//------------------------------------------------------------------------------

use crate::Config;

use meower_auth_entity::client_application::Model as ClientApplicationModel;
use meower_auth_entity::user::Model as UserModel;
use meower_shared::JwtClaims;

use std::fs::File;
use std::io::Read;

use chrono::{ Utc, Duration };
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};

const JWT_EXPIRATION_MINUTES: i64 = 10;


//------------------------------------------------------------------------------
/// Creates JWT token.
//------------------------------------------------------------------------------
pub(crate) fn create_jwt_token
(
    config: &Config,
    user: &UserModel,
    client_application: &ClientApplicationModel,
) -> String
{
    let jwt_claims = create_jwt_claims(config, user, client_application);

    // Encodes JWT claims.
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::RS256;

    let key_path = "./env/".to_string() + &config.jwt_private_key;
    let mut key_data = String::new();
    let mut file = File::open(&key_path).unwrap();
    file.read_to_string(&mut key_data).unwrap();

    let key = EncodingKey::from_rsa_pem(key_data.as_bytes()).unwrap();
    encode(&header, &jwt_claims, &key).unwrap()
}


//------------------------------------------------------------------------------
/// Creates JWT claims.
//------------------------------------------------------------------------------
pub(crate) fn create_jwt_claims
(
    config: &Config,
    user: &UserModel,
    client_application: &ClientApplicationModel,
) -> JwtClaims
{
    let now = Utc::now();
    let iat = now.timestamp();
    let duration = Duration::minutes(JWT_EXPIRATION_MINUTES);
    let exp = (now + duration).timestamp();
    JwtClaims
    {
        jti: meower_utility::get_random_str(128),
        iss: config.url.clone(),
        sub: user.jwt_subject.clone(),
        aud: client_application.client_id.clone(),
        iat,
        exp,
        nbf: iat,

        // Private claims.
        public_user_id: user.public_user_id.clone(),
        user_email: user.email.clone(),
    }
}
