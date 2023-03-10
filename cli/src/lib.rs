// CLI management system library
// Code will be improved later

use std::{
    process,
    io,
};
use engine_core::{
    self,
    DatabaseManager,
};

/// Configures program data
pub struct Config {
    engine_core_config: engine_core::Config,
    // TODO
    // current_database: Option<&'a engine_core::Database<'a>>,
}

impl Config {
    /// Builds a new program configuration
    pub fn build() -> Self {
        Self {
            engine_core_config: engine_core::Config::build(),
        }
    }
}

/// Runs the program
pub fn run(config: Config) {
    let mut engine = config.engine_core_config;

    let help_message = "Write /help for all available commands";

    println!("\n{}\n", "Database Engine Project");
    println!("{}", help_message);

    // Program main loop
    loop {
        let mut input_command = String::new();

        println!("\n{}", "Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                                   List all available commands
  /q                                      Quit
  (FOR TESTING) /test connection          Test connection to database manager
  /connection status                      Display current connection status
  /connect                                Connect to database manager
  /disconnect                             Disconnect from database manager

  /databases                              List all databases
  /create database                        Create a new database
  /delete database                        Delete a database
  /connect database                       Connect to a database
  (DISABLED) /disconnect database         Disconnect from a database

  (DISABLED) /tables                      List all tables in the current database
  (DISABLED) /create table                Create a new table in the currently connected database
  (DISABLED) /delete table                Delete a table in the currently connected database
  (DISABLED) /view table                  View a table in the currently connected database
  (DISABLED) /view table data             View table data in the currently connected database
  More commands in the future...
"
                );
                continue
            }
            "/q" => {
                exit_program()
            },
            "/test connection" => {
                println!("{:?}", engine.database_manager())
            },
            "/connection status" => {
                display_connection_status(engine.database_manager())
            },
            "/connect" => {
                engine.database_manager_mut().connect();
            },
            "/disconnect" => {
                engine.database_manager_mut().disconnect();
            },
            "/databases" => {
                if engine.database_manager().connected() {
                    list_all_databases(engine.database_manager());
                } else {
                    not_connected_to_db_manager();
                }
            },
            "/create database" => {
                if engine.database_manager().connected() {
                    prompt_database_creation(engine.database_manager_mut());
                } else {
                    not_connected_to_db_manager();
                }
            },
            "/delete database" => {
                if engine.database_manager().connected() {
                    prompt_database_deletion(engine.database_manager_mut());
                } else {
                    not_connected_to_db_manager();
                }
            },
            "/connect database" => {
                // Disconnect current database
                // Connect to a database
                // Make that database the current database
            },
            _ => {
                println!("No such command found!");
                println!("{}", help_message);
                continue
            },
        }
    }
}

fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

fn display_connection_status(database_manager: &DatabaseManager) {
    let db_manager_connected = match database_manager.connected() {
        true => "Yes",
        false => "No",
    };
    
    // Display whether connected to database manager
    println!("\nConnected to database manager: {}", db_manager_connected);
    
    // Display connected databases
    println!("Connected databases:");

    for i in database_manager.databases().iter() {
        if i.connected() {
            println!("{}", i.name());
        }
    }
}

// Temporary
fn not_connected_to_db_manager() {
    println!("\
Not connected to database manager! \
Type /connect to connect to database manager."
    );
}

fn prompt_database_creation(database_manager_mut: &mut DatabaseManager) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    io::stdin()
        .read_line(&mut database_name)
        .expect("Failed to read line");

    let database_name = database_name.trim();

    println!("Confirm to create a new database named {}", database_name);
    println!("Yes?: y");
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    let confirm = confirm.trim();

    match confirm {
        // Create database
        "y" => {
            database_manager_mut.create_database(database_name);
        },
        _ => {
            println!("Canceled database creation");
        },
    }
}

fn prompt_database_deletion(database_manager_mut: &mut DatabaseManager) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    io::stdin()
        .read_line(&mut database_name)
        .expect("Failed to read line");

    let database_name = database_name.trim();

    println!("Confirm to delete database named {}", database_name);
    println!("Yes?: y");
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    let confirm = confirm.trim();

    match confirm {
        // Delete database
        "y" => {
            database_manager_mut.delete_database(database_name);
        },
        _ => {
            println!("Canceled database deletion");
        },
    }
}

fn list_all_databases(database_manager: &DatabaseManager) {
    println!(
        "\n{}{}",
        "Number of databases: ",
        database_manager.databases().len(),
    );

    for i in database_manager.databases().iter() {
        println!("{:?}", i);
    }
}