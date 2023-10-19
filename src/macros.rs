macro_rules! get_downcast_ref {
    ($value:expr, $($($ty:ty)|+ => $map:expr),+ $(,)?) => {
        if false {
            unreachable!();
        }
        $($(
        else if let Some(value) = $value.downcast_ref::<$ty>().and_then($map) {
            Some(value)
        }
        )+)+
        else {
            None
        }
    };
}

macro_rules! as_type_method {
    ($($ty:ty $({ $($mapping:tt)* })?),+ $(,)?) => {
        $(
            paste! {
                #[doc = "Returns the field value as `" $ty "`."]
                ///
                /// This method is guaranteed to return `Some(_)` if
                #[doc = "[`.is_" $ty "()`][Self::is_" $ty "] returns `true`."]
                ///
                /// It may also return `Some(_)` if it is possible to perform a lossless conversion of
                #[doc = "the field's value into `" $ty "`."]
                pub fn [<as_ $ty>](&self) -> Option<$ty> {
                    get_downcast_ref!(self.value, $ty => |&v| Some(v), $($($mapping)*)*)
                }
            }
        )*
    };
}

macro_rules! is_type_method {
    ($($ty:ty),+ $(,)?) => {
        $(
            paste! {
                #[doc = "Returns `true` if the field value is of type `" $ty "`."]
                ///
                /// # Example
                ///
                /// ```
                /// use field_access::FieldAccess;
                ///
                /// #[derive(FieldAccess)]
                /// struct Foo {
                #[doc = "    a: " $ty ","]
                /// }
                ///
                #[doc = "let foo = Foo { a: " $ty "::default() };"]
                /// let field = foo.field("a").unwrap();
                ///
                #[doc = "assert!(field.is_" $ty "());"]
                #[doc = "assert!(!field.is::<&" $ty ">());"]
                /// ```
                #[inline]
                pub fn [<is_ $ty>](&self) -> bool {
                    self.is::<$ty>()
                }
            }
        )*
    };
}
