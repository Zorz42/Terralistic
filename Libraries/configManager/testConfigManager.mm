#include <fstream>
#include "testing.hpp"
#include "configManager.hpp"
#include "exception.hpp"

TEST_CLASS(TestConfigManager)

TEST_CASE(testSetKeyGetKey) {
    ConfigFile config;
    config.setStr("testKey", "test value");
    ASSERT(config.getStr("testKey") == "test value");
}

TEST_CASE(testSetIntGetInt) {
    ConfigFile config;
    config.setInt("testKey", 101);
    ASSERT(config.getInt("testKey") == 101);
}

TEST_CASE(testKeyExists) {
    ConfigFile config;
    ASSERT(!config.keyExists("testKey"));
    config.setStr("testKey", "test value");
    ASSERT(config.keyExists("testKey"));
}

TEST_CASE(testDefaultValue) {
    ConfigFile config1;
    config1.setDefaultInt("testKey", 101);
    ASSERT(config1.getInt("testKey") == 101);
    
    ConfigFile config2;
    config2.setInt("testKey", 101);
    config2.setDefaultInt("testKey", 102);
    ASSERT(config2.getInt("testKey") == 101);
    
    ConfigFile config3;
    config3.setDefaultStr("testKey", "test value");
    ASSERT(config3.getStr("testKey") == "test value");
    
    ConfigFile config4;
    config4.setStr("testKey", "test value");
    config4.setDefaultStr("testKey", "incorrect value");
    ASSERT(config4.getStr("testKey") == "test value");
}

TEST_CASE(testSaving) {
    {
        ConfigFile config("test.txt");
        config.setStr("testKey", "test value");
        config.setInt("testInt", 101);
        config.saveConfig();
        
        ConfigFile config2("test.txt");
        ASSERT(config2.getStr("testKey") == "test value");
        ASSERT(config2.getInt("testInt") == 101);
    }
    
    std::remove("test.txt");
}

TEST_CASE(testThrowsKeyException) {
    ConfigFile config;
    ASSERT_THROWS(ConfigKeyError, config.getStr("testKey"));
    config.setStr("testKey", "test value");
    config.getStr("testKey");
}

TEST_CASE(testReadsGeneratedConfig) {
    {
        std::ofstream output_file("test.txt");
        std::string file_content = "testKey:   test value";
        output_file << file_content;
        output_file.close();
        
        ConfigFile config("test.txt");
        ASSERT(config.getStr("testKey") == "test value");
    }
    
    std::remove("test.txt");
}
 
END_TEST_CLASS(TestConfigManager)
