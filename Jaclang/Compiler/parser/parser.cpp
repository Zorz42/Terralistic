#include "parser.hpp"

Parser::Parser() {
    registerAProgramLineType(&ProgramLineTypes::expression);
}

void Parser::parseTokens(const std::vector<Token>& tokens) {
    const Token* curr_token = &tokens[0];
    while(curr_token->type != TokenType::END)
        for(ProgramLineType* type : program_line_types) {
            ProgramLine* line = type->parse(curr_token);
            if(line) {
                program_lines.push_back(line);
                break;
            }
        }
}

const std::vector<ProgramLine*> Parser::getProgramLines() {
    return program_lines;
}

void ExpressionType::print(ProgramLine* line) {
    
}

ProgramLine* ExpressionType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::CONSTANT_INTEGER)
        return nullptr;
    
    Expression* expression = new Expression;
    
    expression->first_value = curr_token->const_int;
    
    while(true) {
        curr_token++;
        OperatorType operator_type;
        switch(curr_token->type) {
            case TokenType::PLUS:
                operator_type = OperatorType::PLUS;
                break;
            case TokenType::MINUS:
                operator_type = OperatorType::MINUS;
                break;
            default:
                operator_type = OperatorType::NONE;
                break;
        }
        
        if(operator_type == OperatorType::NONE)
            break;
        
        curr_token++;
        
        expression->values.emplace_back();
        expression->values.back().first = operator_type;
        expression->values.back().second = curr_token->const_int;
    }
    
    return expression;
}

ProgramLineType* Parser::getProgramLineTypeByID(int id) {
    return program_line_types[id];
}

void Parser::registerAProgramLineType(ProgramLineType* program_line_type) {
    program_line_type->id = (int)program_line_types.size();
    program_line_types.push_back(program_line_type);
}
