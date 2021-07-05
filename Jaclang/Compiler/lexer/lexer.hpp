//
//  lexer.hpp
//  Jaclang
//
//  Created by Jakob Zorz on 04/07/2021.
//

#ifndef lexer_hpp
#define lexer_hpp

#include <vector>
#include <string>
#include <fstream>

enum class TokenType {
    NONE, INDENT, STRING, CONSTANT_INTEGER,
    ASSIGNMENT, EQUALS, LEFT_BRACKET, RIGHT_BRACKET, LEFT_CURLY_BRACKET, RIGHT_CURLY_BRACKET,
    IF, WHILE,
};
 
struct Token {
    TokenType type;
    std::string text;
    int const_int;
};

std::vector<Token> tokenize(std::filebuf* file_buffer);

#endif /* lexer_hpp */
