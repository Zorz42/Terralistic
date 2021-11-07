#pragma once
#include <vector>
#include "exception.hpp"

std::vector<char> compress(std::vector<char>& decompressed_data);
std::vector<char> decompress(std::vector<char>& compressed_data);
