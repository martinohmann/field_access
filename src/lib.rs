#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::match_wildcard_for_single_variants,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use
)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::any::Any;
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessError {
    NoSuchField,
    TypeMismatch,
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        match self {
            AccessError::NoSuchField => f.write_str("no such field"),
            AccessError::TypeMismatch => f.write_str("type mismatch"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AccessError {}

macro_rules! primitive_methods {
    ($($ty:ty => $get:ident),*) => {
        $(
            #[inline]
            fn $get(&self, field: &str) -> Result<$ty, AccessError> {
                self.get_any(field)
                    .and_then(try_downcast_ref)
                    .map(|v| *v)
            }
        )*
    };
}

macro_rules! first_ok {
    ($value:expr, $($ty:ty => $map:expr),*) => {{
        $(if let Some(value) = $value.downcast_ref::<$ty>().map($map) {
            return Ok(value);
        })*

        return Err(AccessError::TypeMismatch);
    }};
}

pub trait FieldAccess {
    fn get_any(&self, field: &str) -> Result<&dyn Any, AccessError>;

    fn get_any_mut(&mut self, field: &str) -> Result<&mut dyn Any, AccessError>;

    #[cfg(feature = "alloc")]
    #[inline]
    fn get_str(&self, field: &str) -> Result<&str, AccessError> {
        self.get_any(field)
            .and_then(|value| first_ok!(value, &str => |v| *v, String => String::as_str))
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn get_str(&self, field: &str) -> Result<&str, AccessError> {
        self.get_any(field).and_then(try_downcast_ref).map(|v| *v)
    }

    primitive_methods! {
        bool => get_bool,
        u8 => get_u8,
        u16 => get_u16,
        u32 => get_u32,
        u64 => get_u64,
        u128 => get_u128,
        i8 => get_i8,
        i16 => get_i16,
        i32 => get_i32,
        i64 => get_i64,
        i128 => get_i128,
        f32 => get_f32,
        f64 => get_f64
    }
}

impl dyn FieldAccess {
    #[inline]
    pub fn get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
        self.get_any(field).and_then(try_downcast_ref)
    }

    #[inline]
    pub fn get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
        self.get_any_mut(field).and_then(try_downcast_mut)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
        self.get_any(field)
            .and_then(|value| first_ok!(value, &[T] => |v| *v, Vec<T> => Vec::as_slice))
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    pub fn get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
        self.get(field).map(|v| *v)
    }

    #[inline]
    pub fn set<T: Any>(&mut self, field: &str, value: T) -> Result<(), AccessError> {
        self.replace(field, value).map(|_| ())
    }

    #[inline]
    pub fn replace<T: Any>(&mut self, field: &str, value: T) -> Result<T, AccessError> {
        Ok(core::mem::replace(self.get_mut(field)?, value))
    }

    #[inline]
    pub fn take<T: Any + Default>(&mut self, field: &str) -> Result<T, AccessError> {
        Ok(core::mem::take(self.get_mut(field)?))
    }
}

macro_rules! forward_dyn_methods {
    () => {
        #[inline]
        pub fn get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
            <dyn FieldAccess>::get(self, field)
        }

        #[inline]
        pub fn get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
            <dyn FieldAccess>::get_mut(self, field)
        }

        #[inline]
        pub fn get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
            <dyn FieldAccess>::get_slice(self, field)
        }

        #[inline]
        pub fn set<T: Any>(&mut self, field: &str, value: T) -> Result<(), AccessError> {
            <dyn FieldAccess>::set(self, field, value)
        }

        #[inline]
        pub fn replace<T: Any>(&mut self, field: &str, value: T) -> Result<T, AccessError> {
            <dyn FieldAccess>::replace(self, field, value)
        }

        #[inline]
        pub fn take<T: Any + Default>(&mut self, field: &str) -> Result<T, AccessError> {
            <dyn FieldAccess>::take(self, field)
        }
    };
}

impl dyn FieldAccess + Send {
    forward_dyn_methods!();
}

impl dyn FieldAccess + Send + Sync {
    forward_dyn_methods!();
}

#[inline]
fn try_downcast_ref<T: Any>(value: &dyn Any) -> Result<&T, AccessError> {
    value.downcast_ref().ok_or(AccessError::TypeMismatch)
}

#[inline]
fn try_downcast_mut<T: Any>(value: &mut dyn Any) -> Result<&mut T, AccessError> {
    value.downcast_mut().ok_or(AccessError::TypeMismatch)
}
