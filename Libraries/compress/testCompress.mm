#include "compress.hpp"
#include "testing.hpp"
#include <string>

TEST_CLASS(TestCompress)

TEST_CASE(testCompress) {
    std::string data = "This string should be compressed...";
    std::vector<char> uncompressed(data.begin(), data.end());
    std::vector<char> compressed = compress(uncompressed);
    std::vector<char> uncompressed_again = decompress(compressed);
    ASSERT(uncompressed == uncompressed_again);
}

TEST_CASE(testLargeCompress) {
    std::vector<char> uncompressed;
    for(int i = 0; i < 100000; i++)
        uncompressed.push_back(rand());
    
    std::vector<char> compressed = compress(uncompressed);
    std::vector<char> uncompressed_again = decompress(compressed);
    ASSERT(uncompressed == uncompressed_again);
}

END_TEST_CLASS(TestCompress)
