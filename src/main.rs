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

fn clean_string(input: &str) -> String {
  let mut tmp = input.replace(" ", "");
  tmp = tmp.replace("\t", "");
  tmp = tmp.replace("\n", "");
  tmp
}

fn do_calculate(input: &str, start_index: usize, operator: char) {
  println!("Start do_calculate: start_index = {}, input.len() = {}, input = '{}'", start_index, input.len(), input);
  println!("Current operator: {}", operator);
  if start_index >= input.len() {
    return;
  }

  let substr: &str = &input[start_index..];
  println!("starting substr = {}", substr);

  // Need to advance the iterator to the index in a rust way
  // https://stackoverflow.com/questions/62186871/how-to-correctly-use-peek-in-rust
  for c in substr.char_indices() {
    if c.1.is_numeric() {
      let substr: &str = &input[c.0..];
      println!("inside loop: current substr = {}", substr);
      let num: (usize, i64) = parse_number_string(substr);
      let end_index: usize = num.0;
      // println!("Got index: {}, number: {}, input.len() {}", num.0, num.1, input.len());
      println!("Got number: {}", num.1);
      do_calculate(substr, end_index, operator);
      break;
    }
    else if !c.1.is_numeric() && validate_operator(c.1) {
      do_calculate(input, c.0, c.1);
      break;
    }
  }
}

// Return the number and the end index
fn parse_number_string(input: &str) -> (usize, i64) {
  let mut end_index: usize = 0;
  for c in input.char_indices() {
    if !c.1.is_numeric() {
      end_index = c.0;
      break;
    } else if c.0 == input.len() - 1 {
      end_index = c.0 + 1;
    }
  }
  if end_index == 0 {
    return (input.len(), 0)
  }

  let substr: &str = &input[..end_index];
  let number: Result<i64, std::num::ParseIntError> = substr.parse::<i64>();
  println!("parse_number_string: substr = {} end_index = {}", substr, end_index);
  match number {
    Ok(num) => {
      (end_index, num)
    },
    Err(err) => {
      fail(&format!("parse_number_string: {}", err));
      (0, 0)
    }
  }
}

fn validate_operator(input: char) -> bool {
  let operators = "+-*/%^()";
  if operators.contains(input) {
    return true
  }
  fail(&format!("invalid operator received: {}", input));
  false
}

fn echo_user_input(user_input_buffer: &str) {
  let cleaned_string = clean_string(user_input_buffer);
  do_calculate(&cleaned_string, 0, 'n');
}
