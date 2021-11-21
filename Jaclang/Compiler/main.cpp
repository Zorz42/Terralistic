#include <iostream>
#include <fstream>
#include "lexer.hpp"
#include "parser.hpp"
#include "error.hpp"
#include "expression.hpp"

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
    
    try {
        std::vector<Token> tokens = tokenize(file.rdbuf());
        
        for(Token& token : tokens)
            printToken(token);
        
        Parser parser;
        parser.registerAProgramLineType(&ProgramLineTypes::expression);
        parser.parseTokens(tokens);
        
        for(ProgramLine* line : parser.getProgramLines())
            parser.getProgramLineTypeByID(line->type_id)->print(line);
    } catch(Error error) {
        printError(error);
    }
    
    return 0;
}
