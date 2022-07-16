#pragma once
#include <vector>
#include "exception.hpp"

std::vector<char> compress(const std::vector<char>& decompressed_data);
std::vector<char> decompress(const std::vector<char>& compressed_data);

EXCEPTION_TYPE(CompressError)
EXCEPTION_TYPE(ArchiveError)
