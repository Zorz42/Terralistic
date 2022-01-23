#include <XCTest/XCTest.h>
#include "exception.hpp"

@interface TestException : XCTestCase

@end

@implementation TestException

void throwException(std::string message) {
    throw Exception(message);
}

- (void)testThrow {
    XCTAssertThrows(throwException("error"));
    
    std::string message;
    try {
        throwException("message");
    } catch(Exception& e) {
        message = e.what();
    }
    XCTAssertEqual(message, "message");
}

@end
