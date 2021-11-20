#include <iostream>
#include <fstream>
#include "lexer.hpp"

#define TOKEN_PRINT_SPACING 25

void printToken(std::string prefix, std::string content) {
    std::cout << prefix;
    for(int i = (int)prefix.size(); i < TOKEN_PRINT_SPACING; i++)
        std::cout << " ";
    std::cout << content << std::endl;
}

int main(int argc, const char * argv[]) {
    if(argc == 1) {
        std::cerr << "Usage: jaclang [input file]" << std::endl;
        return 1;
    }
    
    std::string file_path = argv[1];
    std::ifstream file(file_path);
    if(!file.is_open()) {
        std::cerr << "Could not open file " << file_path << std::endl;
        return 1;
    }
    
    std::vector<Token> tokens = tokenize(file.rdbuf());
    
    for(Token& token : tokens) {
        switch(token.type) {
            case TokenType::ASSIGNMENT:
                printToken("ASSIGNMENT", "=");
                break;
            case TokenType::EQUALS:
                printToken("EQUALS", "==");
                break;
            case TokenType::LEFT_BRACKET:
                printToken("LEFT_BRACKET", "(");
                break;
            case TokenType::RIGHT_BRACKET:
                printToken("RIGHT_BRACKET", ")");
                break;
            case TokenType::LEFT_CURLY_BRACKET:
                printToken("LEFT_CURLY_BRACKET", "{");
                break;
            case TokenType::RIGHT_CURLY_BRACKET:
                printToken("RIGHT_CURLY_BRACKET", "}");
                break;

            case TokenType::IF:
                printToken("IF", "if");
                break;
            case TokenType::WHILE:
                printToken("WHILE", "while");
                break;
                
            case TokenType::INDENT:
                printToken("INDENT", token.text);
                break;
            case TokenType::STRING:
                printToken("STRING", std::string("\"") + token.text + "\"");
                break;
            case TokenType::CONSTANT_INTEGER:
                printToken("CONST_INT", std::to_string(token.const_int));
                break;
                
            case TokenType::NONE:
                break;
        }
    }
    
    return 0;
}
