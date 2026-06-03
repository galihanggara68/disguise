use crate::Cli;
use clap::CommandFactory;
use clap_complete::{Shell, generate};

pub struct CompletionsOptions {
    pub shell: Shell,
}

pub fn handle(options: CompletionsOptions) {
    let mut cmd = Cli::command();
    let cmd_name = cmd.get_name().to_string();

    let mut buf = Vec::new();
    generate(options.shell, &mut cmd, &cmd_name, &mut buf);
    let script = String::from_utf8_lossy(&buf);

    let patched_script = match options.shell {
        Shell::Bash => patch_bash(&script),
        Shell::Zsh => patch_zsh(&script),
        Shell::Fish => patch_fish(&script),
        _ => script.to_string(),
    };

    println!("{}", patched_script);
}

fn patch_bash(script: &str) -> String {
    let mut patched = script.to_string();

    // Patch for run, detail, remove, update to include script names
    let subcommands = ["run", "detail", "remove", "update"];
    for sub in subcommands {
        let pattern = format!("disguise__subcmd__{})", sub);
        let replacement = format!(
            "{}\n            if [[ ${{COMP_CWORD}} -eq 2 ]] ; then\n                COMPREPLY=( $(compgen -W \"$(disguise list --names-only)\" -- \"${{cur}}\") )\n                return 0\n            fi",
            pattern
        );
        patched = patched.replace(&pattern, &replacement);
    }

    // Also patch for tag add/remove
    patched = patched.replace(
        "disguise__subcmd__tag__subcmd__add)",
        "disguise__subcmd__tag__subcmd__add)\n            COMPREPLY=( $(compgen -W \"$(disguise list --names-only)\" -- \"${{cur}}\") )\n            return 0",
    );
    patched = patched.replace(
        "disguise__subcmd__tag__subcmd__remove)",
        "disguise__subcmd__tag__subcmd__remove)\n            COMPREPLY=( $(compgen -W \"$(disguise list --names-only)\" -- \"${{cur}}\") )\n            return 0",
    );

    patched
}

fn patch_zsh(script: &str) -> String {
    let mut patched = script.to_string();

    // Patch for run, detail, remove, update
    let patterns = [
        (
            ":name -- Name of the script:",
            ":name -- Name of the script:($(disguise list --names-only))",
        ),
        (
            ":name -- Name of the script to remove:",
            ":name -- Name of the script to remove:($(disguise list --names-only))",
        ),
        (
            ":name -- Name of the script to update:",
            ":name -- Name of the script to update:($(disguise list --names-only))",
        ),
        (
            "*::scripts -- Scripts to add tags to:_default",
            "*::scripts -- Scripts to add tags to:($(disguise list --names-only))",
        ),
        (
            "*::scripts -- Scripts to remove tags from:_default",
            "*::scripts -- Scripts to remove tags from:($(disguise list --names-only))",
        ),
    ];

    for (old, new) in patterns {
        patched = patched.replace(old, new);
    }

    patched
}

fn patch_fish(script: &str) -> String {
    let mut patched = script.to_string();

    let subcommands = ["run", "detail", "remove", "update"];
    for sub in subcommands {
        let line = format!(
            "complete -c disguise -n \"__fish_disguise_using_subcommand {}\" -f -a \"(disguise list --names-only)\"",
            sub
        );
        patched.push_str(&line);
        patched.push('\n');
    }

    // Patch for tag add/remove
    patched.push_str("complete -c disguise -n \"__fish_disguise_using_subcommand tag; and __fish_disguise_using_subcommand add\" -f -a \"(disguise list --names-only)\"\n");
    patched.push_str("complete -c disguise -n \"__fish_disguise_using_subcommand tag; and __fish_disguise_using_subcommand remove\" -f -a \"(disguise list --names-only)\"\n");

    patched
}
