#include <iostream>
#include "error.hpp"

void printError(Error error) {
    std::cout << "Error: " << error.message << std::endl;
}
