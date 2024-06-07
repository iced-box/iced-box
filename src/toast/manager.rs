use iced::{
    Element,
    Alignment,
    Theme,
    Length,
    Size,
    mouse,
    Renderer,
    time::Instant,
    event::{self, Event},
    Rectangle,
    Vector,
    advanced::{
        renderer,
        overlay::{self},
        Shell,
        layout,
        Widget,
        Layout,
        Clipboard,
        widget::{
            self,
            Tree,
            Operation,
        },
    },
    widget::{
        button,
        column,
        container,
        horizontal_rule,
        horizontal_space,
        row,
        text,
    }
};
use super::{
    Toast,
    overlay::Overlay,
    Status,
    styles::{
        primary,
        secondary,
        success,
        danger,
    },
};


pub const DEFAULT_TIMEOUT: u64 = 5;

/// Manages alerts, controls the display and termination of the alert
pub struct Manager<'a, Message> {
    content: Element<'a, Message>,
    toasts: Vec<Element<'a, Message>>,
    timeout_secs: u64,
    on_close: Box<dyn Fn(usize) -> Message + 'a>,
}

impl<'a, Message> Manager<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(
        content: impl Into<Element<'a, Message>>,
        toasts: &'a [Toast],
        on_close: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        let toasts = toasts
            .iter()
            .enumerate()
            .map(|(index, toast)| {
                container(column![
                    container(
                        row![
                            if toast.with_close {
                                column![
                                    row![
                                        text(toast.title.as_str()),
                                        horizontal_space(),
                                        button("x")
                                        .on_press((on_close)(index))
                                        .padding(0)
                                        .style(|_theme: &Theme, _status: button::Status| {
                                            button::Style {
                                                background: None,
                                                ..button::Style::default()
                                            }
                                        })
                                    ],
                                ]
                            } else {
                                column![
                                    text(toast.title.as_str())
                                ]
                            }
                        ]
                        .align_items(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding(5)
                    .style(match toast.status {
                        Status::Primary => primary,
                        Status::Secondary => secondary,
                        Status::Success => success,
                        Status::Danger => danger,
                    }),

                    if toast.body.is_empty() {
                        column![]
                    } else {
                        column![
                            horizontal_rule(1),
                            container(text(toast.body.as_str()))
                                .width(Length::Fill)
                                .padding(5)
                                .style(container::rounded_box),
                        ]
                    }
                ])
                .max_width(200)
                .into()
            })
            .collect();

        Self {
            content: content.into(),
            toasts,
            timeout_secs: DEFAULT_TIMEOUT,
            on_close: Box::new(on_close),
        }
    }

    /// Sets the amount of time alerts are displayed, the default is 5 seconds.
    pub fn timeout(self, seconds: u64) -> Self {
        Self {
            timeout_secs: seconds,
            ..self
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for Manager<'a, Message> {
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn tag(&self) -> widget::tree::Tag {
        struct Marker;
        widget::tree::Tag::of::<Marker>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Vec::<Option<Instant>>::new())
    }

    fn children(&self) -> Vec<Tree> {
        std::iter::once(Tree::new(&self.content))
            .chain(self.toasts.iter().map(Tree::new))
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let instants = tree.state.downcast_mut::<Vec<Option<Instant>>>();

        // Invalidating removed instants to None allows us to remove
        // them here so that diffing for removed / new toast instants
        // is accurate
        instants.retain(Option::is_some);

        match (instants.len(), self.toasts.len()) {
            (old, new) if old > new => {
                instants.truncate(new);
            }
            (old, new) if old < new => {
                instants.extend(
                    std::iter::repeat(Some(Instant::now())).take(new - old),
                );
            }
            _ => {}
        }

        tree.diff_children(
            &std::iter::once(&self.content)
                .chain(self.toasts.iter())
                .collect::<Vec<_>>(),
        );
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut state.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let instants = state.state.downcast_mut::<Vec<Option<Instant>>>();

        let (content_state, toasts_state) = state.children.split_at_mut(1);

        let content = self.content.as_widget_mut().overlay(
            &mut content_state[0],
            layout,
            renderer,
            translation,
        );

        let toasts = (!self.toasts.is_empty()).then(|| {
            overlay::Element::new(Box::new(Overlay {
                position: layout.bounds().position() + translation,
                toasts: &mut self.toasts,
                state: toasts_state,
                instants,
                on_close: &self.on_close,
                timeout_secs: self.timeout_secs,
            }))
        });
        let overlays =
            content.into_iter().chain(toasts).collect::<Vec<_>>();

        (!overlays.is_empty())
            .then(|| overlay::Group::with_children(overlays).overlay())
    }
}


impl<'a, Message> From<Manager<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(manager: Manager<'a, Message>) -> Self {
        Element::new(manager)
    }
}
