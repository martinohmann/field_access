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

macro_rules! field_getter {
    ($ty:ty { $($rest:tt)+ }) => {
        field_getter!($ty => $ty { $($rest)* });
    };
    ($ty:ty => $ident:tt { $($rest:tt)+ }) => {
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
            #[doc = "assert!(field.is_" $ident "());"]
            #[doc = "assert!(!field.is::<&" $ty ">());"]
            /// ```
            #[inline]
            pub fn [<is_ $ident>](&self) -> bool {
                self.is::<$ty>()
            }

            #[doc = "Returns the field value as `" $ty "`."]
            ///
            /// This method is guaranteed to return `Some(_)` if
            #[doc = "[`.is_" $ident "()`][Self::is_" $ident "] returns `true`."]
            ///
            /// It may also return `Some(_)` if it is possible to perform a lossless conversion of
            #[doc = "the field's value into `" $ty "`."]
            pub fn [<as_ $ident>](&self) -> Option<$ty> {
                get_downcast_ref!(self.value, $($rest)*)
            }
        }
    };
}

macro_rules! field_getters {
    () => {};
    ($ty:ty { $($body:tt)* } $($rest:tt)*) => {
        field_getters!($ty => $ty { $($body)* } $($rest)*);
    };
    ($ty:ty => $ident:tt { $($body:tt)* } $($rest:tt)*) => {
        field_getter!($ty => $ident { $($body)* });
        field_getters!($($rest)*);
    };
}
