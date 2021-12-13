# 变量与可变性

- 声明变量使用 `let` 关键字
  - 默认情况下，变量是不可变的
- 声明变量时，在变量前加上 `mut` 就可以是变量可变 s

- 常量，可是不可变的
  - 常量可以在任何作用域内进行声明，包括全局作用域
  - 常量只可以绑定到常量表达式
  - 声明常量使用 `const` 关键字，它的类型必须被标注
  - 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或者之能在运行时才能计算出的值
- 在程序运行期间，常量在其声明的作用域内一直有效
- 命名规范：Rust 里常量使用全大写字母，每个单词用下划线分割 `_`
  - `const MAX_POINTS: u32 = 100_000;`

## Shadowing

```rs
let mut x = 5;
println!("The value of x is {}", x);

// shadowing, declare a new variable x
// previous x is not available anymore
let x = x + 1;
let mut x = x * 2;
```

## 数据类型

### 标量类型 Scalar Types

A scalar type represents a single value. Rust has four primary scalar types:

- integers
- floating-point numbers
- Booleans
- characters

- `isize` 和 `usize` 类型的大小由计算机架构决定
  - 如果是 64 为计算机 就是 64bits，如果是 32 位计算机就是 32bits
- 主要用来做集合的索引

由于 Integer literals 整数字面值可以是任意一种数值类型，所以你可以加一个后缀来明确它的类型，比如 `57u8`

```rs
// c就是 u8类型
let c = 57u8;
// 等同于
let c: u8 = 57;

// 默认是i32类型
let n = 32;

// 默认是f64类型
let f = 32.1;

```

如果你不确定该使用哪种数值类型，Rust 会采用默认的类型，integer 的默认类型是 `i32`

### Integer Overflow

如果你有一个变量类型是 `u8` ，它只能保存 0 ~ 255 之间的数值，如果你赋值给它 256 就会导致 integer overflow.

Rust has some interesting rules involving this behavior.

- When you're compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust use the term `panicking` when a program exists with an error;
- When you're compiling in release mode with `--release` flag, rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two's complement wrapping.
  - 比如，如果赋值 256 时会变成 0，257 会变成 1

## 浮点类型 Floating-Point Types

Rust 有两种基础的浮点类型 `f32` 和 `f64`

- 默认类型是 `f64` 因为在现代 CPU 上 f64 和 f32 的速度几乎相同，而且精度更高

## 布尔类型

```rs
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## 字符类型

Rust’s char type is the language’s most primitive alphabetic type

```rs
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust 的 `char` 类型是 4 个字节

- four bytes in size and represents a `Unicode Scalar Value`

## 复合类型 Compound Types

- `tuples`
- `arrays`

### Tuple

- tuple 有固定的长度，一旦声明后就无法 grow or shrink in size

```rs
let tup: (i32, f64, u8) = (500, 10.1, 15);

let (x, y, z) = tup;

println!("{}, {}, {}", tup.0, tup.1, tup.2);
println!("{}, {}, {}", x, y, z);
```

> The tuple without any values, `()`, is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.

### Array 数组

```rs
let arr = [1,2,3,4,5];

let arr = [1, 2, 3, 4, 5];
println!("{}", arr[0]);

// 类型: [&str; 12]
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Arrays are useful when you want your data allocated on the `stack` rather than the `heap`

数组不像 Vector 那样灵活. Vector 是标准库提供的类型，它的 size 可以动态的增长或者缩小。

如果数组中的每个元素值都相同，则可以这样声明

```rs
let a = [3; 5];
// 等同于
let a [i32; 5]; [3,3,3,3,3];
```

### 访问数组元素

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
数组是分配在栈 stack 上的，占据单个整块的内存空间。

```rs
let index = [12, 13, 14, 15];
let month = months[index[0]]; // 编译会通过，但是运行时会panic
println!("{}", month);
```

## Functions 函数

- 命名规范，Rust 使用小写 + `_` 分隔

```rs
fn func_one() {

}
```

```rs

fn main() {
  // 5 is argument
  another_function(5);
}

// x is parameter
fn another_function(x: i32) {
  println!("{}", x);
}
```

### Statements and Expressions

在 Rust 中 `statements` 和 `expressions` 是不同的概念

- statements are instructions that perform some action and do not return a value
- expressions evaluate to a resulting value

```rs
// this is a statement
let y = 5;

// Function definitions are also statements;
fn another_function(x: i32) {
  println!("{}", x);
}


fn main() {
    // Statements do not return values
    let x = (let y = 6); // 这里会报错
}
```

- expression 表达式有返回值，比如 `5 + 6` 会返回 11
- `let y = 6;` 这里 `6` 是一个表达式，它返回 6
- 调用函数也是表达式
- 调用 macro 也是表达式
- `{ }` 也是表达式

```rs
fn main() {
  let y = {
    let x = 3;
    x + 1
  };

  println!("{}", y);
}
```

以上的代码块 `{}`

```rs
{
    let x = 3;
    x + 1 // 注意这里没有 ";"
}
```

is a block that evaluates to `4`. That value gets bound to `y` as part of the `let` statement.

> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value

### 函数的返回值

在 Rust 里面，返回值就是函数体里面最后一个表达式的值

```rs
fn hello() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}
```

## if 表达式

在 Rust 中 `if ` 后必须写 bool 类型值，Rust 不会自动进行转换。

```rs
fn main{
  let num = 3;

  if num < 5 {
    println!("true");
  } else {
    println!("false");
  }
}
```

### Using `if` in a `let` statement

因为 `if` 也是一个表达式，所以它也有返回值

```rs
fn main() {
    let num = 6;
    let d = if num > 5 { "bigger" } else { "smaller" };
    println!("{}", d);
}
```

## `loop`

`loop` 关键字告诉 Rust 重复执行一段代码块

```rs
fn main() {
    loop {
      println!("again");
    }
}

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job.

```rs
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}
```

### `for` 循环

```rs
fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }
}

```

### `Range` for loop

`Range` 是标准库提供的一个类型，它会生成 a sequence of numbers

- 指定一个开始数字和一个结束数字，`Range` 可以生成它们之间的数字(不含结束)
  - `(1..4)` 生成 `[1,2,3]`

```rs
fn range_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    // std::ops::Range<i32>
    let c = (1..4);
    for n in c {
        println!("{}!", n);
    }
}

```
