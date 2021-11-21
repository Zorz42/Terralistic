#include "expression.hpp"
#include "error.hpp"

void ExpressionType::print(ProgramLine* line) {
    Expression* expression = (Expression*)line;
    
}

ProgramLine* ExpressionType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::CONSTANT_INTEGER)
        return nullptr;
    
    Expression *expression = new Expression, *curr_expression = expression;
    
    while(true) {
        if(curr_token->type != TokenType::CONSTANT_INTEGER)
            throw Error("Expected value after operator.");
        
        curr_expression->value = curr_token->const_int;
        curr_token++;
        
        switch(curr_token->type) {
            case TokenType::PLUS:
                curr_expression->operator_type = OperatorType::PLUS;
                break;
            case TokenType::MINUS:
                curr_expression->operator_type = OperatorType::MINUS;
                break;
            default:
                curr_expression->operator_type = OperatorType::NONE;
                break;
        }
        
        if(curr_expression->operator_type == OperatorType::NONE) {
            curr_expression->next = nullptr;
            return expression;
        }
        
        curr_expression->next = new Expression;
        curr_expression = curr_expression->next;
        
        curr_token++;
    }
    
    return expression;
}
