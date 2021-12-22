#pragma once
#include <string>
#include <stdexcept>

class Exception : public std::exception {
public:
    Exception(const std::string& message) : message(message) {}
    const std::string message;
    
    const char* what() const throw() {
       return message.c_str();
    }
};
