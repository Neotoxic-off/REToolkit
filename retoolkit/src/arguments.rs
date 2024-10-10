use log::{info, error, warn};
use std::{collections::HashMap};

pub struct Arguments {
    arguments_required: usize,
    arguments_count: usize,
    raw_arguments: Vec<String>,
    arguments_fields: Vec<String>,
    parsed_arguments: HashMap<String, String>
}

impl Arguments {
    pub fn new(arguments: Vec<String>, arguments_fields: Vec<&str>) -> Self {
        let mut arguments_fields_string: Vec<String> = Vec::new();

        for field in arguments_fields.iter() {
            arguments_fields_string.push(field.to_string());
        }

        Arguments {
            arguments_required: 0,
            arguments_count: 0,
            raw_arguments: arguments,
            arguments_fields: arguments_fields_string,
            parsed_arguments: HashMap::new(),
        }
        
    }

    pub fn load(&mut self) -> HashMap<String, String> {
        if self.validate_quantity() == true {
            self.parse();
            self.validate_parsed();
        } else {
            error!("required: {} arguments, found: {}", self.arguments_required, self.arguments_count);
        }


        self.parsed_arguments.clone()
    }

    fn validate_quantity(&mut self) -> bool {
        self.arguments_required = self.arguments_fields.len() * 2;
        self.arguments_count = self.raw_arguments.len();

        self.arguments_count == self.arguments_required
    }

    fn is_flag(&self, field: &str) -> bool {
        let prefix: String = String::from("--");

        field.len() > 2 && field.starts_with(&prefix)
    }

    fn validate_parsed(&self) -> bool {
        let count: usize = self.parsed_arguments.len();

        if count != (self.arguments_required / 2) {
            error!("required {} arguments, parsed: {}", self.arguments_required, count);

            return false;
        }

        true
    }

    fn parse(&mut self) -> () {
        let mut buffer: HashMap<String, String> = HashMap::new();
        let mut current_key: String = String::new();
        let mut current_value: String = String::new();

        for field in self.raw_arguments.iter() {
            if current_key.is_empty() == true {
                if self.is_flag(field) == true {
                    current_key = field.to_owned();
                }
            } else {
                current_value = field.to_owned();
            }
            println!("{}", current_key);
            println!("{}", current_value);

            if current_key.is_empty() == false && current_value.is_empty() == false {
                buffer.insert(current_key.clone(), current_value.clone());
                current_key.clear();
                current_value.clear();
            } else if current_value.is_empty() == false {
                current_value.clear();
            }
        }
        self.parsed_arguments = buffer.clone();
    }
}
