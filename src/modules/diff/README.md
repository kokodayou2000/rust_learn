rust 和 java的部分区别

rust 舍弃 继承 ，采用派生的方式

java 所有的 类，默认继承 Object
其中 Object 有默认的 toString equals 等方法
需要自定义，即重写即可

rust 没有 继承，采用派生的方式
当需要 修改 struct 的展示方式的时候，采用派生的方式
impl fmt::Display for Xxx {}
