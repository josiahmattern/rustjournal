use chrono::Local;
use std::{
    fs,
    io::{self},
    process::Command,
};

fn main() {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let journal_dir = home_dir.join("journal");
    let config_file = journal_dir.join("editor_config.txt");

    // create the journal directory if it does not exist
    if !journal_dir.exists() {
        fs::create_dir_all(&journal_dir).expect("Failed to create journal directory");
    }

    // get date so I can sort files well
    let dt = Local::now();
    let date = dt.format("%Y-%m-%d").to_string();

    //get the editor the user wants and put in config file
    let editor = match fs::read_to_string(&config_file) {
        Ok(editor) => editor,
        Err(_) => {
            println!("Enter your preferred editor (e.g., nvim):");
            let mut editor = String::new();
            io::stdin()
                .read_line(&mut editor)
                .expect("Failed to read line");
            editor = editor.trim().to_string();
            fs::write(&config_file, &editor).expect("Failed to write to config file");
            editor
        }
    };

    let journal_file = journal_dir.join(format!("{}.md", date));

    match Command::new(editor)
        .arg(journal_file.to_str().unwrap()) // Pass the path to the journal file as an argument
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait().expect("failed to wait on child");
        }
        Err(e) => println!("Failed to execute command: {}", e),
    }
}
