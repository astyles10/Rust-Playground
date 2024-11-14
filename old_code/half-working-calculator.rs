use std::{char, io, process::exit, thread::current};

// static OPERATORS: [char; 7] = ['(', ')', '^', '/', '*', '+', '-'];
static OPERATORS: &str = "(^/*+-)";

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
  let res = new_calculate_branch(&cleaned_string);
  println!("Got result: {:?}", res);
}

fn clean_string(input: &str) -> String {
  let mut tmp = input.replace(" ", "");
  tmp = tmp.replace("\t", "");
  tmp = tmp.replace("\n", "");
  tmp
}

fn do_calculate(input: &str, operand: i64, operator: char) -> (usize, i64) {
  // let mut user_input_buffer: String = String::new();
  // let _buffer_size: Result<usize, io::Error> = io::stdin().read_line(&mut user_input_buffer);
  // println!("\nStart do_calculate: start_index = {}, input.len() = {}, input = '{}'", start_index, input.len(), input);

  let mut iter: std::iter::Peekable<std::str::CharIndices<'_>> = input.char_indices().peekable();
  let char_index: Option<(usize, char)> = iter.next();

  let mut res: (usize, i64) = (0, 0);

  match char_index {
    Some((index, char)) => {
      if char.is_numeric() {
        let mut num: (usize, i64) = get_first_number_from_string(input);
        println!("\nGot num: {}", num.1);
        let mut evaluated = false;

        if is_evaluatable(&input[num.0..], operator) {
          println!("Calculating with operator {}, operand {} and number {}...", operator, operand, num.1);
          num.1 = calculate_expression(operator, operand, num.1);
          println!("Calculated value: {}", num.1);
          evaluated = true;
        }

        if iter.peek().is_none() {
          return num;
        }

        if !evaluated {
          res = do_calculate(&input[num.0..], num.1, operator);
        }
      } else if validate_operator(char) {
        println!("\nGot operator: {}", char);
        if char == ')' {
          return (index+1, operand);
        }

        if char == '(' {
          res = new_calculate_branch(&input[index+1..]);
        }
        res = do_calculate(&input[index+1..], operand, char);
      }
    },
    None => {
      println!("End of calculation");
    }
  }
  res
}

fn new_calculate_branch(input: &str) -> (usize, i64) {
  let mut iter: std::iter::Peekable<std::str::CharIndices<'_>> = input.char_indices().peekable();
  let char_index: Option<(usize, char)> = iter.next();
  let mut result: i64 = 0;
  let mut end_index: usize = 0;
  match char_index {
    Some((index, char)) => {
      if char.is_numeric() {
        let num: (usize, i64) = get_first_number_from_string(input);
        let next_operator_index: (usize, char) = get_next_operator_from_string(&input[num.0..]);
        println!("non-signed number: {:?}\noperator_index: {:?}", num, next_operator_index);
        (end_index, result) = do_calculate(&input[next_operator_index.0..], num.1, next_operator_index.1);
      } else if is_sign(char) {
        let num: (usize, i64) = parse_signed_number(&input[index+1..], char);
        let next_operator_index: (usize, char) = get_next_operator_from_string(&input[num.0..]);
        (end_index, result) = do_calculate(&input[next_operator_index.0..], num.1, next_operator_index.1);
        println!("Signed number: {:?}\noperator_index: {:?}", num, next_operator_index);
      } else if is_opening_bracket(char) {
        (end_index, result) = new_calculate_branch(&input[index+1..]);
      }
    },
    None => {
      println!("End branch");
    }
  }
  (end_index, result)
}

fn get_first_number_from_string(input: &str) -> (usize, i64) {
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
  match number {
    Ok(num) => {
      (end_index, num)
    },
    Err(err) => {
      fail(&format!("get_first_number_from_string: {}", err));
      (0, 0)
    }
  }
}

fn get_next_operator_from_string(input: &str) -> (usize, char) {
  let mut iter: std::iter::Peekable<std::str::CharIndices<'_>> = input.char_indices().peekable();
  let char_index: Option<(usize, char)> = iter.next();
  println!("get_next_operator_from_string got input: {}", input);
  match char_index {
    Some((index, val)) => {
      if validate_operator(val) {
        return (index, val);
      }
      (0,'\0')
    },
    None => {
      (0, '\0')
    }
  }
}

fn is_valid_first_operator(input: char) -> bool {
  let is_valid_first_operator: bool = "+-(".contains(input);
  is_valid_first_operator
}

fn is_sign(input: char) -> bool {
  "+-".contains(input)
}

fn is_opening_bracket(input: char) -> bool {
  input == '('
}

fn parse_signed_number(input: &str, sign: char) -> (usize, i64) {
  let num_index = get_first_number_from_string(input);
  if sign == '-' {
    return (num_index.0, 0-num_index.1);
  }
  num_index
}

fn validate_operator(input: char) -> bool {
  let operators = "+-*/%^()";
  let result = operators.contains(input);
  // println!("Got result: {}", result);
  // if !result {
  //   fail(&format!("invalid operator received: {}", input));
  // }
  result
}

fn is_evaluatable(input: &str, current_operator: char) -> bool {
  let mut iter: std::iter::Peekable<std::str::CharIndices<'_>> = input.char_indices().peekable();
  let next_char: Option<&(usize, char)> = iter.peek();
  match next_char {
    Some(val) => {
      println!("is_evaluatable got char: {}", val.1);
      let operator_valid = validate_operator(val.1);
      let precedence_higher = is_operator_precedence_higher(current_operator, val.1);
      let can_evaluate_expression = operator_valid && precedence_higher;
      println!("Expression evalatable = {}", can_evaluate_expression);
      can_evaluate_expression
    },
    None => {
      true
    }
  }
}

fn is_operator_precedence_higher(subject_operator: char, next_operator: char) -> bool {
  if OPERATORS.contains(subject_operator) && OPERATORS.contains(next_operator) {
    let subject_operator_precedence: Option<usize> = OPERATORS.find(subject_operator);
    let next_operator_precedence: Option<usize> = OPERATORS.find(next_operator);
    println!("subj op: {} next op: {}", subject_operator_precedence.unwrap(), next_operator_precedence.unwrap());
    let next_operator_higher_precedence: bool = subject_operator_precedence.unwrap() < next_operator_precedence.unwrap();
    return next_operator_higher_precedence;
  }
  false
}

fn calculate_expression(operator: char, operand1: i64, operand2: i64) -> i64 {
  if operator == '^' {
    return power(operand1, operand2);
  } else if operator == '/' {
    return divide(operand1, operand2);
  } else if operator == '*' {
    return operand1 * operand2;
  } else if operator == '+' {
    return operand1 + operand2;
  } else if operator == '-' {
    return operand1 - operand2;
  }
  
  0
}

fn power(operand: i64, exponent: i64) -> i64 {
  let exponent = u32::try_from(exponent);
  match exponent {
    Ok(exp) => {
      operand.pow(exp)
    },
    Err(e) => {
      fail(&e.to_string());
      0
    }
  }
}

fn divide(operand1: i64, operand2: i64) -> i64 {
  match operand1.checked_div(operand2) {
    None => {
      fail(&"Failed to divide number".to_string());
      0
    },
    Some (result) => {
      result
    }
  }
}
