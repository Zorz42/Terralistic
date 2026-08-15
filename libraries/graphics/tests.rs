#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![allow(clippy::panic)] // a test asserting on the shape of a value has nothing else to say when it is the wrong shape
#![cfg(test)]
mod tests {
    use crate::libraries::graphics as gfx;
    use crate::libraries::graphics::transformation::Transformation;
    use crate::libraries::graphics::{interpolate_colors, Color, FloatPos, FloatSize, IntPos, IntSize, Rect, Surface};

    // --- Color ---

    #[test]
    fn test_set_a_leaves_the_colour_it_was_called_on_alone() {
        let color = Color::new(1, 2, 3, 4);

        assert_eq!(color.set_a(40), Color::new(1, 2, 3, 40));
        // it takes self by value, so the original is untouched
        assert_eq!(color, Color::new(1, 2, 3, 4));
    }

    #[test]
    fn test_interpolate_colors_at_the_ends() {
        let a = Color::new(0, 0, 0, 0);
        let b = Color::new(200, 100, 50, 255);

        assert_eq!(interpolate_colors(a, b, 0.0), a);
        assert_eq!(interpolate_colors(a, b, 1.0), b);
    }

    #[test]
    fn test_interpolate_colors_midpoint() {
        let a = Color::new(0, 0, 0, 0);
        let b = Color::new(200, 100, 50, 254);

        assert_eq!(interpolate_colors(a, b, 0.5), Color::new(100, 50, 25, 127));
    }

    // --- position and size types ---

    #[test]
    fn test_int_pos_arithmetic() {
        assert_eq!(IntPos(3, 4) + IntPos(1, 2), IntPos(4, 6));
        assert_eq!(IntPos(3, 4) - IntPos(1, 2), IntPos(2, 2));
        // a size can be added to a position
        assert_eq!(IntPos(3, 4) + IntSize(1, 2), IntPos(4, 6));
        assert_eq!(IntPos(3, 4) - IntSize(1, 2), IntPos(2, 2));
    }

    #[test]
    fn test_int_pos_goes_negative() {
        assert_eq!(IntPos(0, 0) - IntPos(5, 7), IntPos(-5, -7));
    }

    #[test]
    fn test_float_pos_arithmetic() {
        assert_eq!(FloatPos(3.0, 4.0) + FloatPos(1.0, 2.0), FloatPos(4.0, 6.0));
        assert_eq!(FloatPos(3.0, 4.0) - FloatPos(1.0, 2.0), FloatPos(2.0, 2.0));
        assert_eq!(FloatPos(3.0, 4.0) + FloatSize(1.0, 2.0), FloatPos(4.0, 6.0));
        assert_eq!(FloatPos(3.0, 4.0) - FloatSize(1.0, 2.0), FloatPos(2.0, 2.0));
    }

    #[test]
    fn test_size_arithmetic() {
        assert_eq!(IntSize(3, 4) + IntSize(1, 2), IntSize(4, 6));
        assert_eq!(IntSize(3, 4) - IntSize(1, 2), IntSize(2, 2));
        assert_eq!(FloatSize(3.0, 4.0) + FloatSize(1.0, 2.0), FloatSize(4.0, 6.0));
    }

    /// Float positions compare with a tolerance rather than exactly, so values that differ
    /// far below a pixel are the same position.
    #[test]
    fn test_float_pos_equality_is_approximate() {
        assert_eq!(FloatPos(1.0, 1.0), FloatPos(1.000_01, 1.000_01));
        assert_ne!(FloatPos(1.0, 1.0), FloatPos(1.01, 1.0));

        assert_eq!(FloatSize(1.0, 1.0), FloatSize(1.000_01, 1.000_01));
        assert_ne!(FloatSize(1.0, 1.0), FloatSize(1.01, 1.0));
    }

    #[test]
    fn test_position_conversions_truncate() {
        assert_eq!(IntPos::from(FloatPos(3.9, -1.9)), IntPos(3, -1));
        assert_eq!(FloatPos::from(IntPos(3, -1)), FloatPos(3.0, -1.0));
        assert_eq!(IntSize::from(FloatSize(3.9, 1.2)), IntSize(3, 1));
        assert_eq!(FloatSize::from(IntSize(3, 1)), FloatSize(3.0, 1.0));
    }

    // --- Rect ---

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(100.0, 50.0));

        assert!(rect.contains(FloatPos(50.0, 40.0)), "a point inside should be contained");
        // the bounds are inclusive on both edges
        assert!(rect.contains(FloatPos(10.0, 20.0)));
        assert!(rect.contains(FloatPos(110.0, 70.0)));

        assert!(!rect.contains(FloatPos(9.0, 40.0)));
        assert!(!rect.contains(FloatPos(111.0, 40.0)));
        assert!(!rect.contains(FloatPos(50.0, 19.0)));
        assert!(!rect.contains(FloatPos(50.0, 71.0)));
    }

    #[test]
    fn test_zero_sized_rect_contains_only_its_corner() {
        let rect = Rect::new(FloatPos(5.0, 5.0), FloatSize(0.0, 0.0));
        assert!(rect.contains(FloatPos(5.0, 5.0)));
        assert!(!rect.contains(FloatPos(5.1, 5.0)));
    }

    // --- Surface ---

    #[test]
    fn test_new_surface_is_transparent() {
        let surface = Surface::new(IntSize(4, 3));
        assert_eq!(surface.get_size(), IntSize(4, 3));

        for x in 0..4 {
            for y in 0..3 {
                assert_eq!(*surface.get_pixel(IntPos(x, y)).unwrap(), Color::new(0, 0, 0, 0));
            }
        }
    }

    #[test]
    fn test_surface_pixel_out_of_bounds() {
        let mut surface = Surface::new(IntSize(4, 3));

        surface.get_pixel(IntPos(4, 0)).unwrap_err();
        surface.get_pixel(IntPos(0, 3)).unwrap_err();
        surface.get_pixel(IntPos(-1, 0)).unwrap_err();
        surface.get_pixel(IntPos(0, -1)).unwrap_err();
        surface.get_pixel_mut(IntPos(9, 9)).unwrap_err();
    }

    /// Pixels are stored row by row, so writing one does not disturb its neighbours.
    #[test]
    fn test_surface_set_pixel() {
        let mut surface = Surface::new(IntSize(4, 3));
        *surface.get_pixel_mut(IntPos(1, 2)).unwrap() = Color::new(1, 2, 3, 4);

        assert_eq!(*surface.get_pixel(IntPos(1, 2)).unwrap(), Color::new(1, 2, 3, 4));
        assert_eq!(*surface.get_pixel(IntPos(2, 2)).unwrap(), Color::new(0, 0, 0, 0));
        assert_eq!(*surface.get_pixel(IntPos(1, 1)).unwrap(), Color::new(0, 0, 0, 0));
    }

    #[test]
    fn test_surface_iterates_every_pixel_row_by_row() {
        let surface = Surface::new(IntSize(3, 2));
        let visited: Vec<IntPos> = surface.iter().map(|(pos, _)| pos).collect();

        assert_eq!(visited, vec![IntPos(0, 0), IntPos(1, 0), IntPos(2, 0), IntPos(0, 1), IntPos(1, 1), IntPos(2, 1)]);
    }

    /// `draw` copies a surface in at an offset, multiplying by the given colour.
    #[test]
    fn test_surface_draw_copies_and_tints() {
        let mut target = Surface::new(IntSize(4, 4));
        let mut source = Surface::new(IntSize(2, 2));
        for (_pos, color) in source.iter_mut() {
            *color = Color::new(200, 100, 50, 255);
        }

        // white leaves the colours alone
        target.draw(IntPos(1, 1), &source, Color::new(255, 255, 255, 255)).unwrap();
        assert_eq!(*target.get_pixel(IntPos(1, 1)).unwrap(), Color::new(200, 100, 50, 255));
        assert_eq!(*target.get_pixel(IntPos(2, 2)).unwrap(), Color::new(200, 100, 50, 255));
        // and does not touch pixels outside the copied area
        assert_eq!(*target.get_pixel(IntPos(0, 0)).unwrap(), Color::new(0, 0, 0, 0));
        assert_eq!(*target.get_pixel(IntPos(3, 3)).unwrap(), Color::new(0, 0, 0, 0));
    }

    #[test]
    fn test_surface_draw_out_of_bounds_is_an_error() {
        let mut target = Surface::new(IntSize(2, 2));
        let source = Surface::new(IntSize(2, 2));

        target.draw(IntPos(1, 1), &source, Color::new(255, 255, 255, 255)).unwrap_err();
    }

    /// This round trip is the `.opa` format, which the build script writes and the game
    /// reads back through `include_bytes!`.
    #[test]
    fn test_surface_serialize_round_trip() {
        let mut surface = Surface::new(IntSize(5, 4));
        *surface.get_pixel_mut(IntPos(0, 0)).unwrap() = Color::new(255, 0, 0, 255);
        *surface.get_pixel_mut(IntPos(4, 3)).unwrap() = Color::new(0, 255, 128, 64);

        let bytes = surface.serialize_to_bytes().unwrap();
        let restored = Surface::deserialize_from_bytes(&bytes).unwrap();

        assert_eq!(restored.get_size(), surface.get_size());
        for (pos, color) in surface.iter() {
            assert_eq!(*restored.get_pixel(pos).unwrap(), *color, "pixel {pos:?} differs");
        }
    }

    #[test]
    fn test_surface_deserialize_rejects_garbage() {
        assert!(Surface::deserialize_from_bytes(&[1, 2, 3, 4, 5]).is_err());
    }

    /// A surface's serialized form is its pixels and its size with nothing tying the two
    /// together, so the bytes can describe a 64x64 image holding four pixels. Everything
    /// downstream takes `get_size` at its word - `GpuDevice::create_texture` hands wgpu the
    /// short buffer, which is a validation error and so a panic. Surfaces come out of `.mod`
    /// files, so this has to be caught at the door.
    #[test]
    fn test_a_surface_that_lies_about_its_size_is_rejected() {
        /// The same shape as `Surface`, which is all postcard encodes: fields in order, no names.
        #[derive(serde_derive::Serialize)]
        struct MismatchedSurface {
            pixels: Vec<Color>,
            size: IntSize,
        }

        let bytes = crate::libraries::serialization::serialize(&MismatchedSurface {
            pixels: std::vec![Color::new(1, 2, 3, 4); 4],
            size: IntSize(64, 64),
        })
        .unwrap();

        assert!(Surface::deserialize_from_bytes(&snap::raw::Encoder::new().compress_vec(&bytes).unwrap()).is_err());
    }

    /// The other direction, and the reason the check is an equality rather than a lower bound:
    /// a surface with pixels to spare would upload fine and then be indexed by a size that does
    /// not reach them.
    #[test]
    fn test_a_surface_with_pixels_to_spare_is_rejected_too() {
        #[derive(serde_derive::Serialize)]
        struct MismatchedSurface {
            pixels: Vec<Color>,
            size: IntSize,
        }

        let bytes = crate::libraries::serialization::serialize(&MismatchedSurface {
            pixels: std::vec![Color::new(1, 2, 3, 4); 100],
            size: IntSize(2, 2),
        })
        .unwrap();

        assert!(Surface::deserialize_from_bytes(&snap::raw::Encoder::new().compress_vec(&bytes).unwrap()).is_err());
    }

    // --- Transformation ---

    #[test]
    fn test_transformation_starts_as_identity() {
        let transform = Transformation::new();
        let identity = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (actual, expected) in transform.matrix.iter().zip(identity.iter()) {
            assert!((actual - expected).abs() < f32::EPSILON, "expected identity, got {:?}", transform.matrix);
        }
    }

    #[test]
    fn test_transformation_translate() {
        let mut transform = Transformation::new();
        transform.translate(FloatPos(3.0, 5.0));

        assert!((transform.matrix[6] - 3.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_transformation_stretch() {
        let mut transform = Transformation::new();
        transform.stretch((2.0, 3.0));

        assert!((transform.matrix[0] - 2.0).abs() < f32::EPSILON);
        assert!((transform.matrix[4] - 3.0).abs() < f32::EPSILON);
    }

    /// Stretching after translating scales the axes but leaves the existing offset alone,
    /// which is the order `Rect::render` relies on.
    #[test]
    fn test_transformation_translate_then_stretch() {
        let mut transform = Transformation::new();
        transform.translate(FloatPos(10.0, 20.0));
        transform.stretch((2.0, 4.0));

        assert!((transform.matrix[6] - 10.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 20.0).abs() < f32::EPSILON);
        assert!((transform.matrix[0] - 2.0).abs() < f32::EPSILON);
        assert!((transform.matrix[4] - 4.0).abs() < f32::EPSILON);
    }

    /// Translating after stretching moves in the already scaled space.
    #[test]
    fn test_transformation_stretch_then_translate() {
        let mut transform = Transformation::new();
        transform.stretch((2.0, 4.0));
        transform.translate(FloatPos(10.0, 20.0));

        assert!((transform.matrix[6] - 20.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 80.0).abs() < f32::EPSILON);
    }

    // --- Font ---

    const FONT: &[u8] = include_bytes!("../../Build/Resources/font.opa");
    const FONT_MONO: &[u8] = include_bytes!("../../Build/Resources/font_mono.opa");

    fn font() -> gfx::Font {
        gfx::Font::new_headless(FONT, false).unwrap()
    }

    #[test]
    fn test_font_loads_from_the_shipped_atlas() {
        gfx::Font::new_headless(FONT, false).unwrap();
        gfx::Font::new_headless(FONT_MONO, true).unwrap();
    }

    #[test]
    fn test_font_rejects_garbage() {
        // Font is not Debug, so unwrap_err is unavailable
        assert!(gfx::Font::new_headless(&[1, 2, 3, 4], false).is_err());
    }

    /// One line of text is one character tall, and the width is at least 1 even when
    /// empty, so a zero sized texture is never created.
    #[test]
    fn test_empty_text_is_one_line_tall() {
        let size = font().get_text_size("", None);
        assert_eq!(size.1, 16);
        assert_eq!(size.0, 1);
    }

    #[test]
    fn test_text_width_grows_with_length() {
        let font = font();
        let one = font.get_text_size("a", None).0;
        let three = font.get_text_size("aaa", None).0;

        assert!(three > one, "'aaa' ({three}) should be wider than 'a' ({one})");
    }

    #[test]
    fn test_newline_adds_a_line() {
        let font = font();
        let one_line = font.get_text_size("ab", None).1;
        let two_lines = font.get_text_size("ab\ncd", None).1;

        assert!(two_lines > one_line, "two lines ({two_lines}) should be taller than one ({one_line})");
    }

    /// `get_text_size` starts a string that ends in a newline at height 0 instead of 16,
    /// to avoid counting an empty last line. The compensation is one pixel short, though:
    /// a newline adds the glyph height *plus* `CHAR_SPACING`, so a trailing newline nets
    /// +1 rather than 0.
    #[test]
    fn test_trailing_newline_costs_one_pixel_of_spacing() {
        let font = font();
        let plain = font.get_text_size("a", None).1;
        let trailing = font.get_text_size("a\n", None).1;

        assert_eq!(plain, 16);
        assert_eq!(trailing, 17, "the trailing newline compensation misses CHAR_SPACING");
    }

    /// Each extra line costs the glyph height plus one pixel of spacing.
    #[test]
    fn test_each_line_adds_a_fixed_height() {
        let font = font();
        assert_eq!(font.get_text_size("a", None).1, 16);
        assert_eq!(font.get_text_size("a\nb", None).1, 33);
        assert_eq!(font.get_text_size("a\nb\nc", None).1, 50);
    }

    #[test]
    fn test_width_limit_wraps_onto_more_lines() {
        let font = font();
        let unwrapped = font.get_text_size("a long sentence that will not fit", None);
        let wrapped = font.get_text_size("a long sentence that will not fit", Some(50));

        assert!(wrapped.1 > unwrapped.1, "wrapping should make the text taller");
        assert!(wrapped.0 <= 50 + 16, "wrapped text should stay near the limit, got {}", wrapped.0);
    }

    /// A limit narrower than a single glyph cannot be satisfied, so the glyph stays on the line
    /// it is already on. Wrapping instead would leave a blank line above it, count that line's
    /// height, and then do the same again for every character after it.
    #[test]
    fn test_a_limit_narrower_than_a_glyph_does_not_wrap_every_character() {
        let font = font();

        assert_eq!(font.get_text_size("a", Some(1)).1, 16, "one glyph cannot need two lines");
        assert_eq!(font.get_text_size("ab", Some(1)).1, 33, "one wrap between the two, not one before each");
    }

    /// Whether line `line` of `surface` is exactly `expected` drawn against its left edge,
    /// with nothing else anywhere on it. A line is a glyph cell plus one pixel of spacing.
    fn line_is(surface: &Surface, line: i32, expected: &Surface) -> bool {
        let transparent = Color::new(0, 0, 0, 0);
        (0..surface.get_size().0 as i32).all(|x| {
            (0..16).all(|y| {
                let drawn = surface.get_pixel(IntPos(x, line * 17 + y)).copied().unwrap_or(transparent);
                let wanted = expected.get_pixel(IntPos(x, y)).copied().unwrap_or(transparent);
                drawn == wanted
            })
        })
    }

    /// A width limit breaks between words, not inside them. Testing the limit one character at
    /// a time put the front of a word on one line and its tail on the next, which is how the
    /// error menus - the only text in the game that wraps - used to read.
    #[test]
    fn test_a_width_limit_wraps_whole_words() {
        let font = font();
        let width = |text| font.get_text_size(text, None).0 as i32;
        // Room for "aaa" and the space after it, but one pixel short of "bbb" as well.
        let limit = width("aaa ") + width("bbb") - 1;
        let wrapped = font.create_text_surface("aaa bbb", Some(limit));

        assert_eq!(wrapped.get_size().1, 33, "the two words belong on two lines");
        assert!(line_is(&wrapped, 0, &font.create_text_surface("aaa", None)), "the first word should be whole on the first line");
        assert!(line_is(&wrapped, 1, &font.create_text_surface("bbb", None)), "the second word should be whole, and flush left");
    }

    /// A word that cannot fit on a line of its own has to be broken somewhere, so it keeps the
    /// character-by-character wrap - one line down, rather than looping on a line it will never
    /// fit on either.
    #[test]
    fn test_a_word_wider_than_the_limit_still_breaks() {
        let font = font();
        let limit = font.get_text_size("aaa", None).0;
        let wrapped = font.get_text_size("aa bbbbbbbb", Some(limit as i32));

        assert!(wrapped.0 <= limit, "nothing should be drawn past the limit, got {}", wrapped.0);
        assert!(wrapped.1 > 33, "the long word needs lines of its own, got {}", wrapped.1);
    }

    #[test]
    fn test_created_surface_matches_the_measured_size() {
        let font = font();
        for text in ["", "a", "hello world", "two\nlines"] {
            let size = font.get_text_size(text, None);
            let surface = font.create_text_surface(text, None);
            assert_eq!(surface.get_size(), size, "size mismatch for {text:?}");
        }
    }

    /// Whether a column of a rasterised string has any ink in it.
    fn column_has_ink(surface: &Surface, x: u32) -> bool {
        (0..surface.get_size().1 as i32).any(|y| surface.get_pixel(IntPos(x as i32, y)).is_ok_and(|pixel| pixel.a != 0))
    }

    /// `get_text_size` is an *advance* width, so measuring a prefix has to land exactly where
    /// the next glyph is drawn.
    ///
    /// A space advances `SPACE_WIDTH` further than its (empty) glyph, and the width used to be
    /// sampled before that was added - so `TextInput`, which measures the text before the
    /// cursor this way, put the cursor two pixels left of the character after a space.
    #[test]
    fn test_a_prefix_ending_in_a_space_measures_up_to_the_next_glyph() {
        let font = font();
        let prefix = font.get_text_size("a ", None).0;
        let surface = font.create_text_surface("a b", None);

        let first_ink_after_the_a = (font.get_text_size("a", None).0..surface.get_size().0).find(|&x| column_has_ink(&surface, x));

        assert_eq!(first_ink_after_the_a, Some(prefix), "the cursor after a space belongs where the next glyph starts");
    }

    /// Widths add up: laying two strings out end to end is the same as laying out their
    /// concatenation, which is what makes measuring a prefix meaningful at all.
    #[test]
    fn test_measuring_is_additive() {
        let font = font();
        let width = |text| font.get_text_size(text, None).0;

        assert_eq!(width("ab"), width("a") + width("b"));
        assert_eq!(width("a "), width("a") + width(" "));
        assert_eq!(width("a b"), width("a ") + width("b"));
    }

    #[test]
    fn test_scaled_text_size_multiplies() {
        let font = font();
        let size = font.get_text_size("hello", None);
        let scaled = font.get_text_size_scaled("hello", 3.0, None);

        assert!((scaled.0 - size.0 as f32 * 3.0).abs() < f32::EPSILON);
        assert!((scaled.1 - size.1 as f32 * 3.0).abs() < f32::EPSILON);
    }

    /// A monospaced font pads narrow glyphs out so every character advances by the same
    /// amount - which is the whole reason the debug menu uses one.
    #[test]
    fn test_mono_font_advances_uniformly() {
        let mono = gfx::Font::new_headless(FONT_MONO, true).unwrap();

        let narrow = mono.get_text_size("iiii", None).0;
        let wide = mono.get_text_size("MMMM", None).0;

        assert_eq!(narrow, wide, "a mono font should give 'iiii' and 'MMMM' the same width");
    }

    /// **Including the space.** It is the one character whose advance is not its glyph's width:
    /// trimming leaves a proportional font's space empty, so it is given a width of its own -
    /// and adding that on top of a mono font's padded space made the space two pixels wider
    /// than every other character, which is the whole of what a mono font promises not to do.
    /// The server console is a column of timestamps drawn in this font.
    #[test]
    fn test_a_mono_space_advances_like_every_other_character() {
        let mono = gfx::Font::new_headless(FONT_MONO, true).unwrap();

        assert_eq!(mono.get_text_size("i i", None).0, mono.get_text_size("iii", None).0);
        assert_eq!(mono.get_text_size(" ", None).0, mono.get_text_size("i", None).0);
    }

    /// The proportional font still needs it: its space glyph is trimmed to nothing, so without
    /// a width of its own a space would be a single pixel of character spacing.
    #[test]
    fn test_a_proportional_space_is_wider_than_its_empty_glyph() {
        let font = font();
        assert!(font.get_text_size(" ", None).0 > 1);
    }

    #[test]
    fn test_proportional_font_does_not_advance_uniformly() {
        let font = font();
        assert_ne!(font.get_text_size("iiii", None).0, font.get_text_size("MMMM", None).0);
    }

    /// Rendering text needs the GPU textures a headless font does not have, so it draws
    /// nothing rather than misbehaving. This pins that it is at least safe to construct.
    #[test]
    fn test_headless_font_measures_without_textures() {
        let font = gfx::Font::new_headless(FONT, false).unwrap();
        assert!(font.get_text_size("measured anyway", None).0 > 1);
    }

    // ---------------------------------------------------------------------------------
    // Draw list tests
    //
    // Drawing records a `DrawCommand` rather than issuing a GPU call, so what a primitive
    // draws is assertable without a window - the only tier `cargo test` can run. Every
    // primitive reaches here, because building one without a device is a supported state:
    // `RectArray` stages vertices for an upload that never happens, and `ShadowContext` and
    // `Font` get textures that know their size and own nothing. What these cannot see is the
    // pixels, which is the golden suite's job.
    // ---------------------------------------------------------------------------------

    use gfx::{BlendMode, DrawCommand, DrawList, DrawRecorder, DrawTarget};

    const WHITE: Color = Color::new(255, 255, 255, 255);

    #[test]
    fn test_rect_records_itself_unchanged() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render(&recorder, Color::new(1, 2, 3, 4));

        assert_eq!(recorder.get_commands(), vec![DrawCommand::Rect { rect, color: Color::new(1, 2, 3, 4) }]);
    }

    #[test]
    fn test_rect_outline_records_a_different_command() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render_outline(&recorder, WHITE);

        assert_eq!(recorder.get_commands(), vec![DrawCommand::RectOutline { rect, color: WHITE }]);
    }

    /// A fully transparent draw is dropped at record time rather than being handed to the
    /// backend to blend into nothing.
    #[test]
    fn test_invisible_rects_are_never_recorded() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render(&recorder, WHITE.set_a(0));
        rect.render_outline(&recorder, WHITE.set_a(0));

        assert!(recorder.is_empty());
    }

    /// An outline is four one pixel quads laid on the rectangle's own edge pixels, which an
    /// empty rectangle does not have: the bottom edge is placed at `pos.1 + size.1 - 1.0`, so
    /// with no height it lands a row *above* the top edge, outside the rectangle entirely.
    ///
    /// A `Button` mid-hover-fade reaches this. Its hover rectangle is the button inset by up
    /// to 30 pixels a side, which is more than a small button has to give, and the border it
    /// is drawn with is part way to opaque by then.
    #[test]
    fn test_an_empty_rect_draws_no_outline() {
        let recorder = DrawRecorder::new();

        Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 0.0)).render_outline(&recorder, WHITE);
        Rect::new(FloatPos(50.0, 50.0), FloatSize(40.0, 0.0)).render_outline(&recorder, WHITE);
        Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 40.0)).render_outline(&recorder, WHITE);

        assert!(recorder.is_empty(), "an empty rectangle has no edge pixels to draw");

        // what it would have drawn: two vertical lines, the right hand one a pixel to the left
        // of a rectangle that has no width at all
        let edges = crate::libraries::graphics::wgpu_backend::outline_edges(Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 40.0)));
        assert_eq!(edges[3], Rect::new(FloatPos(49.0, 50.0), FloatSize(1.0, 40.0)));
    }

    /// The outline of a rectangle that does have edges covers its own footprint exactly, in
    /// both directions - a border that spilled outside would be a pixel of another widget.
    #[test]
    fn test_outline_edges_stay_inside_the_rectangle() {
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        for edge in crate::libraries::graphics::wgpu_backend::outline_edges(rect) {
            assert!(edge.pos.0 >= rect.pos.0 && edge.pos.1 >= rect.pos.1, "{edge:?} starts outside {rect:?}");
            assert!(
                edge.pos.0 + edge.size.0 <= rect.pos.0 + rect.size.0 && edge.pos.1 + edge.size.1 <= rect.pos.1 + rect.size.1,
                "{edge:?} ends outside {rect:?}"
            );
        }
    }

    #[test]
    fn test_offscreen_rects_are_culled_at_record_time() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        // past each of the four edges
        Rect::new(FloatPos(-50.0, 10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(10.0, -50.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(200.0, 10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(10.0, 200.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);

        assert!(recorder.is_empty());
    }

    #[test]
    fn test_a_rect_hanging_over_an_edge_still_draws() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        Rect::new(FloatPos(-10.0, -10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(90.0, 90.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);

        assert_eq!(recorder.len(), 2);
    }

    /// Culling `render` but not `render_outline` is a real asymmetry, not an oversight: a
    /// border whose rectangle starts offscreen still has edges that cross the window.
    #[test]
    fn test_outlines_are_not_culled() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        Rect::new(FloatPos(-500.0, -500.0), FloatSize(20.0, 20.0)).render_outline(&recorder, WHITE);

        assert_eq!(recorder.len(), 1);
    }

    #[test]
    fn test_texture_defaults_to_its_whole_self_undyed() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 8.0));

        texture.render(&recorder, 2.0, FloatPos(5.0, 6.0), None, false, None);

        assert_eq!(
            recorder.get_commands(),
            vec![DrawCommand::Texture {
                texture: texture.get_handle(),
                texture_size: FloatSize(16.0, 8.0),
                src_rect: Rect::new(FloatPos(0.0, 0.0), FloatSize(16.0, 8.0)),
                pos: FloatPos(5.0, 6.0),
                scale: 2.0,
                flipped: false,
                color: WHITE,
            }]
        );
    }

    #[test]
    fn test_texture_passes_through_source_rect_flip_and_tint() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 16.0));
        let src = Rect::new(FloatPos(4.0, 4.0), FloatSize(8.0, 8.0));

        texture.render(&recorder, 3.0, FloatPos(1.0, 2.0), Some(src), true, Some(Color::new(9, 8, 7, 6)));

        assert_eq!(
            recorder.get_commands(),
            vec![DrawCommand::Texture {
                texture: texture.get_handle(),
                texture_size: FloatSize(16.0, 16.0),
                src_rect: src,
                pos: FloatPos(1.0, 2.0),
                scale: 3.0,
                flipped: true,
                color: Color::new(9, 8, 7, 6),
            }]
        );
    }

    #[test]
    fn test_an_empty_source_rect_draws_nothing() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 16.0));

        texture.render(&recorder, 1.0, FloatPos(0.0, 0.0), Some(Rect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0))), false, None);

        assert!(recorder.is_empty());
    }

    /// A `Texture::new` owns nothing on the GPU and reports a zero size, so the default
    /// source rectangle is empty and the same early return catches it. Without that, the
    /// backend would be handed a command naming a texture that does not exist.
    #[test]
    fn test_a_texture_with_no_gpu_object_draws_nothing() {
        let recorder = DrawRecorder::new();

        gfx::Texture::new().render(&recorder, 1.0, FloatPos(0.0, 0.0), None, false, None);

        assert!(recorder.is_empty());
    }

    /// A blend mode change only means something relative to the draws around it, which is
    /// why it is a command in the list rather than a call that takes effect immediately.
    #[test]
    fn test_blend_mode_changes_keep_their_place_in_the_order() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));

        rect.render(&recorder, WHITE);
        recorder.set_blend_mode(BlendMode::Multiply);
        rect.render(&recorder, WHITE);
        recorder.set_blend_mode(BlendMode::Alpha);

        assert_eq!(
            recorder.get_commands(),
            vec![
                DrawCommand::Rect { rect, color: WHITE },
                DrawCommand::SetBlendMode(BlendMode::Multiply),
                DrawCommand::Rect { rect, color: WHITE },
                DrawCommand::SetBlendMode(BlendMode::Alpha),
            ]
        );
    }

    #[test]
    fn test_clearing_a_list_empties_it() {
        let mut list = DrawList::new();
        assert!(list.is_empty());

        list.push(DrawCommand::SetBlendMode(BlendMode::Alpha));
        list.push(DrawCommand::SetBlendMode(BlendMode::Multiply));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get_commands().len(), 2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.get_commands(), &[]);
    }

    /// Dropping a texture parks its registry id instead of releasing it, because a command
    /// recorded earlier this frame may still refer to it. A texture that never had an id
    /// must not park anything.
    #[test]
    fn test_dropping_a_gpu_less_texture_parks_nothing() {
        let before = gfx::gpu_device::get_pending_counts();

        drop(gfx::Texture::new_sized(FloatSize(4.0, 4.0)));
        drop(gfx::Texture::new());

        assert_eq!(gfx::gpu_device::get_pending_counts(), before);
    }

    /// Uploading a surface with no device in the process is not an error: it produces a texture
    /// that knows its size and owns nothing, which is what lets layout run headlessly.
    #[test]
    fn test_a_texture_can_be_built_without_a_device() {
        let texture = gfx::Texture::load_from_surface(&Surface::new(IntSize(6, 9)));

        assert_eq!(texture.get_texture_size(), FloatSize(6.0, 9.0));
        assert_eq!(texture.get_handle(), gfx::Texture::new().get_handle());
    }

    /// One mesh, however many rectangles went into it - drawing them in a single call is the
    /// whole reason `RectArray` exists.
    #[test]
    fn test_a_rect_array_records_one_mesh_for_all_of_its_rectangles() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(8.0, 8.0));
        let mut array = gfx::RectArray::new();
        let tex_rect = Rect::new(FloatPos(0.0, 0.0), FloatSize(8.0, 8.0));
        for i in 0..3 {
            array.add_rect(&Rect::new(FloatPos(i as f32 * 10.0, 0.0), FloatSize(10.0, 10.0)), &[WHITE; 4], &tex_rect);
        }

        array.render(&recorder, Some(&texture), FloatPos(4.0, 5.0));

        assert_eq!(recorder.len(), 1);
        let Some(DrawCommand::Mesh { texture: named, pos, .. }) = recorder.get_commands().first().copied() else {
            panic!("a rect array should record a mesh, got {:?}", recorder.get_commands())
        };
        assert_eq!(named, Some((texture.get_handle(), FloatSize(8.0, 8.0))));
        assert_eq!(pos, FloatPos(4.0, 5.0));
    }

    /// A `RectArray` with no texture draws its vertices' own colours, which is how the light
    /// map and the health bar are drawn.
    #[test]
    fn test_an_untextured_rect_array_names_no_texture() {
        let recorder = DrawRecorder::new();
        gfx::RectArray::new().render(&recorder, None, FloatPos(0.0, 0.0));

        assert!(matches!(recorder.get_commands().first(), Some(DrawCommand::Mesh { texture: None, .. })));
    }

    // --- Font drawing ---

    /// Measuring and drawing have to agree to the pixel, because `TextInput` places its cursor
    /// by measuring the text before it and the glyph after it is drawn by `render_text`. The
    /// two share `Font::advance` so that they cannot drift; this is what pins that they don't.
    ///
    /// The space is the character that makes it worth checking: its glyph is trimmed to nothing,
    /// so it records no draw at all and only moves the pen.
    #[test]
    fn test_render_text_draws_each_glyph_where_measuring_says_it_will() {
        let font = font();
        let recorder = DrawRecorder::with_draw_area(FloatSize(1000.0, 100.0));
        let text = "a b";

        font.render_text(&recorder, text, FloatPos(10.0, 20.0), 2.0);

        // `get_text_size` never reports a zero width, so an empty prefix is answered directly -
        // exactly as `TextInput::width_up_to` does it.
        let expected: Vec<f32> = text
            .char_indices()
            .filter(|(_, character)| *character != ' ')
            .map(|(index, _)| {
                if index == 0 {
                    10.0
                } else {
                    10.0 + font.get_text_size(text.get(..index).unwrap(), None).0 as f32 * 2.0
                }
            })
            .collect();
        let drawn: Vec<f32> = recorder
            .get_commands()
            .into_iter()
            .map(|command| match command {
                DrawCommand::Texture { pos, .. } => pos.0,
                other => panic!("text should only draw textures, got {other:?}"),
            })
            .collect();

        assert_eq!(drawn, expected);
    }

    // --- ShadowContext ---

    use crate::libraries::graphics::shadow::{ShadowContext, FADE, TEXTURE_SIZE};

    /// Where every piece of a shadow around `rect` lands, as (destination, source) rectangles.
    /// The pieces are drawn unscaled, so a destination is as big as the region it samples.
    fn shadow_pieces(rect: Rect) -> Vec<(Rect, Rect)> {
        let recorder = DrawRecorder::with_draw_area(FloatSize(4000.0, 4000.0));
        ShadowContext::new().render(&recorder, &rect, 1.0);

        recorder
            .get_commands()
            .into_iter()
            .map(|command| match command {
                DrawCommand::Texture { pos, src_rect, .. } => (Rect::new(pos, src_rect.size), src_rect),
                other => panic!("a shadow should only draw textures, got {other:?}"),
            })
            .collect()
    }

    /// The sizes that matter: below 300 the edge pieces meet in the middle on their own, and
    /// above it they are capped and the gap has to be tiled.
    const SHADOW_SIZES: [FloatSize; 5] = [FloatSize(0.0, 0.0), FloatSize(140.0, 100.0), FloatSize(140.0, 900.0), FloatSize(900.0, 140.0), FloatSize(901.0, 851.0)];

    /// Every piece samples a region of the baked texture, and a region that ran off the edge of
    /// it would be clamped by the sampler into a smear of whatever the last row holds.
    #[test]
    fn test_a_shadow_only_ever_samples_inside_its_own_texture() {
        for size in SHADOW_SIZES {
            for (_, source) in shadow_pieces(Rect::new(FloatPos(300.0, 300.0), size)) {
                assert!(
                    source.pos.0 >= 0.0 && source.pos.1 >= 0.0 && source.pos.0 + source.size.0 <= TEXTURE_SIZE && source.pos.1 + source.size.1 <= TEXTURE_SIZE,
                    "a {size:?} shadow samples {source:?}, outside the {TEXTURE_SIZE}x{TEXTURE_SIZE} texture"
                );
            }
        }
    }

    /// A shadow is drawn *under* an opaque rectangle, so anything it puts inside one is wasted
    /// work - and it is not wasted work if the rectangle turns out to be translucent, it is a
    /// dark smear across it.
    #[test]
    fn test_a_shadow_never_draws_inside_the_rectangle_it_surrounds() {
        for size in SHADOW_SIZES {
            let rect = Rect::new(FloatPos(300.0, 300.0), size);
            for (piece, _) in shadow_pieces(rect) {
                let overlaps = |piece_pos: f32, piece_size: f32, rect_pos: f32, rect_size: f32| piece_pos + piece_size > rect_pos && piece_pos < rect_pos + rect_size;
                assert!(
                    !(overlaps(piece.pos.0, piece.size.0, rect.pos.0, rect.size.0) && overlaps(piece.pos.1, piece.size.1, rect.pos.1, rect.size.1)),
                    "the piece {piece:?} of a {size:?} shadow reaches inside {rect:?}"
                );
            }
        }
    }

    /// And the other direction: the `FADE` band around the rectangle is covered with no gaps.
    /// This is what the tiling exists for - a rectangle over 300 pixels tall outgrows the two
    /// capped edge pieces, and an off-by-one in the middle repeated down the gap is a
    /// transparent stripe up the side of a menu.
    #[test]
    fn test_a_shadow_covers_the_whole_band_around_the_rectangle() {
        for size in SHADOW_SIZES {
            let rect = Rect::new(FloatPos(300.0, 300.0), size);
            let pieces = shadow_pieces(rect);

            // Sampled on a grid offset off every piece boundary, so a point is covered by a
            // piece rather than merely touching one - `Rect::contains` is inclusive.
            let mut x = rect.pos.0 - FADE + 3.75;
            while x < rect.pos.0 + rect.size.0 + FADE {
                let mut y = rect.pos.1 - FADE + 3.75;
                while y < rect.pos.1 + rect.size.1 + FADE {
                    let inside = rect.contains(FloatPos(x, y));
                    if !inside {
                        assert!(pieces.iter().any(|(piece, _)| piece.contains(FloatPos(x, y))), "a {size:?} shadow leaves ({x}, {y}) uncovered");
                    }
                    y += 7.5;
                }
                x += 7.5;
            }
        }
    }

    /// The shadow fades out, so the pieces are drawn translucent - and the intensity a
    /// `RenderRect` passes down has to reach them.
    #[test]
    fn test_shadow_intensity_scales_the_colour_every_piece_is_drawn_with() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(4000.0, 4000.0));
        ShadowContext::new().render(&recorder, &Rect::new(FloatPos(300.0, 300.0), FloatSize(140.0, 100.0)), 0.5);

        for command in recorder.get_commands() {
            let DrawCommand::Texture { color, .. } = command else {
                panic!("a shadow should only draw textures, got {command:?}")
            };
            assert_eq!(color, Color::new(0, 0, 0, 40));
        }
    }
}
