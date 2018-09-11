#pragma once

#include <ctime>
#include <iomanip>

#include <boost/filesystem.hpp>
#include <cryptopp/filters.h>
#include <cryptopp/files.h>
#include <cryptopp/hex.h>

std::_Put_time<char> GetFormattedDateTime();

template <typename T>
std::string HashFile(std::string filepath){
    using namespace CryptoPP;
    // CryptoPP::HashTransformation* checksum = (CryptoPP::HashTransformation*)this->checksumInstance->Clone();
    std::string digest;
    T checksum;
    FileSource fileSource(filepath.c_str(), true, new HashFilter(checksum, new HexEncoder(new StringSink(digest))));

    return digest;
}