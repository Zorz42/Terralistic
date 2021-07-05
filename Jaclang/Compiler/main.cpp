//
//  main.cpp
//  Jaclang
//
//  Created by Jakob Zorz on 04/07/2021.
//

#include <iostream>
#include <fstream>

#include "lexer.hpp"

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
                        std::cout << "= (ASSIGNMENT)" << std::endl;
                        break;
                }
                break;
            case TokenType::KEYWORD:
                switch(token.keyword) {
                    case Keyword::NONE:
                        break;
                    case Keyword::IF:
                        std::cout << "IF" << std::endl;
                        break;
                }
                break;
            case TokenType::INDENT:
                std::cout << token.text << std::endl;
                break;
            case TokenType::STRING:
                std::cout << "\"" << token.text << "\"" << std::endl;
                break;
            case TokenType::CONSTANT_INTEGER:
                std::cout << token.const_int << std::endl;
                break;
        }
    }
    
    return 0;
}
