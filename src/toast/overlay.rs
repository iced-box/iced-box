use iced::{
    mouse,
    Point,
    time::{Instant, Duration},
    Renderer,
    Element,
    Length,
    Alignment,
    Theme,
    Size,
    Vector,
    Rectangle,
    event::{self, Event},
    window,
    advanced::{
        Clipboard,
        layout::{self, Layout},
        Shell,
        renderer,
        overlay,
        widget::{
            self,
            Tree,
        }
    }
};

pub struct Overlay<'a, 'b, Message> {
    pub position: Point,
    pub toasts: &'b mut [Element<'a, Message>],
    pub state: &'b mut [Tree],
    pub instants: &'b mut [Option<Instant>],
    pub on_close: &'b dyn Fn(usize) -> Message,
    pub timeout_secs: u64,
}

impl<'a, 'b, Message> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message>
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
    ) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);

        layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            Length::Fill,
            Length::Fill,
            10.into(),
            10.0,
            Alignment::End,
            self.toasts,
            self.state,
        )
        .translate(Vector::new(self.position.x, self.position.y))
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        if let Event::Window(_, window::Event::RedrawRequested(now)) =
            &event
        {
            let mut next_redraw: Option<window::RedrawRequest> = None;

            self.instants.iter_mut().enumerate().for_each(
                |(index, maybe_instant)| {
                    if let Some(instant) = maybe_instant.as_mut() {
                        let remaining =
                            Duration::from_secs(self.timeout_secs)
                                .saturating_sub(instant.elapsed());

                        if remaining == Duration::ZERO {
                            maybe_instant.take();
                            shell.publish((self.on_close)(index));
                            next_redraw =
                                Some(window::RedrawRequest::NextFrame);
                        } else {
                            let redraw_at =
                                window::RedrawRequest::At(*now + remaining);
                            next_redraw = next_redraw
                                .map(|redraw| redraw.min(redraw_at))
                                .or(Some(redraw_at));
                        }
                    }
                },
            );

            if let Some(redraw) = next_redraw {
                shell.request_redraw(redraw);
            }
        }

        let viewport = layout.bounds();

        self.toasts
            .iter_mut()
            .zip(self.state.iter_mut())
            .zip(layout.children())
            .zip(self.instants.iter_mut())
            .map(|(((child, state), layout), instant)| {
                let mut local_messages = vec![];
                let mut local_shell = Shell::new(&mut local_messages);

                let status = child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    &mut local_shell,
                    &viewport,
                );

                if !local_shell.is_empty() {
                    instant.take();
                }

                shell.merge(local_shell, std::convert::identity);

                status
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let viewport = layout.bounds();

        for ((child, state), layout) in self
            .toasts
            .iter()
            .zip(self.state.iter())
            .zip(layout.children())
        {
            child.as_widget().draw(
                state, renderer, theme, style, layout, cursor, &viewport,
            );
        }
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.toasts
                .iter()
                .zip(self.state.iter_mut())
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.toasts
            .iter()
            .zip(self.state.iter())
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state, layout, cursor, viewport, renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn is_over(
        &self,
        layout: Layout<'_>,
        _renderer: &Renderer,
        cursor_position: Point,
    ) -> bool {
        layout
            .children()
            .any(|layout| layout.bounds().contains(cursor_position))
    }
}
