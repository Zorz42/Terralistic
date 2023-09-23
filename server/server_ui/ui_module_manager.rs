use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

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
    root: ModuleTreeSplit,
}

impl ModuleManager {
    #[allow(dead_code)]
    pub const fn new(root: ModuleTreeSplit) -> Self {
        Self { root }
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

        Ok(Self { root })
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
    fn default_module_tree() -> ModuleTreeSplit {
        ModuleTreeSplit {
            orientation: SplitType::Horizontal,
            split_pos: 0.1,
            first: ModuleTreeNodeType::Module("ServerInfo".to_owned()),
            second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: ModuleTreeNodeType::Module("PlayerList".to_owned()),
                second: ModuleTreeNodeType::Module("Console".to_owned()),
            })),
        }
    }

    pub const fn get_root(&self) -> &ModuleTreeSplit {
        &self.root
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        let root = Self::default_module_tree();
        Self { root }
    }
}
