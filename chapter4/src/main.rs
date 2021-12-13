fn main() {
    string_demo();
    deep_clone();
    ownership_demo();
    return_ownership();
    return_borrowed_ownerhsip();
}

fn string_demo() {
    let a = "hello";
    println!("{}", a);

    let mut s = String::from("hello");
    s.push_str("world");
    println!("{}", s);

    let mut s = String::from("Hello world");
    let word_index = first_word(&s);
    s.clear();
    println!()
}

fn deep_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_demo() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("{}", s); borrow of moved value: `s`
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // He

// return will also transfer ownership

fn return_ownership() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// return borrowed ownership
fn return_borrowed_ownerhsip() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let ref_s2 = &s2;
    let ref_s2_1 = &s2;
    let len = calculate_length_ref(ref_s2);
    println!("The length of '{}' is {}.", ref_s2, len);

    let mut mut_s2 = String::from("hello");
    {
        let mut_se_ref_1 = &mut mut_s2; // 允许
    }
    let mut_se_ref = &mut mut_s2;
    // let mut_se_ref_1 = &mut mut_s2; 报错
    let len = calculate_length_mut_ref(mut_se_ref);
    println!("The length of '{}' is {}.", mut_s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    return (s, length);
}

// use reference to variable as parameters
fn calculate_length_ref(s: &String) -> usize {
    // s.push_str(", world");
    s.len()
}

fn calculate_length_mut_ref(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

// -----------------

fn first_word(s: &String) -> &str {
    // convert our String to an array of bytes
    let bytes = s.as_bytes();

    // create an iterator over the array of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn slice_demo() {
    let s = String::from("hello");
    let slice = &s[0..2];
}
