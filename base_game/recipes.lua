function register_recipes()
    terralistic_print("registering recipes...")

    terralistic_register_recipe(items.hatchet, 1, { items.stone, items.branch, items.fiber }, { 1, 1, 2 })
end