#pragma once
#include <string>
#include <vector>

class _TestCase {
    std::string case_name;
public:
    _TestCase(const std::string& case_name) : case_name(case_name) {}
    void performTest();
};

class _TestClass {
    friend class _TestClassNameSetter;
    std::vector<_TestCase*> test_cases;
    std::string class_name;
public:
    _TestClass();
    virtual void construct() {}
    virtual void destruct() {}
    
    void _performTests();
    void _registerTestCase();
};

class _TestClassNameSetter {
public:
    _TestClassNameSetter(_TestClass* test, const std::string& name);
};

void _setTestName(_TestClass* test, const std::string& name);

#define TEST_CLASS \
static class _TestClassInstance : public _TestClass

#define TEST_CASE(case_name) void case_name()

#define TEST_NAME(name) test; static _TestClassNameSetter test_name_setter(&test, #name);

void performTests();
