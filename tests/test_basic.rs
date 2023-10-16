use field_access::{AnyFieldAccess, FieldAccess};

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
