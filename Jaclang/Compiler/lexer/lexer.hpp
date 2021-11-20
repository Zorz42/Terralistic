#pragma once
#include <vector>
#include <string>
#include <fstream>

enum class TokenType {
    NONE, INDENT, STRING, CONSTANT_INTEGER,
    ASSIGNMENT, EQUALS, LEFT_BRACKET, RIGHT_BRACKET, LEFT_CURLY_BRACKET, RIGHT_CURLY_BRACKET,
    IF, WHILE,
};

class Token {
public:
    TokenType type = TokenType::NONE;
    std::string text;
    int const_int;
};

std::vector<Token> tokenize(std::filebuf* file_buffer);
