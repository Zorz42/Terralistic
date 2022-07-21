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
    virtual void _construct() {}
    virtual void _destruct() {}
    
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

#define CASE_CONSTRUCTOR - (void)setUp

#define CASE_DESTRUCTOR - (void)tearDown

#else

#define TEST_CLASS(name) static class _TestClassInstance ## name : public _TestClass { typedef _TestClassInstance ## name self;

#define TEST_CASE(name) _CaseRegistrator case_registrator ## name = _CaseRegistrator((void (_TestClass::*)())&self::name, this, #name); void name()

#define END_TEST_CLASS(name) } name; static _TestClassNameSetter test_name_setter ## name (&name, #name);

#define ASSERT(x) _assert(x)

#define CASE_CONSTRUCTOR void _construct() override
#define CASE_DESTRUCTOR void _destruct() override

#endif

#define ASSERT_THROWS(exception, expr) {bool threw = false; try{ expr; } catch(const exception& e) { threw = true; }; ASSERT(threw); }

