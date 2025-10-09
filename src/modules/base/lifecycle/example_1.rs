#[cfg(test)]
mod tests {


    #[test]
    fn t1() {
        let a = String::from("a");
        let c;
        // 具体的案例在 "a".len() < "bb".len()
        // 能正确的返回数据，但是逻辑上 b的生命周期小于
        // {
            // b 的生命周期 < c的生命周期
            let b = String::from("bb");
            c  = select_str(a.as_str(), b.as_str());
        // }

        println!("c = {}", c);
    }


    // 'a：参数 x 的引用的生命周期，表示 x 指向的数据至少存活 'a 这么久。
    // 'b：参数 y 的引用的生命周期，表示 y 指向的数据至少存活 'b 这么久。
    // 'c：返回值的引用的生命周期，表示返回的 &str 至少存活 'c 这么久。
    fn select_str<'a,'b,'c>(x: &'a str, y: &'a str) -> &'c str
    where 'a: 'c, 'b: 'c{
        if x.len() < y.len() {
            x
        }else {
            y
        }
    }

    #[test]
    fn t2() {
        let result;
        let string1 = String::from("short");
        {
            let string2 = String::from("longer");
            result = no_longest_with_condition(string1.as_str(), string2.as_str(), 5);
        }
        println!("Result: {}", result); // 这里希望 result 是 string1 的引用
    }

    // 显式生命周期标注
    fn longest_with_condition<'a, 'b>(s1: &'a str, s2: &'b str, threshold: usize) -> &'a str {
        if s1.len() > s2.len() && s1.len() > threshold {
            s1
        } else {
            s1 // 总是返回 s1，确保返回值的生命周期是 'a
        }
    }

    // 隐式生命周期标注
    // 编译器报错的问题是 返回一个数据，但是 参数是两个，编译器不知道返回值具有 s1 的生命周期 还是 s2 的生命周期
    fn no_longest_with_condition<'a,'b>(s1: & 'a str, s2: & 'b str, threshold: usize) -> & 'a str {
        if s1.len() > s2.len() && s1.len() > threshold {
            s1
        } else {
            s1 // 总是返回 s1，确保返回值的生命周期是 'a
        }
    }

}