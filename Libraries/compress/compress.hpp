#pragma once
#include <vector>
#include "exception.hpp"

std::vector<char> compress(std::vector<char>& decompressed_data);
std::vector<char> decompress(std::vector<char>& compressed_data);

class ArchiveCorrutionException : Exception {
public:
    ArchiveCorrutionException() : Exception("Archive is corrupted!") {}
};

class ArchivingException : Exception {
public:
    ArchivingException() : Exception("Could not archive some data!") {}
};
