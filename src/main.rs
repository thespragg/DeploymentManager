use clap::{App, Arg};
use colour::*;
use std::env;
use std::io;

mod config;

fn main() {
    let loc = config::load_config();
    println!("{}", loc.unwrap());

    let matches = App::new("Deploy Manager")
        .version("0.1")
        .author("Alistair Spragg <alistair.spragg@gmail.com>.")
        .about("Handles automated deployment of multi-site applications.")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .value_name("NAME")
                .takes_value(true)
                .about("Compares files in destination folders with the central copy to determine if there's a version mismatch. To check all provide 'all' as the name."),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .value_name("PATH")
                .takes_value(true)
                .about("Sets the deployment location for the current process.")
        )
        .subcommand(
            App::new("new")
                .about("Starts the creation wizard for a new deployed application.")
                .arg(
                    Arg::new("target")
                    .short('t')
                    .long("target")
                    .value_name("PATH")
                    .about("The root folder where your application_deployments.conf file is (or will be) located.")
                    .required(true)
                )
                .arg(
                    Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .about("The name of the new deployed application.")
                    .required(true)
                )
                .arg(
                    Arg::new("path")
                    .short('p')
                    .long("path")
                    .value_name("PATH")
                    .about("Path to the new application.")
                    .default_value(&env::current_dir().unwrap().to_str().unwrap())
                )
        )
        .get_matches();

    if let Some(c) = matches.value_of("check") {
        if c == "all" {
            println!("Checking all applications");
        } else {
            println!("Checking application: {}", c);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") && matches.is_present("path") {
            let n = matches.value_of("name");
            let p = matches.value_of("path");
            green_ln!("Creating new deployed app with name: {}", n.unwrap());
            green_ln!("Using path '{}' as application root", p.unwrap());
            green_ln!("Using version number: 1.0");
            println!("");

            let mut app_type = String::new();
            println!("What type of application is it you're deploying? ([c]#/[v]ue):");
            match io::stdin().read_line(&mut app_type) {
                Ok(_goes_into_input_above) => {}
                Err(_no_updates_is_fine) => {}
            }

            app_type = app_type.trim().to_string();

            if app_type == "c" {
                app_type = "c#".to_string();
            } else if app_type == "v" {
                app_type = "v".to_string();
            }

            green_ln!("Chosen app type is: {}", app_type);

            let mut deploy_folders_vec = Vec::new();

            println!("");
            println!("Enter the folders you'd like the application to be deployed to pressing enter after each one. When finished press enter on the blank line to continue.");

            loop {
                let mut deploy_folder = String::new();
                match io::stdin().read_line(&mut deploy_folder) {
                    Ok(_goes_into_input_above) => {}
                    Err(_no_updates_is_fine) => {}
                }

                if deploy_folder.trim().is_empty() {
                    break;
                }
                deploy_folder = deploy_folder.trim().to_string();
                deploy_folders_vec.push(deploy_folder);
            }

            green_ln!("Application will be deployed to the following folders:");
            for i in &deploy_folders_vec {
                println!("{}", i);
            }

            //Move files to minimart from {p}
            //Create file with relevant info called deploy.conf

            //Append text to DeployedApps.conf in minimart root containing new folder

            green_ln!("{} has been added to the deployment schedule", n.unwrap());
            red_ln!("Deployments will happen at 03:00 GMT");
        }
    }
}
