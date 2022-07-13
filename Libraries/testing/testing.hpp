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
    void performTest();
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
    
    void _performTests();
    void _registerTestCase();
};

class _TestClassNameSetter {
public:
    _TestClassNameSetter(_TestClass* test, const std::string& name);
};

class _CaseRegistrator {
public:
    _CaseRegistrator(void (_TestClass::* case_func)(), _TestClass* test_class, const std::string& case_name) {
        test_class->test_cases.push_back(_TestCase(case_func, case_name, test_class));
    }
};

void _setTestName(_TestClass* test, const std::string& name);

#define _TEST_CLASS(line) static class _TestClassInstance ## line : public _TestClass { \
typedef _TestClassInstance ## line self;

#define _TEST_CLASSP(line) _TEST_CLASS(line)
#define TEST_CLASS _TEST_CLASSP(__LINE__)

#define _TEST_CASE(name, line) _CaseRegistrator case_registrator ## line = _CaseRegistrator((void (_TestClass::*)())&self::case_name, this, #name); void case_name()
#define _TEST_CASEP(name, line) _TEST_CASE(name, line)
#define TEST_CASE(name) _TEST_CASEP(name, __LINE__)

#define _TEST_NAME(name, line) } name; static _TestClassNameSetter test_name_setter ## line (&name, #name);
#define _TEST_NAMEP(name, line) _TEST_NAME(name, line)
#define TEST_NAME(name) _TEST_NAMEP(name, __LINE__)

void performTests();
