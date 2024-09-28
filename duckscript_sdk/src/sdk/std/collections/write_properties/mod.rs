use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use java_properties::write;
use std::collections::HashMap;
use std::str;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "WriteProperties")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["write_properties".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.len() < 1 {
            CommandResult::Error("Missing properties names.".to_string())
        } else {
            let (start_index, prefix) = if arguments.len() > 2 && arguments[0] == "--prefix" {
                (2, arguments[1].as_str())
            } else {
                (0, "")
            };

            let mut data = HashMap::new();
            for argument in &arguments[start_index..] {
                match variables.get(argument) {
                    Some(value) => {
                        let mut key = argument.to_string();
                        if !prefix.is_empty() {
                            key.insert(0, '.');
                            key.insert_str(0, prefix);
                        }

                        data.insert(key, value.to_string());
                    }
                    None => (),
                }
            }

            let mut buffer: Vec<u8> = vec![];
            match write(&mut buffer, &data) {
                Ok(_) => match str::from_utf8(&buffer) {
                    Ok(text) => CommandResult::Continue(Some(text.trim().to_string())),
                    Err(error) => CommandResult::Error(error.to_string()),
                },
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
