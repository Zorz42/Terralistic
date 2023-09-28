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
    fn get_node_by_name_mut(&mut self, name: &str) -> Option<&mut Self> {
        match self {
            Self::Split(split_node) => {
                return if let Some(node) = split_node.first.get_node_by_name_mut(name) {
                    Some(node)
                } else {
                    split_node.second.get_node_by_name_mut(name)
                }
            }
            Self::Module(mod_name) => {
                if mod_name == name {
                    return Some(self);
                }
            }
            _ => {}
        }
        None
    }
}

/// This enum indicates the type of split. `Vertical` splits the window area of that node into 2 areas, one on the left and one on the right. `Horizontal` splits the window area of that node into 2 areas, one on the top and one on the bottom.
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
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

pub struct ModuleManager {
    root: ModuleTreeNodeType,
    path: Vec<bool>,
    //max depth of 5
    depth: usize,
    rect: gfx::Rect,
    pub changed: bool,
    key_buffer: Vec<gfx::Key>,
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
            key_buffer: Vec::new(),
        }
    }
}

impl ModuleManager {
    #[allow(dead_code)]
    pub const fn new(root: ModuleTreeNodeType) -> Self {
        Self {
            root,
            path: Vec::new(),
            depth: 0,
            rect: gfx::Rect {
                pos: gfx::FloatPos(0.0, 0.0),
                size: gfx::FloatSize(1.0, 1.0),
            },
            changed: false,
            key_buffer: Vec::new(),
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

        Ok(Self {
            root,
            path: Vec::new(),
            depth: 0,
            rect: gfx::Rect {
                pos: gfx::FloatPos(0.0, 0.0),
                size: gfx::FloatSize(1.0, 1.0),
            },
            changed: false,
            key_buffer: Vec::new(),
        })
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
            first: ModuleTreeNodeType::Module("ServerInfo".to_owned()),
            second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: ModuleTreeNodeType::Module("PlayerList".to_owned()),
                second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                    orientation: SplitType::Horizontal,
                    split_pos: 0.2,
                    first: ModuleTreeNodeType::Module("Empty1".to_owned()),
                    second: ModuleTreeNodeType::Module("Console".to_owned()),
                })),
            })),
        }))
    }

    pub const fn get_root(&self) -> &ModuleTreeNodeType {
        &self.root
    }

    pub fn on_event(&mut self, event: &gfx::Event) {
        match event {
            gfx::Event::KeyPress(key, _repeat) => {
                //on every change update depth to min(depth, max_depth)
                match *key {
                    gfx::Key::F1 => {
                        //reset
                        self.depth = 0;
                        self.recalculate_selection_rect();
                        self.key_buffer.clear();
                    }
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
            gfx::Event::MouseScroll(_) => {
                todo!("implement resizing with mouse scroll");
            }
            _ => {}
        }
    }

    fn get_node_mut(&mut self, path: Option<&[bool]>, depth: usize) -> &mut ModuleTreeNodeType {
        if self.path.len() < depth + 1 {
            self.path.resize(depth + 1, false);
        }
        let mut node = &mut self.root;
        let path = path.unwrap_or(&self.path);
        for &path_at_depth in path.get(0..depth).unwrap_or(&[]) {
            match node {
                ModuleTreeNodeType::Split(split_node) => {
                    node = if path_at_depth {
                        &mut split_node.second
                    } else {
                        &mut split_node.first
                    };
                }
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn get_empty_name() -> String {
        let mut name = "Empty_".to_owned();
        //append a random number
        name.push_str(&rand::random::<u32>().to_string());
        name
    }

    fn replace_module_with_empty(&mut self, name: &str) {
        if let Some(node) = self.get_node_by_name_mut(name) {
            if let ModuleTreeNodeType::Module(_) = node {
                *node = ModuleTreeNodeType::Module(Self::get_empty_name());
            }
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
        {
            //isolate in a scope to avoid borrow problems
            let new_node = self.get_node_mut(Some(&flipped_path), depth); //one of the children that will replace the parent split node
            std::mem::swap(new_node, &mut temp_node);
        }

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

    fn recalculate_selection_rect(&mut self) {
        if self.path.len() < self.depth + 1 {
            self.path.resize(self.depth + 1, false);
        }
        let (coords, max_depth) = self.get_overlay_rect_coords(&self.root, 0);
        self.depth = self.depth.clamp(0, max_depth);
        self.rect.pos = coords.0;
        self.rect.size = coords.1;
    }

    pub fn render(&self, graphics_context: &mut gfx::GraphicsContext) {
        let window_size = graphics_context.renderer.get_window_size();
        let pos = gfx::FloatPos(
            window_size.0 * self.rect.pos.0,
            window_size.1 * self.rect.pos.1,
        );
        let size = gfx::FloatSize(
            window_size.0 * self.rect.size.0,
            window_size.1 * self.rect.size.1,
        );
        let rect = gfx::Rect::new(pos, size);
        rect.render(graphics_context, gfx::WHITE);
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
