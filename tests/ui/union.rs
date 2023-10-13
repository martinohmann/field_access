use field_access::FieldAccess;

#[derive(FieldAccess)]
pub union Union {
    a: i64,
    b: u8,
}

fn main() {}
