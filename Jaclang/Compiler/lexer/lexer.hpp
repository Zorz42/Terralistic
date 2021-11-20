#pragma once
#include <vector>
#include <string>
#include <fstream>

enum class TokenType {
    NONE, INDENT, STRING, CONSTANT_INTEGER, END,
    // synbols
    ASSIGNMENT, LEFT_BRACKET, RIGHT_BRACKET, LEFT_CURLY_BRACKET, RIGHT_CURLY_BRACKET,
    // operators
    EQUALS, PLUS, MINUS,
    // keywords
    IF, WHILE,
};

class Token {
public:
    TokenType type = TokenType::NONE;
    std::string text;
    int const_int;
};

std::vector<Token> tokenize(std::filebuf* file_buffer);
void printToken(const Token& token);
