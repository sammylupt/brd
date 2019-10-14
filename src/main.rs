use colored::Colorize;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Checkboxes};
use git2::{Branch, BranchType, Branches, Repository};
use std::io;

fn branches_for_repo(repo: &Repository) -> Branches {
    repo.branches(Some(BranchType::Local)).unwrap().into_iter()
}

fn branch_name(branch: &Branch) -> String {
    // this will crash if the branch name doesnt have a proper utf8 name
    branch.name().unwrap().unwrap().to_string()
}

fn main() -> Result<(), io::Error> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let repo = Repository::open(".").expect("Not in a git repo");

    let theme = ColorfulTheme::default();
    let mut menu = Checkboxes::with_theme(&theme);

    let term = Term::stdout();
    Term::clear_screen(&term).expect("Could not clear terminal screen");

    // populate list of branch names
    for maybe_branch in branches_for_repo(&repo) {
        let (branch, _) = maybe_branch.unwrap();
        let name = branch_name(&branch);
        &menu.item(&name);
    }

    println!("{} v{}", "• brd 🐧".cyan().underline().italic(), VERSION);
    println!(
        "{}",
        "  Use <space> to select branches you want to delete and <enter> to delete them\n".dimmed()
    );

    // wait for input. we collect the indices of the selected checkboxes
    let selections: Vec<usize> = menu.paged(true).interact().unwrap();

    if selections.is_empty() {
        return Ok(());
    }

    let branches_to_delete: Vec<Branch> = branches_for_repo(&repo)
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| selections.contains(&idx))
        .map(|(_, maybe_branch)| {
            let (branch, _branch_type) = maybe_branch.unwrap();
            branch
        })
        .collect();

    for mut branch in branches_to_delete {
        let name = branch_name(&branch);

        match branch.delete() {
            Ok(_) => println!("{} {}", "Deleted:".green().bold(), name),
            Err(e) => println!(
                "{} {} \n  {}",
                "Error Deleting:".red().bold(),
                name,
                e.message()
            ),
        }
    }

    Ok(())
}
