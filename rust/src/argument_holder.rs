use clap::ArgMatches;

#[derive(Debug)]
pub struct ArgumentHolder{
    pub directory_a: String,
    pub directory_b: String,
    pub ignore_unchanged: bool,
    pub hash_name: String,
}

impl Default for ArgumentHolder {
    fn default () -> ArgumentHolder {
        ArgumentHolder{directory_a: "".to_string(),
            directory_b: "".to_string(),
            ignore_unchanged: false,
            hash_name: "md5".to_string()}
    }
}

impl ArgumentHolder{
    pub fn from_args (args: ArgMatches) -> ArgumentHolder {
        ArgumentHolder{
            directory_a: args.value_of("DirectoryA").unwrap().to_string(),
            directory_b: args.value_of("DirectoryB").unwrap().to_string(),
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
}