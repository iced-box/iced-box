use iced::{
    theme,
    Theme,
    widget::container,
};

pub fn primary(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    styled(palette.primary.weak)
}

pub fn secondary(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    styled(palette.secondary.weak)
}

pub fn success(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    styled(palette.success.weak)
}

pub fn danger(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    styled(palette.danger.weak)
}

pub fn styled(pair: theme::palette::Pair) -> container::Style {
    container::Style {
        background: Some(pair.color.into()),
        text_color: pair.text.into(),
        ..Default::default()
    }
}
