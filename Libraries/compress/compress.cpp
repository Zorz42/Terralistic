#include <zlib.h>
#include "compress.hpp"

std::vector<char> compress(std::vector<char>& decompressed_data) {
    unsigned long compressed_size = decompressed_data.size() * 1.1 + 12;
    std::vector<char> compressed_data(compressed_size);
    
    compress((Bytef*)&compressed_data[0], &compressed_size, (Bytef*)&decompressed_data[0], decompressed_data.size());
    
    *(unsigned long*)&compressed_data[compressed_size] = decompressed_data.size();
    compressed_data.resize(compressed_size + 8);
    
    return compressed_data;
}

std::vector<char> decompress(std::vector<char>& compressed_data) {
    unsigned long uncompressed_size = *(unsigned long*)&compressed_data[compressed_data.size() - 8];
    std::vector<char> decompressed_data(uncompressed_size);
    
    uncompress((Bytef*)&decompressed_data[0], &uncompressed_size, (Bytef*)&compressed_data[0], compressed_data.size() - 8);
    
    return decompressed_data;
}
