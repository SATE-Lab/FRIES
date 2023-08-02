/*
    This is core module to implement the core functionility of the fuzzing target
*/

#[macro_use]
mod template;
mod util;

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::command::subcmd::Init;

use self::util::{find_this_package, get_project_toml_value, Manifest};

const DEFAULT_FUZZ_DIR_NAME: &str = "fuzz";
const DEFAULT_FUZZ_TARGET_DIR_NAME: &str = "target";

pub struct FuzzProject {
    project_dir_path: PathBuf, //被测试项目的主目录
    fuzz_dir_path: PathBuf,    //测试目录，装有测试程序，默认是 {project_dir}/fuzz
    target_dir_path: PathBuf,  //测试目标所在的目录，默认是 {fuzz_dir}/target_dir
    targets: Vec<String>,      //测试目标，可以后续添加......
}

impl FuzzProject {
    /// 获取待测项目目录
    fn get_project_dir_path(&self) -> PathBuf {
        self.project_dir_path.clone()
    }

    /// 获取fuzz根目录
    fn get_fuzz_dir_path(&self) -> PathBuf {
        self.fuzz_dir_path.clone()
    }

    /// 获取target所在目录
    fn get_target_dir_path(&self) -> PathBuf {
        self.target_dir_path.clone()
    }

    /// 获取target名字
    fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    pub fn create_from_real() -> Result<Self> {
        let project_dir_path = find_this_package()?;
        let mut fuzz_dir_path_opt = None;
        //let mut toml_value;
        for entry in fs::read_dir(project_dir_path.to_owned())? {
            fuzz_dir_path_opt = Some(entry?.path());
            let toml_value = get_project_toml_value(fuzz_dir_path_opt.to_owned().unwrap())?;
            let is_fuzz = toml_value
                .as_table()
                .and_then(|t| t.get("package"))
                .and_then(toml::Value::as_table)
                .and_then(|t| t.get("metadata"))
                .and_then(toml::Value::as_table)
                .and_then(|t| t.get("rust-fuzzer"))
                .and_then(toml::Value::as_bool);
            if is_fuzz == Some(true) {
                break;
            }
        }
        let fuzz_dir_path = fuzz_dir_path_opt.unwrap();

        let target_dir_path = fuzz_dir_path.join(DEFAULT_FUZZ_TARGET_DIR_NAME);

        Ok(FuzzProject {
            project_dir_path,
            fuzz_dir_path,
            target_dir_path,
            targets: Vec::new(),
        })
    }

    /// 构造函数，创建一个FuzzProject实例
    /// 只能被init调用，不可以被外部调用
    pub fn create_from_init(init: &mut Init) -> Result<Self> {
        // 获取当前项目的主目录
        let project_dir_path = find_this_package()?;
        // 获取或生成fuzz test所在的目录
        let fuzz_dir_path = match init.fuzz_dir_wrapper.fuzz_dir.to_owned() {
            Some(dir) => dir, //如果用户提供了，则使用用户的目录
            None => project_dir_path.join(DEFAULT_FUZZ_DIR_NAME), //否则，使用默认目录
        };

        let target_dir_path = fuzz_dir_path.join(DEFAULT_FUZZ_TARGET_DIR_NAME);

        Ok(FuzzProject {
            project_dir_path,
            fuzz_dir_path,
            target_dir_path,
            targets: Vec::new(),
        })
    }

    /// 初始化fuzz文件夹，创建相应文件
    pub fn init(init: &mut Init) -> Result<Self> {
        //先通过create函数
        let fuzz_project = Self::create_from_init(init)?;
        let fuzz_dir_path = fuzz_project.get_fuzz_dir_path();
        let target_dir_path = fuzz_project.get_target_dir_path();

        //创建测试目录
        fs::create_dir(fuzz_dir_path.to_owned()).with_context(|| {
            format!(
                "failed to create fuzz directory {}",
                fuzz_dir_path.display()
            )
        })?;

        //创建target目录
        fs::create_dir(target_dir_path.to_owned()).with_context(|| {
            format!(
                "failed to create target directory {}",
                target_dir_path.display()
            )
        })?;

        //创建测试目录下的Cargo.toml文件
        let fuzz_cargo_toml_path = fuzz_dir_path.join("Cargo.toml");
        let mut fuzz_cargo_toml_file = fs::File::create(&fuzz_cargo_toml_path)
            .with_context(|| format!("failed to create {}", fuzz_cargo_toml_path.display()))?;
        //向测试目录下的Cargo.toml文件写入内容
        let fuzz_manifest = util::Manifest::create(fuzz_dir_path.clone())?;
        fuzz_cargo_toml_file
            .write_fmt(cargo_toml_template!(
                fuzz_manifest.crate_name,
                fuzz_manifest.edition
            ))
            .with_context(|| format!("failed to write {}", fuzz_cargo_toml_path.display()))?;

        //创建测试目录下的.gitignore文件
        let gitignore = fuzz_dir_path.join(".gitignore");
        let mut gitignore_file = fs::File::create(&gitignore)
            .with_context(|| format!("failed to create {}", gitignore.display()))?;
        //写入内容
        gitignore_file
            .write_fmt(gitignore_template!())
            .with_context(|| format!("failed to write to {}", gitignore.display()))?;

        //创建初始化目录下的target
        fuzz_project.create_file_for_target("target1")

        Ok(fuzz_project)
    }

    pub fn add_target(&mut self, target_name: &str) -> Result<()> {
        self.targets.push(target_name.to_string());
    
        Ok(())
    }

    pub fn create_file_for_target(&self, target: &str) -> Result<()> {
        let target_dir_path = self.get_target_dir_path();
        let target_file_path = target_dir_path.join(target);

        fs::create_dir_all(target_dir_path)
            .context("ensuring that `fuzz_targets` directory exists failed")?;

        let mut target_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target_file_path)
            .with_context(|| format!("could not create target file at {:?}", target_file_path))?;
        target_file.write_fmt(target_template!(self.get_project_manifest()?.edition))?;

        let mut cargo = fs::OpenOptions::new()
            .append(true)
            .open(self.get_fuzz_dir_path().join("Cargo.toml"))?;
        cargo.write_fmt(toml_bin_template!(target))?;
        Ok(())
    }

    fn get_project_manifest(&self) -> Result<Manifest> {
        util::Manifest::create(self.get_project_dir_path())
    }
    fn get_fuzz_manifest(&self) -> Result<Manifest> {
        util::Manifest::create(self.get_fuzz_dir_path())
    }
}
