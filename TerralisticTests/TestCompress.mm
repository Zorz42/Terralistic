#include <XCTest/XCTest.h>
#include "compress.hpp"
#include <string>

@interface TestCompress : XCTestCase

@end

@implementation TestCompress

#define DATA_SIZE 1000000
std::vector<char> uncompressed_data(DATA_SIZE), compressed_data(DATA_SIZE);

- (void)setUp {
    srand(0);
    for(int i = 0; i < DATA_SIZE; i++)
        uncompressed_data[i] = rand();
    compressed_data = compress(uncompressed_data);
    
}

void compressData() {
    compress(uncompressed_data);
}

void decompressData() {
    decompress(compressed_data);
}

- (void)testCompress {
    std::string data = "This string should be compressed...";
    std::vector<char> uncompressed(data.begin(), data.end());
    std::vector<char> compressed = compress(uncompressed);
    std::vector<char> uncompressed_again = decompress(compressed);
    XCTAssertEqual(uncompressed, uncompressed_again);
}

- (void)testCompressPerformance {
    [self measureBlock:^{
        compressData();
    }];
}

- (void)testDecompressPerformance {
    [self measureBlock:^{
        decompressData();
    }];
}

@end
