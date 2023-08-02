/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Context, Result};
use std::io::{Read, Write};
use std::{env, fs, path::PathBuf};

pub fn is_fuzz_cargo_toml(value: &mut toml::Value) -> bool {
    false
}


pub fn get_project_toml_value(project_path)


/// 找到当前工作目录所在的项目的主目录
/// 该目录里应有Cargo.toml等配置文件
pub fn find_this_package() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    let mut data = Vec::new();
    //loop {
    let cargo_toml_path = dir.join("Cargo.toml");

    match fs::File::open(&cargo_toml_path) {
        Ok(mut f) => {
            data.clear();

            //读取Cargo.toml内部的值
            f.read_to_end(&mut data).with_context(|| {
                format!("failed to read {} toml file", cargo_toml_path.display())
            })?; //懒惰求值

            let value: toml::Value = toml::from_slice(&data).with_context(|| {
                format!("failed to decode the file at {}", cargo_toml_path.display())
            })?;
            return Ok(dir);
        }
        Err(_) => {
            bail!(
                "We can't find the package according to this working dir:{}",
                env::current_dir().unwrap().display()
            );
        }
    }
    /*
    //如果空了，那就break
    if !dir.pop() {
        break;
    }
    */
    //}
}
