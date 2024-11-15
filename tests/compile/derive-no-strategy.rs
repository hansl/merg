use merg::Merge;

#[derive(Merge)]
struct S {
    #[merge(strategy)]
    field1: u8,
}

fn main() {}
