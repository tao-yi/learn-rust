# Ownership

## Stack vs Heap

Both the stack and the heap are parts of memory that are available to your code to use at runtime

- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

访问 heap 中的数据比访问 stack 中的数据更慢，因为你必须通过指针才知道数据在内存中的位置。

> Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

当你调用函数时，入参和函数的局部变量包括指向 heap 的指针被推入栈中，当函数执行完毕时，它们会被销毁。

> When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

- Contemporary processors are faster if they jump around less in memory.
- 如果数据存放的比较近(stack)，那么处理器的处理速度会更快
- 如果数据存放的比较远(heap)，那么处理器的处理速度会更慢

### Ownership Rules

- Each value in Rust has a variable that's called its `owner`
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

### Variable Scope 变量作用域

```rs
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

### `String` 类型

The types covered previously are all a known size, **can be stored on the stack** and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.

string literals 字符串字面量是不可修改的，比如 `let a = "hello";`

- `String` 类型的值在 heap 上分配，能够存储在编译时未知数量的文本。

```rs
// 类型 &str
let a = "hello";
println!("{}", a);

// 类型 std::string::String
let mut s = String::from("hello");
s.push_str("world");
println!("{}", s);

```

### Memory and Allocation

字符串字面值我们在编译时就知道它的内容了，所以它的内容被 hardcoded 到最终的可执行文件里了，这就是为什么 string 字面值效率高。但是也导致字面值一旦定义就不可修改了。

而 `String` 类型为了支持可变性，需要在`heap`上分配内存来保存内容，因为在编译时我们不知道它的大小。

- `String::from` 调用会在 heap 中申请一段内存空间
  - 在 GC 语言中，GC 会跟踪并自动清理这段内存
  - 在非 GC 语言中，需要手动释放内存，否则会内存泄露
- 在 Rust 中，当拥有这个值的变量离开作用域时，内存会自动释放

```rs
{
  let s = String::from("hello"); // s处在作用域中，是有效的
  // 对S进行操作

} // 此时作用域结束了，s变量拥有值的内存空间被回收
```

There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope.

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

## 变量和数据的交互方式：Move

在 Rust 中，多个变量可以和相同的数据以不同的方式进行交互。

案例一：基础类型

```rs
// bind the value 5 to variable x
let x = 5;

// 1. make a copy of the value in x
// 2. bind the copy to y
let y = x;

// we now have two variables x and y
// both equal to value 5
```

此时有两个值 5，都被压入栈中。

案例二：String 类型

```rs
let s1 = String::from("hello");
let s2 = s1;
```

A `String` 是由 3 个部分组成:

- `ptr`: 指向内存的指针
- `len`: 长度 how much memory, in bytes, the contents of the string is currently using
- `capacity`: 容量 the total amount of memory, in bytes, that the string has received from the allocator

```js
s1 = {
  ptr: 0x41414,
  len: 5,
  capacity: 5,
};
```

`ptr` 指向 heap 中一段连续内存，保存着字符串的内容:

```js
0 -> h
1 -> e
2 -> l
3 -> l
4 -> o
```

当我们赋值 `s1` 给 `s2` 时，`String` 数据被复制了一份。

- 这就像是某些语言中的 `shallow copy`，但是由于 Rust 还让被 copy 的那个变量失效了，所以下 Rust 中这称做 `move`
- 在这个案例中，我们可以说 `s1` was moved into `s2`

```js
s1 = {
  ptr: 0x41414,
  len: 5,
  capacity: 5,
};
```

> After `let s2 = s1;`, Rust considers s1 to no longer be valid.

此时 s1 被 Rust 废弃，无法访问变量 s1。所以当 s1 和 s2 离开作用域时只需要释放 s2 指向的地址，不需要释放 s1 了。

In addition, there's a design choice that's implied by this: Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

> Rust 不会自动进行深拷贝。

## 变量和数据的交互方式: Clone

如果我们确实想要 deep clone 一份 String 数据，也就是说连 heap 中的数据也要克隆一份的话，可以使用 `clone()` 函数。

```rs
// s1保存一个指针指向heap中的地址
let s1 = String::from("hello");
// s2也保存一个指针指向heap中的另外一个地址
let s2 = s1.clone(); //  the heap data does get copied.

println!("s1 = {}, s2 = {}", s1, s2);
```

## 复制 Stack 上的数据: Copy

```rs
// x的值5保存在stack中
let x = 5;
// y直接copy了一份x的值5
let y = x;

println!("x = {}, y = {}", x, y);
```

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
由于 integer 类型的值在编译时已经知道大小了，所以直接保存在 stack 中，对它的 copy 是非常快的。s

Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, an older variable is still usable after assignment. 如果一个类型已经实现了 `Drop` trait, 那么就不可以实现 `Copy` trait

> trait 类似其他语言的 interface

- 任何 Scalar 简单类型和它们的组合类型都可以实现 `Copy`
- 任何需要分配内存的都不能实现 `Copy`

Here are some of the types that implement `Copy`:

- 所有的 integer 类型，比如 `u32`
- 布尔类型 `true`, `false`
- character 类型
- tuples 类型如果只包含了其他实现了 `Copy` 的类型
  - 比如 `(i32, i32)` 实现了 `Copy`，而 `(i32, String)` 没有

## Ownership and Functions

```rs
fn main() {
    let s = String::from("hello");  // s comes into scope

    // 调用函数会将s move到函数的parameter中，所以s就不再可用了
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{}", s); 报错: borrow of moved value: `s`

    // x是基础类型，完全保存在stack中，所以它不会move，只会copy
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope

函数的返回值也可以转移它的所有权(move)

```rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    // some_string is returned and moves out to the calling function
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}
```

- 函数的入参会拿到所有权
- 函数的返回值会返回所有权给函数的调用方

**当一个包含 heap 数据的变量离开作用域时，它的值就会被 drop 函数清理，除非函数的所有权被移动到另一个变量上了。**

**当传值给函数时，要么会发生移动 move，要么会发生复制 copy**

- 每次调用函数传入入参时，如果还想继续使用这个入参，那就必须返回这个值

### 如何让函数使用某个入参，但是不获取它的所有权呢？

- 返回 tuple，这个 tuple 中包含入参

```rs
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

但是每次都要这样会非常烦琐，Rust 有一个特性专门针对这个场景，叫做 `Reference`。

# References and Borrowing

除了借用入参的 ownership 以外，你还可以使用入参的 `reference` 当做参数。

```rs
fn main() {
    let s1 = String::from("hello");

    // 创建一个是s1的引用 &s1
    let len = calculate_length(&s1);

    // s1 还是拥有字符串hello的所有权
    println!("The length of '{}' is {}.", s1, len);
}

// s 是一个引用reference, 并不拥有 s 指向的值的所有权
fn calculate_length(s: &String) -> usize { // s进入作用域
    // s.push_str(", world"); 报错，因为s1默认是不可变的，它的引用&s也是不可变的
    s.len()
} // s离开作用域，被销毁，但是它没有所有权，所以不会销毁s指向的值s
```

```
// stack
s = {
  ptr: <s1的地址>
}

s1 = {
  ptr: <字符串的地址>
}

// heap
// 字符串，在heap中是一段连续的character
['h', 'e', 'l', 'l', 'o']
```

> `&` means references, they allow you to refer to some value without taking ownership of it.
> the opposite of referencing by using `&` is deferencing, which uses deference operator `*`

### Mutable References

```rs
fn main() {
    // 首先s必须声明为mutable
    let mut s = String::from("hello");

    // 其次必须传入 mutable reference
    change(&mut s);
}

// 参数必须接收 mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. This code will fail:

在单个作用域中，某个变量的可变引用同时只能存在一个。

```rs
let mut s = String::from("hello");

let r1 = &mut s;
// 报错: cannot borrow `s` as mutable more than once at a time
let r2 = &mut s;

{
  let r1 = &s;
  let r2 = &s; // 这是允许的，因为都是只读，不会修改数据产生脏数据
}

println!("{}, {}", r1, r2);
```

The benefit of having this restriction is that Rust can prevent data races at compile time.

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

当然我们可以通过创建新的作用域来使用可变引用

```rs
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s; // 此时r1已经被销毁了s
```

如果已经出现 mutable reference 时不可以使用不可变应用。

```rs
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

如果可变引用已经被销毁了，则允许使用不可变应用

```rs
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

## Dangling References 悬空引用

在带指针的语言中，很容易出现悬空指针：

- 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他变量使用了。

If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

编译器会确保 reference 在它引用的数据之前离开作用域。

```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

## The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

# The Slice Type

除了引用，另外一种不持有所有权的数据结构是 `Slice`

```rs
    let s = String::from("hello world");

    let hello = &s[0..5]; // 包含0, 不包含5
    let hello = &s[..5]; // 包含0, 不包含5

    let world = &s[6..11]; // 包含6, 不包含11
    let world = &s[6..]; // 包含6, 不包含11

    let whole = &s[..]; // 指向整个字符串的切片
```

字符串字面值就是切片.
