#include <iostream>
#include <fstream>
#include "lexer.hpp"
#include "parser.hpp"
#include "error.hpp"
#include "expression.hpp"
#include "constantValues.hpp"
#include "variables.hpp"

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
        
        ExpressionType expression_type;
        ConstantIntegerType constant_integer_type;
        VariableManager variable_manager;
        VariableSettingType variable_setting_type(&variable_manager, &expression_type);
        VariableDeclarationType variable_declaration_type(&variable_manager);
        
        Parser parser;
        parser.registerAProgramLineType(&variable_declaration_type);
        parser.registerAProgramLineType(&variable_setting_type);
        parser.registerAProgramLineType(&expression_type);
        expression_type.registerAValueType(&constant_integer_type);
        
        parser.parseTokens(tokens);
        
        for(ProgramLine* line : parser.getProgramLines())
            line->type->print(line, 0);
    } catch(Error error) {
        printError(error);
    }
    
    return 0;
}
