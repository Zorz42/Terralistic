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
TEST_NAME(TestCompress)
