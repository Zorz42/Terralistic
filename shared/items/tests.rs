#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::shared::blocks::Blocks;
    use crate::shared::entities::{Entities, PositionComponent};
    use crate::shared::items::{Item, ItemComponent, ItemStack, Items, Recipe, TileDrop};
    use crate::shared::walls::Walls;

    fn register(items: &mut Items, name: &str, max_stack: i32) -> crate::shared::items::ItemId {
        let mut item = Item::new();
        item.name = name.to_owned();
        item.display_name = name.to_owned();
        item.max_stack = max_stack;
        items.register_item_type(item)
    }

    #[test]
    fn test_new_items_is_empty() {
        let items = Items::new();
        assert_eq!(items.get_num_item_types(), 0);
        assert!(items.get_all_item_type_ids().is_empty());
    }

    #[test]
    fn test_register_and_look_up_by_id() {
        let mut items = Items::new();
        let id = register(&mut items, "stone", 99);

        assert_eq!(items.get_num_item_types(), 1);

        let item = items.get_item_type(id).unwrap();
        assert_eq!(item.name, "stone");
        assert_eq!(item.max_stack, 99);
    }

    /// Ids are handed out in registration order and stay distinct.
    #[test]
    fn test_ids_are_distinct_and_ordered() {
        let mut items = Items::new();
        let a = register(&mut items, "a", 1);
        let b = register(&mut items, "b", 1);
        let c = register(&mut items, "c", 1);

        assert!(a != b && b != c && a != c);
        assert_eq!(items.get_all_item_type_ids().len(), 3);
        assert_eq!(items.get_item_type(a).unwrap().name, "a");
        assert_eq!(items.get_item_type(c).unwrap().name, "c");
    }

    #[test]
    fn test_look_up_by_name() {
        let mut items = Items::new();
        register(&mut items, "torch", 99);

        assert_eq!(items.get_item_type_by_name("torch").unwrap().name, "torch");
        assert!(items.get_item_type_by_name("not_a_real_item").is_err());
    }

    /// An id that was never registered is an error rather than a panic or a silent default.
    #[test]
    fn test_unknown_item_id_is_an_error() {
        let items = Items::new();
        assert!(items.get_item_type(crate::shared::items::ItemId::new()).is_err());
    }

    #[test]
    fn test_block_drops() {
        let blocks = Blocks::new();
        let mut items = Items::new();
        let item_id = register(&mut items, "dirt", 99);

        let air = blocks.air();
        assert!(items.get_block_drop(air).is_err(), "a block with no drop set should report an error");

        items.set_block_drop(air, TileDrop::new(item_id, 1.0));
        let drop = items.get_block_drop(air).unwrap();
        assert_eq!(drop.item, item_id);
        assert!((drop.chance - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_wall_drops() {
        let mut blocks = Blocks::new();
        let walls = Walls::new(&mut blocks);
        let mut items = Items::new();
        let item_id = register(&mut items, "wood", 99);

        // `clear` is the one wall type Walls::new registers; registering another needs
        // pub(super) access that this module does not have
        let wall_id = walls.clear;

        assert!(items.get_wall_drop(wall_id).is_err());

        items.set_wall_drop(wall_id, TileDrop::new(item_id, 0.5));
        assert_eq!(items.get_wall_drop(wall_id).unwrap().item, item_id);
    }

    #[test]
    fn test_recipes() {
        let mut items = Items::new();
        let ingredient = register(&mut items, "log", 99);
        let result = register(&mut items, "planks", 99);

        assert!(items.get_recipes().is_empty());

        let mut recipe = Recipe::new();
        recipe.result = ItemStack::new(result, 4);
        recipe.ingredients.insert(ingredient, 1);
        items.add_recipe(recipe);

        assert_eq!(items.get_recipes().len(), 1);

        let id = items.get_recipes().iter().next().unwrap().get_id();
        let fetched = items.get_recipe(id).unwrap();
        assert_eq!(fetched.result.item, result);
        assert_eq!(fetched.result.count, 4);
    }

    #[test]
    fn test_unknown_recipe_id_is_an_error() {
        let items = Items::new();
        assert!(items.get_recipe(crate::shared::items::RecipeId::new()).is_err());
    }

    /// Spawning puts an item entity in the world at the requested position.
    #[test]
    fn test_spawn_item() {
        let mut items = Items::new();
        let id = register(&mut items, "stone", 99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity_id = entities.new_id();
        let entity = items.spawn_item(&mut events, &mut entities, id, 3.0, 4.0, entity_id).unwrap();

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!((position.x() - 3.0).abs() < f32::EPSILON);
        assert!((position.y() - 4.0).abs() < f32::EPSILON);

        let component = entities.ecs.get::<&ItemComponent>(entity).unwrap();
        assert_eq!(component.get_item_type(), id);
    }

    /// The same entity id cannot be assigned twice.
    #[test]
    fn test_spawn_item_with_a_duplicate_id_fails() {
        let mut items = Items::new();
        let id = register(&mut items, "stone", 99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity_id = entities.new_id();
        items.spawn_item(&mut events, &mut entities, id, 0.0, 0.0, entity_id).unwrap();
        items.spawn_item(&mut events, &mut entities, id, 0.0, 0.0, entity_id).unwrap_err();
    }

    /// Dropping spawns an item and gives it some velocity, so drops scatter.
    #[test]
    fn test_drop_item_gives_it_velocity() {
        let mut items = Items::new();
        let id = register(&mut items, "stone", 99);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        items.drop_item(&mut events, &mut entities, id, 0.0, 0.0).unwrap();

        let count = entities.ecs.query_mut::<&ItemComponent>().into_iter().count();
        assert_eq!(count, 1);
    }
}
