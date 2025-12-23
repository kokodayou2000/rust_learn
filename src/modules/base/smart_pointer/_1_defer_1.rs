use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("");
        &self.0
    }
}

#[test]
fn defer_test_fn_1() {
    let x = MyBox(5);
    assert_eq!(5, *x);  // 自动解引用
    assert_eq!(5, *x);  // 自动解引用
    assert_eq!(5, *x);  // 自动解引用
    assert_eq!(5, *x);  // 自动解引用
    assert_eq!(5, *x);  // 自动解引用
}

