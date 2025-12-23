struct Person {
    name: String,
}

impl Drop for Person  {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}
fn test_drop() {
    println!("start Test");
    // 如果未赋值就直接 drop了
    let _ = Person{
        name: String::from("Test Person"),
    };
    println!("end Test");


    println!("start Test1");
    let a = Person{
        name: String::from("Test Person"),
    };
    println!("end Test1");
}


#[test]
fn it_works() {
    test_drop();
}
