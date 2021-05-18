#[macro_use]
extern crate enumscribe;

#[derive(EnumToString)]
enum Foo {
    #[enumscribe(str = "b", case_insensitive)]
    Baa,
    // #[enumscribe(ignore)]
    Baz(),
    #[enumscribe(other)]
    Lorem { inner: String }
}

fn main() {
    let foo = Foo::Baa;

    // let s: String = foo.into();

    let s: String = <_ as std::convert::Into<String>>::into(foo);

    println!("Hello, {}!", s);
}
