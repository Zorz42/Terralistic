#include <iostream>
#include "lexer.hpp"

#define TOKEN_PRINT_SPACING 25

inline bool isInteger(const std::string& text) {
   char* p;
   strtol(text.c_str(), &p, 10);
   return *p == 0;
}

TokenType getSymbol(char c) {
    switch(c) {
        case '=':
            return TokenType::ASSIGNMENT;
        case '(':
            return TokenType::LEFT_BRACKET;
        case ')':
            return TokenType::RIGHT_BRACKET;
        case '{':
            return TokenType::LEFT_CURLY_BRACKET;
        case '}':
            return TokenType::RIGHT_CURLY_BRACKET;
        case '+':
            return TokenType::PLUS;
        case '-':
            return TokenType::MINUS;
            
        default:
            return TokenType::NONE;
    }
}

TokenType getSymbol(char c1, char c2) {
    switch(c1) {
        case '=':
            switch(c2) {
                case '=': // ==
                    return TokenType::EQUALS;
                default:
                    return TokenType::NONE;
            }
        default:
            return TokenType::NONE;
    }
}

static std::string curr_token;

Token endToken() {
    Token result;
    
    if(curr_token == "if")
        result.type = TokenType::IF;
    else if(curr_token == "while")
        result.type = TokenType::WHILE;
    else if(isInteger(curr_token)) {
        result.const_int = std::stoi(curr_token);
        result.type = TokenType::CONSTANT_INTEGER;
    } else {
        result.text = curr_token;
        result.type = TokenType::IDENT;
    }
    
    curr_token.clear();
    return result;
}

std::vector<Token> tokenize(std::filebuf* file_buffer) {
    std::vector<Token> tokens;
    curr_token.clear();
    for(std::istreambuf_iterator<char> i = file_buffer; i != std::istreambuf_iterator<char>();) {
        char curr = *i;
        i++;
        char next = *i;
        
        TokenType one_symbol = getSymbol(curr);
        TokenType two_symbol = getSymbol(curr, next);
        if(curr == '"') {
            if(!curr_token.empty())
                tokens.push_back(endToken());
            
            while(*i != '"') {
                curr_token.push_back(*i);
                i++;
            }
            Token result;
            result.type = TokenType::STRING;
            result.text = curr_token;
            curr_token.clear();
            tokens.push_back(result);
            i++;
        } else if(two_symbol != TokenType::NONE) {
            if(!curr_token.empty())
                tokens.push_back(endToken());
            Token symbol_token;
            symbol_token.type = two_symbol;
            tokens.push_back(symbol_token);
            i++;
        } else if(one_symbol != TokenType::NONE) {
            if(!curr_token.empty())
                tokens.push_back(endToken());
            Token symbol_token;
            symbol_token.type = one_symbol;
            tokens.push_back(symbol_token);
        } else if(curr == ' ' || curr == '\t' || curr == '\n') {
            if(!curr_token.empty())
                tokens.push_back(endToken());
        } else {
            curr_token.push_back(curr);
        }
    }
    if(!curr_token.empty())
        tokens.push_back(endToken());
    tokens.push_back({TokenType::END});
    return tokens;
}

void printToken(std::string prefix, std::string content) {
    std::cout << prefix;
    for(int i = (int)prefix.size(); i < TOKEN_PRINT_SPACING; i++)
        std::cout << " ";
    std::cout << content << std::endl;
}

void printToken(const Token& token) {
    switch(token.type) {
        case TokenType::IDENT:
            printToken("IDENT", token.text);
            break;
        case TokenType::STRING:
            printToken("STRING", std::string("\"") + token.text + "\"");
            break;
        case TokenType::CONSTANT_INTEGER:
            printToken("CONST_INT", std::to_string(token.const_int));
            break;
        
        case TokenType::NONE:
        case TokenType::END:
            break;
            
            // symbols
        case TokenType::ASSIGNMENT:
            printToken("ASSIGNMENT", "=");
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
            
            // operators
        case TokenType::EQUALS:
            printToken("EQUALS", "==");
            break;
        case TokenType::PLUS:
            printToken("PLUS", "+");
            break;
        case TokenType::MINUS:
            printToken("MINUS", "-");
            break;

            // keywords
        case TokenType::IF:
            printToken("IF", "if");
            break;
        case TokenType::WHILE:
            printToken("WHILE", "while");
            break;
    }
}
