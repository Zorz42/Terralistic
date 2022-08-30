#include <lz4.h>
#include "compress.hpp"
#include "exception.hpp"
#include "testing.hpp"

#include <iostream>
#include "graphics.hpp"

#define COMPRESSION_QUALITY 1.f

std::vector<char> compress(const std::vector<char>& decompressed_data) {
    gfx::Timer timer;
    
    int max_compressed_size = LZ4_compressBound((int)decompressed_data.size());
    std::vector<char> compressed_data(max_compressed_size);
    
    int compressed_size = LZ4_compress_fast(&decompressed_data[0], &compressed_data[0], (int)decompressed_data.size(), max_compressed_size, (1 - COMPRESSION_QUALITY) * 65537);
    
    if(compressed_size <= 0) throw CompressError("Could not archive some data!");
    
    compressed_data.resize(compressed_size + sizeof(unsigned int));
    
    unsigned int decompressed_size = (unsigned int)decompressed_data.size();
    for(int i = 0; i < sizeof(unsigned int); i++)
        compressed_data[compressed_size + i] = decompressed_size >> 8 * i;
    
    //std::cout << "compress " << timer.getTimeElapsed() << "ms" << std::endl;
    
    return compressed_data;
}

std::vector<char> decompress(const std::vector<char>& compressed_data) {
    gfx::Timer timer;
    
    unsigned int decompressed_size = 0, actual_decompressed_size;
    for(int i = 0; i < sizeof(unsigned int); i++)
        decompressed_size += (unsigned int)(unsigned char)compressed_data[compressed_data.size() - sizeof(unsigned int) + i] << i * 8;
    
    actual_decompressed_size = decompressed_size;
    
    std::vector<char> decompressed_data(actual_decompressed_size);
    
    decompressed_size = LZ4_decompress_safe(&compressed_data[0], &decompressed_data[0], (int)compressed_data.size() - sizeof(unsigned int), actual_decompressed_size);
    
    if(decompressed_size <= 0) throw ArchiveError("Archive is corrupted!");
    
    if(actual_decompressed_size != decompressed_size) throw ArchiveError("Uncompressed sizes did not match!");
    
    std::cout << "decompress " << timer.getTimeElapsed() << "ms" << std::endl;
    
    return decompressed_data;
}

