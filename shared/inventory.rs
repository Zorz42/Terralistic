use crate::shared::items::ItemId;
use std::collections::HashMap;

pub struct Recipe {
    pub result: ItemId,
    pub ingredients: HashMap<ItemId, i32>,
}

pub struct Recipes {
    recipes: Vec<Recipe>,
}

impl Recipes {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    #[must_use]
    pub const fn get_recipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }
}


