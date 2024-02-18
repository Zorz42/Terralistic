use std::sync::{Mutex, PoisonError};

use crate::libraries::graphics as gfx;

use super::background_rect::BackgroundRect;
use gfx::UiElement;

const PROGRESS_BAR_WIDTH: i32 = 400;
const PROGRESS_BAR_HEIGHT: i32 = 50;
const PROGRESS_BAR_Y_OFFSET: i32 = 100;

/// Loading screen displays `back_menu` and text which describes the current loading state.
/// The text is shared through the `SharedMut`<String> which is updated by the loading thread.
/// When the string is empty, the loading screen is closed.
pub fn run_loading_screen(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect, loading_text: &Mutex<String>) {
    let mut loading_text_sprite = gfx::Sprite::new();
    loading_text_sprite.orientation = gfx::CENTER;
    loading_text_sprite.scale = 3.0;

    let mut curr_text = String::new();

    let mut loading_back_bar = gfx::RenderRect::new(gfx::FloatPos(0.0, PROGRESS_BAR_Y_OFFSET as f32), gfx::FloatSize(0.0, 0.0));
    loading_back_bar.orientation = gfx::CENTER;
    loading_back_bar.fill_color = gfx::BLACK;
    loading_back_bar.fill_color.a = gfx::TRANSPARENCY;
    loading_back_bar.smooth_factor = 60.0;

    let mut loading_bar = gfx::RenderRect::new(gfx::FloatPos(0.0, PROGRESS_BAR_Y_OFFSET as f32), gfx::FloatSize(0.0, 0.0));
    loading_bar.orientation = gfx::LEFT;
    loading_bar.fill_color = gfx::LIGHT_GREY;
    loading_bar.smooth_factor = 60.0;

    while graphics.is_window_open() && !loading_text.lock().unwrap_or_else(PoisonError::into_inner).is_empty() {
        while graphics.get_event().is_some() {}

        menu_back.set_back_rect_width(PROGRESS_BAR_WIDTH as f32 + 2.0 * gfx::SPACING);

        menu_back.render_back(graphics);

        if curr_text != *loading_text.lock().unwrap_or_else(PoisonError::into_inner) {
            curr_text = loading_text.lock().unwrap_or_else(PoisonError::into_inner).clone();
            if !curr_text.is_empty() {
                let mut progress_bar_progress = -1.0;
                // let ending of the text be the back of the text until the space symbol
                let mut ending = String::new();
                let mut i = curr_text.len() - 1;
                while i > 0 && curr_text.chars().nth(i).unwrap_or(' ') != ' ' {
                    ending.insert(0, curr_text.chars().nth(i).unwrap_or(' '));
                    i -= 1;
                }
                // if the ending is in the format {some number}%, then remove it from the text and display it as a progress bar
                // if the text before % is not a number, then don't display it as a progress bar
                if ending.ends_with('%') {
                    ending.remove(ending.len() - 1);
                    // check if the ending is a number
                    let num = ending.parse::<f32>();
                    if let Ok(num) = num {
                        // remove the ending from the text
                        curr_text.truncate(i);

                        progress_bar_progress = num / 100.0;
                    }
                }

                if (progress_bar_progress - -1.0_f32).abs() < f32::EPSILON {
                    loading_back_bar.size.0 = 0.0;
                    loading_back_bar.size.1 = 0.0;
                    loading_bar.size.0 = 0.0;
                    loading_bar.size.1 = 0.0;
                    loading_bar.pos.0 = graphics.get_window_size().0 / 2.0;
                } else {
                    loading_back_bar.size.0 = PROGRESS_BAR_WIDTH as f32;
                    loading_back_bar.size.1 = PROGRESS_BAR_HEIGHT as f32;
                    loading_bar.size.0 = (PROGRESS_BAR_WIDTH as f32) * progress_bar_progress;
                    loading_bar.size.1 = PROGRESS_BAR_HEIGHT as f32;
                    loading_bar.pos.0 = menu_back.get_back_rect_width(graphics, None) / 2.0 - loading_back_bar.size.0 / 2.0;
                }

                loading_text_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&curr_text, None));
            }
        }

        loading_text_sprite.render(graphics, Some(menu_back.get_back_rect_container()), None);

        loading_back_bar.render(graphics, Some(menu_back.get_back_rect_container()));
        loading_bar.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.update_window();
    }
}
