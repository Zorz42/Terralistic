use std::sync::{Arc, Mutex, PoisonError};

use crate::libraries::graphics::{self as gfx, BaseUiElement, UiElement};

use super::Menu;

const PROGRESS_BAR_WIDTH: i32 = 400;
const PROGRESS_BAR_HEIGHT: i32 = 50;
const PROGRESS_BAR_Y_OFFSET: i32 = 100;

pub struct LoadingScreen {
    loading_text_sprite: gfx::Sprite,
    loading_text: Arc<Mutex<String>>,
    curr_text: String,
    loading_back_bar: gfx::RenderRect,
    loading_bar: gfx::RenderRect,
}

impl LoadingScreen {
    pub fn new(graphics: &mut gfx::GraphicsContext, loading_text: Arc<Mutex<String>>) -> Self {
        let mut loading_text_sprite = gfx::Sprite::new();
        loading_text_sprite.orientation = gfx::CENTER;
        loading_text_sprite.scale = 3.0;

        let mut curr_text = String::new();

        let mut loading_back_bar = gfx::RenderRect::new(gfx::FloatPos(0.0, PROGRESS_BAR_Y_OFFSET as f32), gfx::FloatSize(0.0, 0.0));
        loading_back_bar.orientation = gfx::CENTER;
        loading_back_bar.fill_color = gfx::BLACK;
        loading_back_bar.fill_color.a = gfx::TRANSPARENCY;
        loading_back_bar.smooth_factor = 60.0;

        let mut loading_bar = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
        loading_bar.orientation = gfx::LEFT;
        loading_bar.fill_color = gfx::LIGHT_GREY;
        loading_bar.smooth_factor = 60.0;

        Self {
            loading_text_sprite,
            loading_text,
            curr_text,
            loading_back_bar,
            loading_bar,
        }
    }
}

impl UiElement for LoadingScreen {
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.loading_text_sprite, &self.loading_back_bar, &self.loading_bar]
    }
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.loading_text_sprite, &mut self.loading_back_bar, &mut self.loading_bar]
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), parent_container.rect.size, parent_container.orientation, Some(parent_container))
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, _: &gfx::Container) {
        if self.curr_text != *self.loading_text.lock().unwrap_or_else(PoisonError::into_inner) {
            self.curr_text = self.loading_text.lock().unwrap_or_else(PoisonError::into_inner).clone();
            if !self.curr_text.is_empty() {
                let mut progress_bar_progress = -1.0;
                // let ending of the text be the back of the text until the space symbol
                let mut ending = String::new();
                let mut i = self.curr_text.len() - 1;
                while i > 0 && self.curr_text.chars().nth(i).unwrap_or(' ') != ' ' {
                    ending.insert(0, self.curr_text.chars().nth(i).unwrap_or(' '));
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
                        self.curr_text.truncate(i);

                        progress_bar_progress = num / 100.0;
                    }
                }

                if (progress_bar_progress - -1.0_f32).abs() < f32::EPSILON {
                    self.loading_back_bar.size.0 = 0.0;
                    self.loading_back_bar.size.1 = 0.0;
                    self.loading_bar.size.0 = 0.0;
                    self.loading_bar.size.1 = 0.0;
                } else {
                    self.loading_back_bar.size.0 = PROGRESS_BAR_WIDTH as f32;
                    self.loading_back_bar.size.1 = PROGRESS_BAR_HEIGHT as f32;
                    self.loading_bar.size.0 = (PROGRESS_BAR_WIDTH as f32) * progress_bar_progress;
                    self.loading_bar.size.1 = PROGRESS_BAR_HEIGHT as f32;
                }

                self.loading_text_sprite
                    .set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&self.curr_text, None)));
            }
        }
    }
}

impl Menu for LoadingScreen {
    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }

    fn should_close(&mut self) -> bool {
        self.loading_text.try_lock().map_or(false, |val| val.is_empty())
    }
}
