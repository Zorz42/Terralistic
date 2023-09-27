function register_recipes()
    terralistic_print("registering recipes...")

    terralistic_register_recipe(items.stone, 1, { items.dirt }, { 1 })
    terralistic_register_recipe(items.dirt, 1, { items.stone }, { 1 })
end