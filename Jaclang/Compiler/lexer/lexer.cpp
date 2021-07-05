//
//  lexer.cpp
//  Jaclang
//
//  Created by Jakob Zorz on 04/07/2021.
//
#include <iostream>
#include "lexer.hpp"

Symbol getSymbol(char c) {
    switch(c) {
        case '=':
            return Symbol::ASSIGNMENT;
        default:
            return Symbol::NONE;
    }
}

Symbol getSymbol(char c1, char c2) {
    switch(c1) {
        default:
            return Symbol::NONE;
    }
}


std::vector<Token> tokenize(std::filebuf* file_buffer) {
    for(std::istreambuf_iterator<char> i = file_buffer; i != std::istreambuf_iterator<char>(); i++) {
        
    }
    return {};
}
