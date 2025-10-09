trait 类似 interface 可以定义行为

```java
// 相当于 Rust 的 Animal trait
interface Animal {
    // 静态方法，类似 Rust 的 `new`
    static Animal newAnimal(String name) {
        // 注意：Java 接口的静态方法不能像 Rust 那样直接返回 Self，
        // 这里需要具体类型（如 Sheep），否则需要工厂模式
        return new Sheep(name, false);
    }

    // 抽象方法，相当于 Rust 的 `name` 和 `noise`
    String name();
    String noise();

    // 默认方法，相当于 Rust 的 `talk`
    default void talk() {
        System.out.println(name() + " says " + noise());
    }
}

```
```java 
// 相当于 Rust 的 Sheep 结构体
class Sheep implements Animal {
    private String name;
    private boolean naked;

    // 构造函数，初始化 Sheep
    public Sheep(String name, boolean naked) {
        this.name = name;
        this.naked = naked;
    }

    // 实现 Animal 接口的 name 方法
    @Override
    public String name() {
        return name;
    }

    // 实现 Animal 接口的 noise 方法
    @Override
    public String noise() {
        return naked ? "baaaaah?" : "baaaaah!";
    }

    // 重载 talk 方法（可选）
    @Override
    public void talk() {
        System.out.println(name + " pauses briefly... " + noise());
    }

    // 相当于 Rust 的 is_naked 方法
    public boolean isNaked() {
        return naked;
    }

    // 相当于 Rust 的 shear 方法
    public void shear() {
        if (isNaked()) {
            System.out.println(name + " is already naked...");
        } else {
            System.out.println(name + " gets a haircut!");
            naked = true;
        }
    }
}

```

```java
// 测试代码
public class Main {
    public static void main(String[] args) {
        // 创建 Sheep 实例
        Animal dolly = Animal.newAnimal("Dolly");
        dolly.talk();
        ((Sheep) dolly).shear();
        dolly.talk();
    }
}
```