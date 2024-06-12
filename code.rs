use std::sync::Mutex;

use cosmic::{
    iced::{Length::Fill, Rectangle, Size},
    iced_core::{
        image, layout,
        widget::{tree, Widget},
    },
    Element, Renderer,
};
use cosmic_text::{Attrs, Buffer, Edit, FontSystem, Metrics, SyntaxEditor};

use crate::{FONT_SYSTEM, SWASH_CACHE, SYNTAX_SYSTEM};

pub struct Markdown {
    syntax_editor: Mutex<SyntaxEditor<'static, 'static>>,
    font_system: &'static Mutex<FontSystem>,
}

impl Markdown {
    pub fn new(content: &str, syntax_ext: &str) -> Self {
        let mut font_system = FONT_SYSTEM.get().unwrap().lock().unwrap();
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(14.0, 20.0));
        let syntax_system = SYNTAX_SYSTEM.get().unwrap();

        buffer.borrow_with(&mut font_system).set_text(
            content,
            Attrs::new(),
            cosmic_text::Shaping::Advanced,
        );

        let mut editor = SyntaxEditor::new(buffer, syntax_system, "base16-eighties.dark").unwrap();
        editor.syntax_by_extension(syntax_ext);

        Self {
            syntax_editor: Mutex::new(editor),
            font_system: FONT_SYSTEM.get().unwrap(),
        }
    }
}

pub struct State {
    handle_opt: Mutex<Option<image::Handle>>,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State {
            handle_opt: Mutex::new(None),
        }
    }
}

impl<Message> Widget<Message, cosmic::Theme, Renderer> for Markdown {
    fn size(&self) -> Size<cosmic::iced::Length> {
        Size {
            width: Fill,
            height: Fill,
        }
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn layout(
        &self,
        _tree: &mut cosmic::iced_core::widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();

        layout::Node::new(Size {
            width: max.width,
            height: max.height,
        })
    }

    fn draw(
        &self,
        tree: &cosmic::iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &cosmic::Theme,
        _style: &cosmic::iced_core::renderer::Style,
        layout: cosmic::iced_core::Layout<'_>,
        _cursor: cosmic::iced_core::mouse::Cursor,
        viewport: &cosmic::iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let mut swash_cache = SWASH_CACHE.get().unwrap().lock().unwrap();
        let mut font_system = self.font_system.lock().unwrap();
        let mut editor = self.syntax_editor.lock().unwrap();

        editor.shape_as_needed(&mut font_system, true);

        editor.with_buffer_mut(|buffer| {
            buffer.set_metrics_and_size(
                &mut font_system,
                Metrics::new(14.0, 20.0),
                viewport.width,
                viewport.height,
            );
        });

        let mut pixels_u8 = vec![0; viewport.width as usize * viewport.height as usize * 4];

        let pixels = unsafe {
            std::slice::from_raw_parts_mut(pixels_u8.as_mut_ptr() as *mut u32, pixels_u8.len() / 4)
        };

        let mut handle_opt = state.handle_opt.lock().unwrap();

        if editor.redraw() || handle_opt.is_none() {
            editor.with_buffer(|buffer| {
                buffer.draw(
                    &mut font_system,
                    &mut swash_cache,
                    cosmic_text::Color(0xFFFFFF),
                    |x, y, w, h, color| {
                        draw_rect(
                            pixels,
                            Canvas {
                                w: viewport.width as i32,
                                h: viewport.height as i32,
                            },
                            Canvas {
                                w: w as i32,
                                h: h as i32,
                            },
                            Offset { x, y },
                            color,
                        );
                    },
                )
            })
        }

        *handle_opt = Some(image::Handle::from_pixels(
            viewport.width as u32,
            viewport.height as u32,
            pixels_u8,
        ));

        if let Some(ref handle) = *handle_opt {
            image::Renderer::draw(
                renderer,
                handle.clone(),
                image::FilterMethod::Nearest,
                Rectangle::new(
                    layout.position(),
                    Size::new(viewport.width, viewport.height),
                ),
                [0.0; 4],
            );
        }
    }
}

struct Canvas {
    w: i32,
    h: i32,
}

struct Offset {
    x: i32,
    y: i32,
}

// source: https://github.com/pop-os/cosmic-edit/blob/master/src/text_box.rs#L136-L215
fn draw_rect(
    buffer: &mut [u32],
    canvas: Canvas,
    offset: Canvas,
    screen: Offset,
    cosmic_color: cosmic_text::Color,
) {
    // Grab alpha channel and green channel
    let mut color = cosmic_color.0 & 0xFF00FF00;
    // Shift red channel
    color |= (cosmic_color.0 & 0x00FF0000) >> 16;
    // Shift blue channel
    color |= (cosmic_color.0 & 0x000000FF) << 16;

    let alpha = (color >> 24) & 0xFF;
    match alpha {
        0 => {
            // Do not draw if alpha is zero.
        }
        255 => {
            // Handle overwrite
            for x in screen.x..screen.x + offset.w {
                if x < 0 || x >= canvas.w {
                    // Skip if y out of bounds
                    continue;
                }

                for y in screen.y..screen.y + offset.h {
                    if y < 0 || y >= canvas.h {
                        // Skip if x out of bounds
                        continue;
                    }

                    let line_offset = y as usize * canvas.w as usize;
                    let offset = line_offset + x as usize;
                    buffer[offset] = color;
                }
            }
        }
        _ => {
            let n_alpha = 255 - alpha;
            for y in screen.y..screen.y + offset.h {
                if y < 0 || y >= canvas.h {
                    // Skip if y out of bounds
                    continue;
                }

                let line_offset = y as usize * canvas.w as usize;
                for x in screen.x..screen.x + offset.w {
                    if x < 0 || x >= canvas.w {
                        // Skip if x out of bounds
                        continue;
                    }

                    // Alpha blend with current value
                    let offset = line_offset + x as usize;
                    let current = buffer[offset];
                    if current & 0xFF000000 == 0 {
                        // Overwrite if buffer empty
                        buffer[offset] = color;
                    } else {
                        let rb = ((n_alpha * (current & 0x00FF00FF))
                            + (alpha * (color & 0x00FF00FF)))
                            >> 8;
                        let ag = (n_alpha * ((current & 0xFF00FF00) >> 8))
                            + (alpha * (0x01000000 | ((color & 0x0000FF00) >> 8)));
                        buffer[offset] = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
                    }
                }
            }
        }
    }
}

pub fn markdown(content: &str, syntax_ext: &str) -> Markdown {
    Markdown::new(content, syntax_ext)
}

impl<'a, Message> From<Markdown> for Element<'a, Message> {
    fn from(value: Markdown) -> Self {
        Self::new(value)
    }
}
