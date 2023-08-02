use std::path::PathBuf;

use crates_io_api::{Error, SyncClient};
use run_shell::cmd;

use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct GetDependents {}

impl RunCommand for GetDependents {
    fn run_command(&mut self) -> Result<()> {
        clone_repository();
        Ok(())
    }
}

fn get_crate_dependents_repositorys(name: &str) -> Result<Vec<String>, Error> {
    let mut repos = Vec::new();

    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(200),
    )
    .unwrap();
    // Retrieve summary data.
    //let dependents = client.crate_reverse_dependencies(name)?;
    let dependents = client.crate_reverse_dependencies_page(name, 1)?;
    let mut loop_cnt = 0;
    for dependent in dependents.dependencies {
        loop_cnt += 1;
        let dependent_name = dependent.crate_version.crate_name;

        println!(
            "Find [{}'s] dependent [{}]",
            dependent.dependency.crate_id, dependent_name
        );

        let dependent_crate_reponse = client.get_crate(&dependent_name)?;
        let dependent_crate = dependent_crate_reponse.crate_data;
        let dependent_repository_addr = match dependent_crate.repository {
            Some(repo) => {
                repos.push(repo.clone());
                repo
            }
            None => {
                continue;
            }
        };

        if loop_cnt == 2 {
            break;
        }
        //println!("  {}, {}", dependent_name, dependent_repository_addr);
    }
    Ok(repos)
}

fn clone_repository(repo_addr: &str, dependents_dir_path: PathBuf, num: u32) {
    let target_dir = dependents_dir_path.join("dependent".to_string() + &num.to_string());
    let target_dir = target_dir.to_str().unwrap();
    let cmd = "git clone https://github.com/getsentry/sentry-rust.git ".to_string() + target_dir;
    println!("{}", cmd);
    cmd!(&cmd).run().unwrap();
}

fn clone_dependents_repositories() {
    let mut crate_root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dependents_dir_path = crate_root_path.join("dependents");
    let repos = get_crate_dependents_repositorys("url").unwrap();

    let mut dep_cnt = 0;
    for repo in repos {
        println!("{}", repo);

        clone_repository(&repo, dependents_dir_path.clone(), dep_cnt);
        dep_cnt += 1;
    }
}

fn main() {
    clone_dependents_repositories();
}
