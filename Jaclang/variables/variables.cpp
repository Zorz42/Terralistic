#include <iostream>
#include "variables.hpp"
#include "error.hpp"

void VariableDeclarationType::print(ProgramLine* line, int depth) {
    VariableDeclaration* variable_declaration = (VariableDeclaration*)line;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "variableDeclaration:" << std::endl;
    depth++;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "int " << getVariableByID(variable_declaration->variable_id).name << std::endl;
}

ProgramLine* VariableDeclarationType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::INDENT || curr_token->text != "int")
        return nullptr;
    
    curr_token++;
    VariableDeclaration* variable_declaration = new VariableDeclaration;
    variable_declaration->variable_id = (int)variables.size();
    
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

int VariableDeclarationType::getVariableIdByName(const std::string& name) {
    for(int i = 0; i < variables.size(); i++)
        if(variables[i].name == name)
            return i;
    throw Error("Could not find variable with name " + name);
}

void VariableSettingType::print(ProgramLine* line, int depth) {
    VariableSetting* variable_setting = (VariableSetting*)line;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "variableSetting:" << std::endl;
    
    depth++;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << ProgramLineTypes::variable_declaration.getVariableByID(variable_setting->variable_id).name << std::endl;
    ProgramLineTypes::expression.print(variable_setting->value, depth);
}

ProgramLine* VariableSettingType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::INDENT || (curr_token + 1)->type != TokenType::ASSIGNMENT)
        return nullptr;
    
    VariableSetting* variable_setting = new VariableSetting;
    
    variable_setting->variable_id = ProgramLineTypes::variable_declaration.getVariableIdByName(curr_token->text);
    curr_token += 2;
    Expression* expression = (Expression*)ProgramLineTypes::expression.parse(curr_token);
    if(expression == nullptr)
        throw Error("Expected expression");
    
    variable_setting->value = expression;
    
    return variable_setting;
}
