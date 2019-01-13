extern crate clap;
extern crate linked_hash_map;
extern crate serde_yaml;

use std::io::prelude::*;

use std::fs::File;

use std::process::{Command, Stdio};

use std::vec::Vec;

use linked_hash_map::LinkedHashMap;

use clap::{App, Arg};

const ROFI: &str = "/usr/bin/rofi";

/// A trait for creating actions taken when a menu item is selected.
///
/// A type implementing `RofiAction` is one that, when selected through rofi,
/// has a meaningful action to perform.
trait RofiAction {
    /// Executes the action and the action to be performed next.
    fn run(&self) -> Result<Option<&Box<RofiAction>>, String>;
}

/// A struct for executing shell commands through a rofi menu.
struct RofiCommand {
    /// The name of the command to be executed.
    command: String,
    /// The arguments of the command to be executed.
    args: Vec<String>,
}

impl RofiCommand {
    pub fn new(command: String, args: Vec<String>) -> RofiCommand {
        RofiCommand { command, args }
    }
}

impl RofiAction for RofiCommand {
    fn run(&self) -> Result<Option<&Box<RofiAction>>, String> {
        Command::new(&self.command)
            .args(&self.args)
            .output()
            .or(Err("Failed to run command"))?;
        Ok(None)
    }
}

/// A struct for displaying rofi menus
struct RofiMenu {
    /// The name of the `RofiMenu`.
    name: String,
    /// The prompt displayed in he menu.
    prompt: String,
    /// The options available in the `RofiMenu`.
    options: LinkedHashMap<String, Box<RofiAction>>,
}

impl RofiMenu {
    pub fn new(
        name: String,
        prompt: String,
        options: LinkedHashMap<String, Box<RofiAction>>,
    ) -> RofiMenu {
        RofiMenu {
            name,
            options,
            prompt,
        }
    }

    fn optionstring(&self) -> String {
        let key_vec: Vec<&String> = self.options.keys().collect();
        let res = key_vec.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{}\n", x));
            acc
        });
        res
    }
}

impl RofiAction for RofiMenu {
    fn run(&self) -> Result<Option<&Box<RofiAction>>, String> {
        let mut comm = Command::new(&ROFI)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // Add the arguments:
            //
            // rofi -p <menu name> -l <number of items> -dmenu -i -no-custom
            .arg("-p")
            .arg(&self.prompt)
            .arg("-l")
            .arg(self.options.len().to_string())
            .arg("-dmenu")
            .arg("-i")
            .arg("-no-custom")
            .spawn()
            .or(Err("Failed to spawn process"))?;
        let stdin = comm.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin
            .write_all(&self.optionstring().into_bytes())
            .or(Err("Failed to write in stdin"))?;
        let output = comm.wait_with_output().or(Err("Failed to read stdout"))?;
        let output = String::from_utf8_lossy(&output.stdout);
        if output.is_empty() {
            return Ok(None);
        }
        self.options
            .get::<str>(&output.trim())
            .map_or(Err("Menu item has no action".to_string()), |x| Ok(Some(x)))
    }
}

fn get_config(filename: &str) -> Result<serde_yaml::Value, String> {
    let mut file = File::open(filename).or(Err("Could not open config file"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .or(Err("Could not read config file"))?;
    let yaml_menu: serde_yaml::Value =
        serde_yaml::from_str(contents.as_str()).or(Err("Could not parse config file"))?;
    Ok(yaml_menu)
}

type FnBuild = Fn(&serde_yaml::Value) -> Result<Box<dyn RofiAction>, String>;
struct Builder {
    sub_builders: LinkedHashMap<String, Box<FnBuild>>,
}

/// A struct that builds a RofiAction hierarchy from a YAML file.
impl Builder {
    pub fn new() -> Builder {
        let sub_builders = LinkedHashMap::new();
        Builder { sub_builders }
    }

    /// Add a subbuilder to the builder.
    pub fn add_subbuilder(&mut self, type_str: String, subbuilder: Box<FnBuild>) {
        self.sub_builders.insert(type_str, subbuilder);
    }

    fn build_menu(&self, yaml_menu: &serde_yaml::Value) -> Result<Box<dyn RofiAction>, String> {
        let name = yaml_menu
            .get("name")
            .ok_or("RofiMenu has no name")?
            .as_str()
            .ok_or("Name is not a string")?;
        let prompt = yaml_menu
            .get("prompt")
            .ok_or("RofiMenu has no prompt")?
            .as_str()
            .ok_or("Prompt is not a string")?;
        let options = yaml_menu
            .get("options")
            .ok_or("RofiMenu has no options")?
            .as_sequence()
            .ok_or("Options is not a sequence")?
            .iter()
            .filter_map(|x| if x.is_mapping() { Some(x) } else { None });

        let mut option_map = LinkedHashMap::new();
        for opt in options {
            let display_string = opt
                .get("string")
                .ok_or("Menu option has not string")?
                .as_str()
                .ok_or("Menu option string is not a string")?;
            let action = opt.get("action").ok_or("Menu option has no action")?;
            let action = self.build_action(action)?;
            option_map.insert(String::from(display_string), action);
        }
        Ok(Box::new(RofiMenu::new(
            String::from(name),
            String::from(prompt),
            option_map,
        )))
    }

    fn build_command(
        &self,
        yaml_command: &serde_yaml::Value,
    ) -> Result<Box<dyn RofiAction>, String> {
        let command = yaml_command
            .get("command")
            .ok_or("RofiCommand has no command")?;
        let command = command.as_str().ok_or("Command is not a string")?;
        let command = String::from(command);

        let args = yaml_command
            .get("args")
            .ok_or("RofiCommand has no args")?
            .as_sequence()
            .ok_or("Args is not a sequence")?;
        let args: Vec<String> = args
            .iter()
            .filter_map(|x| x.as_str())
            .map(|x| String::from(x))
            .collect();
        Ok(Box::new(RofiCommand::new(command, args)))
    }

    fn build_action(&self, yaml: &serde_yaml::Value) -> Result<Box<dyn RofiAction>, String> {
        let t = yaml
            .get("type")
            .ok_or("Action with no type")?
            .as_str()
            .ok_or("Type is not string")?;
        match t {
            "RofiMenu" => self.build_menu(yaml),
            "RofiCommand" => self.build_command(yaml),
            _ => {
                let sub_builder = self
                    .sub_builders
                    .get(t)
                    .ok_or(format!("Unknown type {}", t))?;
                sub_builder(yaml)
            }
        }
    }
}

fn create_parser() -> App<'static, 'static> {
    App::new("rmenu")
        .version("0.1")
        .author("mandragore")
        .about("Creates custom rofi menus")
        .arg(
            Arg::with_name("config")
                .long("--config")
                .help("The configuration file")
                .takes_value(true)
                .required(true),
        )
}

fn main() -> Result<(), String> {
    let parser = create_parser();
    let matches = parser.get_matches();
    let conf_filename = matches
        .value_of("config")
        .ok_or("Required argument config is missing")?;
    let builder = Builder::new();
    let yaml_menu = get_config(conf_filename).or(Err("Could not load config file"))?;
    let main_menu = builder.build_action(&yaml_menu)?;
    let mut action = main_menu.run()?;
    loop {
        action = match action {
            None => {
                break;
            }
            Some(a) => a.run()?,
        }
    }
    Ok(())
}
