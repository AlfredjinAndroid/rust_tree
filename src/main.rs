use clap::{ArgAction, Parser};
use walkdir::{DirEntry, WalkDir};

/// 本地文件树
#[derive(Parser, Debug)]
#[command(author = "Alfred", version = "0.1.0", about, long_about = None, disable_help_flag = true, disable_version_flag = true)]
struct Args {
    /// 文件树深度
    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    /// 展示隐藏文件
    #[clap(long, short)]
    a: bool,

    /// 显示目录
    #[arg(long, short = 'D')]
    dir: bool,

    /// 显示文件
    #[arg(long, short = 'F')]
    file: bool,

    /// 展示完全路径
    #[arg(long, short)]
    path: bool,

    /// 显示文件大小
    #[arg(long, short)]
    size: bool,

    /// 帮助
    #[arg(long, short, action = ArgAction::Help)]
    help: Option<bool>,

    /// 当前版本
    #[arg(long, short, action = ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    match Args::try_parse() {
        Ok(args) => {
            parse_command(args);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn parse_command(args: Args) {
    let max_depth = args.depth;
    let show_hidden = args.a;
    let show_dir = args.dir;
    let show_file = args.file;
    let show_path = args.path;
    let show_size = args.size;
    println!(".");
    WalkDir::new(".")
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e, show_hidden) && is_file(e, show_dir, show_file))
        .filter_map(|e| e.ok())
        .for_each(|f| display_file(f, show_path, show_size))
}

fn is_not_hidden(entry: &DirEntry, show_hidden: bool) -> bool {
    if show_hidden {
        return true;
    }
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry, show_dir: bool, show_file: bool) -> bool {
    if !show_dir && !show_file {
        return true;
    }
    match entry.metadata() {
        Ok(f) => show_file && f.is_file() || show_dir && f.is_dir(),
        Err(_) => false,
    }
}

fn display_file(entry: DirEntry, show_path: bool, show_size: bool) {
    let depth = entry.depth();
    let s = "|--";
    let fix = "   |--".repeat(depth - 1);
    if show_path {
        let p_name = entry.path().display();
        println!("{}{}{}", s, fix, p_name);
        return;
    }
    let metadata = entry.metadata();
    let (is_file, len) = match metadata {
        Ok(m) => {
            (m.is_file(), m.len())
        }
        Err(_) => { (false, 0) }
    };
    if is_file && show_size {
        let f_name = entry.file_name().to_str().unwrap_or_else(|| "未知文件");
        println!("{}{}{} [{}KB]", s, fix, f_name, len / 1024);
        return;
    }
    let f_name = entry.file_name().to_str().unwrap_or_else(|| "未知文件");
    println!("{}{}{}", s, fix, f_name);
}