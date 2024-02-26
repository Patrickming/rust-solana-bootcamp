# 习题一
### 第一题
1. 使用 `if` 检查 `i32` 类型的变量是否为正数、负数或零，并打印相应的信息。
```Rust
fn main() {
    let number: i32 = 8i32;
    if number > 0 {
        println!("number > 0");
    } else  if number == 0{
        println!("number == 0");
    } else {
        println!("number < 0");
    }
}
```
拓展：如果是浮点数（`f32、f64`），不能直接比较，一般采取下面的方式：
```Rust
    let a = 3.141f64;
    if (a - 0f64).abs() < f64::EPSILON {
        println!("==");
    } else {
        println!("!=");
    }
    ...
```


2. 使用 `loop` 编写一个无限循环，当循环次数达到10次时，使用 `break`退出循环。
```Rust
    let mut  count = 0; // 加上mut 在loop中才能修改
    loop {
        if count == 10 {
            break;
        }
        //count++; //Rust不支持这种写法
        count += 1;
        //count = count + 1;
    }
    println!("count = {}", count);
```
拓展： loop支持用break返回值，直接绑定到一个变量，比如：
```Rust
let a = loop {
    ...
    if some_condition {
        break some_value;
    }
    ...
}
```

3. 使用 `for` 循环遍历 `1` 到 `10` 的数字，并只打印出其中的偶数。

```Rust
    for value in 1..11 { // 左闭右开区间，所以右边加上1
        if 0 == (value % 2){
            println!("{value}");
        }
    }
```

### 第二题
1. 创建一个函数 `take_onwership`，它获取一个 `String`类型的参数，并打印出来，然后探索函数调用后原变量的状态；创建一个函数 `borrow_string`，它获取一个对 `String`的不可变引用，并打印出字符串的长度。
```Rust
fn take_onwership(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &String) {
    println!("string's len: {}", s.len());
}

fn main() {
    let s1 = String::from("hello, Solana");
    take_onwership(s1); // s1的所有权传入到了函数内部
    //println!("{s1}"); // 此时，s1已经不可见，去掉注释无法通过编译！！！

    let s2 = String::from("hello, Rust");
    borrow_string(&s2); // 传入到函数中的只是不可变引用，没有改变其所有权
    println!("{}", s2); // 所以，s2仍然可用
}

```


2. 分别写出下面程序片断输出结果，并说明原因。
```Rust 
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;
    
    println!("{b}"); 
}
```
编译不通过： error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable

```Rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;
    println!("{c}"); 
}
```
编译通过


```Rust
fn main() {
    let mut a = 10u32;
    let c = &a; 
    let b = &mut a;
    *b = 20;
    println!("{c}");
}
```
编译不通过：error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable


结论：
a.一个变量，在某一时刻，有且只有一个可变引用(`&mut T`)；
b.一个变量，在某一时刻，可以有多个不可变引用（`& T`）；
c. a 与 b是排它的，要么a，要么b
拓展： 上面所说的某个时刻，是由编译器扫描代码决定的，通俗地讲，就是不可变引用与可变引用的定义与使用之间不要出现交叉，如果有交叉，就认为破坏了固有的规则，就无法通过编译。对于代码2中，引用b在`*b = 20`之后就没有再使用，与c之间不存在交叉现象，所以可以通过编译。

### 第三题
1.假设有一个结构体 `Book`，它包含一个对 `String` 的引用。编写一个带有生命周期注释的结构体，并解释为什么需要生命周期。

```Rust
    struct Book<'a> {
        // 'a 是一个生命周期注解， 表示 title 的生命周期
        title: &'a str,
    }
```
解释： 之所以要加上生命周期标 `'a`，是因为这个结构体内出现了一个引用类型的属性`title`，它的内容不属于该结构体本身，还是引用（参照）其他的外部数据，在这个结构体实例的存续期间，必须保证它所引用的内容本身是始终存在的。所以我们显式的指定这个`'a`，就是要编译证保证这一点。

2.实现一个返回最长字符串切片的函数
编写一个函数 `longest`， 它接受两个字符的引用，并返回最长的字符串的引用。尝试调用 `longest` 函数，并处理可能出现的生命周期问题。

```Rust
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x // 如果 x 的长度大于 y，返回 x
        } else {
            y // 否则，返回 y
        }
    }
```
解决： 引用了生命周期'a，'a的具体值可以理解为x、y两者中的最小的那一个生命周期值，这个函数要保证的时，函数调用者接收到这个返回的引用值，在它的使用过程中，能保证x、y所引用的内容本身，不会被释放。


### 第四题
1. 请定义一个`Person`结构体，它应该包括`姓名、年龄和城市`等字段。然后编写一个关联函数 `new`，用于根据给定的参数创建`Person`实例。
2. 为`Person`结构体实现一个方法 `introduce`，该方法的作用是打印出一个介绍个人信息的语句。应能够清晰地表达出这个人的姓名、年龄和所在的城市。

```Rust
// 结构体定义
struct Person {
    name: String,
    age: u8,
    city: String,
}


impl Person {
    //关联函数
    fn new(name: &str, age: u8, city: &str) -> Self {
        Person {
            name: name.to_string(),
            age,
            city: city.to_string(),
        }
    }

    // 方法（参数中有self/&self/&mut self)
    fn introduce(&self) {
        println!("Hello, my name is {}. I'm {} years old and I live in {}. ",
        self.name,
        self.age,
        self.city
        );
    }
}

fn main() {
        let person = Person::new("Alice", 30, "Benjin");
        person.introduce();
}

```



### 第五题
定义一个名为 `TrafficLight` 的枚举，它应该包含红灯、黄灯、绿灯这三种状态。然后为`TrafficLight` 枚举实现一个方法 `duration`，该方法返回每种灯持续的时间（以秒为单位）。最后，使用 `match` 表达式来处理`TrafficLight`实例，根据不同的灯显示相应的行动指示。

```Rust

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}
fn main() {
        let light = TrafficLight::Green;
        match light {
            TrafficLight::Red => println!("STOP for {} seconds.", light.duration()),
            TrafficLight::Yellow => println!("CAUTION for {} seconds.", light.duration()),
            TrafficLight::Green => println!("GO for {} seconds.", light.duration()),
        }
        
        /*
        // 如果，我们仅仅想关注其中的某一种状态，可以使用 `if let`,如下：
        if let TrafficLight::Green = light {
            println!("GO for {} seconds.", light.duration());
        }
        */
}
```
