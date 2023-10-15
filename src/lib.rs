#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::fmt;
use core::iter::FusedIterator;
use core::slice;
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
/// This trait can be implemented to provide access to the methods of the [`FieldAccess`] trait
/// which has a blanket implementation for any type implementing `AnyFieldAccess`.
///
/// Consider automatically implementing it via `#[derive(FieldAccess)]` for structs where you need
/// dynamic field access.
pub trait AnyFieldAccess: Any {
    /// Provides an immutable reference to a struct field.
    fn field_as_any(&self, field: &str) -> Option<&dyn Any>;

    /// Provides a mutable reference to a struct field.
    fn field_as_any_mut(&mut self, field: &str) -> Option<&mut dyn Any>;

    /// Provides the names of all accessible fields.
    fn field_names(&self) -> &'static [&'static str];
}

/// High-level struct field access.
///
/// This trait is automatically implemented by any type implementing
/// [`AnyFieldAccess`]. See its documentation for more details.
pub trait FieldAccess: AnyFieldAccess {
    /// Immutable field access.
    ///
    /// The returned [`FieldRef`] provides methods to immutably interact with the field. See its
    /// documentation for more.
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
    /// The returned [`FieldMut`] provides methods to mutably interact with the field. See its
    /// documentation for more.
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

    /// Returns an iterator over all struct fields.
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8,
    ///     b: u16,
    ///     c: u32
    /// }
    ///
    /// let foo = Foo { a: 1, b: 2, c: 3 };
    /// let tuples: Vec<_> = foo.fields()
    ///                         .map(|field| (field.name().to_owned(), field.as_u8()))
    ///                         .collect();
    /// assert_eq!(tuples, &[(String::from("a"), Ok(1)),
    ///                      (String::from("b"), Ok(2)),
    ///                      (String::from("c"), Ok(3))])
    /// ```
    #[inline]
    fn fields(&self) -> Fields<'_>
    where
        Self: Sized,
    {
        Fields::new(self)
    }
}

impl<T> FieldAccess for T where T: AnyFieldAccess {}

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
            /// See the documentation of [`AccessError`].
            #[inline]
            pub fn [<as_ $ident>](&self) -> Result<$ty, AccessError> {
                self.as_any().and_then(|value| {
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
///
/// A `FieldRef` is a proxy for immutable operations on a struct's field.
///
/// Values of this type are created by [`FieldAccess::field`].
pub struct FieldRef<'a> {
    access: &'a dyn AnyFieldAccess,
    field: &'a str,
}

impl<'a> FieldRef<'a> {
    fn new(access: &'a dyn AnyFieldAccess, field: &'a str) -> Self {
        FieldRef { access, field }
    }

    /// Returns the field's name.
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
    /// let field = foo.field("a");
    ///
    /// assert_eq!(field.name(), "a");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        self.field
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
        self.as_any().map_or(false, <dyn Any>::is::<T>)
    }

    /// Returns `true` if the field exists.
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
        self.as_any().is_ok()
    }

    /// Gets the `TypeId` of the field's value.
    ///
    /// # Example
    ///
    /// ```
    /// use core::any::TypeId;
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: u8
    /// }
    ///
    /// let foo = Foo { a: 1 };
    ///
    /// assert_eq!(foo.field("a").type_id(), Ok(TypeId::of::<u8>()));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn type_id(&self) -> Result<TypeId, AccessError> {
        self.as_any().map(<dyn Any>::type_id)
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
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn get<T: Any>(&self) -> Result<&T, AccessError> {
        self.as_any()
            .and_then(|value| value.downcast_ref().ok_or(AccessError::TypeMismatch))
    }

    /// Tries to obtain an immutable reference to the value as `&dyn Any`.
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
    /// let field = foo.field("a");
    /// let any = field.as_any().unwrap();
    ///
    /// assert_eq!(any.downcast_ref::<u8>(), Some(&42u8));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn as_any(&self) -> Result<&dyn Any, AccessError> {
        self.access
            .field_as_any(self.field)
            .ok_or(AccessError::NoSuchField)
    }

    /// Tries to obtain an the value as `&[T]`.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::{AccessError, FieldAccess};
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: Vec<u8>
    /// }
    ///
    /// let foo = Foo { a: vec![1, 2, 3] };
    ///
    /// assert_eq!(foo.field("a").as_slice(), Ok(foo.a.as_slice()));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn as_slice<T: Any>(&self) -> Result<&[T], AccessError> {
        self.as_any().and_then(|value| {
            match_downcast_ref!(
                value,
                &[T] => |&v| Some(v),
                Vec<T> => |v| Some(v.as_slice())
            )
        })
    }

    /// Tries to obtain an the value as `&[T]`.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::{AccessError, FieldAccess};
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: &'static [u8]
    /// }
    ///
    /// let foo = Foo { a: &[1, 2, 3] };
    ///
    /// assert_eq!(foo.field("a").as_slice(), Ok(foo.a));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[cfg(not(feature = "alloc"))]
    #[inline]
    pub fn as_slice<T: Any>(&self) -> Result<&[T], AccessError> {
        self.get().map(|&v| v)
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
        }
        f64 {
            f64 => |&v| Some(v),
            f32 => |&v| Some(v.into()),
        }
    }
}

/// A mutable struct field reference.
///
/// A `FieldMut` is a proxy for mutable operations on a struct's field.
///
/// Values of this type are created by [`FieldAccess::field_mut`].
pub struct FieldMut<'a> {
    access: &'a mut dyn AnyFieldAccess,
    field: &'a str,
}

impl<'a> FieldMut<'a> {
    fn new(access: &'a mut dyn AnyFieldAccess, field: &'a str) -> Self {
        FieldMut { access, field }
    }

    /// Returns the field's name.
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
    /// let mut foo = Foo { a: 1 };
    /// let mut field = foo.field_mut("a");
    ///
    /// assert_eq!(field.name(), "a");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        self.field
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
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn get_mut<T: Any>(&mut self) -> Result<&mut T, AccessError> {
        self.as_any_mut()
            .and_then(|value| value.downcast_mut().ok_or(AccessError::TypeMismatch))
    }

    /// Tries to obtain a mutable reference to the value as `&mut dyn Any`.
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
    /// let mut foo = Foo { a: 42 };
    ///
    /// let mut field = foo.field_mut("a");
    /// let any = field.as_any_mut().unwrap();
    ///
    /// if let Some(value) = any.downcast_mut::<u8>() {
    ///     *value = 42;
    /// }
    ///
    /// assert_eq!(foo.a, 42);
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn as_any_mut(&mut self) -> Result<&mut dyn Any, AccessError> {
        self.access
            .field_as_any_mut(self.field)
            .ok_or(AccessError::NoSuchField)
    }

    /// Sets the value of the field.
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
    /// foo.field_mut("a").set(42u8).unwrap();
    ///
    /// assert_eq!(foo.a, 42);
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn set<T: Any>(&mut self, value: T) -> Result<(), AccessError> {
        self.replace(value).map(|_| ())
    }

    /// Replaces the value of the field, returning the previous value.
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
    /// assert_eq!(foo.field_mut("a").replace(42u8), Ok(1));
    /// assert_eq!(foo.a, 42);
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn replace<T: Any>(&mut self, value: T) -> Result<T, AccessError> {
        self.get_mut().map(|dest| core::mem::replace(dest, value))
    }

    /// Swaps the value of the field and another mutable location.
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
    /// let mut value = 2u8;
    ///
    /// assert_eq!(foo.field_mut("a").swap(&mut value), Ok(()));
    /// assert_eq!(foo.a, 2);
    /// assert_eq!(value, 1);
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn swap<T: Any>(&mut self, value: &mut T) -> Result<(), AccessError> {
        self.get_mut().map(|dest| core::mem::swap(dest, value))
    }

    /// Takes the value of the field, replacing it with its default value.
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
    /// let mut foo = Foo { a: 42 };
    ///
    /// assert_eq!(foo.field_mut("a").take(), Ok(42u8));
    /// assert_eq!(foo.a, 0);
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation of [`AccessError`].
    #[inline]
    pub fn take<T: Any + Default>(&mut self) -> Result<T, AccessError> {
        self.replace(T::default())
    }
}

/// An immutable iterator over all fields of a struct.
///
/// Values of this type are created by [`FieldAccess::fields`].
#[derive(Clone)]
pub struct Fields<'a> {
    access: &'a dyn AnyFieldAccess,
    field_names: slice::Iter<'a, &'static str>,
}

impl<'a> Fields<'a> {
    fn new(access: &'a dyn AnyFieldAccess) -> Self {
        Fields {
            access,
            field_names: access.field_names().iter(),
        }
    }
}

impl<'a> Iterator for Fields<'a> {
    type Item = FieldRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.field_names
            .next()
            .map(|name| FieldRef::new(self.access, name))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.field_names.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Fields<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.field_names
            .next_back()
            .map(|name| FieldRef::new(self.access, name))
    }
}

impl<'a> ExactSizeIterator for Fields<'a> {}
impl<'a> FusedIterator for Fields<'a> {}
