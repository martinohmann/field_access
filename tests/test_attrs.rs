use field_access::{AnyFieldAccess, FieldAccess};

#[test]
fn public_fields_only() {
    #[allow(dead_code)]
    #[derive(FieldAccess, Default)]
    #[field_access(public)]
    struct Foo {
        a: u8,
        pub b: u8,
    }

    let foo = Foo::default();

    assert_eq!(foo.field_names(), &["b"]);
    assert!(foo.field("a").is_none());
    assert!(foo.field("b").is_some());
}

#[test]
fn skip_field() {
    #[allow(dead_code)]
    #[derive(FieldAccess, Default)]
    struct Foo {
        a: u8,
        #[field_access(skip)]
        b: u8,
    }

    let foo = Foo::default();

    assert_eq!(foo.field_names(), &["a"]);
    assert!(foo.field("a").is_some());
    assert!(foo.field("b").is_none());
}
