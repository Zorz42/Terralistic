#include <iostream>
#include "testing.hpp"
#include "exception.hpp"

static std::vector<_TestClass*>* test_classes = nullptr;

_TestClass::_TestClass() {
    if(test_classes == nullptr)
        test_classes = new std::vector<_TestClass*>;
    
    test_classes->push_back(this);
}

bool _TestClass::_performTests() {
    std::cout << "Performing tests for [" << class_name << "]" << std::endl;
    int tests_passed = 0, tests_failed = 0;
    construct();
    for(_TestCase& test_case : test_cases) {
        if(test_case.performTest())
            tests_passed++;
        else
            tests_failed++;
    }
    destruct();
    std::cout << "Test results for [" << class_name << "]: " << tests_passed << " PASSED, " << tests_failed << " FAILED" << std::endl;
    return tests_failed == 0;
}

void _TestClass::_assert(bool x) {
    if(!x)
        throw Exception("Assert failure");
}

bool _TestCase::performTest() {
    bool result = true;
    std::cout << "Case " << case_name << "... ";
    try {
        (*instance.*case_func)();
    } catch(std::exception& e) {
        result = false;
    }
    std::cout << (result ? "PASSED" : "FAILED") << std::endl;
    return result;
}

_TestClassNameSetter::_TestClassNameSetter(_TestClass* test, const std::string& name) {
    test->class_name = name;
}

bool performTests() {
    if(test_classes == nullptr) {
        std::cout << "No tests to run!" << std::endl;
        return true;
    }
    
    int classes_passed = 0, classes_failed = 0;
    for(_TestClass* test_class : *test_classes) {
        if(test_class->_performTests())
            classes_passed++;
        else
            classes_failed++;
        std::cout << std::endl;
    }
    
    std::cout << "Overall results for all classes: " << classes_passed << " CLASSES PASSED, " << classes_failed << " CLASSES FAILED" << std::endl;
    return classes_failed == 0;
}

_CaseRegistrator::_CaseRegistrator(void (_TestClass::* case_func)(), _TestClass* test_class, const std::string& case_name) {
    test_class->test_cases.push_back(_TestCase(case_func, case_name, test_class));
}
