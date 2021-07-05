//
//  lexer.cpp
//  Jaclang
//
//  Created by Jakob Zorz on 04/07/2021.
//
#include <iostream>
#include "lexer.hpp"

inline bool isInteger(const std::string& text) {
    // copied from internet and modified
   char* p;
   strtol(text.c_str(), &p, 10);

   return *p == 0;
}

Symbol getSymbol(char c) {
    switch(c) {
        case '=':
            return Symbol::ASSIGNMENT;
        case '(':
            return Symbol::LEFT_BRACKET;
        case ')':
            return Symbol::RIGHT_BRACKET;
        case '{':
            return Symbol::LEFT_CURLY_BRACKET;
        case '}':
            return Symbol::RIGHT_CURLY_BRACKET;
            
        default:
            return Symbol::NONE;
    }
}

Symbol getSymbol(char c1, char c2) {
    switch(c1) {
        case '=':
            switch(c2) {
                case '=': // ==
                    return Symbol::EQUALS;
                default:
                    return Symbol::NONE;
            }
        default:
            return Symbol::NONE;
    }
}

static std::string curr_token;

Token endToken() {
    static const std::vector<std::string> keywords = {
        "if",
    };
    Token result;
    auto position = std::find(keywords.begin(), keywords.end(), curr_token);
    if(position != keywords.end()) {
        result.keyword = Keyword(int(Keyword::NONE) + int(position - keywords.begin()) + 1);
        result.type = TokenType::KEYWORD;
    } else if(isInteger(curr_token)) {
        result.const_int = std::stoi(curr_token);
        result.type = TokenType::CONSTANT_INTEGER;
    } else {
        result.text = curr_token;
        result.type = TokenType::INDENT;
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
        
        Symbol one_symbol = getSymbol(curr);
        Symbol two_symbol = getSymbol(curr, next);
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
        } else if(two_symbol != Symbol::NONE) {
            if(!curr_token.empty())
                tokens.push_back(endToken());
            Token symbol_token;
            symbol_token.symbol = two_symbol;
            symbol_token.type = TokenType::SYMBOL;
            tokens.push_back(symbol_token);
            i++;
        } else if(one_symbol != Symbol::NONE) {
            if(!curr_token.empty())
                tokens.push_back(endToken());
            Token symbol_token;
            symbol_token.symbol = one_symbol;
            symbol_token.type = TokenType::SYMBOL;
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
    return tokens;
}
