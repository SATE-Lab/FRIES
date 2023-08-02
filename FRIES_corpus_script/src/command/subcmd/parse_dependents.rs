use core::panic;
use std::{
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
    usize,
};

use crate::command::RunCommand;
use crates_io_api::SyncClient;
use run_shell::cmd;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct ParseDependents {
    /// 名字
    #[structopt(long = "name")]
    pub lib_name: String,

    /// 最大数目
    #[structopt(short = "n", long = "num", default_value = "500")]
    pub max_num: usize,

    /// 是否只要解析
    #[structopt(long = "clone")]
    pub clone: bool,

    /// 是否只要解析
    #[structopt(long = "parse")]
    pub parse: bool,

    /// 去重
    #[structopt(long = "dedup")]
    pub dedup: bool,
}

impl RunCommand for ParseDependents {
    fn run_command(&mut self) {
        if self.clone {
            info!("ParseDependents");
            clone_dependent_repositories(&self.lib_name, self.max_num);
            return;
        }

        if self.parse {
            parse(&self.lib_name, self.max_num);
            return;
        }

        if self.dedup {
            dedup_dep_or_order_info(&self.lib_name, "depinfo");
            dedup_dep_or_order_info(&self.lib_name, "orderinfo");
            dedup_funcinfo(&self.lib_name);
            dedup_seq(&self.lib_name);
            return;
        }

        //如果什么参数都没，就执行all
        clone_dependent_repositories(&self.lib_name, self.max_num);
        parse(&self.lib_name, 10000); //如果是直接执行完的话，就不限制
        dedup_dep_or_order_info(&self.lib_name, "depinfo");
        dedup_dep_or_order_info(&self.lib_name, "orderinfo");
        dedup_funcinfo(&self.lib_name);
        dedup_seq(&self.lib_name);
    }
}

/// 对每个dependent仓库都执行git clone，下载到特定的文件夹
fn clone_dependent_repositories(lib_name: &str, max_dependent_num: usize) {
    // Body
    println!("{}", max_dependent_num);
    let dependents_dir_path = get_dependents_dir_path(lib_name);
    let repo_addrs = get_crate_dependent_repositories(lib_name, max_dependent_num).unwrap();

    let mut num = 0;
    for repo_addr in &repo_addrs {
        clone_repository(lib_name, repo_addr, dependents_dir_path.clone(), num);
        num += 1;
    }

    let cmd =
        "ls".to_string() + dependents_dir_path.to_str().unwrap() + "-l | wc -l >> line_count.txt";
    cmd!(&cmd).run().unwrap();

    /// 为某个crate从crates.io上爬下dependents
    /// 参数：1.名字 2.依赖最大数目
    /// 返回值：github网址组成的向量
    fn get_crate_dependent_repositories(
        name: &str,
        max_dependent_num: usize,
    ) -> Option<Vec<String>> {
        println!(
            "-----------------Begin to clone dependents of the crate [{}]---------------",
            name
        );
        println!("I will get at most [{}] dependents.", max_dependent_num);

        let mut repos = Vec::new();

        // Instantiate the client.
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(200),
        )
        .unwrap();

        let mut page_idx: u64 = 0; //页号
        let mut dependents_num = 0; //已获取的dependents的数量
        loop {
            //page_idx从1开始，每轮递增，一页一页去拷贝
            page_idx += 1;

            //获得第page_idx页的dependents
            let dependents = match client.crate_reverse_dependencies_page(name, page_idx) {
                Ok(dep) => {
                    println!(
                        "\x1b[92mPulling down the lib {}'s reverse dependencies page {}\x1b[0m",
                        name, page_idx
                    );

                    dep
                }
                Err(_) => {
                    println!(
                        "\x1b[91mPage {} doesn't exist\nEnding cloning\x1b[0m",
                        page_idx
                    );
                    //在dependents页面被爬光后，直接返回
                    return Some(repos);
                }
            };

            if dependents.dependencies.len() == 0 {
                //这一页是空的，所以直接break
                println!("找不到其他依赖");
                return Some(repos);
            }

            //对每一个dependent进行遍历
            for dependent in dependents.dependencies {
                let dependent_name = dependent.crate_version.crate_name;

                let dependent_crate_reponse = client.get_crate(&dependent_name).unwrap();
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
                println!(
                    "Find [{}'s] dependent{} [{}], The repository is [{}].",
                    dependent.dependency.crate_id,
                    dependents_num,
                    dependent_name,
                    dependent_repository_addr
                );

                //每次遍历都会加1
                dependents_num += 1;
                // 在dependents达到数量时直接返回
                if dependents_num >= max_dependent_num {
                    return Some(repos);
                }
            }
        }
    }

    /// 执行git clone https://github.com/xxx .../dependents/dependentxxx
    fn clone_repository(lib_name: &str, repo_addr: &str, dependents_dir_path: PathBuf, num: u32) {
        //创建目标文件夹，比如/Users/yxz/dependency/dep/dependents/dependent0
        let target_dir =
            dependents_dir_path.join(lib_name.to_string() + "_dependent" + &num.to_string());
        let target_dir = target_dir.to_str().unwrap();

        //如果文件夹存在，先删除
        if Path::new(target_dir).exists() {
            //remove_dir_all(target_dir).unwrap();
            println!(
                "\x1b[92m{}'s dependent {} has exists, we don't have to clone it.\x1b[0m",
                lib_name, num
            );
            return;
        }

        let mut repo_addr = repo_addr.to_string();
        let repo_addr = match repo_addr.find("//") {
            Some(insert_pos) => {
                //插入账号密码；比如 https://:@github.com/wasmcloud/weld
                repo_addr.insert_str(insert_pos + 2, ":@");
                repo_addr.as_str()
            }
            None => repo_addr.as_str(),
        };

        //执行git clone
        let cmd = "timeout 30s git clone ".to_string() + repo_addr + " " + target_dir;
        println!("\x1b[93m{}\x1b[0m", cmd);
        match cmd!(&cmd).run() {
            Ok(_) => {
                println!("\x1b[92mClone {} successfully\x1b[0m", repo_addr);
            }
            Err(_) => {
                println!("\x1b[91mFailed to clone {}\x1b[0m", repo_addr);
            }
        }
    }
}

fn get_valid_dependent_crates(lib_name: &str) -> Vec<PathBuf> {
    println!("We will get valid dependent crates");

    let mut vec_pathbuf = vec![];

    let dependents_dir_path = get_dependents_dir_path(lib_name);
    //对于clone下来的每一个dependent，检查
    for entry in fs::read_dir(dependents_dir_path.clone()).unwrap() {
        let entry_path = entry.unwrap().path();
        let entry_path_str = entry_path.to_str().unwrap();
        println!(
            "Find entry in directory dependents : {}!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
            entry_path_str
        );

        let split = entry_path_str.split('/');
        let entry_name_str = split.last().unwrap();
        let mut split = entry_name_str.split("_dep"); //找到待测package的名字
        let dependency_lib_name = split.next().unwrap();

        //如果文件夹前缀和待测的library不匹配，就跳过；否则，继续处理，对manifest进行识别
        if dependency_lib_name != lib_name
            || entry_path.to_str().unwrap().to_string().contains("61")
        {
            continue;
        }
        println!("entry_path {}", entry_path.display());
        let mut v = parse_dependent_manifest(lib_name, entry_path);
        vec_pathbuf.append(&mut v);
    }
    println!("Finish finding valid dependent crates!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    return vec_pathbuf;

    /// 辅助函数
    /// =====================================================================================
    fn parse_dependent_manifest(lib_name: &str, dependent_path: PathBuf) -> Vec<PathBuf> {
        parse_package_manifest(lib_name, dependent_path)
    }

    fn parse_package_manifest(lib_name: &str, dependent_path: PathBuf) -> Vec<PathBuf> {
        if dependent_path.to_str().unwrap().to_string().contains("184")
            || dependent_path.to_str().unwrap().to_string().contains("191")
        {
            return vec![];
        }
        let mut crates = vec![];

        let toml_path = dependent_path.join("Cargo.toml");
        let manifest = match cargo_toml::Manifest::from_path(toml_path) {
            Ok(x) => x,
            Err(_) => return vec![],
        };
        match manifest.package {
            Some(package) => {
                println!("This dependent has a root package. Start to parse package.workspace.");
                match package.workspace {
                    Some(_) => {
                        // 这种情况说明package存在于某个根workspace
                        if parse_if_lib_exist_in_package(lib_name, &dependent_path) {
                            crates.push(dependent_path.clone());
                        }
                    }
                    None => match manifest.workspace {
                        Some(workspace) => {
                            // 这种情况说明package和workspace都存在
                            // root package
                            if parse_if_lib_exist_in_package(lib_name, &dependent_path) {
                                crates.push(dependent_path.clone());
                            }

                            // 解析workspace
                            for member in workspace.members {
                                if member == "." || member == "./" {
                                    continue;
                                }
                                let member_path = dependent_path.join(member.clone());
                                println!(
                                    "parse workspace member {}, path = {}",
                                    member,
                                    member_path.display()
                                );
                                let mut paths = parse_package_manifest(lib_name, member_path);
                                crates.append(&mut paths);
                            }
                        }
                        None => {
                            // 只有package，无workspace
                            if parse_if_lib_exist_in_package(lib_name, &dependent_path) {
                                crates.push(dependent_path.clone());
                            }
                        }
                    },
                }
            }
            None => {
                println!("This dependent has a virtual manifest. Start to parse every workspace.");
                match manifest.workspace {
                    Some(workspace) => {
                        for member in workspace.members {
                            let member_path = dependent_path.join(member.clone());
                            println!(
                                "parse workspace member {}, path = {}",
                                member,
                                member_path.display()
                            );
                            let member_path = dependent_path.join(member);
                            let mut paths = parse_package_manifest(lib_name, member_path);
                            crates.append(&mut paths);
                        }
                    }
                    None => {
                        panic!("Illegal manifest.");
                    }
                }
            }
        }
        crates
    }

    fn parse_if_lib_exist_in_package(lib_name: &str, package_path: &PathBuf) -> bool {
        let toml_path = package_path.join("Cargo.toml");
        let manifest = cargo_toml::Manifest::from_path(toml_path.clone()).unwrap();

        let package = manifest.package.unwrap();
        let package_name = package.name;
        println!(
            "\x1b[96mStart to parse package [{}], we will find whether lib [{}] is one of its dependencies\x1b[0m\nToml path is {}",
            package_name, lib_name, toml_path.display()
        );

        let dependencies = manifest.dependencies;
        for (dependency_name, _) in dependencies {
            if dependency_name == lib_name {
                println!("\x1b[92mFind dependency [{}], hit!\x1b[0m", dependency_name);
                return true;
            } else {
                /*println!(
                    "Find dependency [{}], but it doesn't match lib [{}]",
                    dependency_name, lib_name
                );*/
            }
        }
        println!(
            "\x1b[96mEnd parsing package [{}], we will find whether lib [{}] is one of its dependencies\x1b[0m",
            package_name, lib_name
        );
        false
    }
}

/// 对某个路径下的crate执行cargo +fuzz doc --target-dir=tested
fn parse_each_valid_crate(dir_path: PathBuf) -> Output {
    println!("parse path {}: cargo +fuzz doc", dir_path.display());
    let args = vec!["60s", "cargo", "+fuzz", "doc"];
    Command::new("timeout")
        .args(args)
        .current_dir(dir_path)
        .output()
        .unwrap()
}

fn parse(lib_name: &str, max_crates_num: usize) {
    use std::time::Instant;
    let start = Instant::now();

    let crate_dir_paths = get_valid_dependent_crates(lib_name);
    for dir_path in &crate_dir_paths {
        println!("candidate path: {}", dir_path.display());
    }

    //开放不同线程去做这个事
    let mut threads = vec![];

    let total_thread_num = std::cmp::min(crate_dir_paths.len(), max_crates_num);
    let running_cnt = Arc::new(AtomicUsize::new(0));
    let finish_cnt = Arc::new(AtomicUsize::new(0));
    let succ_cnt = Arc::new(AtomicUsize::new(0));

    let mut parsed_num = 0;
    for (idx, dir_path) in crate_dir_paths.clone().into_iter().enumerate() {
        parsed_num += 1;
        if parsed_num >= max_crates_num {
            break;
        }

        let running_cnt_copy = running_cnt.clone();
        let finish_cnt_copy = finish_cnt.clone();
        let succ_cnt_copy = succ_cnt.clone();
        let handle = thread::spawn(move || {
            running_cnt_copy.fetch_add(1, Ordering::SeqCst);
            println!(
                "thread{} start, there are totally {} threads.",
                idx, total_thread_num
            );
            let exit_status = parse_each_valid_crate(dir_path.clone());

            running_cnt_copy.fetch_sub(1, Ordering::SeqCst);
            finish_cnt_copy.fetch_add(1, Ordering::SeqCst);

            if exit_status.status.success() {
                succ_cnt_copy.fetch_add(1, Ordering::SeqCst);
                let duration = start.elapsed();
                let finish_num = finish_cnt_copy.load(Ordering::SeqCst);
                println!(
                    "从开始解析到现在，程序执行时间：{:?}, 线程[{}]结束了，一共[{}]个线程，结束了[{}]个线程，有[{}]个线程正常结束，还剩下[{}]个线程",
                    duration,
                    idx,
                    total_thread_num,
                    finish_num,
                    succ_cnt_copy.load(Ordering::SeqCst),
                    max_crates_num - finish_num,
                );
            } else {
                println!(
                    "{:?} 线程没有正常退出，说明需要自定义的编译过程",
                    dir_path.clone()
                );
            }
        });
        threads.push(handle);

        //保证运行的线程数不超过10
        while running_cnt.load(Ordering::SeqCst) > 3 {
            println!(
                "Num of threads is {}, large than 10. sleep",
                running_cnt.load(Ordering::SeqCst)
            );
            sleep(Duration::new(1, 0));
        }
        println!(
            "There are {} running threads, {} dead threads",
            running_cnt.load(Ordering::SeqCst),
            finish_cnt.load(Ordering::SeqCst)
        );
        sleep(Duration::new(0, 100000000));
    }
    //确保所有的线程都已经退出
    for handle in threads {
        let _ = handle.join();
    }

    let duration = start.elapsed();
    let finish_num = finish_cnt.load(Ordering::SeqCst);
    println!(
        "所有线程都结束了，主线程即将退出。从开始解析到结束，程序执行时间：{:?}，一共[{}]个线程，结束了[{}]个线程，还剩下[{}]个线程",
        duration,
        total_thread_num,
        finish_num,
        max_crates_num - finish_num,
    );
    return;
}

/// 获得dependents对应的文件夹，一般是 Project_HOME/dependents/
fn get_dependents_dir_path(lib: &str) -> PathBuf {
    //let crate_root_path = current_dir().unwrap();
    //PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dependents_dir_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib)
        .join("dependents");
    dependents_dir_path
}

fn dedup_dep_or_order_info(lib_name: &str, info_file_name: &str) {
    let depinfo_file_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join(info_file_name);

    let file = File::open(depinfo_file_path.clone()).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    let mut total_cnt: usize = 0;

    println!("read each line");

    let binding = lib_name.to_string().replace("-", "_");
    let prefix = binding.as_str();
    for line in reader.lines() {
        let content = line.unwrap();
        //println!("{}", content);

        //如果不包含libname说明不对，跳过
        if !content.contains(prefix) {
            continue;
        }

        let parts: Vec<&str> = content.split(":   ").collect();
        if parts.len() <= 1 {
            continue;
        }

        let pair_and_num_str = parts[1].to_string();

        let parts: Vec<&str> = pair_and_num_str.split("   ").collect();
        //println!("{:?}", parts);

        let pair = parts[0].to_string() + "   " + parts[1];

        if !pair.starts_with(&(binding.clone() + "::")) {
            continue;
        }
        //println!("{:?}", pair);
        let num: usize = parts[2].parse().unwrap();
        total_cnt += num;

        if map.get(&pair).is_some() {
            //如果里面有这个序列，就加1
            map.insert(pair.clone(), map.get(&pair).unwrap() + num);
        } else {
            //如果没有，就设置为1
            map.insert(pair.clone(), num);
        }
    }

    let file_ans_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join(info_file_name.to_string() + ".txt");
    let file = File::create(file_ans_path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut vec: Vec<(&String, &usize)> = map.iter().collect(); // 转换为元组的向量
    vec.sort_by(|a, b| b.1.cmp(a.1)); // 按照值进行排序

    println!(
        "Total {} in {}, {} after dedup",
        total_cnt,
        info_file_name,
        vec.len()
    );
    for (idx, (pair, cnt)) in vec.iter().enumerate() {
        let red_or_line = " | ";
        /*let out_line = &format!(
            "\x1b[92midx:\x1b[0m {} {} \x1b[92mcnt:\x1b[0m {} {} {}",
            idx, red_or_line, cnt, red_or_line, pair,
        );*/

        let out_line = &format!("{} {} {} {} {}", idx, red_or_line, cnt, red_or_line, pair,);
        //println!("{}", out_line);
        writer
            .write((out_line.to_owned() + "\n").as_bytes())
            .unwrap();
    }
}

fn dedup_funcinfo(lib_name: &str) {
    let funcinfo_file_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join("funcinfo");

    let file = File::open(funcinfo_file_path.clone()).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    println!("read each line");

    let binding = lib_name.to_string().replace("-", "_");
    let prefix = binding.as_str();
    for line in reader.lines() {
        let content = line.unwrap();
        //println!("{}", content);

        //如果不包含libname说明不对，跳过
        if !content.contains(prefix) {
            continue;
        }

        let parts: Vec<&str> = content.split(":   ").collect();
        if parts.len() <= 2 {
            continue;
        }

        let func_and_num_str = parts[2].to_string();
        //println!("!!!!!!!!!!{}", func_and_num_str);
        let parts: Vec<&str> = func_and_num_str.split("   ").collect();

        let func = parts[0].to_string();
        if !func.starts_with(&(binding.clone() + "::")) {
            continue;
        }
        let num: usize = parts[1].parse().unwrap();

        if map.get(&func).is_some() {
            //如果里面有这个序列，就加1
            map.insert(func.clone(), map.get(&func).unwrap() + num);
        } else {
            //如果没有，就设置为1
            map.insert(func.clone(), num);
        }
    }

    let file_ans_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join("funcinfo.txt");
    let file = File::create(file_ans_path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut vec: Vec<(&String, &usize)> = map.iter().collect(); // 转换为元组的向量
    vec.sort_by(|a, b| b.1.cmp(a.1)); // 按照值进行排序

    for (idx, (func, cnt)) in vec.iter().enumerate() {
        let red_or_line = " | ";
        /*let out_line = &format!(
            "\x1b[92midx:\x1b[0m {} {} \x1b[92mcnt:\x1b[0m {} {} {}",
            idx, red_or_line, cnt, red_or_line, func,
        );*/
        let out_line = &format!("{} {} {} {} {}", idx, red_or_line, cnt, red_or_line, func,);
        //println!("{}", out_line);
        writer
            .write((out_line.to_owned() + "\n").as_bytes())
            .unwrap();
    }
}
fn dedup_seq(lib_name: &str) {
    let seq_file_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join("seq");

    let file = File::open(seq_file_path.clone()).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    println!("read each line");

    for line in reader.lines() {
        let content = line.unwrap();
        //println!("{}", content);

        //如果不包含libname说明不对，跳过
        if !content.contains(&lib_name.replace("-", "_")) {
            continue;
        }

        let parts: Vec<&str> = content.split(": ").collect();
        if parts.len() <= 1 {
            continue;
        }

        let seq = parts[1].to_string();
        //println!("{}", seq.clone());

        if map.get(&seq).is_some() {
            //如果里面有这个序列，就加1
            map.insert(seq.clone(), map.get(&seq).unwrap() + 1);
        } else {
            //如果没有，就设置为1
            map.insert(seq.clone(), 1);
        }
    }

    let seq_file_ans_path = PathBuf::from(EXPERIMENT_ROOT_PATH)
        .join(lib_name)
        .join("seq-dedup.ans");
    let file = File::create(seq_file_ans_path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut vec: Vec<(&String, &i32)> = map.iter().collect(); // 转换为元组的向量
    vec.sort_by(|a, b| b.1.cmp(a.1)); // 按照值进行排序

    for (idx, (seq, cnt)) in vec.iter().enumerate() {
        let parts: Vec<&str> = seq.split(" ").collect();

        let red_or_line = " | ";
        let out_line = &format!(
            "\x1b[92midx:\x1b[0m {} {} \x1b[92mcnt:\x1b[0m {} {} \x1b[92mseq_len:\x1b[0m {} {} {}",
            idx,
            red_or_line,
            cnt,
            red_or_line,
            parts.len() - 1,
            red_or_line,
            seq,
        );
        //println!("{}", out_line);
        writer
            .write((out_line.to_owned() + "\n").as_bytes())
            .unwrap();
    }
}

const EXPERIMENT_ROOT_PATH: &'static str = "/home/yxz/workspace/fuzz/experiment_root/";
