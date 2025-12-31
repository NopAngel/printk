// main.rs - main file
//
//    with: rust
//

use printk::{printk, printk_at_y, Printk};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let pk = Printk::new();

    if args.len() < 2 {
        show_help(&pk)?;
        return Ok(());
    }

    match args[1].as_str() {
        "print" => {
            if args.len() < 3 {
                pk.println("{error} Use: printk print <message>")?;
                return Ok(());
            }
            let message = &args[2..].join(" ");
            printk(message)?;
        }

        "print-y" => {
            if args.len() < 4 {
                pk.println("{error} Use: printk print-y <line> <message>")?;
                return Ok(());
            }
            let y: i16 = args[2].parse()?;
            let message = &args[3..].join(" ");
            printk_at_y(y, message)?;
        }

        "demo" => {
            run_demo()?;
        }

        "icons" => {
            list_icons(&pk)?;
        }

        _ => {
            show_help(&pk)?;
        }
    }
    Ok(())
}

// show help func

fn show_help(pk: &Printk) -> Result<(), Box<dyn std::error::Error>> {
    // Extract the version that is in `Cargo.toml` and place it in a variable
    let version = env!("CARGO_PKG_VERSION");
    println!("\n                     Printk v{}",version);


    pk.println("       {help}  Print messages with Nerd Font icons")?;
    println!();
    println!("Use:");
    println!("  printk print <message>");
    println!("  printk print-y <line> <message>");
    println!("  printk demo");
    println!("  printk icons");
    println!();
    pk.println("{star} Examples:")?;
    println!(r#"  printk print "{{info}} Hellooo, worldd! {{success}}""#);
    println!(r#"  printk print-y 5 "{{warning}} Online alert 5""#);
    println!(r#"  printk print "{{rust}} Made in Rust {{check}}""#);
    println!();
    pk.println("{arrow} Note: use {{ }} to display literal keys")?;

    Ok(())
}

fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    let pk = Printk::new();

    print!("\x1B[2J\x1B[1;1H");

    pk.println("{gear} Example")?;
    pk.println("{arrow} ======================")?;
    println!();

    pk.println("{info} Test1")?;
    pk.println("{success} Successful operation")?;
    pk.println("{warning} Important warning")?;
    pk.println("{error} Critical error")?;
    println!();

    print!("\x1B[16;1H");
    println!();

    let pk_red = Printk::new().with_color(colored::Color::Red);
    pk_red.println("{heart} Red Color")?;

    let pk_green = Printk::new().with_color(colored::Color::Green);
    pk_green.println("{check} Green Color")?;

    println!();
    pk.println("{rust} File Rust {check}")?;
    pk.println("{python} Script Python {star}")?;
    pk.println("{git} Commit {branch}")?;
    println!();

    pk.println("{clock} Process:")?;
    for i in 1..=5 {
        let current_line = 22;
        pk.clear_from_y(current_line)?;
        pk.print_at_y(current_line, &format!("  {{arrow-right}} Processing... {}/5", i))?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    pk.clear_from_y(22)?;
    pk.println("{success} Demonstration complete!")?;
    println!();

    Ok(())
}

fn list_icons(pk: &Printk) -> Result<(), Box<dyn std::error::Error>> {
    pk.println("{folder} Available icons:")?;
    pk.println("{arrow} ==================")?;
    println!();

    let icons = pk.list_icons();

    let categories = [
        ("Files", vec!["folder", "file", "doc", "img", "pdf", "zip"]),
        ("Status", vec!["ok", "error", "warning", "info", "question", "help", "success", "fail", "check", "warn"]),
        ("Arrows", vec!["arrow", "arrow-right", "arrow-left", "arrow-up", "arrow-down"]),
        ("UI", vec!["gear", "home", "star", "heart", "trash", "edit", "add", "plus", "minus", "close", "search"]),
        ("Programming", vec!["rust", "python", "js", "ts", "java", "go", "c", "cpp"]),
        ("Git", vec!["git", "branch", "commit", "merge"]),
    ];

    for (category_name, icon_names) in categories.iter() {
        pk.println(&format!("{{folder}} {}:", category_name))?;

        for icon_name in icon_names {
            if let Some(icon) = pk.get_icon(icon_name) {
                println!("  {{{}}} → {}", icon_name, icon);
            }
        }
        println!();
    }

    pk.println(&format!("{{info}} Total: {} available icons", icons.len()))?;
    Ok(())
}
