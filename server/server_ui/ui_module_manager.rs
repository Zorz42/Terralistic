use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};

use crate::libraries::graphics as gfx;
use crate::server::server_ui::UiMessageType;

/// This enum indicates the type of the `ModuleTree` Node.
/// `Nothing` means that the node and its window area are empty.
/// `Split` means that the node's window area splits into 2 more nodes.
/// `Module` means that the node is a module which takes up the node's window area.
/// Ideally Nothing should never be used as that means the upper node splits into a module ans nothing, when it itself should just be a module. Nothing may be used for editing the tree in the future
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub enum ModuleTreeNodeType {
    Nothing,
    Split(Box<ModuleTreeSplit>),
    Module(String),
}

impl ModuleTreeNodeType {
    #[allow(dead_code)] //will be used
    fn get_node_by_name_mut(&mut self, name: &str) -> Option<&mut Self> {
        match self {
            Self::Split(split_node) => {
                return if let Some(node) = split_node.first.get_node_by_name_mut(name) {
                    Some(node)
                } else {
                    split_node.second.get_node_by_name_mut(name)
                };
            }
            Self::Module(mod_name) => {
                if mod_name == name {
                    return Some(self);
                }
            }
            Self::Nothing => {}
        }
        None
    }
}

/// This enum indicates the type of split. `Vertical` splits the window area of that node into 2 areas, one on the left and one on the right. `Horizontal` splits the window area of that node into 2 areas, one on the top and one on the bottom.
#[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum SplitType {
    Vertical,
    Horizontal,
}

/// This struct is used to save the module's positions on the screen in a binary tree like structure.
/// The `orientation` indicates the type of split.
/// The `split_pos` indicates the position of the split, with the number from 0 to 1 indicating what part of the area is assigned to the left or top subpart, and the rest is allocated to the bottom part.
/// The `first` and `second` indicate the first and second nodes of the split. The first node is the left or top node, and the second node is the right or bottom node, depending on the orientation.
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ModuleTreeSplit {
    pub orientation: SplitType,
    pub split_pos: f32,
    pub first: ModuleTreeNodeType,
    pub second: ModuleTreeNodeType,
}

#[derive(PartialEq, Eq)]
enum EditMode {
    Select,
    Name,
    Resize,
}

pub struct ModuleManager {
    root: ModuleTreeNodeType,
    path: Vec<bool>,
    //max depth of 5
    depth: usize,
    rect: gfx::Rect,
    pub changed: bool,
    name_buffer: String,
    mode: EditMode,
    renderer: ModuleManagerRenderer,
}

impl Default for ModuleManager {
    fn default() -> Self {
        let root = Self::default_module_tree();
        let path = Vec::with_capacity(5);
        Self {
            root,
            path,
            depth: 1,
            rect: gfx::Rect {
                pos: gfx::FloatPos(0.0, 0.0),
                size: gfx::FloatSize(1.0, 1.0),
            },
            changed: false,
            name_buffer: String::new(),
            mode: EditMode::Select,
            renderer: ModuleManagerRenderer::new(),
        }
    }
}

impl ModuleManager {
    pub fn new(root: ModuleTreeNodeType) -> Self {
        Self {
            root,
            ..Self::default()
        }
    }

    /// Reads the module tree from the save file in `server_data/ui_config.json`. if the file doesn't exist or is not a valid format, use the default config
    pub fn from_save_file(config_path: &Path) -> Self {
        match Self::try_from_save_file(config_path) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("{e}");
                Self::default()
            }
        }
    }

    fn try_from_save_file(config_path: &Path) -> Result<Self> {
        let config_file_path = config_path.join("ui_config.json");
        let file = File::open(config_file_path)?;
        let reader = BufReader::new(file);
        let root = serde_json::from_reader(reader)?;

        Ok(Self::new(root))
    }

    fn try_save_to_file(&self, config_path: &Path) -> Result<()> {
        let mut file = File::create(config_path.join("ui_config.json"))?;
        let json_str = serde_json::to_string_pretty(&self.root)?;
        let res = file.write(json_str.as_bytes());
        if let Err(err) = res {
            return Err(anyhow!("Failed to write ui_config.json: {err}"));
        }

        Ok(())
    }

    pub fn save_to_file(&self, config_path: &Path, sender: &Sender<UiMessageType>) {
        if let Err(e) = self.try_save_to_file(config_path) {
            eprintln!("{e}");
            if let Err(e) = sender.send(UiMessageType::UiToSrvConsoleMessage(e.to_string())) {
                eprintln!("{e}");
            }
        }
    }

    /// Creates the default module tree and returns it
    fn default_module_tree() -> ModuleTreeNodeType {
        ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
            orientation: SplitType::Horizontal,
            split_pos: 0.1,
            first: ModuleTreeNodeType::Module("server_info".to_owned()),
            second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: ModuleTreeNodeType::Module("player_list".to_owned()),
                second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                    orientation: SplitType::Horizontal,
                    split_pos: 0.2,
                    first: ModuleTreeNodeType::Module("empty_1".to_owned()),
                    second: ModuleTreeNodeType::Module("console".to_owned()),
                })),
            })),
        }))
    }

    pub fn get_root_mut(&mut self) -> &mut ModuleTreeNodeType {
        &mut self.root
    }

    pub fn on_event(&mut self, event: &gfx::Event, graphics_context: &gfx::GraphicsContext) {
        match event {
            gfx::Event::KeyPress(key, _repeat) => {
                if *key == gfx::Key::F1 {
                    //reset
                    self.depth = 0;
                    self.recalculate_selection_rect();
                    self.mode = EditMode::Select;
                }
                if *key == gfx::Key::F2 && self.mode == EditMode::Select {
                    self.name_buffer.clear();
                    self.mode = EditMode::Name;
                }
                if *key == gfx::Key::F3 && self.mode == EditMode::Select {
                    if let ModuleTreeNodeType::Split(split) = self.get_node(None, self.depth) {
                        self.renderer.split_orientation = split.orientation;
                        self.mode = EditMode::Resize;
                    }
                }
                match self.mode {
                    EditMode::Select => {
                        self.handle_select_mode_key_events(*key);
                    }
                    EditMode::Name => {
                        self.handle_rename_mode_key_events(*key);
                    }
                    EditMode::Resize => {
                        self.handle_resize_mode_key_events(*key);
                    }
                }
            }
            gfx::Event::MouseScroll(scroll) => {
                self.handle_mouse_scroll(*scroll);
            }
            gfx::Event::TextInput(text) => {
                if self.mode == EditMode::Name {
                    self.name_buffer.push_str(text);
                    self.renderer
                        .update_texture(graphics_context, &self.name_buffer);
                }
            }
            gfx::Event::KeyRelease(_, _) => {}
        }
    }

    fn handle_select_mode_key_events(&mut self, key: gfx::Key) {
        match key {
            gfx::Key::Down => {
                self.depth += 1;
                self.recalculate_selection_rect();
            }
            gfx::Key::Up => {
                if self.depth > 0 {
                    self.depth -= 1;
                    self.recalculate_selection_rect();
                }
            }
            gfx::Key::Space => {
                if self.depth > 0 {
                    let path_at_depth = self.path.get_mut(self.depth - 1);
                    if let Some(path_at_depth) = path_at_depth {
                        *path_at_depth = !*path_at_depth;
                        self.recalculate_selection_rect();
                    }
                }
            }
            gfx::Key::V => {
                self.split(self.depth, SplitType::Vertical, 0.5);
                self.changed = true;
            }
            gfx::Key::S => {
                self.split(self.depth, SplitType::Horizontal, 0.5);
                self.changed = true;
            }
            gfx::Key::Q => {
                self.delete(self.depth);
                self.recalculate_selection_rect();
                self.changed = true;
            }
            gfx::Key::X => {
                self.swap(self.depth);
                self.recalculate_selection_rect();
                self.changed = true;
            }
            gfx::Key::R => {
                self.root = Self::default_module_tree();
                self.recalculate_selection_rect();
                self.changed = true;
            }
            _ => {}
        }
    }

    fn handle_rename_mode_key_events(&mut self, key: gfx::Key) {
        match key {
            gfx::Key::Enter => {
                let name = self.name_buffer.clone();
                self.replace_module_with_empty(&name);
                let node = self.get_node_mut(None, self.depth);
                *node = ModuleTreeNodeType::Module(name);
                self.mode = EditMode::Select;
                self.recalculate_selection_rect();
                self.changed = true;
            }
            gfx::Key::Escape => {
                self.mode = EditMode::Select;
            }
            gfx::Key::Backspace => {
                self.name_buffer.pop();
            }
            _ => {}
        }
    }

    fn handle_resize_mode_key_events(&mut self, key: gfx::Key) {
        match key {
            gfx::Key::Enter | gfx::Key::Escape => {
                self.mode = EditMode::Select;
            }
            _ => {}
        }
    }

    fn handle_mouse_scroll(&mut self, scroll: f32) {
        if self.mode == EditMode::Resize {
            let node = self.get_node_mut(None, self.depth);
            if let ModuleTreeNodeType::Split(split) = node {
                split.split_pos += scroll * 0.01;
                split.split_pos = split.split_pos.clamp(0.0, 1.0);
                self.changed = true;
                self.recalculate_selection_rect();
            }
        }
    }

    fn get_node_mut(&mut self, path: Option<&[bool]>, depth: usize) -> &mut ModuleTreeNodeType {
        if self.path.len() < depth + 1 {
            self.path.resize(depth + 1, false);
        }
        let mut node = &mut self.root;
        let path = path.unwrap_or(&self.path);
        for &path_at_depth in path.get(0..depth).unwrap_or(&[]) {
            if let ModuleTreeNodeType::Split(split_node) = node {
                node = if path_at_depth {
                    &mut split_node.second
                } else {
                    &mut split_node.first
                };
            } else {
                break;
            }
        }
        node
    }

    fn get_node(&self, path: Option<&[bool]>, depth: usize) -> &ModuleTreeNodeType {
        let mut node = &self.root;
        let path = path.unwrap_or(&self.path);
        for &path_at_depth in path.get(0..depth).unwrap_or(&[]) {
            if let ModuleTreeNodeType::Split(split_node) = node {
                node = if path_at_depth {
                    &split_node.second
                } else {
                    &split_node.first
                };
            } else {
                break;
            }
        }
        node
    }

    fn get_empty_name() -> String {
        let mut name = "empty_".to_owned();
        //append a random number
        name.push_str(&rand::random::<u32>().to_string());
        name
    }

    fn replace_module_with_empty(&mut self, name: &str) {
        if let Some(node) = self.get_node_by_name_mut(name) {
            *node = ModuleTreeNodeType::Module(Self::get_empty_name());
        }
    }

    fn get_node_by_name_mut(&mut self, name: &str) -> Option<&mut ModuleTreeNodeType> {
        self.root.get_node_by_name_mut(name)
    }

    fn split(&mut self, depth: usize, orientation: SplitType, split_pos: f32) {
        if self.path.len() < depth + 1 {
            self.path.resize(depth + 1, false);
        }
        let name = Self::get_empty_name();

        let old_node = self.get_node_mut(None, depth);

        let mut new_node = ModuleTreeNodeType::Nothing;
        std::mem::swap(&mut new_node, old_node);
        *old_node = ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
            orientation,
            split_pos,
            first: new_node,
            second: ModuleTreeNodeType::Module(name),
        }));
    }

    fn delete(&mut self, depth: usize) {
        if depth == 0 {
            self.root = ModuleTreeNodeType::Module("Empty".to_owned());
            return;
        }
        let mut flipped_path = self.path.clone(); //looks like a stupid way to flip the last element but avoids many Option<T> cases
        if let Some(last) = flipped_path.get_mut(depth - 1) {
            *last = !*last;
        }

        let mut temp_node = ModuleTreeNodeType::Nothing;

        let new_node = self.get_node_mut(Some(&flipped_path), depth); //one of the children that will replace the parent split node
        std::mem::swap(new_node, &mut temp_node);

        let old_node = self.get_node_mut(None, depth - 1); //the split that will be replaced by one of its children
        std::mem::swap(&mut temp_node, old_node);
    }

    fn swap(&mut self, depth: usize) {
        let node = self.get_node_mut(None, depth);
        if let ModuleTreeNodeType::Split(split) = node {
            split.split_pos = 1.0 - split.split_pos;
            std::mem::swap(&mut split.first, &mut split.second);
        }
    }

    pub fn render_selection(&self, graphics_context: &gfx::GraphicsContext) {
        self.renderer.render_selection(graphics_context, &self.rect);
    }

    pub fn render_overlay(&self, graphics_context: &gfx::GraphicsContext) {
        self.renderer
            .render_overlay(graphics_context, &self.rect, &self.mode);
    }

    fn recalculate_selection_rect(&mut self) {
        if self.path.len() < self.depth + 1 {
            self.path.resize(self.depth + 1, false);
        }
        let (coords, max_depth) = self.get_overlay_rect_coords(&self.root, 0);
        self.depth = self.depth.clamp(0, max_depth);
        self.rect.pos = coords.0;
        self.rect.size = coords.1;
    }

    fn get_overlay_rect_coords(
        &self,
        node: &ModuleTreeNodeType,
        depth: usize,
    ) -> ((gfx::FloatPos, gfx::FloatSize), usize) {
        if depth == self.depth {
            return ((gfx::FloatPos(0.0, 0.0), gfx::FloatSize(1.0, 1.0)), depth);
        }
        match node {
            ModuleTreeNodeType::Split(node) => match node.orientation {
                SplitType::Vertical => {
                    let split_factors = self.get_vertical_split_factors(node, depth);
                    let sub_node = if *self.path.get(depth).unwrap_or(&false) {
                        &node.second
                    } else {
                        &node.first
                    };
                    let (sub_node_factors, max_depth) =
                        self.get_overlay_rect_coords(sub_node, depth + 1);

                    (
                        Self::calculate_factors(split_factors, sub_node_factors),
                        max_depth,
                    )
                }
                SplitType::Horizontal => {
                    let split_factors = self.get_horizontal_split_factors(node, depth);
                    let sub_node = if *self.path.get(depth).unwrap_or(&false) {
                        &node.second
                    } else {
                        &node.first
                    };
                    let (sub_node_factors, max_depth) =
                        self.get_overlay_rect_coords(sub_node, depth + 1);

                    (
                        Self::calculate_factors(split_factors, sub_node_factors),
                        max_depth,
                    )
                }
            },
            _ => ((gfx::FloatPos(0.0, 0.0), gfx::FloatSize(1.0, 1.0)), depth),
        }
    }
    fn get_vertical_split_factors(
        &self,
        node: &ModuleTreeSplit,
        depth: usize,
    ) -> (gfx::FloatPos, gfx::FloatSize) {
        if *self.path.get(depth).unwrap_or(&false) {
            (
                gfx::FloatPos(node.split_pos, 0.0),
                gfx::FloatSize(1.0 - node.split_pos, 1.0),
            )
        } else {
            (gfx::FloatPos(0.0, 0.0), gfx::FloatSize(node.split_pos, 1.0))
        }
    }

    fn get_horizontal_split_factors(
        &self,
        node: &ModuleTreeSplit,
        depth: usize,
    ) -> (gfx::FloatPos, gfx::FloatSize) {
        if *self.path.get(depth).unwrap_or(&false) {
            (
                gfx::FloatPos(0.0, node.split_pos),
                gfx::FloatSize(1.0, 1.0 - node.split_pos),
            )
        } else {
            (gfx::FloatPos(0.0, 0.0), gfx::FloatSize(1.0, node.split_pos))
        }
    }

    fn calculate_factors(
        split_factors: (gfx::FloatPos, gfx::FloatSize),
        sub_node_factors: (gfx::FloatPos, gfx::FloatSize),
    ) -> (gfx::FloatPos, gfx::FloatSize) {
        (
            gfx::FloatPos(
                split_factors.0 .0 + sub_node_factors.0 .0 * split_factors.1 .0,
                split_factors.0 .1 + sub_node_factors.0 .1 * split_factors.1 .1,
            ),
            gfx::FloatSize(
                split_factors.1 .0 * sub_node_factors.1 .0,
                split_factors.1 .1 * sub_node_factors.1 .1,
            ),
        )
    }
}

struct ModuleManagerRenderer {
    name_sprite: gfx::Sprite,
    vertical_arrow_sprite: gfx::Sprite,
    horizontal_arrow_sprite: gfx::Sprite,
    split_orientation: SplitType,
}

impl ModuleManagerRenderer {
    fn new() -> Self {
        let mut name_sprite = gfx::Sprite::new();
        name_sprite.orientation = gfx::CENTER;
        name_sprite.scale = 3.0;

        let mut vertical_arrow_sprite = gfx::Sprite::new();
        vertical_arrow_sprite.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/vertical_resize_arrow.opa"
            ))
            .unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(0, 0))),
        );
        vertical_arrow_sprite.orientation = gfx::CENTER;
        vertical_arrow_sprite.scale = 4.0;

        let mut horizontal_arrow_sprite = gfx::Sprite::new();
        horizontal_arrow_sprite.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/horizontal_resize_arrow.opa"
            ))
            .unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(0, 0))),
        );
        horizontal_arrow_sprite.orientation = gfx::CENTER;
        horizontal_arrow_sprite.scale = 4.0;

        Self {
            name_sprite,
            vertical_arrow_sprite,
            horizontal_arrow_sprite,
            split_orientation: SplitType::Horizontal,
        }
    }

    fn get_pos_size(rect: &gfx::Rect, graphics_context: &gfx::GraphicsContext) -> gfx::Rect {
        let window_size = graphics_context.renderer.get_window_size();
        let pos = gfx::FloatPos(window_size.0 * rect.pos.0, window_size.1 * rect.pos.1);
        let size = gfx::FloatSize(window_size.0 * rect.size.0, window_size.1 * rect.size.1);
        gfx::Rect::new(pos, size)
    }

    fn render_selection(&self, graphics_context: &gfx::GraphicsContext, fraction_rect: &gfx::Rect) {
        let rect = Self::get_pos_size(fraction_rect, graphics_context);
        rect.render(graphics_context, gfx::WHITE);
    }

    fn render_overlay(
        &self,
        graphics_context: &gfx::GraphicsContext,
        fraction_rect: &gfx::Rect,
        edit_mode: &EditMode,
    ) {
        if *edit_mode == EditMode::Select {
            return;
        }
        let rect = Self::get_pos_size(fraction_rect, graphics_context);
        let color = gfx::Color::new(0, 0, 0, 150);
        rect.render(graphics_context, color);

        if *edit_mode == EditMode::Name {
            self.render_rename_overlay(graphics_context, &rect);
        }
        if *edit_mode == EditMode::Resize {
            self.render_resize_overlay(graphics_context, &rect);
        }
    }

    fn render_rename_overlay(&self, graphics_context: &gfx::GraphicsContext, rect: &gfx::Rect) {
        let container =
            gfx::Container::new(graphics_context, rect.pos, rect.size, gfx::TOP_LEFT, None);
        self.name_sprite.render(graphics_context, Some(&container));
    }

    fn render_resize_overlay(&self, graphics_context: &gfx::GraphicsContext, rect: &gfx::Rect) {
        let container =
            gfx::Container::new(graphics_context, rect.pos, rect.size, gfx::TOP_LEFT, None);
        if self.split_orientation == SplitType::Horizontal {
            self.vertical_arrow_sprite
                .render(graphics_context, Some(&container));
        } else {
            self.horizontal_arrow_sprite
                .render(graphics_context, Some(&container));
        }
    }

    fn update_texture(&mut self, graphics_context: &gfx::GraphicsContext, text: &str) {
        let text_surface = &graphics_context.font.create_text_surface(text, None);
        self.name_sprite.texture = gfx::Texture::load_from_surface(text_surface);
    }
}
