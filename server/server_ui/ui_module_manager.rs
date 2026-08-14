use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};

use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::BaseUiElement;
use crate::libraries::ui::UiContext;
use crate::libraries::ui::{area_at_path, DockNode, DockSplit, SplitType};
use crate::server::server_ui::UiMessageType;

#[derive(PartialEq, Eq)]
enum EditMode {
    Select,
    Name,
    Resize,
}

pub struct ModuleManager {
    root: DockNode,
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
    pub fn new(root: DockNode) -> Self {
        Self { root, ..Self::default() }
    }

    /// Reads the module tree from the save file in `server_data/ui_config.json`. if the file doesn't exist or is not a valid format, use the default config
    pub fn from_save_file(config_path: &Path) -> Self {
        Self::try_from_save_file(config_path).unwrap_or_else(|e| {
            eprintln!("{e}");
            Self::default()
        })
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
    fn default_module_tree() -> DockNode {
        DockNode::Split(Box::from(DockSplit {
            orientation: SplitType::Horizontal,
            split_pos: 0.1,
            first: DockNode::Pane("server_info".to_owned()),
            second: DockNode::Split(Box::from(DockSplit {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: DockNode::Pane("player_list".to_owned()),
                second: DockNode::Split(Box::from(DockSplit {
                    orientation: SplitType::Horizontal,
                    split_pos: 0.2,
                    first: DockNode::Pane("empty_1".to_owned()),
                    second: DockNode::Pane("console".to_owned()),
                })),
            })),
        }))
    }

    pub const fn get_root_mut(&mut self) -> &mut DockNode {
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
                    if let DockNode::Split(split) = self.get_node(None, self.depth) {
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
                    self.renderer.update_texture(graphics_context, &self.name_buffer);
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
                *node = DockNode::Pane(name);
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

    const fn handle_resize_mode_key_events(&mut self, key: gfx::Key) {
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
            if let DockNode::Split(split) = node {
                split.split_pos += scroll * 0.01;
                split.split_pos = split.split_pos.clamp(0.0, 1.0);
                self.changed = true;
                self.recalculate_selection_rect();
            }
        }
    }

    fn get_node_mut(&mut self, path: Option<&[bool]>, depth: usize) -> &mut DockNode {
        if self.path.len() < depth + 1 {
            self.path.resize(depth + 1, false);
        }
        let path = path.unwrap_or(&self.path);
        let walked: Vec<bool> = path.get(0..depth).unwrap_or(&[]).to_vec();
        self.root.at_path_mut(&walked)
    }

    fn get_node(&self, path: Option<&[bool]>, depth: usize) -> &DockNode {
        let path = path.unwrap_or(&self.path);
        self.root.at_path(path.get(0..depth).unwrap_or(&[]))
    }

    fn get_empty_name() -> String {
        let mut name = "empty_".to_owned();
        //append a random number
        name.push_str(&rand::random::<u32>().to_string());
        name
    }

    fn replace_module_with_empty(&mut self, name: &str) {
        if let Some(node) = self.get_node_by_name_mut(name) {
            *node = DockNode::Pane(Self::get_empty_name());
        }
    }

    fn get_node_by_name_mut(&mut self, name: &str) -> Option<&mut DockNode> {
        self.root.find_pane_mut(name)
    }

    fn split(&mut self, depth: usize, orientation: SplitType, split_pos: f32) {
        if self.path.len() < depth + 1 {
            self.path.resize(depth + 1, false);
        }
        let name = Self::get_empty_name();

        let old_node = self.get_node_mut(None, depth);

        let mut new_node = DockNode::Nothing;
        std::mem::swap(&mut new_node, old_node);
        *old_node = DockNode::Split(Box::from(DockSplit {
            orientation,
            split_pos,
            first: new_node,
            second: DockNode::Pane(name),
        }));
    }

    fn delete(&mut self, depth: usize) {
        if depth == 0 {
            self.root = DockNode::Pane("Empty".to_owned());
            return;
        }
        let mut flipped_path = self.path.clone(); //looks like a stupid way to flip the last element but avoids many Option<T> cases
        if let Some(last) = flipped_path.get_mut(depth - 1) {
            *last = !*last;
        }

        let mut temp_node = DockNode::Nothing;

        let new_node = self.get_node_mut(Some(&flipped_path), depth); //one of the children that will replace the parent split node
        std::mem::swap(new_node, &mut temp_node);

        let old_node = self.get_node_mut(None, depth - 1); //the split that will be replaced by one of its children
        std::mem::swap(&mut temp_node, old_node);
    }

    fn swap(&mut self, depth: usize) {
        let node = self.get_node_mut(None, depth);
        if let DockNode::Split(split) = node {
            split.split_pos = 1.0 - split.split_pos;
            std::mem::swap(&mut split.first, &mut split.second);
        }
    }

    pub fn render_selection(&self, graphics_context: &gfx::GraphicsContext) {
        ModuleManagerRenderer::render_selection(graphics_context, &self.rect);
    }

    pub fn render_overlay(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        self.renderer.render_overlay(graphics_context, &self.rect, &self.mode);
    }

    fn recalculate_selection_rect(&mut self) {
        if self.path.len() < self.depth + 1 {
            self.path.resize(self.depth + 1, false);
        }
        let (area, reached_depth) = area_at_path(&self.root, &self.path, self.depth);
        self.depth = self.depth.clamp(0, reached_depth);
        self.rect.pos = area.pos;
        self.rect.size = area.size;
    }
}

struct ModuleManagerRenderer {
    name_sprite: ui::Sprite,
    vertical_arrow_sprite: ui::Sprite,
    horizontal_arrow_sprite: ui::Sprite,
    split_orientation: SplitType,
}

impl ModuleManagerRenderer {
    fn new() -> Self {
        let mut name_sprite = ui::Sprite::new();
        name_sprite.orientation = ui::CENTER;
        name_sprite.scale = 3.0;

        let mut vertical_arrow_sprite = ui::Sprite::new();
        vertical_arrow_sprite.set_texture(gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/vertical_resize_arrow.opa")));
        vertical_arrow_sprite.orientation = ui::CENTER;
        vertical_arrow_sprite.scale = 4.0;

        let mut horizontal_arrow_sprite = ui::Sprite::new();
        horizontal_arrow_sprite.set_texture(gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/horizontal_resize_arrow.opa")));
        horizontal_arrow_sprite.orientation = ui::CENTER;
        horizontal_arrow_sprite.scale = 4.0;

        Self {
            name_sprite,
            vertical_arrow_sprite,
            horizontal_arrow_sprite,
            split_orientation: SplitType::Horizontal,
        }
    }

    fn get_pos_size(rect: &gfx::Rect, graphics_context: &gfx::GraphicsContext) -> gfx::Rect {
        let window_size = graphics_context.get_window_size();
        let pos = gfx::FloatPos(window_size.0 * rect.pos.0, window_size.1 * rect.pos.1);
        let size = gfx::FloatSize(window_size.0 * rect.size.0, window_size.1 * rect.size.1);
        gfx::Rect::new(pos, size)
    }

    fn render_selection(graphics_context: &gfx::GraphicsContext, fraction_rect: &gfx::Rect) {
        let rect = Self::get_pos_size(fraction_rect, graphics_context);
        rect.render(graphics_context, ui::WHITE);
    }

    fn render_overlay(&mut self, graphics_context: &mut gfx::GraphicsContext, fraction_rect: &gfx::Rect, edit_mode: &EditMode) {
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

    fn render_rename_overlay(&mut self, graphics_context: &mut gfx::GraphicsContext, rect: &gfx::Rect) {
        let container = ui::Container::new(graphics_context, rect.pos, rect.size, ui::TOP_LEFT, None);
        self.name_sprite.render(graphics_context, &container);
    }

    fn render_resize_overlay(&mut self, graphics_context: &mut gfx::GraphicsContext, rect: &gfx::Rect) {
        let container = ui::Container::new(graphics_context, rect.pos, rect.size, ui::TOP_LEFT, None);
        if self.split_orientation == SplitType::Horizontal {
            self.vertical_arrow_sprite.render(graphics_context, &container);
        } else {
            self.horizontal_arrow_sprite.render(graphics_context, &container);
        }
    }

    fn update_texture(&mut self, graphics_context: &gfx::GraphicsContext, text: &str) {
        let text_surface = &graphics_context.font.create_text_surface(text, None);
        self.name_sprite.set_texture(gfx::Texture::load_from_surface(text_surface));
    }
}
