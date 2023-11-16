/* 
  Resources:
  https://fasterthanli.me/articles/a-half-hour-to-learn-rust
  https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html
  https://doc.rust-lang.org/book/ch05-01-defining-structs.html
*/

use core::prelude::v1;

use crate::registers::Registers;

pub mod registers;

pub struct MyStruct {
    pub field_1: i32,
    pub field_2: bool,
    pub str_field: String
}

fn generate_struct() -> MyStruct {
    return MyStruct {
        field_1: 45,
        field_2: true,
        str_field: String::from("kekw")
    };
}

struct Color(i32, i32, i32);

enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String)
}

impl IpAddr {
  fn print(&self) -> String {
    match self {
      Self::V4(byte1, byte2, byte3, byte4) => {
        return format!("{}.{}.{}.{}", byte1.to_ascii_lowercase(), byte2.to_ascii_lowercase(), byte3.to_ascii_lowercase(), byte4.to_ascii_lowercase());
      }
      Self::V6(v6_str) => v6_str.to_string()
    }
  }
}

enum TestAddress {
  Type1(u8, u8, u8),
  Type2(String)
}

impl std::fmt::Display for TestAddress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Self::Type1(v1, v2, v3) => {
          write!(f, "{}, {}, {}", v1, v2, v3)
        },
        Self::Type2(value) => {
          write!(f, "{}", value)
        }
      }
  }
}

fn my_function(in_value: &mut i32) {
    // Stack variables such as integers just need to be dereferenced when passed by reference
    // In C, in_value: &mut: int32 would be int_32t* inValue
    *in_value = 42;
}

fn function_with_return() -> u32 {
    73
}

fn inline_function(feeling_lucky: bool) -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

fn takes_ownership(in_value: String) {
  println!("takes_ownership: took string {:p}: {}", &in_value, in_value)
}

fn returns_ownership(in_value: String) -> String {
  // in_value = String::from("newly assigned string");
  in_value
}

fn takes_reference(in_value: &String) {
  println!("takes_reference: took string {:p}: {}", &in_value, in_value)
}

fn string_ref_append(in_string: &mut String) {
  in_string.push_str(" with more!");
}

fn get_first_word(in_string: &str) -> &str {
  let str_bytes: &[u8] = in_string.as_bytes();

  for (i, &item) in str_bytes.iter().enumerate() {
    if item == b' ' {
      return &in_string[0..i];
    }
  }
  &in_string[..]
}

// ? operator is a 'try'
fn add_opt_values(in_val1: Option<i32>, in_val2: Option<i32>) -> Option<i32> {
  Some(in_val1? + in_val2?)
}

fn test_ref<'a>(in_string: &'a str, in_str_2: &'a str) -> &'a str {
  // &i32 - a reference
  // &'a i32 - reference with explicit lifetime
  if in_string.len() > in_str_2.len() {
    in_string
  } else {
    in_str_2
  }
}

/*
  'a, 'b, '..z are used to specify the lifetime/scope of particular variables
  e.g. This code won't compile, because in_str_1 is in scope a, in_str_2 is in scope b

  fn test_ref<'a, 'b>(in_str_1: &'a str, in_str_2: &'b str) -> &'a str {
    // &i32 - a reference
    // &'a i32 - reference with explicit lifetime
    if in_string.len() > in_str_2.len() {
      in_string
    } else {
      in_str_2
    }
  }

*/

fn main() {
    // Underscore prefix tells the compiler that the variable is unused to prevent warnings
    let _x: i32 = 13;

    // Why doesn't y need to be declared mutable? 
    let mut y: i32 = 0;
    // Passing a known-size data type such as an integer makes a copy rather than passing by reference
    my_function(&mut y);
    println!("Y = {}, because a copy of y is made when passed as parameter to my_function()", y);

    // Underscore throws away result:
    let _ = 32;
    let _ = function_with_return();

    // Only third my_val is accessible, first two are deleted
    let _my_val: u32 = 10;
    let _my_val: u32 = function_with_return();
    let my_val: &str = "Reeee string";

    println!("my_val: {}", my_val);

    let my_pair: (&str, u32) = ("string value", 2341);
    let (str_from_pair, num_from_pair): (&str, u32) = (my_pair.0, my_pair.1);
    println!("Tuple str: {} num: {}", str_from_pair, num_from_pair);

    // Tuple can be disassembled using underscore to retrieve a single value
    let (_, _my_num) = (my_pair.0, my_pair.1);

    // Code blocks define scope, but can also be an expression:
    let inline_value = {
        let y = 1;
        let z = 2;
        y + z
    };
    let feeling_lucky: bool = true;
    let match_val = inline_function(feeling_lucky);
    println!(
        "Inline value: {} | Match value = {}",
        inline_value, match_val
    );

    // In Rust, it is efficient to return by value as values are moved instead of copied
    let struct_test: MyStruct = generate_struct();
    println!(
        "Struct field 1: {}, field 2: {}",
        struct_test.field_1, struct_test.field_2
    );

    takes_ownership(struct_test.str_field);
    // println!("struct test: {}", struct_test.str_field);

    // Struct tuple: struct Color(i32, i32, i32)
    // a structured tuple, or a struct without names
    let _black = Color(0, 0, 0);
    _black.0;

    // Memory is returned once variable is out of scope
    {
        let _my_string = String::from("hello");
    } // my_string memory is returned to the heap (Rust "Drop" function is called)

    // Value 1 is bound to x. Then value of x is copied and bound to y
    let x = 1;
    let _y = x;

    /*

    Consider the following code:

     > let my_string_1: String = String::from("kekw");
     > let my_str_cpy: String = my_string_1
     > println!("{}", my_string_1);

    In C/C++, my_str_cpy would share the same pointer to "kekw" as my_string_1,
    because my_string_1 is a pointer to a value on the heap
    In Rust, the pointer is moved from my_string_1 to my_str_cpy, because the behaviour of
    assignment is ambiguous: Does the string get copied & re allocated (expensive), or does the pointer get copied? (cheap but not safe)
    Rust's behaviour is to invalidate the initial variable and move the pointer to the new variable

    Fixed size values such as integers are known at compile time, which means that:

     > let x = 1;
     > let y = x;

    Copies the value from x into y. Besides having a known size, integers are pushed entirely on the stack.
    I.e. There is no difference between a deep and shallow copy.


    */ 

    // Ownership of Heap-based variables such as strings is handed to a function when passed as parameter
    let my_string = String::from("hello string");
    takes_ownership(my_string);
    // > println!("my_string: {:p}", &my_string);

    // my_string is allocated a new pointer in spite of returning the same string
    let my_string = String::from("original string");
    let mut my_string = returns_ownership(my_string);
    println!("my_string: {:p} - \"{}\"", &my_string, my_string);

    // Values passed as reference are an address which points to the original address (i.e. C double pointer)
    // Difference being is a reference is guaranteed to point to valid memory
    takes_reference(&my_string);
    // Note that references must be marked as mutable to be modified within a function

    string_ref_append(&mut my_string);
    println!("my_string: {:p} - \"{}\"", &my_string, my_string);

    /* 
    
      Dangling references
      The compiler will not allow the following code:

      fn main() {
        let my_string = dangle();
        println!("my_string: {}", my_string);
      }

      fn dangle() -> &String {
        let s = String::from("kekwe");
        return &s;
      }

      because 's' is deallocated once dangle() completes, whilst returning a pointer to nothing

    */
    let test_string = String::from("Test string");

    let first_word = get_first_word(&test_string);
    println!("First word: {}", first_word);

    let mut regs: Registers = Registers::init_registers();
    regs.set_single(230, 'a');
    println!("register a = {}", regs.get_single('a'));

    // Compiler can infer the type
    let _optional_value = Some(10);

    // 'None' value can be instantiated, but requires explicit type:
    let _empty_value: Option<i32> = None;

    // Can extract value inside optional with match statement
    match add_opt_values(_optional_value, _empty_value) {
      Some(val) => {
        println!("Added optionals: {}", val);
      },
      None => {
        println!("One or more values were empty :<");
      }
    }
    // Alternatively use Option::ok_or()
    let kekw = _optional_value.ok_or("An error");
    // Optionals must be copied instead of moved
    println!("Post match optional value = {:p}", &_optional_value);

    // How do you access this?
    let _v4_addr = IpAddr::V4(192, 168, 10, 1);
    let _v6_addr: IpAddr = IpAddr::V6(String::from("::1"));
    // Believe it needs to have functions etc to access member variables?
    println!("v4 addr = {}", _v4_addr.print());
    println!("v6 addr = {}", _v6_addr.print());

    let test_addr: TestAddress = TestAddress::Type1(10, 20, 30);
    println!("kekw {}", test_addr);
    
    let string_1: String = String::from("my string 1");


    let bad_ref: &str;

    {
      let string_2: String = String::from("short stick");
      // bad_ref = test_ref(&string_1, &string_2);
    }
    // println!("bad_ref: {}", bad_ref);

    // let _my_str_ref: &str = test_ref(&string_1, &string_2);
}
