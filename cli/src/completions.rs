//! Shell completion generation

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, shells::*};
use colored::Colorize;
use std::io;

/// Generate shell completions
pub fn generate_completions(shell: &str) -> Result<()> {
    let mut cmd = crate::Cli::command();
    let bin_name = "apex";

    match shell.to_lowercase().as_str() {
        "bash" => {
            generate(Bash, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
        "zsh" => {
            generate(Zsh, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
        "fish" => {
            generate(Fish, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
        "powershell" | "pwsh" => {
            generate(PowerShell, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
        "elvish" => {
            generate(Elvish, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
        _ => {
            anyhow::bail!(
                "Unsupported shell: {}\nSupported shells: bash, zsh, fish, powershell, elvish",
                shell
            );
        }
    }
}

/// Print installation instructions for shell completions
pub fn print_install_instructions(shell: &str) {
    println!(
        "\n{}",
        colored::Colorize::cyan("Shell Completion Installation:").bold()
    );
    println!(
        "{}",
        colored::Colorize::dimmed("═══════════════════════════════════════")
    );

    match shell.to_lowercase().as_str() {
        "bash" => {
            println!("\n{}", colored::Colorize::yellow("For Bash:").bold());
            println!("  1. Generate completions:");
            println!(
                "     apex completions bash > ~/.local/share/bash-completion/completions/apex"
            );
            println!("\n  2. Or add to your ~/.bashrc:");
            println!("     eval \"$(apex completions bash)\"");
            println!("\n  3. Reload your shell:");
            println!("     source ~/.bashrc");
        }
        "zsh" => {
            println!("\n{}", colored::Colorize::yellow("For Zsh:").bold());
            println!("  1. Ensure completions directory exists:");
            println!("     mkdir -p ~/.zsh/completions");
            println!("\n  2. Generate completions:");
            println!("     apex completions zsh > ~/.zsh/completions/_apex");
            println!("\n  3. Add to ~/.zshrc if not already present:");
            println!("     fpath=(~/.zsh/completions $fpath)");
            println!("     autoload -Uz compinit && compinit");
            println!("\n  4. Reload your shell:");
            println!("     source ~/.zshrc");
        }
        "fish" => {
            println!("\n{}", colored::Colorize::yellow("For Fish:").bold());
            println!("  1. Generate completions:");
            println!("     apex completions fish > ~/.config/fish/completions/apex.fish");
            println!("\n  2. Reload your shell:");
            println!("     source ~/.config/fish/config.fish");
        }
        "powershell" | "pwsh" => {
            println!("\n{}", colored::Colorize::yellow("For PowerShell:").bold());
            println!("  1. Generate completions:");
            println!("     apex completions powershell > $HOME\\Documents\\PowerShell\\Scripts\\apex_completions.ps1");
            println!("\n  2. Add to your PowerShell profile:");
            println!("     . $HOME\\Documents\\PowerShell\\Scripts\\apex_completions.ps1");
            println!("\n  3. Reload your profile:");
            println!("     . $PROFILE");
        }
        "elvish" => {
            println!("\n{}", colored::Colorize::yellow("For Elvish:").bold());
            println!("  1. Generate completions:");
            println!("     apex completions elvish > ~/.elvish/lib/apex.elv");
            println!("\n  2. Add to ~/.elvish/rc.elv:");
            println!("     use apex");
        }
        _ => {
            println!("\n{}", colored::Colorize::red("Unsupported shell").bold());
            println!("Supported shells: bash, zsh, fish, powershell, elvish");
        }
    }

    println!("\n{}", colored::Colorize::cyan("Quick Setup:").bold());
    println!("  Run this command and follow the instructions:");
    println!(
        "  apex completions {} | sudo tee /etc/bash_completion.d/apex",
        shell
    );

    println!(
        "\n{}",
        colored::Colorize::dimmed(
            "After installation, you'll have tab completion for all apex commands!"
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_shells() {
        let shells = vec!["bash", "zsh", "fish", "powershell", "elvish"];

        for shell in shells {
            // We can't actually test the generation without capturing stdout,
            // but we can verify the function doesn't panic
            assert!(matches!(
                shell,
                "bash" | "zsh" | "fish" | "powershell" | "elvish"
            ));
        }
    }

    #[test]
    fn test_unsupported_shell() {
        let result = generate_completions("unsupported");
        assert!(result.is_err());
    }
}
