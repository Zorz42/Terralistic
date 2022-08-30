#include <zlib.h>
#include "compress.hpp"
#include "exception.hpp"
#include "testing.hpp"

std::vector<char> compress(const std::vector<char>& decompressed_data) {
    unsigned long compressed_size = decompressed_data.size() * 1.01 + 12;
    std::vector<char> compressed_data(compressed_size);
    
    if(compress((Bytef*)&compressed_data[0], &compressed_size, (const Bytef*)&decompressed_data[0], decompressed_data.size()) != Z_OK) throw CompressError("Could not archive some data!");
    
    unsigned int decompressed_size = (unsigned int)decompressed_data.size();
    for(int i = 0; i < sizeof(unsigned int); i++)
        compressed_data[compressed_size + i] = decompressed_size >> 8 * i;
    
    compressed_data.resize(compressed_size + sizeof(unsigned int));
    
    return compressed_data;
}

std::vector<char> decompress(const std::vector<char>& compressed_data) {
    unsigned long decompressed_size = 0, actual_decompressed_size;
    for(int i = 0; i < sizeof(unsigned int); i++)
        decompressed_size += (unsigned int)(unsigned char)compressed_data[compressed_data.size() - sizeof(unsigned int) + i] << i * 8;
    
    actual_decompressed_size = decompressed_size;
    
    std::vector<char> decompressed_data(decompressed_size);
    
    if(uncompress((Bytef*)&decompressed_data[0], &actual_decompressed_size, (const Bytef*)&compressed_data[0], compressed_data.size() - sizeof(unsigned int)) != Z_OK) throw ArchiveError("Archive is corrupted!");
    
    if(actual_decompressed_size != decompressed_size) throw ArchiveError("Uncompressed sizes did not match!");
    
    return decompressed_data;
}
