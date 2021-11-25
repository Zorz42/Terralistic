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
    std::cout << "int " << variable_manager->getVariableByID(variable_declaration->variable_id).name << std::endl;
}

ProgramLine* VariableDeclarationType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::INDENT || curr_token->text != "int")
        return nullptr;
    
    curr_token++;
    VariableDeclaration* variable_declaration = new VariableDeclaration(this);
    
    if(curr_token->type != TokenType::INDENT)
        throw Error("Expected variable name after type.");
    
    if(variable_manager->variableExists(curr_token->text))
        throw Error("Redefinition of " + curr_token->text);
    
    Variable& variable = variable_manager->addVariable(curr_token->text);
    variable_declaration->variable_id = variable.id;
    
    curr_token++;
    
    return variable_declaration;
}

Variable& VariableManager::getVariableByID(int id) {
    return variables[id];
}

bool VariableManager::variableExists(const std::string& name) {
    for(Variable& variable : variables)
        if(variable.name == name)
            return true;
    return false;
}

Variable& VariableManager::getVariableByName(const std::string& name) {
    for(Variable& variable : variables)
        if(variable.name == name)
            return variable;
    throw Error("Could not find variable with name " + name);
}

int VariableManager::getVariableIdByName(const std::string& name) {
    for(int i = 0; i < variables.size(); i++)
        if(variables[i].name == name)
            return i;
    throw Error("Could not find variable with name " + name);
}

Variable& VariableManager::addVariable(const std::string& name) {
    Variable variable(name, (int)variables.size());
    variables.push_back(variable);
    return variables.back();
}

void VariableSettingType::print(ProgramLine* line, int depth) {
    VariableSetting* variable_setting = (VariableSetting*)line;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "variableSetting:" << std::endl;
    
    depth++;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << variable_manager->getVariableByID(variable_setting->variable_id).name << std::endl;
    expression_type->print(variable_setting->value, depth);
}

ProgramLine* VariableSettingType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::INDENT || (curr_token + 1)->type != TokenType::ASSIGNMENT)
        return nullptr;
    
    VariableSetting* variable_setting = new VariableSetting(this);
    
    variable_setting->variable_id = variable_manager->getVariableIdByName(curr_token->text);
    curr_token += 2;
    Expression* expression = (Expression*)expression_type->parse(curr_token);
    if(expression == nullptr)
        throw Error("Expected expression");
    
    variable_setting->value = expression;
    
    return variable_setting;
}

std::vector<Instruction*> VariableDeclarationType::toInstructions(ProgramLine* line) {
    return {};
}

std::vector<Instruction*> VariableSettingType::toInstructions(ProgramLine* line) {
    return {};
}
