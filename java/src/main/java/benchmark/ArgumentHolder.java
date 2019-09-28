package src.main.java.benchmark;

import java.util.HashSet;
import java.nio.file.Path;
import java.nio.file.FileSystem;
import java.nio.file.FileSystems;


class ArgumentHolder{
    public Path DirectoryA;
    public Path DirectoryB;
    public Boolean IgnoreUnchanged;
    public String ChecksumName;

    public ArgumentHolder(){
        this.DirectoryA = null;
        this.DirectoryB = null;
        IgnoreUnchanged = false;
        ChecksumName = "MD5";
    }

    public Boolean parse(String[] args){
        if(args.length < 2){
            // Not enough args
            return false;
        }

        // Parse the directories
        FileSystem fs = FileSystems.getDefault();
        this.DirectoryA = fs.getPath(args[0]);
        this.DirectoryB = fs.getPath(args[1]);

        Boolean hasHashOption = false;
        HashSet<String> hashes = new HashSet<String>();
        hashes.add("--md5");
        // Not supported yet
        //hashes.add("--crc32");
        //hashes.add("--adler32");
        hashes.add("--sha1");
        hashes.add("--sha256");

        // Go over additional flags
        for (int i = 2; i < args.length; ++i){
            String arg = args[i].toLowerCase();

            // Check for ignore unchanged files flag
            if(arg == "--ignore-unchanged" || arg == "-u"){
                this.IgnoreUnchanged = true;
            }

            if(hashes.contains(arg)) {
                if(hasHashOption){
                    // can't specify more than one hash at a time
                    return false;
                }

                this.ChecksumName = arg.substring(2).toUpperCase();
                hasHashOption = true;
            }
        }

        return true;
    }

}