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

macro_rules! impl_primitive_access {
    ($ty:ty, $get:ident, $try_get:ident) => {
        #[inline]
        fn $get(&self, field: &str) -> Option<$ty> {
            self.$try_get(field).ok()
        }

        #[inline]
        fn $try_get(&self, field: &str) -> Result<$ty, AccessError> {
            self.try_get_any(field)
                .and_then(try_downcast_ref::<$ty>)
                .map(|v| *v)
        }
    };
}

pub trait FieldAccess {
    #[inline]
    fn get_any(&self, field: &str) -> Option<&dyn Any> {
        self.try_get_any(field).ok()
    }

    fn try_get_any(&self, field: &str) -> Result<&dyn Any, AccessError>;

    #[inline]
    fn get_any_mut(&mut self, field: &str) -> Option<&mut dyn Any> {
        self.try_get_any_mut(field).ok()
    }

    fn try_get_any_mut(&mut self, field: &str) -> Result<&mut dyn Any, AccessError>;

    impl_primitive_access!(bool, get_bool, try_get_bool);
    impl_primitive_access!(u8, get_u8, try_get_u8);
    impl_primitive_access!(u16, get_u16, try_get_u16);
    impl_primitive_access!(u32, get_u32, try_get_u32);
    impl_primitive_access!(u64, get_u64, try_get_u64);
    impl_primitive_access!(u128, get_u128, try_get_u128);
    impl_primitive_access!(i8, get_i8, try_get_i8);
    impl_primitive_access!(i16, get_i16, try_get_i16);
    impl_primitive_access!(i32, get_i32, try_get_i32);
    impl_primitive_access!(i64, get_i64, try_get_i64);
    impl_primitive_access!(i128, get_i128, try_get_i128);
    impl_primitive_access!(f32, get_f32, try_get_f32);
    impl_primitive_access!(f64, get_f64, try_get_f64);

    #[inline]
    fn get_str(&self, field: &str) -> Option<&str> {
        self.try_get_str(field).ok()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn try_get_str(&self, field: &str) -> Result<&str, AccessError> {
        self.try_get_any(field)
            .and_then(|value| match value.downcast_ref::<&str>() {
                Some(v) => Ok(*v),
                None => try_downcast_ref::<String>(value).map(String::as_str),
            })
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn try_get_str(&self, field: &str) -> Result<&str, AccessError> {
        self.try_get_any(field)
            .and_then(try_downcast_ref::<&str>)
            .map(|v| *v)
    }
}

impl dyn FieldAccess + '_ {
    #[inline]
    pub fn get<T: Any>(&self, field: &str) -> Option<&T> {
        self.try_get(field).ok()
    }

    #[inline]
    pub fn try_get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
        self.try_get_any(field).and_then(try_downcast_ref::<T>)
    }

    #[inline]
    pub fn get_mut<T: Any>(&mut self, field: &str) -> Option<&mut T> {
        self.try_get_mut(field).ok()
    }

    #[inline]
    pub fn try_get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
        self.try_get_any_mut(field).and_then(try_downcast_mut::<T>)
    }

    #[inline]
    pub fn get_slice<T: Any>(&self, field: &str) -> Option<&[T]> {
        self.try_get_slice(field).ok()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn try_get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
        self.try_get_any(field)
            .and_then(|value| match value.downcast_ref::<&[T]>() {
                Some(v) => Ok(*v),
                None => try_downcast_ref::<Vec<T>>(value).map(Vec::as_slice),
            })
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    pub fn try_get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
        self.try_get_any(field)
            .and_then(try_downcast_ref::<&[T]>)
            .map(|v| *v)
    }

    #[inline]
    pub fn set<T: Any>(&mut self, field: &str, value: T) -> Result<(), AccessError> {
        let f = self.try_get_mut(field)?;
        *f = value;
        Ok(())
    }
}

macro_rules! forward_field_access_methods {
    () => {
        #[inline]
        pub fn get<T: Any>(&self, field: &str) -> Option<&T> {
            <dyn FieldAccess>::get(self, field)
        }

        #[inline]
        pub fn try_get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
            <dyn FieldAccess>::try_get(self, field)
        }

        #[inline]
        pub fn get_mut<T: Any>(&mut self, field: &str) -> Option<&mut T> {
            <dyn FieldAccess>::get_mut(self, field)
        }

        #[inline]
        pub fn try_get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
            <dyn FieldAccess>::try_get_mut(self, field)
        }

        #[inline]
        pub fn get_slice<T: Any>(&self, field: &str) -> Option<&[T]> {
            <dyn FieldAccess>::get_slice(self, field)
        }

        #[inline]
        pub fn try_get_slice<T: Any>(&self, field: &str) -> Result<&[T], AccessError> {
            <dyn FieldAccess>::try_get_slice(self, field)
        }

        #[inline]
        pub fn set<T: Any>(&mut self, field: &str, value: T) -> Result<(), AccessError> {
            <dyn FieldAccess>::set(self, field, value)
        }
    };
}

impl dyn FieldAccess + Send + '_ {
    forward_field_access_methods!();
}

impl dyn FieldAccess + Send + Sync + '_ {
    forward_field_access_methods!();
}

#[inline]
fn try_downcast_ref<T: Any>(value: &dyn Any) -> Result<&T, AccessError> {
    value.downcast_ref().ok_or(AccessError::TypeMismatch)
}

#[inline]
fn try_downcast_mut<T: Any>(value: &mut dyn Any) -> Result<&mut T, AccessError> {
    value.downcast_mut().ok_or(AccessError::TypeMismatch)
}
