use std::fs;

use clap::Parser;

mod commands;
use commands::subcommands::{BotCommands, Cli, SubCommands};

mod global;
use global::content::{
    file_content, file_customize_content, package_json_content, FILE_EXAMPLE_CONTENT, MAIN_CONTENT,
    TSCONFIG_JSON,
};

fn main() {
    let args = Cli::parse();

    match args.cmd {
        SubCommands::New(t) => {
            let name = t.name;
            let src = format!("{}/{}/", name, "src");
            match fs::create_dir_all(&name) {
                Ok(_) => match fs::create_dir_all(&src) {
                    Ok(_) => {
                        let main = String::from("main.ts");

                        package_json_content(&name, "1.0.0");
                        let create_tsconfig = format!("{}/{}.json", name, "tsconfig");

                        match fs::write(create_tsconfig, TSCONFIG_JSON) {
                            Ok(_) => println!("Created tsconfig.json file"),
                            Err(_) => println!("err"),
                        }
                        let create_main = format!("{}/{}", src, main);
                        match fs::write(create_main, MAIN_CONTENT) {
                            Ok(_) => {
                                println!("Default structure of the created project");
                                let commands = "commands";

                                let dir_commands = format!("{}/{}/", src, commands);
                                match fs::create_dir_all(&dir_commands) {
                                    Ok(_) => {
                                        let dir_example = format!("{}{}/", dir_commands, "ping");
                                        match fs::create_dir_all(&dir_example) {
                                            Ok(_) => {
                                                let file_example =
                                                    format!("{}{}.ts", dir_example, "ping.command");
                                                let example = format!(
                                                    "cd {} && npm install && npm run dev",
                                                    name
                                                );
                                                match fs::write(file_example, FILE_EXAMPLE_CONTENT)
                                                {
                                                    Ok(_) => {
                                                        println!(
                                                            "\nProject started successfully!\n\n{}",
                                                            example
                                                        );
                                                    }
                                                    Err(erro) => println!("{}", erro),
                                                }
                                            }
                                            Err(erro) => println!("{}", erro),
                                        }
                                    }
                                    Err(erro) => println!("{}", erro),
                                }
                            }
                            Err(erro) => println!("{}", erro),
                        }
                    }
                    Err(erro) => println!("{}", erro),
                },
                Err(erro) => println!("{}", erro),
            }
        }
        SubCommands::Generate(t) => match t.command {
            BotCommands::Command(cmd) => {
                let name = cmd.name;

                let path = format!("src/{}/{}/", "commands", &name);
                match fs::create_dir_all(&path) {
                    Ok(_) => {
                        // let file = format!("{}{}.command.ts", path, &name);
                        file_content(&path, &name);
                    }
                    Err(erro) => println!("{}", erro),
                }
            }
            BotCommands::Customize(t) => {
                let name = t.name;

                let path = format!("src/commands/{}/customize", name);
                match fs::create_dir_all(&path) {
                    Ok(_) => file_customize_content(&path, &name),
                    Err(erro) => {
                        println!("{}", erro)
                    }
                }
            }
        },
    }
}
