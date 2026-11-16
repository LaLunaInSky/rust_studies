use iced::{
    advanced::{
        widget::{
            self,
            Widget
        },
        layout::{
            self,
            Layout
        },
        renderer
    },
    border,
    mouse,
    Color,
    Element,
    Length,
    Rectangle,
    Size,
};

pub struct CircleState {
    radius: f32,
}

impl CircleState {
    pub fn new(
        radius: f32
    ) -> Self {
        Self {
            radius
        }
    }
}

pub fn circle_state(
    radius: f32
) -> CircleState {
    CircleState::new(radius)
}

impl <Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
    for CircleState 
where 
    Renderer: renderer::Renderer, 
{
    fn size(
        &self
    ) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(
            Size::new(
                self.radius * 2.0,
                self.radius * 2.0
            )
        )
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.radius),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
    }
}

impl<Message, Theme, Renderer> From<CircleState>
    for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer
{
    fn from(
        circle: CircleState
    ) -> Self {
        Self::new(circle)
    }
}