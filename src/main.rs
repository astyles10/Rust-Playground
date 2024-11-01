use std::{char, io, process::exit};

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

fn echo_user_input(user_input_buffer: &str) {
  let cleaned_string = clean_string(user_input_buffer);
  do_calculate(&cleaned_string, 0, 0, '+');
}

fn clean_string(input: &str) -> String {
  let mut tmp = input.replace(" ", "");
  tmp = tmp.replace("\t", "");
  tmp = tmp.replace("\n", "");
  tmp
}

fn do_calculate(input: &str, start_index: usize, operand: i64, operator: char) -> i64 {
  // let mut user_input_buffer: String = String::new();
  // let _buffer_size: Result<usize, io::Error> = io::stdin().read_line(&mut user_input_buffer);
  // println!("\nStart do_calculate: start_index = {}, input.len() = {}, input = '{}'", start_index, input.len(), input);

  let mut iter: std::iter::Peekable<std::str::CharIndices<'_>> = input.char_indices().peekable();
  let char_index: Option<(usize, char)> = iter.next();

  match char_index {
    Some((index, char)) => {
      if char.is_numeric() {
        let num: (usize, i64) = parse_number_string(input);
        println!("Got num: {}", num.1);

        // Need to check if we can evaluate here...

        if iter.peek().is_none() || num.0 >= input.len() {
          let calculation = calculate_expression(operator, operand, num.1);
          println!("Answer: {}", calculation);
          return calculation;
        }

        return do_calculate(&input[num.0..], num.0, num.1, operator);
      } else if validate_operator(char) {
        println!("Got operator: {}", char);
        // Need to see if we can evaluate here
        return do_calculate(&input[index+1..], index+1, operand, char);
      }
      0
    },
    None => {
      0
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
  // println!("parse_number_string: substr = {} end_index = {}", substr, end_index);
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
  let result = operators.contains(input);
  // println!("Got result: {}", result);
  if !result {
    fail(&format!("invalid operator received: {}", input));
  }
  result
}

fn is_evaluatable(iter: std::iter::Peekable<std::str::CharIndices<'_>>) {
  
}

fn calculate_expression(operator: char, operand1: i64, operand2: i64) -> i64 {
  if operator == '+' {
    return add(operand1, operand2);
  }
  0
}

fn add(operand1: i64, operand2: i64) -> i64 {
  operand1 + operand2
}
