extern crate rayon;
extern crate crypto;
extern crate clap;
extern crate chrono;

mod argument_holder;

use argument_holder::ArgumentHolder as ArgHolder;
use clap::{Arg, App, ArgGroup};

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

fn main() {
    let arg_holder = setup_arguments();
    println!("Starting diff of '{}' and '{}' ({})", arg_holder.directory_a, arg_holder.directory_b, arg_holder.hash_name);
    println!("Starting at {}", chrono::Local::now());

    // TODO
    // scan_directory
    // reconcile
    // writeResults

    println!("Finished at {}", chrono::Local::now());
}
