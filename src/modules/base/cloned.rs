#[cfg(test)]
mod tests {
    // cloned 是针对于 迭代器的
    // 迭代器里所有引用元素都克隆成拥有值
    #[test]
    fn test_cloned() {

        let v = vec![1, 2, 3, 4];

        // 单个值的 clone
        let x = &v[0];
        let y = x.clone();          // i32 实现了 Copy，所以几乎零成本

        // 迭代器里的每个元素都 clone 一次
        let v2: Vec<i32> = v.iter().cloned().collect();           // 得到 [1,2,3,4] 的拥有值副本

        // 等价于：
        let v3: Vec<i32> = v.iter().map(|&x| x).collect();        // 因为 i32: Copy，所以 &i32 → i32 自动 copy
        let v4: Vec<i32> = v.iter().map(|x| x.clone()).collect(); // 也能工作，但啰嗦

    }
}