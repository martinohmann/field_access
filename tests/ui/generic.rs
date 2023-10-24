use field_access::FieldAccess;

#[derive(FieldAccess)]
pub struct Foo<T> {
    a: T,
}

fn main() {}
