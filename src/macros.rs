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
            /// This method is guaranteed to return `Some(_)` if
            #[doc = concat!("[`.is_", stringify!($ident), "()`][Self::is_", stringify!($ident) ,"]")]
            /// returns `true`.
            ///
            /// It may also return `Some(_)` if it is possible to perform a lossless conversion of
            /// the field's value into
            #[doc = concat!("`", stringify!($ty), "`")]
            /// .
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
