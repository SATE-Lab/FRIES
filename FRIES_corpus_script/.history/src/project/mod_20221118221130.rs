/*
    This is core module to implement the core functionility of the fuzzing target
*/
mod util;

use anyhow::{Context, Result};
use std::fmt::format;
use std::fs;
use std::path::PathBuf;

use crate::command::subcmd::Init;

use self::util::find_this_package;

const DEFAULT_FUZZ_DIR_NAME: &str = "fuzz";
const DEFAULT_FUZZ_TARGET_DIR_NAME: &str = "target";

pub struct FuzzProject {
    project_dir_path: PathBuf, //被测试项目的主目录
    fuzz_dir_path: PathBuf,    //测试目录，装有测试程序，默认是 {project_dir}/fuzz
    target_dir_path: PathBuf,  //测试目标所在的目录，默认是 {fuzz_dir}/target_dir
    targets: Vec<String>,      //测试目标，可以后续添加......
}

impl FuzzProject {
    pub fn project_dir(&self)->PathBuf{
        self.project_dir_path.clone()
    }



    /// 构造函数，创建一个FuzzProject实例
    /// 只能被init调用，不可以被外部调用
    #[allow(dead_code)]
    fn create(init: &mut Init) -> Result<Self> {
        // 获取当前项目的主目录
        let project_dir_path = find_this_package()?;
        // 获取或生成fuzz test所在的目录
        let fuzz_dir_path = match init.fuzz_dir_wrapper.fuzz_dir.to_owned() {
            Some(dir) => dir, //如果用户提供了，则使用用户的目录
            None => project_dir_path.join(DEFAULT_FUZZ_DIR_NAME), //否则，使用默认目录
        };

        let target_dir_path = match init.target_dir.to_owned() {
            Some(dir) => dir,
            None => fuzz_dir_path.join(DEFAULT_FUZZ_TARGET_DIR_NAME),
        };

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
        let mut fuzz_project = Self::create(init)?;

        fs::create_dir(fuzz_project.fuzz_dir_path.to_owned()).with_context(|| {
            format!(
                "failed to create fuzz directory {}",
                fuzz_project.fuzz_dir_path.display()
            )
        })?;

        fs::create_dir(fuzz_project.target_dir_path.to_owned()).with_context(|| {
            format!(
                "failed to create target directory {}",
                fuzz_project.target_dir_path.display()
            )
        })?;

        let fuzz_cargo_toml = fuzz_project.
        

        Ok(fuzz_project)
    }

    pub fn add_target(&mut self, target_name: &str) -> Result<()> {
        self.targets.push(target_name.to_string());
        Ok(())
    }
}
