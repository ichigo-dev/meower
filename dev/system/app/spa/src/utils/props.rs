//------------------------------------------------------------------------------
//! Common Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// OptionProp.
//------------------------------------------------------------------------------
pub struct OptionProp<T>(pub Option<T>);

impl<T> Into<ReadSignal<Option<T>>> for OptionProp<T>
{
    fn into( self ) -> ReadSignal<Option<T>>
    {
        *create_signal(self.0)
    }
}

impl<T> Into<Signal<Option<T>>> for OptionProp<T>
{
    fn into( self ) -> Signal<Option<T>>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// StrProp.
//------------------------------------------------------------------------------
pub struct StrProp(pub &'static str);

impl Into<ReadSignal<String>> for StrProp
{
    fn into( self ) -> ReadSignal<String>
    {
        *create_signal(self.0.to_string())
    }
}

impl Into<Signal<String>> for StrProp
{
    fn into( self ) -> Signal<String>
    {
        create_signal(self.0.to_string())
    }
}


//------------------------------------------------------------------------------
/// StringProp.
//------------------------------------------------------------------------------
pub struct StringProp(pub String);

impl Into<ReadSignal<String>> for StringProp
{
    fn into( self ) -> ReadSignal<String>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<String>> for StringProp
{
    fn into( self ) -> Signal<String>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// BoolProp.
//------------------------------------------------------------------------------
pub struct BoolProp(pub bool);

impl Into<ReadSignal<bool>> for BoolProp
{
    fn into( self ) -> ReadSignal<bool>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<bool>> for BoolProp
{
    fn into( self ) -> Signal<bool>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// UsizeProp.
//------------------------------------------------------------------------------
pub struct UsizeProp(pub usize);

impl Into<ReadSignal<usize>> for UsizeProp
{
    fn into( self ) -> ReadSignal<usize>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<usize>> for UsizeProp
{
    fn into( self ) -> Signal<usize>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// U8Prop.
//------------------------------------------------------------------------------
pub struct U8Prop(pub u8);

impl Into<ReadSignal<u8>> for U8Prop
{
    fn into( self ) -> ReadSignal<u8>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<u8>> for U8Prop
{
    fn into( self ) -> Signal<u8>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// U16Prop.
//------------------------------------------------------------------------------
pub struct U16Prop(pub u16);

impl Into<ReadSignal<u16>> for U16Prop
{
    fn into( self ) -> ReadSignal<u16>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<u16>> for U16Prop
{
    fn into( self ) -> Signal<u16>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// U32Prop.
//------------------------------------------------------------------------------
pub struct U32Prop(pub u32);

impl Into<ReadSignal<u32>> for U32Prop
{
    fn into( self ) -> ReadSignal<u32>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<u32>> for U32Prop
{
    fn into( self ) -> Signal<u32>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// U64Prop.
//------------------------------------------------------------------------------
pub struct U64Prop(pub u64);

impl Into<ReadSignal<u64>> for U64Prop
{
    fn into( self ) -> ReadSignal<u64>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<u64>> for U64Prop
{
    fn into( self ) -> Signal<u64>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// U128Prop.
//------------------------------------------------------------------------------
pub struct U128Prop(pub u128);

impl Into<ReadSignal<u128>> for U128Prop
{
    fn into( self ) -> ReadSignal<u128>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<u128>> for U128Prop
{
    fn into( self ) -> Signal<u128>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// IsizeProp.
//------------------------------------------------------------------------------
pub struct IsizeProp(pub isize);

impl Into<ReadSignal<isize>> for IsizeProp
{
    fn into( self ) -> ReadSignal<isize>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<isize>> for IsizeProp
{
    fn into( self ) -> Signal<isize>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// I8Prop.
//------------------------------------------------------------------------------
pub struct I8Prop(pub i8);

impl Into<ReadSignal<i8>> for I8Prop
{
    fn into( self ) -> ReadSignal<i8>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<i8>> for I8Prop
{
    fn into( self ) -> Signal<i8>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// I16Prop.
//------------------------------------------------------------------------------
pub struct I16Prop(pub i16);

impl Into<ReadSignal<i16>> for I16Prop
{
    fn into( self ) -> ReadSignal<i16>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<i16>> for I16Prop
{
    fn into( self ) -> Signal<i16>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// I32Prop.
//------------------------------------------------------------------------------
pub struct I32Prop(pub i32);

impl Into<ReadSignal<i32>> for I32Prop
{
    fn into( self ) -> ReadSignal<i32>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<i32>> for I32Prop
{
    fn into( self ) -> Signal<i32>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// I64Prop.
//------------------------------------------------------------------------------
pub struct I64Prop(pub i64);

impl Into<ReadSignal<i64>> for I64Prop
{
    fn into( self ) -> ReadSignal<i64>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<i64>> for I64Prop
{
    fn into( self ) -> Signal<i64>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// I128Prop.
//------------------------------------------------------------------------------
pub struct I128Prop(pub i128);

impl Into<ReadSignal<i128>> for I128Prop
{
    fn into( self ) -> ReadSignal<i128>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<i128>> for I128Prop
{
    fn into( self ) -> Signal<i128>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// F32Prop.
//------------------------------------------------------------------------------
pub struct F32Prop(pub f32);

impl Into<ReadSignal<f32>> for F32Prop
{
    fn into( self ) -> ReadSignal<f32>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<f32>> for F32Prop
{
    fn into( self ) -> Signal<f32>
    {
        create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// F64Prop.
//------------------------------------------------------------------------------
pub struct F64Prop(pub f64);

impl Into<ReadSignal<f64>> for F64Prop
{
    fn into( self ) -> ReadSignal<f64>
    {
        *create_signal(self.0)
    }
}

impl Into<Signal<f64>> for F64Prop
{
    fn into( self ) -> Signal<f64>
    {
        create_signal(self.0)
    }
}
