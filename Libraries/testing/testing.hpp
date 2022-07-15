#pragma once
#include <string>
#include <vector>

class _TestClass;

class _TestCase {
    std::string case_name;
    void (_TestClass::* case_func)();
    _TestClass* instance;
public:
    _TestCase(void (_TestClass::* case_func)(), const std::string& case_name, _TestClass* instance) : case_func(case_func), case_name(case_name), instance(instance) {}
    bool performTest();
};

class _TestClass {
    friend class _TestClassNameSetter;
    friend class _CaseRegistrator;
    std::vector<_TestCase> test_cases;
    std::string class_name;
public:
    _TestClass();
    virtual void construct() {}
    virtual void destruct() {}
    
    bool _performTests();
    void _registerTestCase();
    
    void _assert(bool x);
};

class _TestClassNameSetter {
public:
    _TestClassNameSetter(_TestClass* test, const std::string& name);
};

class _CaseRegistrator {
public:
    _CaseRegistrator(void (_TestClass::* case_func)(), _TestClass* test_class, const std::string& case_name);
};

void _setTestName(_TestClass* test, const std::string& name);

bool performTests();

#if defined(XCTESTING) && defined(__OBJC__)

#include <XCTest/XCTest.h>

#define TEST_CLASS(name) @interface name : XCTestCase @end @implementation name

#define TEST_CASE(name) - (void)name

#define END_TEST_CLASS(name) @end

#define ASSERT(x) XCTAssertTrue(x)

#else

#define TEST_CLASS(name) static class _TestClassInstance ## name : public _TestClass { typedef _TestClassInstance ## name self;

#define _TEST_CASE(name, line) _CaseRegistrator case_registrator ## line = _CaseRegistrator((void (_TestClass::*)())&self::name, this, #name); void name()
#define _TEST_CASEP(name, line) _TEST_CASE(name, line)
#define TEST_CASE(name) _TEST_CASEP(name, __LINE__)

#define _END_TEST_CLASS(name, line) } name; static _TestClassNameSetter test_name_setter ## line (&name, #name);
#define _END_TEST_CLASSP(name, line) _END_TEST_CLASS(name, line)
#define END_TEST_CLASS(name) _END_TEST_CLASSP(name, __LINE__)

#define ASSERT(x) _assert(x)

#endif

