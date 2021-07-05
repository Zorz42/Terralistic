//
//  main.cpp
//  Jaclang
//
//  Created by Jakob Zorz on 04/07/2021.
//

#include <iostream>
#include <fstream>

#include "lexer.hpp"

#define TOKEN_PRINT_SPACING 20

void printToken(std::string prefix, std::string content) {
    std::cout << prefix;
    for(int i = prefix.size(); i < TOKEN_PRINT_SPACING; i++)
        std::cout << " ";
    std::cout << content << std::endl;
}

int main(int argc, const char * argv[]) {
    std::string file_path = argv[1];
    std::ifstream file(file_path);
    
    std::vector<Token> tokens = tokenize(file.rdbuf());
    
    for(Token& token : tokens) {
        switch(token.type) {
            case TokenType::SYMBOL:
                switch(token.symbol) {
                    case Symbol::NONE:
                        break;
                    case Symbol::ASSIGNMENT:
                        printToken("ASSIGNMENT", "=");
                        break;
                    case Symbol::EQUALS:
                        printToken("EQUALS", "==");
                        break;
                    case Symbol::LEFT_BRACKET:
                        printToken("LEFT_BRACKET", "(");
                        break;
                    case Symbol::RIGHT_BRACKET:
                        printToken("RIGHT_BRACKET", ")");
                        break;
                    case Symbol::LEFT_CURLY_BRACKET:
                        printToken("LEFT_CURLY_BRACKET", "{");
                        break;
                    case Symbol::RIGHT_CURLY_BRACKET:
                        printToken("RIGHT_CURLY_BRACKET", "}");
                        break;
                }
                break;
            case TokenType::KEYWORD:
                switch(token.keyword) {
                    case Keyword::NONE:
                        break;
                    case Keyword::IF:
                        printToken("KEYWORD", "IF");
                        break;
                    case Keyword::WHILE:
                        printToken("KEYWORD", "WHILE");
                        break;
                }
                break;
            case TokenType::INDENT:
                printToken("INDENT", token.text);
                break;
            case TokenType::STRING:
                printToken("STRING", (std::string)"\"" + token.text + "\"");
                break;
            case TokenType::CONSTANT_INTEGER:
                printToken("CONST_INT", std::to_string(token.const_int));
                break;
        }
    }
    
    return 0;
}
