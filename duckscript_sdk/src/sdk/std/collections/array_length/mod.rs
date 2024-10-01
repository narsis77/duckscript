use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ArrayLength")
    }

    fn aliases(&self) -> Vec<String> {
        vec![
            "array_length".to_string(),
            "arrlen".to_string(),
            "array_size".to_string(),
        ]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Array handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(arguments.state);

            let key = &arguments.args[0];

            match state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::List(list) => CommandResult::Continue(Some(list.len().to_string())),
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => CommandResult::Error(
                    format!("Array for handle: {} not found.", key).to_string(),
                ),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
