use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct Address {
    city: String,
    country: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    // 重点在这里：外部看 &Person 是不可变的，但我们仍然能改 address
    address: RefCell<Address>,
    /// 用 Rc + RefCell 实现多个所有者共享可变状态
    /// 共享一份不可变的东西，但这个东西的拥有者可以替换掉它
    shared_note: RefCell<Rc<String>>,
}

impl Person {
    // 接收 &self，但内部仍然可以修改
    fn update_city(&self, new_city: &str) {
        self.address.borrow_mut().city = new_city.to_string();
    }

    /// borrow_mut 的作用
    /// 检查当前这个 RefCell 是否已经有人在借（不可变借用或可变借用）
    /// 如果没人借 → 给我一个 RefMut<String>（相当于 &mut String）
    /// 如果已经有人借了 → panic（运行时错误）
    fn add_note(&self, text: &str) {
        let mut note = self.shared_note.borrow_mut();
        let new_note = Rc::new(format!("{} | {}", note, text));
        *note = new_note;
    }
}

#[test]
fn test_refcell_meaningful() {
    let person = Person {
        name: "小明".to_string(),
        address: RefCell::new(Address {
            city: "上海".to_string(),
            country: "中国".to_string(),
        }),
        shared_note: RefCell::new(Rc::new("初始备注".to_string())),
    };

    // 多个地方持有 &Person
    let p1 = &person;
    let p2 = &person;
    let p3 = &person;

    p1.update_city("深圳");
    p2.update_city("杭州");   // 后来的覆盖前面的
    p3.add_note("已搬家");
    p3.add_note("2026年");

    println!("{:#?}", person);
    // 可以看到：虽然 &person 是不可变的，但内容被多次修改了

}