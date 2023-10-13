#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
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
use paste::paste;

pub use field_access_derive::*;

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

macro_rules! first_ok {
    ($value:expr, $($ty:ty => $map:expr),*) => {{
        $(if let Some(value) = $value.downcast_ref::<$ty>().map($map) {
            return Ok(value);
        })*

        return Err(AccessError::TypeMismatch);
    }};
}

pub trait FieldAccess: Any {
    fn get_field(&self, field: &str) -> Result<&dyn Any, AccessError>;

    fn get_field_mut(&mut self, field: &str) -> Result<&mut dyn Any, AccessError>;

    #[inline]
    fn field<'a>(&'a self, field: &'a str) -> FieldRef<'a>
    where
        Self: Sized,
    {
        FieldRef::new(field, self)
    }

    #[inline]
    fn field_mut<'a>(&'a mut self, field: &'a str) -> FieldMut<'a>
    where
        Self: Sized,
    {
        FieldMut::new(field, self)
    }

    #[inline]
    fn as_dyn_field_access(&self) -> &dyn FieldAccess
    where
        Self: Sized,
    {
        self
    }

    #[inline]
    fn as_dyn_field_access_mut(&mut self) -> &mut dyn FieldAccess
    where
        Self: Sized,
    {
        self
    }
}

impl dyn FieldAccess {
    #[inline]
    pub fn get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
        self.get_field(field).and_then(try_downcast_ref)
    }

    #[inline]
    pub fn get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
        self.get_field_mut(field).and_then(try_downcast_mut)
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
        self.replace(field, T::default())
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

macro_rules! primitive_getters {
    () => {};
    ($ty:ty $(,$rest:tt)*) => {
        primitive_getters!($ty => $ty $(,$rest)*);
    };
    ($ty:ty => $ident:tt $(,$rest:tt)*) => {
        paste! {
            #[inline]
            pub fn [<is_ $ident>](&self) -> bool {
                self.[<as_ $ident>]().is_ok()
            }

            #[inline]
            pub fn [<as_ $ident>](&self) -> Result<$ty, AccessError> {
                self.access.get(self.field).map(deref)
            }
        }

        primitive_getters!($($rest),*);
    };
}

macro_rules! getters {
    () => {
        #[inline]
        pub fn get<T: Any>(&self) -> Result<&T, AccessError> {
            self.access.get(self.field)
        }

        #[cfg(feature = "alloc")]
        #[inline]
        pub fn get_slice<T: Any>(&self) -> Result<&[T], AccessError> {
            self.access.get_field(self.field)
                       .and_then(|value| first_ok!(value, &[T] => deref, Vec<T> => Vec::as_slice))
        }

        #[cfg(not(feature = "alloc"))]
        #[inline]
        pub fn get_slice<T: Any>(&self) -> Result<&[T], AccessError> {
            self.access.get(self.field).map(deref)
        }

        #[inline]
        pub fn is_str(&self) -> bool {
            self.as_str().is_ok()
        }

        #[cfg(feature = "alloc")]
        #[inline]
        pub fn as_str(&self) -> Result<&str, AccessError> {
            self.access.get_field(self.field)
                       .and_then(|value| first_ok!(value, &str => deref, String => String::as_str))
        }

        #[cfg(not(feature = "alloc"))]
        #[inline]
        pub fn as_str(&self) -> Result<&str, AccessError> {
            self.access.get(self.field).map(deref)
        }

        primitive_getters!(u8, u16, u32, u64, u128);
        primitive_getters!(i8, i16, i32, i64, i128);
        primitive_getters!(f32, f64);
    };
}

pub struct FieldRef<'a> {
    field: &'a str,
    access: &'a dyn FieldAccess,
}

impl<'a> FieldRef<'a> {
    fn new(field: &'a str, access: &'a dyn FieldAccess) -> Self {
        FieldRef { field, access }
    }

    getters!();
}

pub struct FieldMut<'a> {
    field: &'a str,
    access: &'a mut dyn FieldAccess,
}

impl<'a> FieldMut<'a> {
    fn new(field: &'a str, access: &'a mut dyn FieldAccess) -> Self {
        FieldMut { field, access }
    }

    getters!();

    #[inline]
    pub fn get_mut<T: Any>(&mut self) -> Result<&mut T, AccessError> {
        self.access.get_mut(self.field)
    }

    #[inline]
    pub fn set<T: Any>(&mut self, value: T) -> Result<(), AccessError> {
        self.access.set(self.field, value)
    }

    #[inline]
    pub fn replace<T: Any>(&mut self, value: T) -> Result<T, AccessError> {
        self.access.replace(self.field, value)
    }

    #[inline]
    pub fn take<T: Any + Default>(&mut self) -> Result<T, AccessError> {
        self.access.take(self.field)
    }
}

#[inline]
fn try_downcast_ref<T: Any>(value: &dyn Any) -> Result<&T, AccessError> {
    value.downcast_ref().ok_or(AccessError::TypeMismatch)
}

#[inline]
fn try_downcast_mut<T: Any>(value: &mut dyn Any) -> Result<&mut T, AccessError> {
    value.downcast_mut().ok_or(AccessError::TypeMismatch)
}

#[inline]
fn deref<T: Copy>(t: &T) -> T {
    *t
}
