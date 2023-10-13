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

/// Low-level struct field access.
///
/// This trait can be implemented to provides access to the methods of the
/// [`FieldAccess`](FieldAccess) trait which has a blanket implementation for any type
/// implementing `AnyFieldAccess`.
///
/// Consider automatically implementing it via `#[derive(FieldAccess)]` for structs where you need
/// dynamic field access.
pub trait AnyFieldAccess: Any {
    /// Provides an immutable reference to a struct field.
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`][AccessError].
    fn field_as_any(&self, field: &str) -> Result<&dyn Any, AccessError>;

    /// Provides a mutable reference to a struct field.
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`][AccessError].
    fn field_as_any_mut(&mut self, field: &str) -> Result<&mut dyn Any, AccessError>;
}

/// High-level struct field access.
pub trait FieldAccess: AnyFieldAccess {
    #[inline]
    fn has_field(&self, field: &str) -> bool {
        self.field_as_any(field).is_ok()
    }

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
}

impl<T> FieldAccess for T where T: AnyFieldAccess {}

impl dyn FieldAccess {
    #[inline]
    pub fn get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
        self.field_as_any(field).and_then(try_downcast_ref)
    }

    #[inline]
    pub fn get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
        self.field_as_any_mut(field).and_then(try_downcast_mut)
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

macro_rules! match_downcast_ref {
    ($value:expr, $($($ty:ty)|+ => $map:expr),* $(,)?) => {{
        $($(if let Some(value) = $value.downcast_ref::<$ty>().and_then($map) {
            return Ok(value);
        })*)*

        return Err(AccessError::TypeMismatch);
    }};
}

macro_rules! primitive_getter {
    ($ty:ty { $($rest:tt)+ }) => {
        primitive_getter!($ty => $ty { $($rest)* });
    };
    ($ty:ty => $ident:tt { $($rest:tt)+ }) => {
        paste! {
            /// Returns `true` if the field value is of type
            #[doc = concat!("`", stringify!($ty), r"`.")]
            #[inline]
            pub fn [<is_ $ident>](&self) -> bool {
                self.is::<$ty>()
            }

            /// Returns the field value as
            #[doc = concat!("`", stringify!($ty), "`")]
            /// .
            ///
            /// This method is guaranteed to return `Ok(_)` if
            #[doc = concat!("[`.is_", stringify!($ident), "()`][Self::is_", stringify!($ident) ,"]")]
            /// returns `true`.
            ///
            /// It may also return `Ok(_)` if it is possible to perform a lossless conversion of
            /// the field's value into
            #[doc = concat!("`", stringify!($ty), "`")]
            /// .
            ///
            /// # Errors
            ///
            /// See the documentation of [`AccessError`][AccessError].
            #[inline]
            pub fn [<as_ $ident>](&self) -> Result<$ty, AccessError> {
                self.access.field_as_any(self.field).and_then(|value| {
                    match_downcast_ref!(
                        value,
                        $($rest)*
                    )
                })
            }
        }
    };
}

macro_rules! primitive_getters {
    () => {};
    ($ty:ty { $($body:tt)* } $($rest:tt)*) => {
        primitive_getters!($ty => $ty { $($body)* } $($rest)*);
    };
    ($ty:ty => $ident:tt { $($body:tt)* } $($rest:tt)*) => {
        primitive_getter!($ty => $ident { $($body)* });
        primitive_getters!($($rest)*);
    };
}

macro_rules! immutable_field_methods {
    () => {
        #[inline]
        pub fn is<T: Any>(&self) -> bool {
            self.access.field_as_any(self.field)
                       .map(|field| field.is::<T>())
                       .unwrap_or(false)
        }

        #[inline]
        pub fn get<T: Any>(&self) -> Result<&T, AccessError> {
            self.access.get(self.field)
        }

        #[cfg(feature = "alloc")]
        #[inline]
        pub fn as_slice<T: Any>(&self) -> Result<&[T], AccessError> {
            self.access.field_as_any(self.field).and_then(|value| {
                match_downcast_ref!(
                    value,
                    &[T] => |&v| Some(v),
                    Vec<T> => |v| Some(v.as_slice())
                )
            })
        }

        #[cfg(not(feature = "alloc"))]
        #[inline]
        pub fn as_slice<T: Any>(&self) -> Result<&[T], AccessError> {
            self.access.get(self.field).map(|&v| v)
        }

        #[cfg(feature = "alloc")]
        primitive_getters! {
            &str => str {
                &str => |&v| Some(v),
                String => |v| Some(v.as_str())
            }
        }

        #[cfg(not(feature = "alloc"))]
        primitive_getters! {
            &str => str {
                &str => |&v| Some(v)
            }
        }

        primitive_getters! {
            u8 {
                u8 => |&v| Some(v),
                u16 | u32 | u64 | u128 => |&v| v.try_into().ok(),
            }
            u16 {
                u16 => |&v| Some(v),
                u8 => |&v| Some(v.into()),
                u32 | u64 | u128 => |&v| v.try_into().ok(),
            }
            u32 {
                u32 => |&v| Some(v),
                u8 | u16 => |&v| Some(v.into()),
                u64 | u128 => |&v| v.try_into().ok(),
            }
            u64 {
                u64 => |&v| Some(v),
                u8 | u16 | u32 => |&v| Some(v.into()),
                u128 => |&v| v.try_into().ok(),
            }
            u128 {
                u128 => |&v| Some(v),
                u8 | u16 | u32 | u64 => |&v| Some(v.into()),
            }
            i8 {
                i8 => |&v| Some(v),
                i16 | i32 | i64 | i128 => |&v| v.try_into().ok(),
            }
            i16 {
                i16 => |&v| Some(v),
                i8 => |&v| Some(v.into()),
                i32 | i64 | i128 => |&v| v.try_into().ok(),
            }
            i32 {
                i32 => |&v| Some(v),
                i8 | i16 => |&v| Some(v.into()),
                i64 | i128 => |&v| v.try_into().ok(),
            }
            i64 {
                i64 => |&v| Some(v),
                i8 | i16 | i32 => |&v| Some(v.into()),
                i128 => |&v| v.try_into().ok(),
            }
            i128 {
                i128 => |&v| Some(v),
                i8 | i16 | i32 | i64 => |&v| Some(v.into()),
            }
            f32 {
                f32 => |&v| Some(v),
                f64 => |&v| Some(v as f32),
            }
            f64 {
                f64 => |&v| Some(v),
                f32 => |&v| Some(v.into()),
            }
        }
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

    immutable_field_methods!();
}

pub struct FieldMut<'a> {
    field: &'a str,
    access: &'a mut dyn FieldAccess,
}

impl<'a> FieldMut<'a> {
    fn new(field: &'a str, access: &'a mut dyn FieldAccess) -> Self {
        FieldMut { field, access }
    }

    immutable_field_methods!();

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
