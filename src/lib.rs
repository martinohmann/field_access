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

/// The type returned for all errors that may occur when accessing a struct field.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessError {
    /// The field does not exist on the struct.
    NoSuchField,
    /// The field exists, but it was accessed using an incompatible type.
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
/// This trait can be implemented to provide access to the methods of the
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
///
/// This trait is automatically implemented by any type implementing
/// [`AnyFieldAccess`](AnyFieldAccess). See its documentation for more details.
pub trait FieldAccess: AnyFieldAccess {
    /// Returns `true` if a struct has a certain field.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 1 };
    ///
    /// assert!(foo.has_field("a"));
    /// assert!(!foo.has_field("b"));
    /// ```
    #[inline]
    fn has_field(&self, field: &str) -> bool {
        self.field_as_any(field).is_ok()
    }

    /// Immutable field access.
    ///
    /// The returned [`FieldRef`](FieldRef) provides methods to immutably interact with the field.
    /// See its documentation for more.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 1 };
    ///
    /// assert!(foo.field("a").is_u8());
    /// ```
    #[inline]
    fn field<'a>(&'a self, field: &'a str) -> FieldRef<'a>
    where
        Self: Sized,
    {
        FieldRef::new(self, field)
    }

    /// Mutable field access.
    ///
    /// The returned [`FieldMut`](FieldMut) provides methods to mutably interact with the field.
    /// See its documentation for more.
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let mut foo = Foo { a: 1 };
    ///
    /// assert_eq!(foo.field_mut("a").replace(2u8), Ok(1));
    /// assert_eq!(foo.a, 2);
    /// ```
    #[inline]
    fn field_mut<'a>(&'a mut self, field: &'a str) -> FieldMut<'a>
    where
        Self: Sized,
    {
        FieldMut::new(self, field)
    }
}

impl<T> FieldAccess for T where T: AnyFieldAccess {}

impl dyn FieldAccess {
    #[inline]
    fn get<T: Any>(&self, field: &str) -> Result<&T, AccessError> {
        self.field_as_any(field)
            .and_then(|value| value.downcast_ref().ok_or(AccessError::TypeMismatch))
    }

    #[inline]
    fn get_mut<T: Any>(&mut self, field: &str) -> Result<&mut T, AccessError> {
        self.field_as_any_mut(field)
            .and_then(|value| value.downcast_mut().ok_or(AccessError::TypeMismatch))
    }

    #[inline]
    fn set<T: Any>(&mut self, field: &str, value: T) -> Result<(), AccessError> {
        self.replace(field, value).map(|_| ())
    }

    #[inline]
    fn replace<T: Any>(&mut self, field: &str, value: T) -> Result<T, AccessError> {
        self.get_mut(field)
            .map(|dest| core::mem::replace(dest, value))
    }

    #[inline]
    fn take<T: Any + Default>(&mut self, field: &str) -> Result<T, AccessError> {
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

/// An immutable struct field reference.
pub struct FieldRef<'a> {
    access: &'a dyn FieldAccess,
    field: &'a str,
}

impl<'a> FieldRef<'a> {
    fn new(access: &'a dyn FieldAccess, field: &'a str) -> Self {
        FieldRef { access, field }
    }

    /// Returns `true` if the field is of type `T`.
    ///
    /// Please note that this also returns `false` if the field does not exist.
    ///
    /// To check for existence, use [`.exists()`](Self::exists).
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 1 };
    ///
    /// assert!(foo.field("a").is::<u8>());
    /// assert!(!foo.field("a").is::<&str>());
    /// ```
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        self.access
            .field_as_any(self.field)
            .map(|field| field.is::<T>())
            .unwrap_or(false)
    }

    /// Returns `true` if the field exists.
    ///
    /// If you don't have a `FieldRef` already, it is usually more convenient to use
    /// [`FieldAccess::has_field`](FieldAccess::has_field) instead.
    ///
    /// To check if the field exists and has a certain type, use [`.is::<T>()`](Self::is).
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 1 };
    ///
    /// assert!(foo.field("a").exists());
    /// assert!(!foo.field("b").exists());
    /// ```
    #[inline]
    pub fn exists(&self) -> bool {
        self.access.has_field(self.field)
    }

    /// Tries to obtain an immutable reference to the value of type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::{AccessError, FieldAccess};
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 42 };
    ///
    /// // Field `a` exists.
    /// assert_eq!(foo.field("a").get::<u8>(), Ok(&42u8));
    /// assert_eq!(foo.field("a").get::<&str>(), Err(AccessError::TypeMismatch));
    ///
    /// // Field `b` does not exist.
    /// assert_eq!(foo.field("b").get::<&str>(), Err(AccessError::NoSuchField));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`][AccessError].
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
            u16 | u8 => |&v| Some(v.into()),
            u64 | u128 => |&v| v.try_into().ok(),
        }
        u64 {
            u64 => |&v| Some(v),
            u32 | u16 | u8 => |&v| Some(v.into()),
            u128 => |&v| v.try_into().ok(),
        }
        u128 {
            u128 => |&v| Some(v),
            u8 | u16 | u32 | u64 => |&v| Some(v.into()),
        }
    }

    primitive_getters! {
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
            i16 | i8 => |&v| Some(v.into()),
            i64 | i128 => |&v| v.try_into().ok(),
        }
        i64 {
            i64 => |&v| Some(v),
            i32 | i16 | i8 => |&v| Some(v.into()),
            i128 => |&v| v.try_into().ok(),
        }
        i128 {
            i128 => |&v| Some(v),
            i8 | i16 | i32 | i64 => |&v| Some(v.into()),
        }
    }

    primitive_getters! {
        f32 {
            f32 => |&v| Some(v),
            f64 => |&v| Some(v as f32),
        }
        f64 {
            f64 => |&v| Some(v),
            f32 => |&v| Some(v.into()),
        }
    }
}

/// A mutable struct field reference.
pub struct FieldMut<'a> {
    access: &'a mut dyn FieldAccess,
    field: &'a str,
}

impl<'a> FieldMut<'a> {
    fn new(access: &'a mut dyn FieldAccess, field: &'a str) -> Self {
        FieldMut { access, field }
    }

    /// Tries to obtain a mutable reference to the value of type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::{AccessError, FieldAccess};
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let mut foo = Foo { a: 1 };
    ///
    /// // Field `a` exists.
    /// if let Ok(field) = foo.field_mut("a").get_mut::<u8>() {
    ///     *field = 42;
    /// }
    ///
    /// assert_eq!(foo.field("a").as_u8(), Ok(42u8));
    /// assert_eq!(foo.field_mut("a").get_mut::<&str>(), Err(AccessError::TypeMismatch));
    ///
    /// // Field `b` does not exist.
    /// assert_eq!(foo.field_mut("b").get_mut::<&str>(), Err(AccessError::NoSuchField));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`][AccessError].
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
