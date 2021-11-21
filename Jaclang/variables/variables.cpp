#include <iostream>
#include "variables.hpp"
#include "error.hpp"

void VariableDeclarationType::print(ProgramLine* line, int depth) {
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "variableDeclaration:" << std::endl;
    depth++;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "int" << std::endl;
}

ProgramLine* VariableDeclarationType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::INDENT || curr_token->text != "int")
        return nullptr;
    curr_token++;
    VariableDeclaration* variable_declaration = new VariableDeclaration;
    
    if(curr_token->type != TokenType::INDENT)
        throw Error("Expected variable name after type.");
    
    for(Variable& variable : variables)
        if(variable.name == curr_token->text)
            throw Error("Redefinition of " + curr_token->text);
    
    Variable variable;
    variable.name = curr_token->text;
    
    variables.push_back(variable);
    
    curr_token++;
    
    return variable_declaration;
}

Variable& VariableDeclarationType::getVariableByID(int id) {
    return variables[id];
}

Variable& VariableDeclarationType::getVariableByName(const std::string& name) {
    for(Variable& variable : variables)
        if(variable.name == name)
            return variable;
    throw Error("Could not find variable with name " + name);
}
