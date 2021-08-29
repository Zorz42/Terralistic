#ifndef compress_hpp
#define compress_hpp

#include <vector>

std::vector<char> compress(std::vector<char>& decompressed_data);
std::vector<char> decompress(std::vector<char>& compressed_data);

#endif