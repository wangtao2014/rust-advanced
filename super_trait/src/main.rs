use std::ops::Add;
use std::fmt;

fn main() {
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 5 }
    );

    let human = Human;
    human.fly();
    // 为了能够调用 Pilot trait 或 Wizard trait 的 fly 方法，我们需要使用更明显的语法以便能指定我们指的是哪个 fly 方法
    Pilot::fly(&human);
    Wizard::fly(&human);

    // todo!()  // any code following this expression is unreachable
    // println!("A baby dog is called a {}", Dog::baby_name());

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

/// 关联类型在 trait 定义中指定占位符类型
struct Counter;

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
/// 默认泛型类型参数和运算符重载
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl OutlinePrint for Point {

}

/// 完全限定语法与消歧义：调用相同名称的方法

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("This is your wizard.");
    }
}

impl Human {
    fn fly(&self) {
        println!("This is your human.");
    }
}

/// 完全限定语法
trait Animal {
    fn body_name() -> String;
}

struct Dog;

impl Animal for Dog {
    fn body_name() -> String {
        String::from("puppy")
    }
}

impl Dog {
    fn body_name() -> String {
        String::from("Spot")
    }
}

/// 父 trait 用于在另一个 trait 中使用某 trait 的功能

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
