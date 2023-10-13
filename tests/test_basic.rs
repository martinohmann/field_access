use field_access::{AccessError, FieldAccess};

#[test]
fn it_works() {
    #[allow(dead_code)]
    #[derive(FieldAccess)]
    struct Foo {
        a: i64,
    }

    let foo = Foo { a: 1 };

    assert!(foo.get_dyn("a").is_ok());
    assert!(foo.get_dyn("b").is_err());

    let dyn_foo = foo.as_dyn();

    assert_eq!(dyn_foo.get("a"), Ok(&1i64));
    assert_eq!(dyn_foo.get::<&str>("a"), Err(AccessError::TypeMismatch));
    assert_eq!(dyn_foo.get::<u8>("b"), Err(AccessError::NoSuchField));
}
