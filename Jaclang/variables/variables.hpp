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
    int getVariableIdByName(const std::string& name);
};

class VariableSettingType : public ProgramLineType {
public:
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
};

namespace ProgramLineTypes {
    inline VariableDeclarationType variable_declaration;
    inline VariableSettingType variable_setting;
};

class VariableDeclaration : public ProgramLine {
public:
    VariableDeclaration() : ProgramLine(ProgramLineTypes::variable_declaration.id) {}
    int variable_id;
};

class VariableSetting : public ProgramLine {
public:
    VariableSetting() : ProgramLine(ProgramLineTypes::variable_setting.id) {}
    int variable_id;
    Expression* value;
};

