#include "parser.hpp"
#include "error.hpp"

void Parser::parseTokens(const std::vector<Token>& tokens) {
    const Token* curr_token = &tokens[0];
    while(curr_token->type != TokenType::END) {
        bool recognised = true;
        for(ProgramLineType* type : program_line_types) {
            ProgramLine* line = type->parse(curr_token);
            if(line) {
                program_lines.push_back(line);
                recognised = true;
                break;
            }
        }
        if(!recognised)
            throw Error("Code not recognised.");
    }
}

const std::vector<ProgramLine*> Parser::getProgramLines() {
    return program_lines;
}

void Parser::registerAProgramLineType(ProgramLineType* program_line_type) {
    program_line_types.push_back(program_line_type);
}
