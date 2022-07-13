#include <iostream>
#include "testing.hpp"

static std::vector<_TestClass*>* test_classes = nullptr;

_TestClass::_TestClass() {
    if(test_classes == nullptr)
        test_classes = new std::vector<_TestClass*>;
    
    test_classes->push_back(this);
}

void _TestClass::_performTests() {
    std::cout << "Performing tests for [" << class_name << "]" << std::endl;
    construct();
    for(_TestCase& test_case : test_cases)
        test_case.performTest();
    destruct();
}

void _TestCase::performTest() {
    std::cout << "Case " << case_name << std::endl;
    (*instance.*case_func)();
}

_TestClassNameSetter::_TestClassNameSetter(_TestClass* test, const std::string& name) {
    test->class_name = name;
}

TEST_CLASS
    TEST_CASE(TestTest) {
        std::cout << "testing" << std::endl;
    }
TEST_NAME(Test)

struct A {
    typedef A self;
    void (self::*a)() = &self::test_func;
    void test_func() {
        
    }
};

void performTests() {
    if(test_classes == nullptr) {
        std::cout << "No tests to run!" << std::endl;
        return;
    }
    
    for(_TestClass* test_class : *test_classes)
        test_class->_performTests();
}
