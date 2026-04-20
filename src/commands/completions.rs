use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

pub fn handle<C: CommandFactory>(shell: Shell) {
    let mut cmd = C::command();
    let cmd_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, cmd_name, &mut io::stdout());
}
