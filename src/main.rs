use std::io::{self, Write, BufRead};
use std::path::Path;
use std::fs::{self, File};

struct Command {
  op: String,
  key: String,
  value: String
}

fn main() {

    setup_db();

    let mut input = String::new();

    loop {
      io::stdin().read_line(&mut input).unwrap();
      let command = parse_input(&input);
      process_command(command);
      input.clear();
    }
    
}

fn setup_db() {
  // create db.txt file if it doesn't exist
  let result;
  
  if !std::path::Path::new("db.txt").exists() {
    result = std::fs::File::create("db.txt");
  } else {
    result = Ok(std::fs::File::open("db.txt").unwrap());
  }
  
  match result {
    Ok(_) => println!("Created db.txt"),
    Err(_) => panic!("Failed to create db.txt")
  } 
}

fn parse_input(input: &String) -> Command {
  let mut input = input.split_whitespace();
  
  let op = match input.next() {
    Some("get") => "get",
    Some("set") => "set",
    Some(_) => panic!("Invalid command"),
    None => panic!("Invalid command")
  };

  let key = match input.next() {
    Some(key) => key,
    None => panic!("Invalid command")
  };

  let value = match input.next() {
    Some(value) => value,
    None => {
      if op == "set" {
        panic!("Invalid command")
      } else {
        ""
      }
    }
  };

  return Command {
    op: String::from(op),
    key: String::from(key),
    value: String::from(value)
  }
}

fn process_command(command: Command) {
  if command.op == "set" {
    set_key(command.key, command.value);
  } else if command.op == "get" {
    let value = get_key(command.key);
    println!("Value: {}", value);
  } else {
    panic!("Invalid command");
  }
}

fn set_key(key: String, value: String) {
  let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("db.txt")
    .unwrap();

  let line = format!("{} {}\n", key, value);
  file.write_all(line.as_bytes()).unwrap(); 
}

fn get_key(key: String) -> String {
  // search db.txt for key and return value
  if let Ok(lines) = read_lines("db.txt") {
    let mut value = String::new();

    for line in lines {
      if let Ok(line) = line {
        let mut line = line.split_whitespace();
        let line_key = line.next().unwrap();
        let line_value = line.next().unwrap();

        if line_key == key {
          value = String::from(line_value);
        }
      }
    }

    return value;
  } else {
    panic!("Failed to read db.txt");
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

