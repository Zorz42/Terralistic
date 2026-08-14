use serde_derive::{Deserialize, Serialize};

use crate::libraries::graphics as gfx;

/// Which way a `DockSplit` divides its area.
///
/// `Vertical` puts `first` on the left and `second` on the right; `Horizontal` puts `first`
/// on top and `second` below.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum SplitType {
    Vertical,
    Horizontal,
}

/// A node in a dock layout: nothing, a split into two more nodes, or a named pane.
///
/// The name is all this layer knows about a pane. What is drawn there is looked up by the
/// owner, which is what keeps this a layout and not a widget container.
#[derive(Serialize, Deserialize, Debug)]
pub enum DockNode {
    /// An empty area. Ideally unused - a split into a pane and nothing should just be the
    /// pane - but it is what an area being edited passes through.
    Nothing,
    Split(Box<DockSplit>),
    Pane(String),
}

impl DockNode {
    /// Finds the pane with this name, anywhere in the tree.
    #[must_use]
    pub fn find_pane_mut(&mut self, name: &str) -> Option<&mut Self> {
        match self {
            Self::Split(split) => split.first.find_pane_mut(name).or_else(|| split.second.find_pane_mut(name)),
            Self::Pane(pane_name) if pane_name == name => Some(self),
            Self::Pane(_) | Self::Nothing => None,
        }
    }

    /// Walks `path` from this node, one bool per level: false takes `first`, true takes
    /// `second`. Stops early at anything that is not a split, which is what makes a path
    /// longer than the tree harmless.
    #[must_use]
    pub fn at_path(&self, path: &[bool]) -> &Self {
        let mut node = self;
        for &second in path {
            let Self::Split(split) = node else { break };
            node = if second { &split.second } else { &split.first };
        }
        node
    }

    /// `at_path`, mutably.
    #[must_use]
    pub fn at_path_mut(&mut self, path: &[bool]) -> &mut Self {
        let mut node = self;
        for &second in path {
            let Self::Split(split) = node else { break };
            node = if second { &mut split.second } else { &mut split.first };
        }
        node
    }
}

/// A node divided in two.
///
/// `split_pos` runs 0 to 1 and is the fraction of the area given to `first`; the rest goes
/// to `second`.
#[derive(Serialize, Deserialize, Debug)]
pub struct DockSplit {
    pub orientation: SplitType,
    pub split_pos: f32,
    pub first: DockNode,
    pub second: DockNode,
}

/// Where a node sits inside its dock, as fractions of the whole area.
///
/// Fractions rather than pixels, so a layout survives the window being resized and can be
/// saved without recording the size it was saved at.
///
/// No `Eq`: these are fractions, and two areas that describe the same region can differ in
/// the last bit after a few nestings.
#[allow(clippy::derive_partial_eq_without_eq, reason = "fractions do not compare exactly")]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DockArea {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
}

impl DockArea {
    /// The whole dock.
    #[must_use]
    pub const fn whole() -> Self {
        Self {
            pos: gfx::FloatPos(0.0, 0.0),
            size: gfx::FloatSize(1.0, 1.0),
        }
    }

    /// This area's own subdivision, expressed in the parent's fractions.
    ///
    /// Nesting is a multiply and an offset rather than a second coordinate system, which is
    /// what lets a path of any depth resolve by folding this along it.
    #[must_use]
    pub fn nest(self, inner: Self) -> Self {
        Self {
            pos: gfx::FloatPos(self.pos.0 + inner.pos.0 * self.size.0, self.pos.1 + inner.pos.1 * self.size.1),
            size: gfx::FloatSize(self.size.0 * inner.size.0, self.size.1 * inner.size.1),
        }
    }

    /// The half of a split this area is, given which side and which way it divides.
    #[must_use]
    pub fn half(orientation: SplitType, split_pos: f32, second: bool) -> Self {
        match (orientation, second) {
            (SplitType::Vertical, false) => Self {
                pos: gfx::FloatPos(0.0, 0.0),
                size: gfx::FloatSize(split_pos, 1.0),
            },
            (SplitType::Vertical, true) => Self {
                pos: gfx::FloatPos(split_pos, 0.0),
                size: gfx::FloatSize(1.0 - split_pos, 1.0),
            },
            (SplitType::Horizontal, false) => Self {
                pos: gfx::FloatPos(0.0, 0.0),
                size: gfx::FloatSize(1.0, split_pos),
            },
            (SplitType::Horizontal, true) => Self {
                pos: gfx::FloatPos(0.0, split_pos),
                size: gfx::FloatSize(1.0, 1.0 - split_pos),
            },
        }
    }

    /// Turns fractions into pixels inside a rectangle of `size`.
    #[must_use]
    pub fn to_rect(self, size: gfx::FloatSize) -> gfx::Rect {
        gfx::Rect::new(gfx::FloatPos(self.pos.0 * size.0, self.pos.1 * size.1), gfx::FloatSize(self.size.0 * size.0, self.size.1 * size.1))
    }
}

/// The area a path leads to, and how deep it actually got.
///
/// The depth comes back because a path can be longer than the tree - the node it names may
/// not be a split - and the caller usually wants to know where it really landed.
#[must_use]
pub fn area_at_path(root: &DockNode, path: &[bool], max_depth: usize) -> (DockArea, usize) {
    fn walk(node: &DockNode, path: &[bool], depth: usize, max_depth: usize) -> (DockArea, usize) {
        if depth == max_depth {
            return (DockArea::whole(), depth);
        }

        let DockNode::Split(split) = node else {
            return (DockArea::whole(), depth);
        };

        let second = *path.get(depth).unwrap_or(&false);
        let here = DockArea::half(split.orientation, split.split_pos, second);
        let sub_node = if second { &split.second } else { &split.first };
        let (inner, reached) = walk(sub_node, path, depth + 1, max_depth);

        (here.nest(inner), reached)
    }

    walk(root, path, 0, max_depth)
}
