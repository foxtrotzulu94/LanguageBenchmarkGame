use clap::ArgMatches;
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ArgumentHolder{
    pub directory_a: PathBuf,
    pub directory_b: PathBuf,
    pub ignore_unchanged: bool,
    pub hash_name: String,
}

impl Default for ArgumentHolder {
    fn default () -> ArgumentHolder {
        ArgumentHolder{
            directory_a: PathBuf::new(),
            directory_b: PathBuf::new(),
            ignore_unchanged: false,
            hash_name: "md5".to_string()
        }
    }
}

impl ArgumentHolder{
    pub fn from_args (args: ArgMatches) -> ArgumentHolder {
        ArgumentHolder{
            directory_a: Path::new(args.value_of("DirectoryA").unwrap()).canonicalize().unwrap(),
            directory_b: Path::new(args.value_of("DirectoryB").unwrap()).canonicalize().unwrap(),
            ignore_unchanged: match args.value_of("ignore-unchanged"){
                Some(_) => true,
                None => false,
            },
            hash_name:
                // TODO: somehow make this a macro
                if args.is_present("md5") {
                    "md5".to_string()
                }
                else if args.is_present("sha1") {
                    "sha1".to_string()
                }
                else if args.is_present("sha256") {
                    "sha256".to_string()
                }
                else if args.is_present("adler32") {
                    "adler32".to_string()
                }
                else if args.is_present("crc32") {
                    "crc32".to_string()
                }
                else{
                    "md5".to_string()
                }
        }
    }

    fn get_checksum(checksum_name : &str) -> Box<Digest>{
        return match checksum_name {
            "sha1" => Box::new(Sha1::new()),
            "sha256" => Box::new(Sha256::new()),
            "adler32" => panic!("adler32 not implemented"),
            "crc32" => panic!("crc32 not implemented"),
            "md5" | _ => Box::new(Md5::new())
        }
    }

    pub fn get_checksum_generator(&self) -> impl Fn() -> Box<Digest>{
        let checksum_name = self.hash_name.clone();
        return move || ArgumentHolder::get_checksum(checksum_name.as_str());
    }
}