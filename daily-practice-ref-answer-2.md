# 习题二参考答案

### 第一题
1. 创建一个泛型函数 `print_info`，它接受一个实现了 `Debug` 特征的参数，并打印出该参数的调试信息。这个函数应该能够处理任何实现了 `Debug` 特征的类型。

参考代码：
```Rust
use std::fmt::Debug;

// 创建一个泛型函数 print_info，
// 它接受一个实现了 Debug 特征的参数，
// 并打印出该参数的调试信息。
// 这个函数应该能够处理任何实现了 Debug 特征的类型。
fn print_info<T: Debug>(item: T) {
    println!("{:?}", item);
}
```
2. 创建一个泛型函数 `largest`，它接受一个任意类型的切片，并返回切片中最大的元素。要求这个函数只能用于元素类型实现了 `PartialOrd` 和 `Copy` 特征的情况。

参考代码：
```Rust
fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
        let mut  largrst_value = list[0];
        for &item in list.iter() {
            if item > largrst_value {
                largrst_value = item;
            }
        }
        largrst_value
}
```

### 第二题
1. 定义一个名为 `Drawable` 的特征，它包含一个名为 `draw` 的方法。然后定义两个结构体 `Circle` 和 `Square`，并为它们实现 `Drawable` 特征。
2. 创建一个名为 `display` 的函数，它接受一个实现了 `Drawable` 特征的参数。在这个函数内部，调用传入参数的 `draw` 方法。

参考代码：
```Rust
// 定义Drawable特征，包含draw方法
trait Drawable {
    fn draw(&self);
}

// 定义Circle结构体
struct Circle {
    radius: f64,
}

// 为Circle结构体实现Drawable特征
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

// 定义Square结构体
struct Square {
    side: f64,
}

// 为Square结构体实现Drawable特征
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

// 创建一个名为 display 的函数，它接受一个实现了 Drawable 特征的参数。
// 在这个函数内部，调用传入参数的 draw 方法。
// display函数，接受实现了Drawable特征的类型作为参数
fn display(item: &impl Drawable) {
    item.draw(); // 调用draw方法
}



fn display2(item: Box<dyn Drawable> ) {
    item.draw(); // 调用draw方法
}

fn display_in_container(container: &[Box<dyn Drawable>]) { // trait object
    for item in container.iter() {
        item.draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        let circle = Circle {radius: 8.0};
        let square = Square {side: 6.0};


        /*
        display(&circle);
        display(&square);

        display2(Box::new(circle));
        display2(Box::new(square));
        */


        let c: Vec<Box<dyn Drawable>>  = vec![
            Box::new(circle),
            Box::new(square),
        ];

        display_in_container(&c);


    }
}
```

### 第三题
用`Rust`写一个冒泡排序函数，要求支持范型，并支持指定按`长序`、`降序`。

参考代码：
```Rust
// 用Rust写一个冒泡排序函数，
// 要求支持范型，
// 并支持指定按升序、降序排序。

// &[T] <=== (Vec<T> [T;n])

pub fn bubble_sort<T: Ord> (arr: &mut [T], ascending: bool) {


    if arr.is_empty() { return; }

    let arr_len = arr.len();
    //比较的轮数为待排序元素总长度-1，即： arr.len() -1 轮
    // 每进行一轮比较交换，把最大（小）的元素移动到未排序数组的最后面
    for i in 0..arr_len - 1 {
        for j in 0..arr_len - 1 - i {

            if ascending  {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            } else {
                if arr[j] < arr[j+1] {
                    arr.swap(j, j+1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr1 = vec![4, 2, 1, 5, 3];
        bubble_sort(&mut arr1, true); // 升序
        println!("Sorted array 1 (ascending): {:?}", arr1);

        let mut arr2 = vec![4, 2, 1, 5, 3];
        bubble_sort(&mut arr2, false); // 降序
        println!("Sorted array 2 (descending): {:?}", arr2);

        let mut arr3 = vec!['d', 'b', 'a', 'c'];
        bubble_sort(&mut arr3, false);
        println!("Sorted array 2: {:?}", arr3);
    }
}
```

### 第四题
1. 设计一个名为 `divide` 的函数，它接受两个`f64`类型的参数，返回类型为`Result<f64, String>` 。当除数为零时，函数应返回一个描述错误的字符串。
2. 设计一个名为`get_first`的函数，它接受一个 `Vec<i32>` 并返回一个 `Option<i32>`，如果向量为空，则返回 `None`。
3. 对于上面这个函数，编写代码示例，展示如何使用 `match`表达式来处理`divide`和`get_first`函数的返回值。进一步，展示如何使用`map、and_then、or_else`等组合操作符来优化处理`Option`和`Result`的代码。

解答：
1. 参考代码1
```Rust
// 设计一个名为 divide 的函数，它接受两个f64类型的参数，
// 返回类型为Result<f64, String> 。
// 当除数为零时，函数应返回一个描述错误的字符串。

fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0f64 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

// 设计一个名为get_first的函数，
// 它接受一个 Vec<i32> 并返回一个 Option<i32>，
// 如果向量为空，则返回 None。
fn get_first(vec: Vec<i32>) -> Option<i32> {
    //let value = vec.get(0);
    let v = vec.first().cloned();
    v
}

#[cfg(test)]
mod tests {
    use crate::p4::{divide, get_first};

    #[test]
    fn test_divide() {
        let result = divide(10.0, 0f64);
        match result {
            Ok(q) => println!("Quotient is: {}", q),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_get_first() { // 测试 get_first函数
        /*
        let v1 = vec![1,2,3,
            //"1".to_string(),
            //"2".to_string(),
            //"3".to_string()
        ];

        let v1 = vec![];

        let value = get_first(v1);
        println!("{:?}", value);
         */

        let numbers = vec![1,2,3,];
        let first_number = get_first(numbers);
        match first_number {
            Some(n) => println!("The first number is:{}", n),
            None => println!("The vector is empty."),
        }

        //println!("{:?}", numbers);

    }
}
```

2. 参考代码2:

上面代码虽然可以实现题目的要求，但题目本身不完美（比如调用`get_first`后，原数据就丢失了所有权），正常的情况应该是如下的实现：
```Rust
// 编写代码示例，
// 展示如何使用match表达式来处理divide和get_first函数的返回值。
// 进一步，
// 展示如何使用map、and_then、or_else等组合操作符
// 来优化处理Option和Result的代码。
// 定义一个除法函数，返回Result
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(numerator / denominator)
    }
}

// 定义一个函数，获取数组的第一个元素，返回Option
fn get_first<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        let numbers = vec![10.0, 20.0, 30.0];
        let result = divide(100.0, numbers[1]); // 10

        // 使用match处理Result
        match result {
            Ok(value) => println!("Division succeeded: {}", value),
            Err(e) => println!("Division failed: {}", e),
        }

        // 使用match处理Option
        match get_first(&numbers) { // 10
            Some(first) => println!("First number is: {}", first),
            None => println!("No first number."),
        }

        // 使用组合操作符优化处理Result和Option
        let division = divide(100.0, numbers[1]).map(|value| value * 2.0); // 10
        let first_number = get_first(&numbers).map(|&first| first * 2.0); // 20

        // 使用and_then来链接多个操作
        let result = divide(100.0, numbers[1])
            .and_then(|value| Ok(get_first(&numbers).map(|&first| value / first))); // 0.5

        // 打印优化后的处理结果
        println!("Optimized division: {:?}", division);
        println!("Optimized first number: {:?}", first_number);
        println!("Combined operation result: {:?}", result);
    }
}
```

### 第五题
编写一个函数，它尝试执行多个可能失败的操作，并返回一个自定义的错误类型（原题目是 `Box<dyn Error>`，这样会更容易实现，所以改成自定义的错误类型，这样更能考察自定义Error的相关知识点）的`Result`，用于统一这些各种可能的错误。展示如何在一个函数中处理不同类型的错误，并返回统一的自定义错误类型。

参考代码：
```Rust
// 编写一个函数，
// 它尝试执行多个可能失败的操作，
// 并返回一个自定义的错误类型的Result。
// 展示如何在一个函数中处理不同类型的错误，并返回统一的错误类型。

use std::error::Error;
use std::fs::File;
use std::io::{self,Read};
use std::num::ParseIntError;

// 自定义错误类型，可以存储不同种类的错误
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

// 实现`Error` trait，允许`MyError`与其他错误类型兼容
// pub trait Error: Debug + Display 要实现Error Trait，必须还要实现 Debug + Display
impl Error for MyError {}

// 实现`fmt::Display`，定义错误的显示格式
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            MyError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

// 从`io::Error` 转换为 `MyError`
impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        MyError::Io(value)
    }
}

// 从`ParseIntError` 转换为 `MyError`
impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> Self {
        MyError::Parse(value)
    }
}

struct A {
    a: i32,
}

impl From<i32> for A {
    fn from(value: i32) -> Self {
        A {a: value}
    }
}

fn f(a: A) {}

// 尝试读取文件并解析为数字
fn read_and_parse(filename: &str) -> Result<usize, MyError> {
    let mut file = File::open(filename)?; // ---> A err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ---> B err
    let num = contents.trim().parse()?; // ----C err
    Ok(num)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::p5::{A, f, MyError, read_and_parse};

    #[test]
    fn test_read_and_parse() -> Result<(), MyError> {
        match read_and_parse("number.txt") {
            Ok(n) => println!("The number is: {}", n),
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    }

    fn test_from() {
        let a = A::from(3);
        f(a);
        f(3.into());
    }

}
```
