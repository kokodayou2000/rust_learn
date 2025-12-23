// Cow（Clone on Write）：写时克隆
// 只有在真正需要修改数据时，才会从借用状态克隆为拥有所有权的堆分配数据
// 可通过模式匹配或 is_borrowed()/is_owned() 检查当前状态

use alloc::borrow::Cow;

#[test]
fn cow_test1() {
    let original = String::from("initial"); // 用非空字符串更明显

    let mut cow_val = Cow::Borrowed(&original);

    // 初始为 Borrowed 状态：仅借用数据，未发生堆分配
    println!("Borrowed: {}", matches!(cow_val, Cow::Borrowed(_)));
    // 或者：println!("Borrowed: {}", cow_val.is_borrowed());

    // 尝试修改 → 会触发克隆
    cow_val.to_mut().push_str(" mutated");

    // 现在变为 Owned 状态：拥有独立的堆上 String
    println!("Owned: {}", matches!(cow_val, Cow::Owned(_)));
    // 或者：println!("Owned: {}", cow_val.is_owned());

    // original 依然保持原样，未受影响
    assert_eq!(original, "initial");
    assert_eq!(cow_val.into_owned(), "initial mutated");
}