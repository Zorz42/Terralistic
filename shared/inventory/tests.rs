#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::shared::entities::Entities;
    use crate::shared::inventory::Inventory;
    use crate::shared::items::{Item, ItemComponent, ItemId, ItemStack, Items, Recipe};

    const DROP_POS: (f32, f32) = (0.0, 0.0);

    /// Builds an `Items` with a single registered type of the given stack limit.
    fn items_with_max_stack(max_stack: i32) -> (Items, ItemId) {
        let mut items = Items::new();
        let mut item = Item::new();
        item.name = "test_item".to_owned();
        item.max_stack = max_stack;
        let id = items.register_item_type(item);
        (items, id)
    }

    /// Counts loose item entities in the world, i.e. items that were dropped rather
    /// than fitting into the inventory.
    fn dropped_count(entities: &mut Entities) -> usize {
        entities.ecs.query_mut::<&ItemComponent>().into_iter().count()
    }

    #[test]
    fn test_new_inventory_is_empty() {
        let inventory = Inventory::new(5);
        assert_eq!(inventory.get_size(), 5);
        for i in 0..5 {
            assert!(inventory.get_item(i).unwrap().is_none());
        }
    }

    #[test]
    fn test_get_item_out_of_bounds() {
        let inventory = Inventory::new(2);
        inventory.get_item(2).unwrap_err();
        inventory.get_item(99).unwrap_err();
    }

    #[test]
    fn test_set_item_out_of_bounds() {
        let mut inventory = Inventory::new(2);
        let (_items, id) = items_with_max_stack(10);
        inventory.set_item(2, Some(ItemStack::new(id, 1))).unwrap_err();
    }

    /// A stack with a non-positive count is normalised to an empty slot rather than
    /// being stored as a zero-count stack.
    #[test]
    fn test_set_item_with_zero_count_clears_the_slot() {
        let mut inventory = Inventory::new(2);
        let (_items, id) = items_with_max_stack(10);

        inventory.set_item(0, Some(ItemStack::new(id, 0))).unwrap();
        assert!(inventory.get_item(0).unwrap().is_none());

        inventory.set_item(1, Some(ItemStack::new(id, -3))).unwrap();
        assert!(inventory.get_item(1).unwrap().is_none());
    }

    #[test]
    fn test_has_changed_only_flips_on_a_real_change() {
        let mut inventory = Inventory::new(2);
        let (_items, id) = items_with_max_stack(10);

        // writing None over an already empty slot is not a change
        inventory.set_item(0, None).unwrap();
        assert!(!inventory.has_changed);

        inventory.set_item(0, Some(ItemStack::new(id, 1))).unwrap();
        assert!(inventory.has_changed);

        inventory.has_changed = false;
        // writing the same stack again is not a change
        inventory.set_item(0, Some(ItemStack::new(id, 1))).unwrap();
        assert!(!inventory.has_changed);
    }

    #[test]
    fn test_get_item_count_sums_across_slots() {
        let mut inventory = Inventory::new(4);
        let (_items, id) = items_with_max_stack(99);

        assert_eq!(inventory.get_item_count(id), 0);

        inventory.set_item(0, Some(ItemStack::new(id, 3))).unwrap();
        inventory.set_item(2, Some(ItemStack::new(id, 4))).unwrap();

        assert_eq!(inventory.get_item_count(id), 7);
    }

    #[test]
    fn test_give_item_fills_an_empty_slot() {
        let (items, id) = items_with_max_stack(99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(4);

        inventory.give_item(ItemStack::new(id, 5), DROP_POS, &items, &mut entities, &mut events).unwrap();

        assert_eq!(inventory.get_item(0).unwrap().unwrap().count, 5);
        assert_eq!(dropped_count(&mut entities), 0);
    }

    /// Giving an item that is already present tops up the existing stack instead of
    /// consuming a second slot.
    #[test]
    fn test_give_item_merges_into_an_existing_stack() {
        let (items, id) = items_with_max_stack(99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(4);

        inventory.set_item(0, Some(ItemStack::new(id, 10))).unwrap();
        inventory.give_item(ItemStack::new(id, 5), DROP_POS, &items, &mut entities, &mut events).unwrap();

        assert_eq!(inventory.get_item(0).unwrap().unwrap().count, 15);
        assert!(inventory.get_item(1).unwrap().is_none());
    }

    /// Once the first stack hits `max_stack` the remainder spills into the next free slot.
    #[test]
    fn test_give_item_overflows_into_a_new_slot() {
        let (items, id) = items_with_max_stack(10);
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(4);

        inventory.give_item(ItemStack::new(id, 25), DROP_POS, &items, &mut entities, &mut events).unwrap();

        assert_eq!(inventory.get_item(0).unwrap().unwrap().count, 10);
        assert_eq!(inventory.get_item(1).unwrap().unwrap().count, 10);
        assert_eq!(inventory.get_item(2).unwrap().unwrap().count, 5);
        assert_eq!(dropped_count(&mut entities), 0);
    }

    /// What does not fit is dropped into the world as loose item entities.
    #[test]
    fn test_give_item_drops_the_remainder_when_full() {
        let (items, id) = items_with_max_stack(10);
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(1);

        // one slot holding at most 10, so 3 of the 13 have nowhere to go
        inventory.give_item(ItemStack::new(id, 13), DROP_POS, &items, &mut entities, &mut events).unwrap();

        assert_eq!(inventory.get_item(0).unwrap().unwrap().count, 10);
        assert_eq!(dropped_count(&mut entities), 3);
    }

    #[test]
    fn test_can_craft() {
        let (_items, id) = items_with_max_stack(99);
        let mut inventory = Inventory::new(4);

        let mut recipe = Recipe::new();
        recipe.result = ItemStack::new(id, 1);
        recipe.ingredients.insert(id, 5);

        assert!(!inventory.can_craft(&recipe));

        inventory.set_item(0, Some(ItemStack::new(id, 4))).unwrap();
        assert!(!inventory.can_craft(&recipe));

        inventory.set_item(1, Some(ItemStack::new(id, 1))).unwrap();
        assert!(inventory.can_craft(&recipe), "ingredients may be spread across slots");
    }

    #[test]
    fn test_craft_without_ingredients_fails() {
        let (items, id) = items_with_max_stack(99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(4);

        let mut recipe = Recipe::new();
        recipe.result = ItemStack::new(id, 1);
        recipe.ingredients.insert(id, 5);

        inventory.craft(&recipe, DROP_POS, &items, &mut entities, &mut events).unwrap_err();
    }

    /// Crafting removes exactly the ingredient count and hands back the result.
    #[test]
    fn test_craft_consumes_ingredients_and_gives_result() {
        let mut items = Items::new();

        let mut ingredient = Item::new();
        ingredient.name = "ingredient".to_owned();
        ingredient.max_stack = 99;
        let ingredient_id = items.register_item_type(ingredient);

        let mut result = Item::new();
        result.name = "result".to_owned();
        result.max_stack = 99;
        let result_id = items.register_item_type(result);

        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let mut inventory = Inventory::new(4);
        inventory.set_item(0, Some(ItemStack::new(ingredient_id, 8))).unwrap();

        let mut recipe = Recipe::new();
        recipe.result = ItemStack::new(result_id, 1);
        recipe.ingredients.insert(ingredient_id, 5);

        inventory.craft(&recipe, DROP_POS, &items, &mut entities, &mut events).unwrap();

        assert_eq!(inventory.get_item_count(ingredient_id), 3);
        assert_eq!(inventory.get_item_count(result_id), 1);
        assert_eq!(dropped_count(&mut entities), 0);
    }

    #[test]
    fn test_selected_item() {
        let mut inventory = Inventory::new(4);
        let (_items, id) = items_with_max_stack(99);

        assert!(inventory.get_selected_item().is_none());

        inventory.set_item(2, Some(ItemStack::new(id, 7))).unwrap();
        inventory.selected_slot = Some(2);

        assert_eq!(inventory.get_selected_item().unwrap().count, 7);
    }

    #[test]
    fn test_swap_with_selected_item() {
        let mut items_vec = Items::new();

        let mut a = Item::new();
        a.name = "a".to_owned();
        a.max_stack = 99;
        let a_id = items_vec.register_item_type(a);

        let mut b = Item::new();
        b.name = "b".to_owned();
        b.max_stack = 99;
        let b_id = items_vec.register_item_type(b);

        let mut inventory = Inventory::new(4);
        inventory.set_item(0, Some(ItemStack::new(a_id, 1))).unwrap();
        inventory.set_item(1, Some(ItemStack::new(b_id, 2))).unwrap();
        inventory.selected_slot = Some(0);

        inventory.swap_with_selected_item(1).unwrap();

        assert_eq!(inventory.get_item(0).unwrap().unwrap().item, b_id);
        assert_eq!(inventory.get_item(1).unwrap().unwrap().item, a_id);
    }

    /// With no slot selected the swap is a no-op rather than an error or a lost item.
    #[test]
    fn test_swap_with_no_selection_does_nothing() {
        let mut inventory = Inventory::new(4);
        let (_items, id) = items_with_max_stack(99);
        inventory.set_item(1, Some(ItemStack::new(id, 2))).unwrap();
        inventory.selected_slot = None;

        inventory.swap_with_selected_item(1).unwrap();

        assert_eq!(inventory.get_item(1).unwrap().unwrap().count, 2);
    }

    #[test]
    fn test_transfer_items_from() {
        let (_items, id) = items_with_max_stack(99);

        let mut source = Inventory::new(3);
        source.set_item(1, Some(ItemStack::new(id, 6))).unwrap();

        let mut target = Inventory::new(3);
        target.transfer_items_from(source);

        assert_eq!(target.get_item(1).unwrap().unwrap().count, 6);
    }
}
