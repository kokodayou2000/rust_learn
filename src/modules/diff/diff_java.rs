
// java 可以使用 重写的方式来重写例如 toString 等方法
// rust 采用 impl fmt::Display for Xxx 的方式来进行派生

use std::fmt;
use std::fmt::{Display, Formatter};

enum Gender {
    Male,
    Female,
    Other,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 使用 match 显式转换枚举值为字符串，避免递归
        let gender_str = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Other => "Other",
        };
        write!(f, "{}", gender_str)
    }
}

// 定义 Person 结构体
struct Person {
    name: String,
    gender: Gender,
}

impl Person {
    // 构造函数
    fn new(name: String, gender: Gender) -> Person {
        Person { name, gender }
    }

}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.gender)
    }
}

fn diff_java_override_to_string(name: String,gender: Gender) -> Person {
    let p1 = Person::new(name, gender);
    // 显示信息
    println!("{}",p1);
    p1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn t1() {
        let p1 = diff_java_override_to_string(String::from("Tom"),Gender::Female);
        assert_eq!(p1.to_string(), "(Tom, Female)");
    }
}
