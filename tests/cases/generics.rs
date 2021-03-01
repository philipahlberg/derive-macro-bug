use derive_macro_bug::Macro;

#[derive(Macro)]
struct Foo<T> {
    bar: T,
}

fn main() {}
