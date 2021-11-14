#include <zlib.h>
#include "compress.hpp"
#include "exception.hpp"

std::vector<char> compress(std::vector<char>& decompressed_data) {
    unsigned long compressed_size = decompressed_data.size() * 1.01 + 12;
    std::vector<char> compressed_data(compressed_size);
    
    if(compress((Bytef*)&compressed_data[0], &compressed_size, (Bytef*)&decompressed_data[0], decompressed_data.size()) != Z_OK)
        throw Exception("Could not archive some data!");
    
    unsigned int decompressed_size = (unsigned int)decompressed_data.size();
    for(int i = 0; i < sizeof(unsigned int); i++)
        compressed_data[compressed_size + i] = decompressed_size >> 8 * i;
    
    compressed_data.resize(compressed_size + sizeof(unsigned int));
    
    return compressed_data;
}

std::vector<char> decompress(std::vector<char>& compressed_data) {
    unsigned long uncompressed_size = 0;
    for(int i = 0; i < sizeof(unsigned int); i++)
        uncompressed_size += (unsigned int)(unsigned char)compressed_data[compressed_data.size() - sizeof(unsigned int) + i] << i * 8;
    
    std::vector<char> decompressed_data(uncompressed_size);
    
    if(uncompress((Bytef*)&decompressed_data[0], &uncompressed_size, (Bytef*)&compressed_data[0], compressed_data.size() - sizeof(unsigned int)) != Z_OK)
        throw Exception("Archive is corrupted!");
    
    return decompressed_data;
}
