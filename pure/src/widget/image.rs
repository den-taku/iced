//! Display images in your user interface.
use crate::widget::{Tree, Widget};
use crate::Element;

use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::widget::image;
use iced_native::{Length, Point, Rectangle};

use std::hash::Hash;

pub use image::Image;

impl<Message, Renderer, Handle> Widget<Message, Renderer> for Image<Handle>
where
    Handle: Clone + Hash,
    Renderer: iced_native::image::Renderer<Handle = Handle>,
{
    fn width(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::width(self)
    }

    fn height(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::height(self)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        <Self as iced_native::Widget<Message, Renderer>>::layout(
            self, renderer, limits,
        )
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        <Self as iced_native::Widget<Message, Renderer>>::draw(
            self,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }
}

impl<'a, Message, Renderer, Handle> From<Image<Handle>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::image::Renderer<Handle = Handle> + 'a,
    Handle: Clone + Hash + 'a,
{
    fn from(image: Image<Handle>) -> Self {
        Self::new(image)
    }
}
