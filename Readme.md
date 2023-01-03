# 高级 trait

```Rust

```

## 关联类型在 trait 定义中指定占位符类型
```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```
    - 关联类型和泛型的区别？
        - 泛型：每次实现 trait 时标注类型；可以为一个类型多次实现某个 trait（不同的泛型参数）
        - 关联类型：无需标注类型；无法为单个类型多次实现 trait

## 默认泛型类型参数和运算符重载
    - 默认参数类型主要用于如下两个方面：
        - 扩展类型而不破坏现有代码。
        - 在大部分用户都不需要的特定情况进行自定义。

        
```Rust
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

## 完全限定语法与消歧义：调用相同名称的方法


## 父 trait 用于在另一个 trait 中使用某 trait 的功能

## newtype 模式用以在外部类型上实现外部 trait
    - 孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体中创建一个新类型