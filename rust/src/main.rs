extern crate rayon;
extern crate crypto;
extern crate clap;
extern crate chrono;

mod argument_holder;
mod file_result;
mod results;
mod util;

use argument_holder::ArgumentHolder as ArgHolder;
use file_result::*;
use results::*;
use clap::{Arg, App, ArgGroup};
use crypto::digest::Digest;

use std::io::{Read, Write};
use std::iter::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::collections::{HashMap, HashSet};
use std::thread;
use std::path::{Path,PathBuf};

fn setup_arguments() -> ArgHolder {
    let args = App::new("Rust Language Benchmark")
                .version("1.0")
                .about("Rust implementation of the Language Benchmark Trial")
                .arg(Arg::with_name("DirectoryA")
                    .help("Directory to parse")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("DirectoryB")
                    .help("Directory to parse")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("ignore-unchanged")
                    .short("u")
                    .help("Ignore unchagned files in the final output"))
                .args_from_usage(
                    "--md5 'MD5 hash (default)'
                    --sha1 'SHA1 hash'
                    --sha256 'SHA256 hash'
                    --adler32 'Adler 32-bit checksum'
                    --crc32 'Cyclic Redundancy Check 32-bit checksum'")
                .group(ArgGroup::with_name("checksum")
                    .args(&["md5", "sha1", "sha256","adler32", "crc32"])
                    .multiple(false))
                .get_matches();

    return ArgHolder::from_args(args);
}

fn hash_file(filepath: &PathBuf, mut checksum: Box<Digest>) -> String {
    let mut file = File::open(filepath).unwrap();
    const BLOCK_SIZE: usize = 64 * 1024;// 4KB blocks
    let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    
    // Read the file by chunks until there's no more to read
    while let Ok(read_amount) = file.read(&mut buffer) {
        if read_amount <= 0{
            break;
        }

        checksum.input(&buffer[0..read_amount]);
    }

    return checksum.result_str();
}

fn scan_directory(directory_path: &Path, checksum_generator: impl Fn() -> Box<Digest>) -> ScanDirectoryResult{
    let mut file_results = HashMap::new();

    // Scope this block so Rust nows we need to retake ownership of file_results
    {
        let cut_index = directory_path.to_str().unwrap().len()+1;
        let mut visit_file = |some_entry : &fs::DirEntry| {
            let file_info = some_entry.metadata().unwrap();
            let canonical_name : String = some_entry.path().to_str().unwrap().chars().skip(cut_index).collect();

            file_results.insert(
                canonical_name.clone(), 
                FileResult{
                    filepath: canonical_name,
                    hash: hash_file(&some_entry.path(), checksum_generator()),
                    size: file_info.len(),
                    time_modified: match file_info.modified(){
                        Ok(time) => time,
                        Err(e) => {
                            println!("Error reading mod_time for file {}", some_entry.path().to_str().unwrap());
                            panic!(e)
                        }
                    }
            });
        };

        match util::visit_dirs(Path::new(&directory_path), &mut visit_file){
            Ok(()) => (),
            Err(e) => panic!("{}",e),
        };
    }

    return file_results;
}

fn reconcile(results_a: ScanDirectoryResult, results_b : ScanDirectoryResult) -> ReconcileResult{
    let paths_a : HashSet<String> = HashSet::from_iter(results_a.keys().cloned());
    let paths_b : HashSet<String> = HashSet::from_iter(results_b.keys().cloned());

    let suspected_conflicts : HashSet<String> = paths_a.intersection(&paths_b).map(|x| x.clone()).collect();
    let suspected_conflicts_iter = suspected_conflicts.iter();

    let unchanged_paths : HashSet<String> = HashSet::from_iter(
        suspected_conflicts_iter
            .filter(|entry| results_a.get(*entry).unwrap() == results_b.get(*entry).unwrap()).cloned()
    );
    let conflicts : HashSet<String> = suspected_conflicts.difference(&unchanged_paths).map(|path| path.clone()).collect();

    // First do dir_a
    let mut patch_info_a = PatchResult::new();
    patch_info_a.insert(
        ReconcileOperation::Add, 
        paths_b.difference(&paths_a).map(|path| results_b.get(path).unwrap().clone()).collect()
    );
    patch_info_a.insert(
        ReconcileOperation::Unchanged, 
        unchanged_paths.clone().into_iter().map(|path| results_a.get(&path).unwrap().clone()).collect()
    );
    patch_info_a.insert(
        ReconcileOperation::Conflict, 
        conflicts.clone().into_iter().map(|path| results_a.get(&path).unwrap().clone()).collect()
    );

    // Then do dir_b
    let mut patch_info_b = PatchResult::new();
    patch_info_b.insert(
        ReconcileOperation::Add, 
        paths_a.difference(&paths_b).map(|path| results_a.get(path).unwrap().clone()).collect()
    );
    patch_info_b.insert(
        ReconcileOperation::Unchanged, 
        unchanged_paths.clone().into_iter().map(|path| results_b.get(&path).unwrap().clone()).collect()
    );
    patch_info_b.insert(
        ReconcileOperation::Conflict, 
        conflicts.clone().into_iter().map(|path| results_b.get(&path).unwrap().clone()).collect()
    );

    // Return the tuple corresponding to a Reconcile Result
    return (patch_info_a, patch_info_b);
}

fn write_results(changes: ReconcileResult, args: ArgHolder){
    let out_file_name = "reference.patch";
    fs::remove_file(out_file_name).unwrap();
    let mut out_file = OpenOptions::new().write(true).create_new(true).open(out_file_name).unwrap();

    // All the write methods have an "unwrap" at the end to check the Result of the operation
    // If any of the writes fails, the program panics and exits

    out_file.write_fmt(format_args!("# Results for {}\n", chrono::Local::now())).unwrap();
    out_file.write_fmt(format_args!("# Reconciled '{0}' '{1}'\n", args.directory_a.to_str().unwrap(), args.directory_b.to_str().unwrap())).unwrap();
    write_patch_results(args.directory_a, changes.0, &out_file, args.ignore_unchanged);
    out_file.write_fmt(format_args!("\n")).unwrap();
    write_patch_results(args.directory_b, changes.1, &out_file, args.ignore_unchanged);
    out_file.write_fmt(format_args!("\n")).unwrap();
}

fn write_patch_results(directory: PathBuf, patch : PatchResult, mut out_file : &File, ignore_unchanged: bool){
    out_file.write_fmt(format_args!("{}\n",directory.to_str().unwrap())).unwrap();
    
    let mut lines : Vec<Line> = Vec::new();
    // Linearize the result
    for action in patch{
        let operation = action.0;
        if ignore_unchanged && operation == ReconcileOperation::Unchanged{
            continue;
        }

        for path in action.1{
            lines.push((operation, path));
        }
    }

    // Sort and append to out_file
    lines.sort_unstable_by(|x, y| x.1.filepath.cmp(&y.1.filepath));
    for line in lines{
        out_file.write_fmt(format_args!("{} {}\n", line.0, line.1)).unwrap();
    }
}

fn main() {
    let arg_holder = setup_arguments();
    let dir_a = arg_holder.directory_a.clone();
    let dir_b = arg_holder.directory_b.clone();
    
    println!("Starting diff of '{}' and '{}' ({})", arg_holder.directory_a.to_str().unwrap(), arg_holder.directory_b.to_str().unwrap(), arg_holder.hash_name);
    println!("Starting at {}", chrono::Local::now());

    // Scope this block, since we'll take ownership of arg_holder again
    let patch_result : ReconcileResult;
    {
        let checksum_generator_a = arg_holder.get_checksum_generator();
        let checksum_generator_b = arg_holder.get_checksum_generator();

        let thread_a = move ||{ return scan_directory(dir_a.as_path(), checksum_generator_a); };
        let thread_b = move ||{ return scan_directory(dir_b.as_path(), checksum_generator_b); };

        let scan_a = thread::spawn(thread_a);
        let scan_b = thread::spawn(thread_b);
        patch_result = reconcile(scan_a.join().unwrap(), scan_b.join().unwrap());
    }
    
    // Write the results
    write_results(patch_result, arg_holder);
    println!("Finished at {}", chrono::Local::now());
}
