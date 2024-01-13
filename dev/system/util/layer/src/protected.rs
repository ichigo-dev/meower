//------------------------------------------------------------------------------
//! Protected.
//------------------------------------------------------------------------------

use std::task::{ Poll, Context };
use meower_type::{ JwtClaims, JWT_CLAIMS_KEY };

use axum::body::Body;
use axum::response::Response;
use axum_extra::extract::cookie::CookieJar;
use http::Request;
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation,
};
use jsonwebtoken::errors::ErrorKind;
use tower::Service;
use tower_layer::Layer;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct ProtectedLayer
{
    jwt_audience: Vec<String>,
    jwt_secret: String,
}

impl ProtectedLayer
{
    //--------------------------------------------------------------------------
    /// Creates a new layer.
    //--------------------------------------------------------------------------
    pub fn new( jwt_audience: &Vec<String>, jwt_secret: &str ) -> Self
    {
        Self
        {
            jwt_audience: jwt_audience.to_vec(),
            jwt_secret: jwt_secret.to_string(),
        }
    }
}

impl<S> Layer<S> for ProtectedLayer
{
    type Service = ProtectedService<S>;

    //--------------------------------------------------------------------------
    /// Wraps a service.
    //--------------------------------------------------------------------------
    fn layer( &self, service: S ) -> Self::Service
    {
        ProtectedService
        {
            inner: service,
            jwt_audience: self.jwt_audience.clone(),
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}


//------------------------------------------------------------------------------
/// Service.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct ProtectedService<S>
{
    inner: S,
    jwt_audience: Vec<String>,
    jwt_secret: String,
}

impl<S> Service<Request<Body>> for ProtectedService<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    //--------------------------------------------------------------------------
    /// Polls the service.
    //--------------------------------------------------------------------------
    fn poll_ready
    (
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>
    {
        self.inner.poll_ready(cx)
    }

    //--------------------------------------------------------------------------
    /// Calls the service.
    //--------------------------------------------------------------------------
    fn call( &mut self, req: Request<Body> ) -> Self::Future
    {
        let cookie = CookieJar::from_headers(&req.headers());

        // Gets the JWT claims from the cookie.
        let jwt_claims_cookie = cookie
            .get(JWT_CLAIMS_KEY)
            .map(|cookie| cookie.value().to_string())
            .unwrap_or("".to_string());

        // Validates the JWT claims.
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&self.jwt_audience);
        if let Err(e) = decode::<JwtClaims>
        (
            &jwt_claims_cookie,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        {
            match e.kind()
            {
                ErrorKind::ExpiredSignature => println!("Expired signature"),
                _ => {},
            }
        }

        self.inner.call(req)
    }
}
