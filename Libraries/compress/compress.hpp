#pragma once
#include <vector>

std::vector<char> compress(const std::vector<char>& decompressed_data);
std::vector<char> decompress(const std::vector<char>& compressed_data);
