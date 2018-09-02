#pragma once

#include <boost/filesystem.hpp>
#include <unordered_map>
#include <unordered_set>
#include <future>
#include <tuple>

#include "argument_holder.hpp"
#include "file_result.hpp"

enum class ReconcileOperation : char{
    ADD = '+',
    UNCHANGED = '=',
    CONFLICT = '!'
};

// Short-hand for worker results
typedef std::unordered_map<std::string, FileResultPtr> scan_result;
typedef std::unordered_map<ReconcileOperation, std::vector<FileResultPtr>> patch_result;
typedef std::shared_ptr<patch_result> patch_result_ptr;
typedef std::pair<patch_result_ptr, patch_result_ptr> reconcile_result;

// Short hand for intermediate/working data structures
typedef std::unordered_set<std::string> string_set;

namespace fs = boost::filesystem;

class Worker{
private:
    // An instance of the checksum function to use
    const checksum_ptr checksumInstance;

    // Result of the last reconcile operation if it was saved
    std::shared_ptr<reconcile_result> lastReconcile;

    // Internal implementation of Scan Directory
    scan_result scanDirectoryInternal(std::string path);

    // Hashes a given file
    std::string hashFile(std::string filepath);

    // Fills the set with the keys of the scan_result
    void populateSetWithKeys(std::unordered_set<std::string>& set, scan_result& result);

    // Creates a patch that can introduce changes in source to target
    patch_result_ptr createPatchData(scan_result& src, string_set& pathsSrc,
        scan_result& target, string_set& pathsTarget,
        string_set& unchanged, string_set& conflicts);

    // Write an individual patch result
    void WritePatchResult(std::string directory, patch_result_ptr result, std::ostream& output, bool ignoreUnchanged);

public:
    // ctor w/ checksum object instance
    Worker(const checksum_ptr instance);

    // Asynchronously run scanDirectory
    std::future<scan_result> scanDirectory(std::string path);

    // Run the reconcile operation
    void Reconcile(scan_result& a, scan_result& b, bool keepResult);

    // Write the results to a file
    void WriteResult(std::string dirA, std::string dirB, std::string destination, bool ignoreUnchanged);
};