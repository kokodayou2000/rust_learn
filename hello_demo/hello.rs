fn hello(name: Option<&str>) {
    match name {
        Some(name) => println!("Hello, {}",name),
        _ => println("Hello World"),
    }
}

fn main(){
    hello(None);
    hello(Some("Deng"));

}