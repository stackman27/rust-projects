// we import all the modules from house.rs
mod house;

// use crate = current rust project that we are in
// use self = current module we are in
// use super = parent module we are part of
use house::{bedroom1, bedroom2};
use std::collections::HashMap;

fn main() {
    // rust_mod();

    // mutability_func_cond_loops ();

    // structs_and_methods();

    // enums_options_pattern_matching();

    // traits_and_generics();

    hashMaps();
}

// learning mod
fn rust_mod() {
    println!("Hello, world!");

    println!("{}", bedroom1::is_light_on());
    println!("{}", bedroom2::is_light_on());
    println!("{}", bedroom1::is_bedroom2_light_on());
    println!("House number {}", house::HOUSE_NUMBER);
}

fn mutability_func_cond_loops() {
    // Rust-learning: Mutability, functions, conditions and loops
    let mut arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr); // The `{:?}` placeholder tells Rust to use the Debug trait for formatting the array `arr`

    arr[2] = 4;
    println!("{:?}", arr);

    arr[2] = square(arr[2]);
    println!("{:?}", arr);

    mutate_array(&mut arr[1]);
    println!("{:?}", arr);
}

// Simple Rust function
fn square(x: u32) -> u32 {
    x * x
}

// function that mutates our array
// Rust has immutable and mutable reference
fn mutate_array(x: &mut u32) {
    *x = (*x) * (*x)
}

fn loops() {
    // 1. loop
    let mut i = 0;
    let a = loop {
        i += 1;

        if i > 10 {
            break i * 2;
        }
    };

    println!("COUNTER: {}", a);

    // 2. while
    let mut j = 0;
    let new_arr = [1, 3, 4, 5, 6];
    while j < new_arr.len() {
        println!("ARR VALUES: {}", new_arr[j]);
        j += 1;
    }

    // 3. For loop
    for a in &new_arr {
        println!("{}", a)
    }
}

fn ownership_and_borrowing() {
    // ************* OWNERSHIP AND BORROWING *************** //
    // Ownership is really unique to RUST
    // At compile time rust auto-detects where memory needs to be allocated and deallocated without the need of garbage collector
    //
    // Rust OwnerShip system
    // 1. Each value in RUst has a variable that's called owner.
    // 2. There acan only be one owner at a time.
    // 3. when the owner goes out of scope, the value will be dropped (meaning memory will be returned back to the system).

    // WHY DOES THIS CAUSE PROBLEM?
    // we transfer ownership from s1 to s2
    // s2 goes out of scope, heap value gets deallocated
    // s1 goes out of scope, it'll try to deallocate heap value(BUT IT DOESNOT EXIST!!!, heap value gets deallocated twice, program crashes!!)

    let s1: String = String::from("Hwllo World!");

    // UNCOMMENT THIS AND IT WILL ERROR
    // let s2 = s1;
    // println!("{}", s1);
    // println!("{}", s2);

    // Fix 1 (deep copy)
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    // Fix 2 (2 variable access same data)
    // Concept: Borrowing and Referencing
    // Borrower only gets access to the data for a period of time
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);

    // Key concept
    // 1. Unlimited amount of immutable borrows at any given time.
    // 2. One mutable borrow at any given time.
    // 3. Cannot mix mutable borrows and immutable borrows at the same time.
    // example, this will error as we are trying to mutate a borrowed reference
    // s1.push_str("again!");

    // println!("{}", s1);
    // println!("{}", s2);
    // watch this video on ownership & borrowing for more details: https://www.youtube.com/watch?v=q2UnbA2dkc8&list=PLkO5ggdQuRaaeFke7nWS4ajhFVZ1biE7_&index=4
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    logged_in_count: u32,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn structs_and_methods() {
    let user = build_user("sishir".to_string(), "sishirg27@gmail.com".to_string());

    let user2 = User {
        username: "stackman".to_string(),
        email: "stackman@gmail.com".to_string(),
        ..user
    };

    println!("{:?}", user);
    println!("{:?}", user2);

    let rect1 = Rectangle::new(10, 5);
    let mut rect2 = rect1.clone();
    rect2.set_width(20);
    println!("{:?}", rect1);
    println!("{:?}", rect2);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        logged_in_count: 0,
        active: true,
    }
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle { length, width }
    }

    // self is an instance of Rectangle
    fn clone(&self) -> Rectangle {
        Rectangle { ..*self }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

// Enums, Options and Pattern Matching
#[derive(Debug)] // this is called attribute
enum IPAddr_ {
    V4(u8, u8, u8, u8), // this is what's unique about rust
    V6(String),
    V7 { x: u32, y: u32 },
}

fn enums_options_pattern_matching() {
    // How to set enum values? - answer below
    let address1 = IPAddr_::V4(127, 0, 0, 1);

    let address2 = IPAddr_::V6("::1".to_string());

    let address3 = IPAddr_::V7 { x: 10, y: 20 };

    println!("{:?}", address1);
    println!("{:?}", address2);

    // How to access enum values? - answer below
    match address1 {
        IPAddr_::V4(127, 0, 0, 1) => println!("localhost"),
        IPAddr_::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
        IPAddr_::V6(s) => println!("{}", s),
        IPAddr_::V7 { x, y } => {
            let z = x + y;
            println!("{}", z);
        }
        _ => println!("Dont know this value"), // this is for all other values (think of it like switch default)
    }

    // Option enum (checks for null values and pointers)
    let num: Option<u32> = Some(5);

    match num {
        Some(n) => println!("{}", n),
        None => println!("NO value!"),
    }
}

// Trait are like interfaces in golang
pub trait MyTrait {
    fn sum(&self, x: u32, y: u32) -> u32;
    fn subtract(&self, x: u32, y: u32) -> u32;
}

struct MyStruct {}

impl MyTrait for MyStruct {
    fn sum(&self, x: u32, y: u32) -> u32 {
        x + y
    }

    fn subtract(&self, x: u32, y: u32) -> u32 {
        x - y
    }
}

struct MyStruct2 {
    size: u32,
}

impl MyTrait for MyStruct2 {
    fn sum(&self, x: u32, y: u32) -> u32 {
        self.size
    }

    fn subtract(&self, x: u32, y: u32) -> u32 {
        self.size
    }
}

fn print_sum(m: &impl MyTrait) {
    println!("sum: {}", m.sum(10, 20))
}

fn traits_and_generics() {
    let my_struct = MyStruct {};

    let my_struct_2 = MyStruct2 { size: 10 };

    print_sum(&my_struct);
    print_sum(&my_struct_2);

    // GENERICS
    // 1. Userd to reduce code duplication on different data types
    // 2. Tells the compiler to generate same code for different data types (during compile time)
    // eg: enum Option<T> {Some(T); None}

    let num_list = vec![2, 3, 4, 1, 2];
    let char_list = vec!['a', 'v', 'z', 'e'];

    let res_largest_num = largest_value(&num_list);
    let res_largest_char = largest_value(&char_list);

    println!("largest num {}", res_largest_num);
    println!("largest char {}", res_largest_char);

    // combining traits and generics

    let m1 = MyNewStruct::new(10, 12);
    let m2 = MyNewStruct::new([10], [12]);

    m1.print_ab();
    m2.print_ab(); //this errors because it doesnot support display trait
}

// Generic function that gives larges value for int and char
fn largest_value<T>(list: &[T]) -> T
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Combining Generics and Traits
struct MyNewStruct<T> {
    a: T,
    b: T,
}

impl<T> MyNewStruct<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T> MyNewStruct<T>
where
    T: std::fmt::Debug,
{
    pub fn print_ab(&self) {
        println!("a: {:?}, b: {:?}", self.a, self.b);
    }
}

// Hash Maps

fn hashMaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // change this to  scores.insert(10, String::from("Blue")); to have i32 as key and string as value
    scores.insert(String::from("Red"), 50);

    // Accessing values
    let blue_score = scores.get("Blue");
    println!("Score for Blue: {:?}", blue_score);

    // Looping through all key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}
