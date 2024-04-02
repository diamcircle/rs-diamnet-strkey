use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = diamnet_strkey::VERSION;
        println!("diamnet-strkey {} ({})", v.pkg, v.rev);
    }
}
