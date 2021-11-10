//! Label with mnemonic, never performs a line wrap
//!
use crate::layout::{Limits, Node};
use crate::{layout, renderer, text};
use crate::{Hasher, Layout, Point, Rectangle, Widget};
use iced_core::{alignment, Color, Length, Size};
use std::borrow::Cow;
use std::hash::Hash;

/// State of a TextLabel
#[derive(Debug, Default)]
pub struct State {
    marked: Option<char>,
    hovered: bool,
}

/// Label with mnemonic and never wraps lines.
#[derive(Debug)]
pub struct TextLabel<'a, Renderer: text::Renderer> {
    state: &'a mut State,
    text: Cow<'a, str>,
    mnemonic: Option<char>,
    size: Option<u16>,
    color: Option<Color>,
    font: Renderer::Font,
    width: Length,
    height: Length,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TextLabel<'a, Renderer>
where
    Renderer: text::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or(renderer.default_size());

        let bounds = limits.max();

        let (width, height) =
            renderer.measure(&self.text, size, self.font, bounds);

        let size = limits.resolve(Size::new(width, height));

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            style,
            layout,
            self.text.as_ref(),
            self.font,
            self.size,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
        );
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.text.hash(state);
        self.mnemonic.hash(state);
        self.size.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

/// Draws text using the same logic as the [`Text`] widget.
///
/// Specifically:
///
/// * If no `size` is provided, the default text size of the `Renderer` will be
///   used.
/// * If no `color` is provided, the [`renderer::Style::text_color`] will be
///   used.
/// * The alignment attributes do not affect the position of the bounds of the
///   [`Layout`].
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &renderer::Style,
    layout: Layout<'_>,
    content: &str,
    font: Renderer::Font,
    size: Option<u16>,
    color: Option<Color>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
) where
    Renderer: text::Renderer,
{
    let bounds = layout.bounds();

    let x = match horizontal_alignment {
        alignment::Horizontal::Left => bounds.x,
        alignment::Horizontal::Center => bounds.center_x(),
        alignment::Horizontal::Right => bounds.x + bounds.width,
    };

    let y = match vertical_alignment {
        alignment::Vertical::Top => bounds.y,
        alignment::Vertical::Center => bounds.center_y(),
        alignment::Vertical::Bottom => bounds.y + bounds.height,
    };

    renderer.fill_text(crate::text::Text {
        content,
        size: f32::from(size.unwrap_or(renderer.default_size())),
        bounds: Rectangle { x, y, ..bounds },
        color: color.unwrap_or(style.text_color),
        font,
        horizontal_alignment,
        vertical_alignment,
    });
}

// impl<Renderer: text::Renderer> Clone for TextLabel<'static, Renderer> {
//     fn clone(&self) -> TextLabel<'static, Renderer> {
//         Self {
//             state: self.state,
//             text: self.text.clone(),
//             mnemonic: self.mnemonic,
//             size: self.size,
//             color: self.color,
//             font: self.font,
//             width: self.width,
//             height: self.height,
//             horizontal_alignment: self.horizontal_alignment,
//             vertical_alignment: self.vertical_alignment,
//         }
//     }
// }
