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

    test_alias();

    test_sized_trait();

    let result = do_twice(add_one, 5);
    assert_eq!(result, 12);

    test_closure();
    // let result = returns_closure();
    // let num = result(12);
    // println!("{:?}", result);
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

/// 类型别名
pub fn test_alias() {
    type KilloMeters = i32;
    let x: i32 = 5;
    let y: KilloMeters = x * 2;

    println!("x + y = {}", x + y);

}

// never type 永远不会返回值
// ! 在函数从不返回的时候充当返回值
pub fn test_never_type() -> ! {
    loop {
        println!("hello world!");
    }

    panic!("hello world!"); // panic 不会返回值
}

// 动态大小类型和 Sized trait
pub fn test_sized_trait() {
    let s1: &str = "hello world";
    let s2: &str = "How's it going?";

    // doesn't have a size known at compile-time
    println!("{}, {}", s1, s2);
}

// 函数指针
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, s: i32) -> i32 {
    f(s) + f(s)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

pub fn test_closure() {
    let numbers = vec![1, 2, 3, 4, 5];
    // let results: Vec<String> = numbers.iter().map(|i| i.to_string()).collect(); // 闭包
    let results: Vec<String> = numbers.iter().map(ToString::to_string).collect(); // 函数指针
    println!("{:?}", results);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

// 返回闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


