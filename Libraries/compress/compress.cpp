#include <zlib.h>
#include "compress.hpp"

std::vector<char> compress(std::vector<char>& decompressed_data) {
    unsigned long compressed_size = decompressed_data.size() * 1.1 + 12;
    std::vector<char> compressed_data(compressed_size);
    
    if(compress((Bytef*)&compressed_data[0], &compressed_size, (Bytef*)&decompressed_data[0], decompressed_data.size()) != Z_OK) {
        throw ArchivingException();
        return {};
    }
    
    int decompressed_size = (int)decompressed_data.size();
    for(int i = 0; i < sizeof(int); i++)
        compressed_data[compressed_size + i] = decompressed_size >> 8 * i;
    
    compressed_data.resize(compressed_size + sizeof(int));
    
    return compressed_data;
}

std::vector<char> decompress(std::vector<char>& compressed_data) {
    unsigned long uncompressed_size = 0;
    for(int i = 0; i < sizeof(int); i++)
        uncompressed_size += (int)compressed_data[compressed_data.size() - sizeof(int) + i] << i * 8;
    
    std::vector<char> decompressed_data(uncompressed_size);
    
    if(uncompress((Bytef*)&decompressed_data[0], &uncompressed_size, (Bytef*)&compressed_data[0], compressed_data.size() - sizeof(int)) != Z_OK) {
        throw ArchiveCorrutionException();
        return {};
    }
    
    return decompressed_data;
}
