pub struct my_struct {
    pub field_1: i32,
    pub field_2: bool,
}

fn generate_struct() -> my_struct {
    return my_struct {
        field_1: 45,
        field_2: true,
    };
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

fn get_first_word(in_string: &String) -> &str {
  let str_bytes: &[u8] = in_string.as_bytes();

  for (i, &item) in str_bytes.iter().enumerate() {
    if item == b' ' {
      return &in_string[0..i];
    }
  }
  &in_string[..]
}

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
    let struct_test: my_struct = generate_struct();
    println!(
        "Struct field 1: {}, field 2: {}",
        struct_test.field_1, struct_test.field_2
    );

    // let inline_value

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
}
