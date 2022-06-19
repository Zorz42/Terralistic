#pragma once
#include <string>
#include <stdexcept>
#include <utility>

class Exception : public std::exception {
public:
    explicit Exception(std::string  message) : message(std::move(message)) {}
    const std::string message;
    
    const char* what() const noexcept override {
       return message.c_str();
    }
};
