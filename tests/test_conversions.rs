use field_access::FieldAccess;
use paste::paste;

macro_rules! assert_converts {
    ($var:ident, $ty:ty => $other_ty:ty, $($rest:tt)+) => {
        paste! {
            assert_eq!(
                $var.field(stringify!($ty)).unwrap().[<as_ $other_ty>](),
                $($rest)*,
                concat!(stringify!($ty), " as ", stringify!($other_ty))
            );
        }
    };
}

macro_rules! none {
    ($var:ident, $($ty:ty => $($tys:ty)|+),+ $(,)?) => {
        $($(assert_converts!($var, $ty => $tys, None);)*)*
    };
}

macro_rules! some {
    ($var:ident, $const:ident, $($ty:ty => $($tys:ty)|+),+ $(,)?) => {
        $($(assert_converts!($var, $ty => $tys, Some($ty::$const as $tys));)*)*
    };
}

macro_rules! some_min {
    ($var:ident, $($rest:tt)+) => { some!($var, MIN, $($rest)+); };
}

macro_rules! some_max {
    ($var:ident, $($rest:tt)+) => { some!($var, MAX, $($rest)+); };
}

#[test]
fn unsigned_conversions_min() {
    #[derive(FieldAccess)]
    struct Foo {
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        usize: usize,
    }

    let foo = Foo {
        u8: u8::MIN,
        u16: u16::MIN,
        u32: u32::MIN,
        u64: u64::MIN,
        u128: u128::MIN,
        usize: usize::MIN,
    };

    some_min!(
        foo,
        u8 => u8 | u16 | u32 | u64 | u128 | usize,
        u16 => u8 | u16 | u32 | u64 | u128 | usize,
        u32 => u8 | u16 | u32 | u64 | u128 | usize,
        u64 => u8 | u16 | u32 | u64 | u128 | usize,
        u128 => u8 | u16 | u32 | u64 | u128 | usize,
        usize => u8 | u16 | u32 | u64 | u128 | usize,
    );
}

#[test]
fn unsigned_conversions_max() {
    #[derive(FieldAccess)]
    struct Foo {
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        usize: usize,
    }

    let foo = Foo {
        u8: u8::MAX,
        u16: u16::MAX,
        u32: u32::MAX,
        u64: u64::MAX,
        u128: u128::MAX,
        usize: usize::MAX,
    };

    some_max!(
        foo,
        u8 => u8 | u16 | u32 | u64 | u128 | usize,
        u16 => u16 | u32 | u64 | u128 | usize,
        u32 => u32 | u64 | u128 | usize,
        u64 => u64 | u128,
        u128 => u128,
        usize => u128 | usize,
    );

    none!(
        foo,
        u16 => u8,
        u32 => u8 | u16,
        u64 => u8 | u16 | u32,
        u128 => u8 | u16 | u32 | u64 | usize,
        usize => u8 | u16,
    );

    #[cfg(target_pointer_width = "32")]
    {
        some_max!(foo, usize => u32);
        none!(foo, usize => u64, u64 => usize);
    }
    #[cfg(target_pointer_width = "64")]
    {
        none!(foo, usize => u32);
        some_max!(foo, usize => u64, u64 => usize);
    }
}

#[test]
fn signed_conversions_min() {
    #[derive(FieldAccess)]
    struct Foo {
        i8: i8,
        i16: i16,
        i32: i32,
        i64: i64,
        i128: i128,
        isize: isize,
    }

    let foo = Foo {
        i8: i8::MIN,
        i16: i16::MIN,
        i32: i32::MIN,
        i64: i64::MIN,
        i128: i128::MIN,
        isize: isize::MIN,
    };

    some_min!(
        foo,
        i8 => i8 | i16 | i32 | i64 | i128 | isize,
        i16 => i16 | i32 | i64 | i128 | isize,
        i32 => i32 | i64 | i128 | isize,
        i64 => i64 | i128,
        i128 => i128,
        isize => i128 | isize,
    );

    none!(
        foo,
        i16 => i8,
        i32 => i8 | i16,
        i64 => i8 | i16 | i32,
        i128 => i8 | i16 | i32 | i64 | isize,
        isize => i8 | i16,
    );

    #[cfg(target_pointer_width = "32")]
    {
        some_min!(foo, isize => i32);
        none!(foo, isize => i64, i64 => isize);
    }

    #[cfg(target_pointer_width = "64")]
    {
        none!(foo, isize => i32);
        some_min!(foo, isize => i64, i64 => isize);
    }
}

#[test]
fn signed_conversions_max() {
    #[derive(FieldAccess)]
    struct Foo {
        i8: i8,
        i16: i16,
        i32: i32,
        i64: i64,
        i128: i128,
        isize: isize,
    }

    let foo = Foo {
        i8: i8::MAX,
        i16: i16::MAX,
        i32: i32::MAX,
        i64: i64::MAX,
        i128: i128::MAX,
        isize: isize::MAX,
    };

    some_max!(
        foo,
        i8 => i8 | i16 | i32 | i64 | i128 | isize,
        i16 => i16 | i32 | i64 | i128 | isize,
        i32 => i32 | i64 | i128 | isize,
        i64 => i64 | i128,
        i128 => i128,
        isize => i128 | isize,
    );
    none!(
        foo,
        i16 => i8,
        i32 => i8 | i16,
        i64 => i8 | i16 | i32,
        i128 => i8 | i16 | i32 | i64 | isize,
        isize => i8 | i16,
    );

    #[cfg(target_pointer_width = "32")]
    {
        some_max!(foo, isize => i32);
        none!(foo, isize => i64, i64 => isize);
    }

    #[cfg(target_pointer_width = "64")]
    {
        none!(foo, isize => i32);
        some_max!(foo, isize => i64, i64 => isize);
    }
}
