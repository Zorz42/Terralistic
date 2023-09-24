use crate::libraries::graphics as gfx;
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use crate::libraries::graphics::{FloatPos, FloatSize};

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
    path: [bool; 5],//max depth of 5
    depth: usize,
    rect_transform: (FloatPos, FloatSize),
}

impl ModuleManager {
    #[allow(dead_code)]
    pub const fn new(root: ModuleTreeNodeType) -> Self {
        Self { root, path: [false; 5], depth: 0, rect_transform: (FloatPos(0.0, 0.0), FloatSize(1.0, 1.0))}
    }

    #[allow(dead_code)]
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

        Ok(Self { root, path: [false; 5], depth: 0, rect_transform: (FloatPos(0.0, 0.0), FloatSize(1.0, 1.0)) })
    }

    #[allow(dead_code)]
    pub fn save_to_file(&self, config_path: &Path) -> Result<()> {
        let mut file = File::create(config_path.join("ui_config.json"))?;
        let json_str = serde_json::to_string_pretty(&self.root)?;
        let res = file.write(json_str.as_bytes());
        if let Err(err) = res {
            println!("Failed to write ui_config.json: {err}");
        }

        Ok(())
    }

    /// Creates the default module tree and returns it
    fn default_module_tree() -> ModuleTreeNodeType {
        ModuleTreeNodeType::Split(Box::from(
            ModuleTreeSplit {
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
                        first: ModuleTreeNodeType::Module("Empty".to_owned()),
                        second: ModuleTreeNodeType::Module("Console".to_owned()),
                    })),
                })),
            }
        ))
    }

    pub const fn get_root(&self) -> &ModuleTreeNodeType {
        &self.root
    }

    pub fn on_event(&mut self, event: &gfx::Event) {
        match event {
            gfx::Event::KeyPress(key, _repeat) => {//on every change update depth to min(depth, max_depth)
                if *key == gfx::Key::V {
                    todo!()
                }
                if *key == gfx::Key::F1 {
                    self.depth = 0;
                    let (coords, _) = self.get_overlay_rect_coords(&self.root, 0);
                    self.rect_transform = coords;
                }
            }
            gfx::Event::MouseScroll(_) => {
                todo!()
            }
            _ => {}
        }
    }

    pub fn render(&self, graphics_context: &mut gfx::GraphicsContext) {
        let window_size = graphics_context.renderer.get_window_size();
        let pos = FloatPos(window_size.0 * self.rect_transform.0.0, window_size.1 * self.rect_transform.0.1);
        let size = FloatSize(window_size.0 * self.rect_transform.1.0, window_size.1 * self.rect_transform.1.1);
        let rect = gfx::Rect::new(pos, size);
        rect.render(graphics_context, gfx::WHITE);
    }

    fn get_overlay_rect_coords(&self, node: &ModuleTreeNodeType, depth: usize) -> ((FloatPos, FloatSize), usize) {
        if depth == self.depth {
            return ((FloatPos(0.0, 0.0), FloatSize(1.0, 1.0)), depth);
        }
        match node {
            ModuleTreeNodeType::Split(node) => {
                match node.orientation {
                    SplitType::Vertical => {
                        let split_factors = self.get_vertical_split_factors(node, depth);
                        let sub_node = if *self.path.get(depth).unwrap_or(&false) { &node.second } else { &node.first };
                        let (sub_node_factors, max_depth) = self.get_overlay_rect_coords(sub_node, depth + 1);

                        (Self::calculate_factors(split_factors, sub_node_factors), max_depth)
                    }
                    SplitType::Horizontal => {
                        let split_factors = self.get_horizontal_split_factors(node, depth);
                        let sub_node = if *self.path.get(depth).unwrap_or(&false) { &node.second } else { &node.first };
                        let (sub_node_factors, max_depth) = self.get_overlay_rect_coords(sub_node, depth + 1);

                        (Self::calculate_factors(split_factors, sub_node_factors), max_depth)
                    }
                }
            },
            _ => {
                ((FloatPos(0.0, 0.0), FloatSize(1.0, 1.0)), depth)
            },
        }
    }
    fn get_vertical_split_factors(&self, node: &ModuleTreeSplit, depth: usize) -> (FloatPos, FloatSize) {
        if *self.path.get(depth).unwrap_or(&false) {
            (FloatPos(node.split_pos, 0.0), FloatSize(1.0 - node.split_pos, 1.0))
        } else {
            (FloatPos(0.0, 0.0), FloatSize(node.split_pos, 1.0))
        }
    }

    fn get_horizontal_split_factors(&self, node: &ModuleTreeSplit, depth: usize) -> (FloatPos, FloatSize) {
        if *self.path.get(depth).unwrap_or(&false) {
            (FloatPos(0.0, node.split_pos), FloatSize(1.0, 1.0 - node.split_pos))
        } else {
            (FloatPos(0.0, 0.0), FloatSize(1.0, node.split_pos))
        }
    }

    fn calculate_factors(split_factors: (FloatPos, FloatSize), sub_node_factors: (FloatPos, FloatSize)) -> (FloatPos, FloatSize) {
        (
            FloatPos(split_factors.0.0 + sub_node_factors.0.0 * split_factors.1.0, split_factors.0.1 + sub_node_factors.0.1 * split_factors.1.1),
            FloatSize(split_factors.1.0 * sub_node_factors.1.0, split_factors.1.1 * sub_node_factors.1.1)
        )
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        let root = Self::default_module_tree();
        Self { root, path: [false; 5], depth: 0, rect_transform: (FloatPos(0.0, 0.0), FloatSize(1.0, 1.0)) }
    }
}
