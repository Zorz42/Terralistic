#include <XCTest/XCTest.h>
#include "configManager.hpp"

@interface TestConfigManager : XCTestCase

@end

@implementation TestConfigManager

- (void)testSetKeyGetKey {
    ConfigFile config;
    config.setStr("testKey", "test value");
    XCTAssertEqual(config.getStr("testKey"), "test value");
}

- (void)testSetIntGetInt {
    ConfigFile config;
    config.setInt("testKey", 101);
    XCTAssertEqual(config.getInt("testKey"), 101);
}

- (void)testKeyExists {
    ConfigFile config;
    XCTAssertFalse(config.keyExists("testKey"));
    config.setStr("testKey", "test value");
    XCTAssertTrue(config.keyExists("testKey"));
}

- (void)testDefaultValue {
    ConfigFile config1;
    config1.setDefaultInt("testKey", 101);
    XCTAssertEqual(config1.getInt("testKey"), 101);
    
    ConfigFile config2;
    config2.setInt("testKey", 101);
    config2.setDefaultInt("testKey", 102);
    XCTAssertEqual(config2.getInt("testKey"), 101);
    
    ConfigFile config3;
    config3.setDefaultStr("testKey", "test value");
    XCTAssertEqual(config3.getStr("testKey"), "test value");
    
    ConfigFile config4;
    config4.setStr("testKey", "test value");
    config4.setDefaultStr("testKey", "incorrect value");
    XCTAssertEqual(config4.getStr("testKey"), "test value");
}

- (void)testSaving {
    ConfigFile config("test.txt");
    config.setStr("testKey", "test value");
    config.setInt("testInt", 101);
    config.saveConfig();
    
    ConfigFile config2("test.txt");
    XCTAssertEqual(config2.getStr("testKey"), "test value");
    XCTAssertEqual(config2.getInt("testInt"), 101);
}

- (void)testThrowsKeyException {
    ConfigFile config;
    bool threw = false;
    try {
        config.getStr("testKey");
    } catch(Exception e) {
        threw = true;
    }
    XCTAssertTrue(threw);
    config.setStr("testKey", "test value");
    config.getStr("testKey");
}

@end
