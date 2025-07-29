use crate::{config::{IconConfig, IconResource}, utils::icon_resource_path};
use core::panic;
use std::{io::{BufRead, BufReader}, path::PathBuf, process::{exit, ChildStderr, ChildStdout, Stdio}};

#[derive(clap::Parser)]
pub(crate) struct UpdateArgs {
    #[clap(short, long, value_delimiter = ',', num_args = 1..)]
    name: Option<Vec<String>>,
}

pub(crate)  fn command_update(config: IconConfig, args: UpdateArgs) {
    let mut resources: Vec<IconResource> = Vec::new();

    if let Some(names) = args.name {
        if names.is_empty() {
            resources = config.resources;
        } else {
            for name in names {
                if let Some(resource) = config.resources.iter().find(|r| r.name == name) {
                    resources.push(resource.clone())
                } else {
                    panic!("Resource '{}' not found", name);
                }
            }
        }
    } else {
        resources = config.resources;
    }

    println!("Updating resources for '{}'", resources.iter().map(|r| r.name.clone()).collect::<Vec<String>>().join(", "));

    let icon_resource_path = icon_resource_path(config.resource_path);

    for resource in resources {
        let dst = icon_resource_path.join(PathBuf::from(resource.name.clone()));

        if !dst.exists() {
            println!("Cloning resource '{}' from '{}' into '{}'", resource.name, resource.url, dst.display());
            run_git_clone(&icon_resource_path, &resource.url, &resource.name);
        } else {
            println!("Updating resource '{}' from '{}' into '{}'", resource.name, resource.url, dst.display());

            let output =  std::process::Command::new("git")
                .current_dir(&dst)
                .args(&["config", "--get", "remote.origin.url"])
                .output().expect("Failed to get remote url");

            if output.status.success() {
                let remote_url = String::from_utf8(output.stdout).expect("Failed to parse remote url");
                if resource.url.trim() != remote_url.trim() {
                    println!("Remote url mismatch, expected '{}' but got '{}'", resource.url, remote_url);
                    exit(1)
                }
            } else {
                println!("Failed to get remote url: {}", String::from_utf8(output.stderr).expect("Failed to parse remote url"));
                exit(output.status.code().unwrap_or(1))
            }
        }

        println!("Running git sparse-checkout...");
        run_git_sparse_checkout(&dst, resource.srcs, &resource.name);

        let rev = resource.rev.unwrap_or("master".to_string());

        println!("Running git checkout...");
        run_git_checkout(&dst, &rev, &resource.name);

        println!("Running git pull...");
        run_git_pull(&dst, &rev, &resource.name);

        println!("Successfully updated resource '{}'", resource.name);
    }
}

fn handle_std_reader<T: std::io::Read + Send + 'static>(io: T) -> std::thread::JoinHandle<()> {
    let reader = BufReader::new(io);
    
    std::thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("stdout: {line}");
            } else {
                break;
            }
        }
    })
}

fn handle_child_std(out: ChildStdout, err: ChildStderr) {
    let mut handles = Vec::new();
    handles.push(handle_std_reader(out));
    handles.push(handle_std_reader(err));

    for handle in handles {
        let _ = handle.join();
    }
}

pub(crate) fn run_git_clone(cwd: &PathBuf, url: &str, name: &str) {

    let mut child = std::process::Command::new("git")
        .current_dir(cwd)
        .arg("clone")
        .arg("--progress")
        .arg("--depth=1")
        .arg("--filter=blob:none")
        .arg("--no-checkout")
        .arg(url)
        .arg(name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start git clone");

    let std_out = child.stdout.take().expect("Failed to capture stdout");
    let std_err = child.stderr.take().expect("Failed to capture stderr");

    handle_child_std(std_out, std_err);

    let status = child.wait().expect("Failed to wait for git clone");

    if !status.success() {
        println!("Failed to clone resource '{}'", name);
        exit(status.code().unwrap_or(1));
    }

}

pub(crate) fn run_git_sparse_checkout(cwd: &PathBuf, srcs: Vec<String>, name: &str) {

    // make sure srcs are full paths (e.g. /src)
    let srcs = srcs.iter().map(|s| if s.starts_with('/') { s.to_string() } else { format!("/{s}") }).collect::<Vec<String>>();

    let mut child = std::process::Command::new("git")
        .current_dir(cwd)
        .arg("sparse-checkout")
        .arg("set")
        .arg("--no-cone")
        .args(srcs)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start git sparse-checkout");

    let std_out = child.stdout.take().expect("Failed to capture stdout");
    let std_err = child.stderr.take().expect("Failed to capture stderr");

    handle_child_std(std_out, std_err);

    let status = child.wait().expect("Failed to wait for git sparse-checkout");

    if !status.success() {
        println!("Failed to sparse-checkout resource '{}'", name);
        exit(status.code().unwrap_or(1));
    }

}

pub(crate) fn run_git_checkout(cwd: &PathBuf, rev: &str, name: &str) {

    let mut child = std::process::Command::new("git")
        .current_dir(cwd)
        .arg("checkout")
        //.arg(format!("origin/{}", rev))
        .arg(rev)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start git checkout");

    let std_out = child.stdout.take().expect("Failed to capture stdout");
    let std_err = child.stderr.take().expect("Failed to capture stderr");

    handle_child_std(std_out, std_err);

    let status = child.wait().expect("Failed to wait for git checkout");

    if !status.success() {
        println!("Failed to checkout resource '{}'", name);
        exit(status.code().unwrap_or(1));
    }

}

pub(crate) fn run_git_pull(cwd: &PathBuf, rev: &str, name: &str) {

    // git pull --depth 1 origin master

    let mut child = std::process::Command::new("git")
        .current_dir(cwd)
        .arg("pull")
        .arg("--progress")
        .arg("--depth=1")
        .arg("origin")
        .arg(rev)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start git pull");

    let std_out = child.stdout.take().expect("Failed to capture stdout");
    let std_err = child.stderr.take().expect("Failed to capture stderr");

    handle_child_std(std_out, std_err);

    let status = child.wait().expect("Failed to wait for git pull");

    if !status.success() {
        println!("Failed to pull resource '{}'", name);
        exit(status.code().unwrap_or(1));
    }

}