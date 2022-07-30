use std::{
    fs::{create_dir, read_to_string, write},
    io::{stdin, stdout, Write},
    num::ParseIntError,
    path::Path,
};

use json::object;
use rand::prelude::*;

struct Settings {
    file_path: String,
    random_selection: bool,
    remove_on_select: bool,
}

fn get_int_input(prompt: &str) -> Result<usize, ParseIntError> {
    let mut input = String::new();
    print!("{} >>> ", prompt);
    stdout().flush().expect("Failed to flush output buffer");
    stdin().read_line(&mut input).expect("Failed to read input");
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(&input)
        .parse::<usize>()
}

fn select_element(random: bool, remove: bool, input_list: &mut Vec<String>) -> String {
    return if input_list.len() > 0 {
        if random {
            if remove {
                input_list.remove(thread_rng().gen_range(0..input_list.len()))
            } else {
                input_list
                    .choose(&mut thread_rng())
                    .expect("Failed to select random input element")
                    .to_owned()
            }
        } else {
            if remove {
                input_list.remove(0)
            } else {
                panic!("non random selection and keep on select are not supported in combination");
            }
        }
    } else {
        "".to_string()
    };
}

fn select_elements(file_path: &str, random: bool, remove: bool, input_list: &mut Vec<String>) {
    let mut selection_list = vec![
        select_element(random, remove, input_list),
        select_element(random, remove, input_list),
        select_element(random, remove, input_list),
        select_element(random, remove, input_list),
        select_element(random, remove, input_list),
    ];

    println!("Selected elements:");
    for (i, selection) in selection_list.iter().enumerate() {
        println!("{}. {}", i + 1, selection);
    }
    println!();

    let mut selection_index: usize;
    loop {
        selection_index = match get_int_input("What would you like to do next") {
            Ok(index) => index,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        } - 1;

        if selection_index >= selection_list.len() {
            println!("Selection index is too high");
        } else {
            break;
        }
    }

    println!("Element selected: {}", selection_list[selection_index]);
    if remove {
        selection_list.remove(selection_index);
    }

    input_list.append(&mut selection_list);
    write(file_path, input_list.join("\n")).expect("Failed to write contents back to the file");
}

fn main() {
    // create settings dir if not exists
    if !Path::new("./res").exists() {
        create_dir("./res").expect("Failed to create resource directory on first run");
    }

    if !Path::new("./res/settings.json").exists() {
        write(
            "./res/settings.json",
            format!(
                "{:#}",
                object! {
                    file_path: "./res/input.list",
                    random_selection: false,
                    remove_on_select: true,
                }
            ),
        )
        .expect("Failed to write settings to settings.json file");
    }

    if !Path::new("./res/input.list").exists() {
        write("./res/input.list", "").expect("Failed to create input list file");
    }

    // load in settings
    let mut settings = {
        let settings_json = json::parse(
            read_to_string(Path::new("./res/settings.json"))
                .expect("Failed to read input settings file")
                .as_str(),
        )
        .expect("Failed to parse input file into proper json text");

        Settings {
            file_path: settings_json["file_path"]
                .as_str()
                .expect("Failed to parse file_path input parameters from json system")
                .to_string(),
            random_selection: settings_json["random_selection"]
                .as_bool()
                .expect("Failed to parse random_selection input parameters from json system"),
            remove_on_select: settings_json["remove_on_select"]
                .as_bool()
                .expect("Failed to parse remove_on_select input parameters from json system"),
        }
    };

    let mut input_list = read_to_string(&settings.file_path)
        .expect("Failed to read input list file")
        .split("\n")
        .map(|input_line| String::from(input_line))
        .collect::<Vec<_>>();

    loop {
        println!("Selection options are:");
        println!("1. Select items from input list");
        println!("2. Reload settings");
        println!("3. Quit");
        println!();

        match match get_int_input("What option would you like to run") {
            Ok(input) => input,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        } {
            1 => select_elements(
                &settings.file_path,
                settings.random_selection,
                settings.remove_on_select,
                &mut input_list,
            ),
            2 => {
                settings = {
                    let settings_json = json::parse(
                        read_to_string(Path::new("./res/settings.json"))
                            .expect("Failed to read input settings file")
                            .as_str(),
                    )
                    .expect("Failed to parse input file into proper json text");

                    Settings {
                        file_path: settings_json["file_path"]
                            .as_str()
                            .expect("Failed to parse file_path input parameters from json system")
                            .to_string(),
                        random_selection: settings_json["random_selection"].as_bool().expect(
                            "Failed to parse random_selection input parameters from json system",
                        ),
                        remove_on_select: settings_json["remove_on_select"].as_bool().expect(
                            "Failed to parse remove_on_select input parameters from json system",
                        ),
                    }
                }
            }
            3 => break,
            _ => {}
        }
    }
}
