use merg::Merge;

#[derive(Merge)]
struct S {
    #[merge(ignore)]
    field1: Option<u8>,
}

fn main() {}
