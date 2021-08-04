#include "serverPlayers.hpp"

static std::vector<Recipe> recipes;

const std::vector<Recipe>& getRecipes() {
    return recipes;
}

void initRecipes() {
    recipes = {
        Recipe({{ItemType::STONE_BLOCK, 1}}, {ItemType::DIRT, 2}),
    };
}
