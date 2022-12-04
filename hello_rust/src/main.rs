fn hello(name: Option<&str>) {

    match name {

        Some(name) => println!("Hello, {}!", name),

        None => println!("Hello, World!"),

    }

}



fn main() {

    hello(None);

    hello(Some("Binlogo"));

}
