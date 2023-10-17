#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::fmt;
use core::iter::FusedIterator;
use core::mem;
use core::ops;
use core::slice;
use paste::paste;

/// Derive macro for automatically implementing [`AnyFieldAccess`] on structs.
#[cfg(feature = "derive")]
pub use field_access_derive::FieldAccess;

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
    /// Returns `Some(_)` if the field is accessible, otherwise `None`.
    ///
    /// The returned [`Field`] provides methods to immutably interact with the field. See its
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
    /// assert!(foo.field("a").is_some());
    /// assert!(foo.field("b").is_none());
    /// ```
    #[inline]
    fn field(&self, field: &str) -> Option<Field<'_>> {
        self.field_as_any(field).map(Field::new)
    }

    /// Mutable field access.
    ///
    /// Returns `Some(_)` if the field is accessible, otherwise `None`.
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
    /// assert!(foo.field_mut("a").is_some());
    /// assert!(foo.field_mut("b").is_none());
    /// ```
    #[inline]
    fn field_mut(&mut self, field: &str) -> Option<FieldMut<'_>> {
        self.field_as_any_mut(field).map(FieldMut::new)
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
    ///                         .map(|(name, field)| (name, field.as_u8()))
    ///                         .collect();
    /// assert_eq!(tuples, &[("a", Some(1)),
    ///                      ("b", Some(2)),
    ///                      ("c", Some(3))])
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

/// An immutable struct field reference.
///
/// A `FieldRef` is a proxy for immutable operations on a struct's field.
///
/// Values of this type are created by [`FieldAccess::field`].
#[derive(Debug, Clone)]
pub struct Field<'a> {
    value: &'a dyn Any,
}

impl<'a> Field<'a> {
    fn new(value: &'a dyn Any) -> Self {
        Field { value }
    }

    /// Returns `true` if the field is of type `T`.
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
    /// let field = foo.field("a").unwrap();
    ///
    /// assert!(field.is::<u8>());
    /// assert!(!field.is::<&str>());
    /// ```
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        self.value.is::<T>()
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
    /// let field = foo.field("a").unwrap();
    ///
    /// assert_eq!(field.type_id(), TypeId::of::<u8>());
    /// ```
    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.value.type_id()
    }

    /// Obtains an immutable reference to the value of type `T`.
    ///
    /// Returns `Some(_)` if field's value is of type `T`, `None` otherwise.
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
    /// let foo = Foo { a: 42 };
    /// let field = foo.field("a").unwrap();
    ///
    /// assert_eq!(field.get::<u8>(), Some(&42u8));
    /// assert_eq!(field.get::<&str>(), None);
    /// ```
    #[inline]
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }

    /// Obtains an immutable reference to the value as `&dyn Any`.
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
    /// let foo = Foo { a: 42 };
    /// let field = foo.field("a").unwrap();
    /// let any = field.as_any();
    ///
    /// assert_eq!(any.downcast_ref::<u8>(), Some(&42u8));
    /// ```
    #[inline]
    pub fn as_any(&self) -> &dyn Any {
        self.value
    }

    /// Returns `true` if the field value is of type `&[T]`.
    #[inline]
    pub fn is_slice<T: Any>(&self) -> bool {
        self.is::<&[T]>()
    }

    /// Obtain an immutable reference to the value as `&[T]`.
    ///
    /// Returns `Some(_)` if field's value deferences to `&[T]`, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: Vec<u8>
    /// }
    ///
    /// let foo = Foo { a: vec![1, 2, 3] };
    /// let field = foo.field("a").unwrap();
    ///
    /// assert_eq!(field.as_slice(), Some(&[1u8, 2, 3][..]));
    /// ```
    #[cfg(feature = "alloc")]
    pub fn as_slice<T: Any>(&self) -> Option<&[T]> {
        get_downcast_ref!(
            self.value,
            &[T] => |&v| Some(v),
            Vec<T> => |v| Some(v.as_slice())
        )
    }

    /// Obtain an immutable reference to the value as `&[T]`.
    ///
    /// Returns `Some(_)` if field's value is of type `&[T]`, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use field_access::FieldAccess;
    ///
    /// #[derive(FieldAccess)]
    /// struct Foo {
    ///     a: &'static [u8]
    /// }
    ///
    /// let foo = Foo { a: &[1, 2, 3] };
    /// let field = foo.field("a").unwrap();
    ///
    /// assert_eq!(field.as_slice(), Some(&[1u8, 2, 3][..]));
    /// ```
    #[cfg(not(feature = "alloc"))]
    #[inline]
    pub fn as_slice<T: Any>(&self) -> Option<&[T]> {
        self.get().copied()
    }

    /// Returns `true` if the field value is of type `Vec<T>`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn is_vec<T: Any>(&self) -> bool {
        self.is::<Vec<T>>()
    }

    /// Returns `true` if the field value is of type `String`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn is_string(&self) -> bool {
        self.is::<String>()
    }

    #[cfg(feature = "alloc")]
    field_getter! {
        &str => str {
            &str => |&v| Some(v),
            String => |v| Some(v.as_str())
        }
    }

    #[cfg(not(feature = "alloc"))]
    field_getter! {
        &str => str {
            &str => |&v| Some(v)
        }
    }

    field_getter! {
        bool {
            bool => |&v| Some(v)
        }
    }

    field_getters! {
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

    field_getters! {
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

    field_getters! {
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
#[derive(Debug)]
pub struct FieldMut<'a> {
    value: &'a mut dyn Any,
}

impl<'a> FieldMut<'a> {
    fn new(value: &'a mut dyn Any) -> Self {
        FieldMut { value }
    }

    /// Obtains a mutable reference to the value of type `T`.
    ///
    /// Returns `Some(_)` if field's value is of type `T`, `None` otherwise.
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
    /// let mut field = foo.field_mut("a").unwrap();
    ///
    /// if let Some(field) = field.get_mut::<u8>() {
    ///     *field = 42;
    /// }
    ///
    /// assert_eq!(field.as_u8(), Some(42u8));
    /// assert_eq!(field.get_mut::<&str>(), None);
    /// ```
    #[inline]
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.value.downcast_mut::<T>()
    }

    /// Obtains a mutable reference to the value as `&mut dyn Any`.
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
    /// let mut foo = Foo { a: 42 };
    ///
    /// let mut field = foo.field_mut("a").unwrap();
    /// let any = field.as_any_mut();
    ///
    /// if let Some(value) = any.downcast_mut::<u8>() {
    ///     *value = 42;
    /// }
    ///
    /// assert_eq!(foo.a, 42);
    /// ```
    #[inline]
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self.value
    }

    /// Sets the value of the field.
    ///
    /// Returns `true` if it was possible to replace the field's value with a value of type `T`,
    /// `false` otherwise.
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
    /// let mut field = foo.field_mut("a").unwrap();
    ///
    /// assert!(field.set(42u8));
    /// assert_eq!(foo.a, 42);
    /// ```
    #[inline]
    pub fn set<T: Any>(&mut self, value: T) -> bool {
        self.replace(value).is_some()
    }

    /// Replaces the value of the field, returning the previous value.
    ///
    /// Returns `Some(old_value)` if it was possible to replace the field's value with a value of
    /// type `T`, `None` otherwise.
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
    /// let mut field = foo.field_mut("a").unwrap();
    ///
    /// assert_eq!(field.replace(42u8), Some(1));
    /// assert_eq!(foo.a, 42);
    /// ```
    #[inline]
    pub fn replace<T: Any>(&mut self, value: T) -> Option<T> {
        self.get_mut().map(|dest| mem::replace(dest, value))
    }

    /// Swaps the value of the field and another mutable location.
    ///
    /// Returns `true` if it was possible to replace the field's value with a value of type `T`,
    /// `false` otherwise.
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
    /// let mut value = 2u8;
    /// let mut field = foo.field_mut("a").unwrap();
    ///
    /// assert!(field.swap(&mut value));
    /// assert_eq!(foo.a, 2);
    /// assert_eq!(value, 1);
    /// ```
    #[inline]
    pub fn swap<T: Any>(&mut self, value: &mut T) -> bool {
        self.get_mut().map(|dest| mem::swap(dest, value)).is_some()
    }

    /// Takes the value of the field, replacing it with its default value.
    ///
    /// Returns `Some(_)` if it was possible to replace the field's value with the default value of
    /// type `T`, `None` otherwise.
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
    /// let mut foo = Foo { a: 42 };
    /// let mut field = foo.field_mut("a").unwrap();
    ///
    /// assert_eq!(field.take(), Some(42u8));
    /// assert_eq!(foo.a, 0);
    /// ```
    #[inline]
    pub fn take<T: Any + Default>(&mut self) -> Option<T> {
        self.replace(T::default())
    }
}

impl<'a> AsRef<Field<'a>> for FieldMut<'a> {
    fn as_ref(&self) -> &Field<'a> {
        // SAFETY: `FieldMut` and `Field` share the same memory layout and we're holding an
        // immutable reference.
        unsafe { &*(self as *const FieldMut).cast::<Field>() }
    }
}

impl<'a> ops::Deref for FieldMut<'a> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

/// An immutable iterator over all fields of a struct.
///
/// Values of this type are created by [`FieldAccess::fields`].
#[derive(Clone)]
pub struct Fields<'a> {
    access: &'a dyn FieldAccess,
    field_names: slice::Iter<'a, &'static str>,
}

impl<'a> Fields<'a> {
    fn new(access: &'a dyn FieldAccess) -> Self {
        Fields {
            access,
            field_names: access.field_names().iter(),
        }
    }
}

impl<'a> fmt::Debug for Fields<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.access.field_names()).finish()
    }
}

impl<'a> Iterator for Fields<'a> {
    type Item = (&'static str, Field<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.field_names
            .next()
            .and_then(|&name| self.access.field(name).map(|field| (name, field)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.field_names.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Fields<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.field_names
            .next_back()
            .and_then(|&name| self.access.field(name).map(|field| (name, field)))
    }
}

impl<'a> ExactSizeIterator for Fields<'a> {}
impl<'a> FusedIterator for Fields<'a> {}
