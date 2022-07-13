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

#define _TEST_CLASS(line) static class _TestClassInstance ## line : public _TestClass
#define _TEST_CLASSP(line) _TEST_CLASS(line)
#define TEST_CLASS _TEST_CLASSP(__LINE__)

#define TEST_CASE(case_name) void case_name()

#define _TEST_NAME(name, line) name; static _TestClassNameSetter test_name_setter ## line (&name, #name);
#define _TEST_NAMEP(name, line) _TEST_NAME(name, line)
#define TEST_NAME(name) _TEST_NAMEP(name, __LINE__)

void performTests();
