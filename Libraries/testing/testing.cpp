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
    for(_TestCase* test_case : test_cases)
        test_case->performTest();
}

void _TestCase::performTest() {
    std::cout << "Case " << case_name << std::endl;
}

_TestClassNameSetter::_TestClassNameSetter(_TestClass* test, const std::string& name) {
    test->class_name = name;
}

TEST_CLASS {
    TEST_CASE(TestTest) {
        
    }
} TEST_NAME(Test)

void performTests() {
    if(test_classes == nullptr) {
        std::cout << "No tests to run!" << std::endl;
        return;
    }
    
    for(_TestClass* test_class : *test_classes)
        test_class->_performTests();
}
