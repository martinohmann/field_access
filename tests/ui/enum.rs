use field_access::FieldAccess;

#[derive(FieldAccess)]
pub enum Enum {
    A(i64),
    B(u8),
}

fn main() {}
