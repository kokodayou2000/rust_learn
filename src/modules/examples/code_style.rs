pub(crate) struct Person{
    pub name: String
}

impl Person {
    pub(crate) fn new() -> Person {
        println!("init");
        Person {
            name: "init".to_string()
        }
    }
}

impl Person{
    pub(crate) fn f1(&self){
        println!("f1");
    }
}

impl Person{
    pub(crate) fn f2(&self){
        println!("f2");
    }
}

#[test]
fn t1() {
    let p1 = Person::new();
    p1.f1();
    p1.f2();
}