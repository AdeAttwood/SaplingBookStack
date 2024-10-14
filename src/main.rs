mod client;
mod commit;
mod github;
mod repository;
mod revset;
mod stack;

use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "book-stack")]
#[command(about = "Managing sapling stacks with bookmarks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Preview your current stack of changes
    Preview,
    /// Print your current stack state in JSON format
    Json,
    /// Update your current stack. This will push each change, relinking them with the upstream PRs
    /// if possible.
    Update,
}

fn command_preview() -> Result<(), String> {
    let repo = repository::Repository::new()?;
    let stack = stack::build_stack()?;

    println!("Stack for: {}", repo.default_path);
    println!();

    for item in stack {
        match github::pull_request(&repo.url, &item.head.name()?) {
            Some(pr) => println!("Change: {}", pr.url),
            None => println!("Change: {}", item.compare_url(&repo.url)),
        }

        for commit in item.commits {
            println!("  - {} ({})", commit.title, commit.short_node);
        }

        println!();
    }

    Ok(())
}

fn command_update() -> Result<(), String> {
    let repo = repository::Repository::new()?;
    let stack = stack::build_stack()?;

    // TODO: Uplink all the current prs in the stack
    // sl pr unlink "top % public()"

    for item in stack {
        let mut upstream_object_id = item.head.node.clone();
        let mut notes = match github::pull_request(&repo.url, &item.head.name()?) {
            Some(pr) => {
                println!("Change: {}", pr.url);

                upstream_object_id = pr.head_ref_oid.clone();

                let notes_content = client::notes(&pr.head_ref_oid)?;
                serde_json::from_str::<Vec<stack::Change>>(&notes_content)
                    .map_err(|e| e.to_string())?
            }
            None => {
                println!("Change: {}", item.compare_url(&repo.url));
                Vec::new()
            }
        };

        // Add the current change to the notes and add them to the object. If we have already
        // pushed this change, we don't need to add it again.
        if upstream_object_id != item.head.node {
            notes.push(item.clone());
            client::add_stack_note(&item.head.node, notes)?;

            // We don't even need to push this change
            client::push(&item.head.node, &item.head.name()?)?;
        }

        for commit in item.commits {
            println!("  - {} ({})", commit.title, commit.short_node);
        }

        println!();
    }

    Ok(())
}

fn command_json() -> Result<(), String> {
    let stack = stack::build_stack()?;

    print!(
        "{}",
        serde_json::to_string(&stack).map_err(|e| e.to_string())?
    );

    Ok(())
}

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Commands::Preview => command_preview(),
        Commands::Json => command_json(),
        Commands::Update => command_update(),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
