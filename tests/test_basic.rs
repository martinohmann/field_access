use field_access::{AccessError, AnyFieldAccess, FieldAccess};

#[test]
fn it_works() {
    #[allow(dead_code)]
    #[derive(FieldAccess)]
    struct Foo {
        a: u8,
    }

    let mut foo = Foo { a: 1 };

    assert!(foo.field("a").exists());
    assert!(!foo.field("b").exists());

    assert_eq!(foo.field("a").get::<&str>(), Err(AccessError::TypeMismatch));
    assert_eq!(foo.field("b").get::<u8>(), Err(AccessError::NoSuchField));

    assert_eq!(foo.field("a").as_u8(), Ok(1));
    assert!(foo.field_mut("a").set(2u8).is_ok());
    assert_eq!(foo.a, 2);
}

#[test]
fn field_names() {
    #[derive(FieldAccess)]
    struct Foo {
        a: u8,
        b: &'static str,
        c: f64,
    }

    let foo = Foo {
        a: 1,
        b: "b",
        c: 1.0,
    };

    assert_eq!(foo.field_names(), &["a", "b", "c"]);
}
