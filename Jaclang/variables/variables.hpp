#pragma once
#include "expression.hpp"

class Variable {
public:
    std::string name;
};

class VariableDeclarationType : public ProgramLineType {
    std::vector<Variable> variables;
public:
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    Variable& getVariableByID(int id);
    Variable& getVariableByName(const std::string& name);
};

namespace ProgramLineTypes {
    inline VariableDeclarationType variable_declaration;
};

class VariableDeclaration : public ProgramLine {
public:
    VariableDeclaration() : ProgramLine(ProgramLineTypes::variable_declaration.id) {}
};
