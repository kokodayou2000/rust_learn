rust 中智能指针指的是 实现了

1. Deref trait
2. Drop trait


_1_defer_1 使用 deref 构建了一个自己的 Box
实际上  deref 就是 *xx 的语法糖


arc 处理多线程下，线程外和线程内 所有权的问题
配合 thread move 使用

