#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>

#include "argument_holder.hpp"

ArgumentHolder::ArgumentHolder(){
    this->DirectoryA = "";
    this->DirectoryB = "";
    this->ChecksumName = "md5";
    this->ShouldIgnoreUnchanged = false;
}

bool ArgumentHolder::Parse(int argc, char** argv){
    if(argc < 3){
        // Not enough arguments
        return false;
    }

    std::vector<std::string> arguments;
    auto loadArguments = [&](const char* c_str){ arguments.push_back(std::string(c_str)); };
    
    // ignore the 1st argument since it's the program name
    std::for_each(&argv[1], &argv[argc], loadArguments);
    
    return this->Parse(arguments);
}

bool ArgumentHolder::Parse(std::vector<std::string>& args){
    fs::path pathA(args[0]), pathB(args[1]);
    
    // Paths coming in as "/a/b/../" or "/a/." will be converted to "/a"
    // HACK: Boost fs doesn't correctly normalize paths, it's not consistent
    //       So we attach a "file" and use parent_path to obtain a consistent form
    this->DirectoryA = (pathA/"x").normalize().parent_path();
    this->DirectoryB = (pathB/"x").normalize().parent_path();

    std::unordered_set<std::string> hashOptions = {"--md5", "--crc32", "--adler32", "--sha1", "--sha256"};
    bool hasHashOption = false;
    std::string checksumName;
    for(unsigned int i=2; i < args.size() ; i++){
        std::string& arg = args[i];

        // Check for ignore unchanged files flag
        if(arg == "--ignore-unchanged" || arg == "-u"){
            this->ShouldIgnoreUnchanged = true;
        }

        // Check for a hash selection
        auto entry = hashOptions.find(arg);
        if(entry != hashOptions.end()){
            if(hasHashOption){
                // error out
                return false;
            }

            checksumName = (*entry).substr(2);
            hasHashOption = true;
        }
    }

    if(checksumName.length() > 2){
        this->ChecksumName = checksumName;
    }

    return true;
}