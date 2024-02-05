//------------------------------------------------------------------------------
/// SetTimeout.
//------------------------------------------------------------------------------

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C"
{
    fn setTimeout( closure: &Closure<dyn FnMut()>, millis: u32 ) -> f64;
    fn clearTimeout( token: f64 );
}

#[wasm_bindgen]
pub struct Timeout
{
    closure: Closure<dyn FnMut()>,
    millis: u32,
    token: f64,
}

impl Timeout
{
    //--------------------------------------------------------------------------
    /// Creates a new instance.
    //--------------------------------------------------------------------------
    pub fn new<F: 'static>( f: F, millis: u32 ) -> Self
    where
        F: FnMut()
    {
        let closure = Closure::new(f);
        Self { closure, millis, token: 0 as f64 }
    }

    //--------------------------------------------------------------------------
    /// Start.
    //--------------------------------------------------------------------------
    pub fn start( &mut self )
    {
        let token = setTimeout(&self.closure, self.millis);
        self.token = token;
    }

    //--------------------------------------------------------------------------
    /// Cancels.
    //--------------------------------------------------------------------------
    pub fn cancel( &self )
    {
        clearTimeout(self.token);
    }
}

impl Drop for Timeout
{
    fn drop( &mut self )
    {
        clearTimeout(self.token);
    }
}
