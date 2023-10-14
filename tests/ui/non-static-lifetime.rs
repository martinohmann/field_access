use field_access::FieldAccess;

#[derive(FieldAccess)]
pub struct Foo<'a> {
    field: &'a [u8],
}

fn main() {}
