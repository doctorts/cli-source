use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommands,
}

#[derive(Parser)]
pub enum SubCommands {
    New(New),
    Generate(Generate),
}

#[derive(Parser)]
pub struct Generate {
    #[clap[subcommand]]
    pub command: BotCommands,
}

#[derive(Parser)]
pub enum BotCommands {
    Command(Command),
    Customize(Customize),
}

#[derive(Parser)]
pub struct Customize {
    pub name: String,
}

#[derive(Parser)]
pub struct Command {
    pub name: String,
}

#[derive(Parser)]
pub struct New {
    pub name: String,
}
