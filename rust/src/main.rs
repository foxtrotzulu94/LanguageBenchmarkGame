extern crate rayon;
extern crate crypto;
extern crate clap;
extern crate chrono;

mod argument_holder;
mod scan_util;
mod util;

use argument_holder::ArgumentHolder as ArgHolder;
use scan_util::*;
use clap::{Arg, App, ArgGroup};
use crypto::digest::Digest;

use std::io::Read;
use std::fs;
use std::fs::File;
use std::collections::HashMap;
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
    const block_size: usize = 64 * 1024;// 4KB blocks
    let mut buffer: [u8; block_size] = [0; block_size];
    
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
            
            // let hash_val = ;
            // println!("{} - {}", canonical_name, hash_val);

            file_results.insert(
                canonical_name.clone(), 
                FileResult{
                    filepath: canonical_name,
                    hash: hash_file(&some_entry.path(), checksum_generator()),
                    size: file_info.len(),
                    time_modified: file_info.modified().unwrap()
            });
        };

        match util::visit_dirs(Path::new(&directory_path), &mut visit_file){
            Ok(()) => (),
            Err(e) => panic!("{}",e),
        };
    }

    return file_results;
}

fn reconcile(resultsA: ScanDirectoryResult, resultsB : ScanDirectoryResult) -> ReconcileResult{
    //TODO
    return (HashMap::new(), HashMap::new());
}

fn write_results(changes: ReconcileResult, args: ArgHolder){
    //TODO
}

fn write_patch_results(directory: String, ) -> Vec<String>{
    return vec![];
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
    
    write_results(patch_result, arg_holder);

    println!("Finished at {}", chrono::Local::now());
}
