use crate::libraries::graphics as gfx;
use crate::libraries::ui::{approach, BaseUiElement, Container, RenderRect, Scrollable, UiContext, BLUR, BOTTOM, SHADOW_INTENSITY, SPACING, TOP, TRANSPARENCY};

/// One entry in a `ListPage`. The page owns the arrangement, so a row says how tall it is and
/// takes the position it is given; what it contains is the caller's.
pub trait ListRow: BaseUiElement {
    /// How tall this row is. Rows may differ.
    fn get_row_height(&self) -> f32;

    /// Where the page has decided this row goes, relative to the page.
    fn set_row_pos(&mut self, pos: gfx::FloatPos);

    /// Whether the row responds to the mouse. The page says `false` while the pointer is over
    /// a bar, so a row scrolled under one does not light up through it.
    fn set_row_enabled(&mut self, enabled: bool) {
        let _ = enabled;
    }
}

/// A full-screen page holding a scrolling list of rows between a title bar and a button bar.
///
/// It owns the two bars, the scrollable and the arrangement: rows stacked from the top with
/// `SPACING` between, a scroll extent following their total height, and a top bar that fades
/// in once there is something under it to separate. What goes *in* the bars and rows is not in
/// scope - the caller owns those and lists them in its own `get_sub_elements`.
pub struct ListPage {
    /// The bar behind the title. Fades in as the list scrolls under it.
    pub top_rect: RenderRect,
    /// The bar behind the buttons. Shown only when the list is long enough to scroll.
    pub bottom_rect: RenderRect,
    pub scrollable: Scrollable,
    /// How far the top bar has faded in, 0 to 1.
    top_rect_visibility: f32,
}

impl ListPage {
    /// A page `width` wide, with bars of the given heights.
    #[must_use]
    pub fn new(width: f32, top_height: f32, bottom_height: f32) -> Self {
        let mut top_rect = RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, top_height));
        top_rect.orientation = TOP;

        let mut bottom_rect = RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, bottom_height));
        bottom_rect.fill_color.a = TRANSPARENCY / 2;
        bottom_rect.shadow_intensity = SHADOW_INTENSITY;
        bottom_rect.blur_radius = BLUR;
        bottom_rect.orientation = BOTTOM;

        let mut scrollable = Scrollable::new();
        scrollable.rect.pos.1 = SPACING;
        scrollable.rect.size.0 = width;
        scrollable.scroll_smooth_factor = 100.0;
        scrollable.boundary_smooth_factor = 40.0;
        scrollable.orientation = TOP;

        Self {
            top_rect,
            bottom_rect,
            scrollable,
            top_rect_visibility: 0.0,
        }
    }

    /// Whether the list is long enough to scroll, which is when the bottom bar earns its place.
    #[must_use]
    pub fn is_scrollable(&self) -> bool {
        self.scrollable.scroll_size > self.scrollable.rect.size.1
    }

    /// Whether the top bar is drawing anything yet. Fully faded out, it is not worth a blur.
    #[must_use]
    pub fn is_top_rect_visible(&self) -> bool {
        self.top_rect_visibility > 0.0
    }

    /// Lays the rows out, sizes the bars and the scroll extent, and fades the top bar. Call it
    /// once per frame from the owner's `update_inner`, before anything reads a row's position.
    pub fn update(&mut self, graphics: &dyn UiContext, parent_container: &Container, rows: &mut [&mut dyn ListRow]) {
        // `get_scroll_y` already carries the scrollable's own `SPACING` offset
        let mut current_y = self.scrollable.get_scroll_y() + self.top_rect.size.1;
        let mut total_height = 0.0;
        for row in rows.iter_mut() {
            row.set_row_pos(gfx::FloatPos(0.0, current_y));
            let height = row.get_row_height();
            current_y += height + SPACING;
            total_height += height + SPACING;
        }
        // the gap after the last row is not part of the list
        self.scrollable.scroll_size = (total_height - SPACING).max(0.0);

        // A row under a bar must not light up through it, so the whole list stops hovering
        // while the pointer is over either one.
        let mouse_y = graphics.get_mouse_pos().1;
        let hoverable = mouse_y > self.top_rect.size.1 && mouse_y < graphics.get_window_size().1 - self.bottom_rect.size.1;
        for row in rows {
            row.set_row_enabled(hoverable);
        }

        self.top_rect.size.0 = parent_container.get_absolute_rect().size.0;
        self.bottom_rect.size.0 = parent_container.get_absolute_rect().size.0;
        self.scrollable.rect.size.1 = graphics.get_window_size().1 - self.top_rect.size.1 - self.bottom_rect.size.1;

        // 5 pixels rather than 0, so a list resting at the top does not flicker the bar in and
        // out as the scroll settles.
        let visible_target = if self.scrollable.get_scroll_pos() > 5.0 { 1.0 } else { 0.0 };
        self.top_rect_visibility = approach(self.top_rect_visibility, visible_target, 20.0, 0.01);

        self.top_rect.fill_color.a = (self.top_rect_visibility * f32::from(TRANSPARENCY) / 2.0) as u8;
        self.top_rect.blur_radius = (self.top_rect_visibility * BLUR as f32) as i32;
        self.top_rect.shadow_intensity = (self.top_rect_visibility * SHADOW_INTENSITY as f32) as i32;
    }
}
