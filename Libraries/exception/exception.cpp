#include "exception.hpp"

const char* Exception::what() const throw() {
    return message.c_str();
}
