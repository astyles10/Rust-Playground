use std::{io, process::exit};

fn main () {
  let mut user_input_buffer: String = String::new();
  let buffer_size: Result<usize, io::Error> = io::stdin().read_line(&mut user_input_buffer);
  match buffer_size {
    Err(err) => {
      fail(&err.to_string());
    },
    Ok(bytes_read) => println!("Read {} characters from stdin", bytes_read)
  };
  echo_user_input(&user_input_buffer);
}

fn fail (error: &String) {
  println!("Exiting due to fail: {}", error);
  exit(1);
}

fn remove_whitespace_from_string(input: &str) -> String {
  let mut tmp = input.replace(" ", "");
  tmp = tmp.replace("\t", "");
  tmp
}

fn get_first_number_from_string(input: &str) {
  let mut start_index: Option<usize> = None;
  let mut end_index: usize = 0;
  for c in input.char_indices() {
    if c.1.is_numeric() && start_index.is_none() {
      start_index = Some(c.0);
    } else if !c.1.is_numeric() && start_index.is_some() {
      end_index = c.0;
      break;
    }
  }

  match start_index {
    None => {
      fail(&String::from("Could not find number"));
    },
    Some(start_index) => {
      println!("Got start index: {}", start_index);
      println!("Got end index: {}", end_index);
      let substr : &str = &input[start_index..end_index];
      println!("Got string slice: {}", substr);
    }
  };
  // Can get non-mutable value by dereferencing as seen here:
  // let start_index: usize = *start_index.get_or_insert(0);
}

fn get_operator_from_string(input: &str) {
  for c in input.chars() {
    // Check if + - * / ^ % !
  }
}

fn echo_user_input(user_input_buffer: &str) {
  let cleaned_string = remove_whitespace_from_string(user_input_buffer);
  get_first_number_from_string(&cleaned_string);
}
