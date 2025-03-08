use iced::theme::palette::Extended;
use iced::widget::{button, column, container, row, scrollable, text, Column, Container, Row};
use iced::{Element, Fill, Font};
use sheets_diff::core::diff::UnifiedDiffKind;

use super::{message::Message, state::State};
use crate::core::consts::{APP_THEME, BASE_SIZE, FOOTER_NOTE, GUIDANCE};
use crate::core::font::diff_font;

/// iced view handler
pub fn handle(state: &State) -> Element<Message> {
    let palette = APP_THEME.extended_palette();
    let diff_font = Font::with_name(diff_font());

    let column_with_components = column![
        header(state),
        diff_viewer(state, palette, &diff_font),
        footer(state, palette),
    ]
    .spacing(10);
    container(column_with_components)
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

/// header
fn header<'a>(state: &'a State) -> Column<'a, Message> {
    let old_button = button(
        text("Left")
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(BASE_SIZE * 7.2)
    .height(BASE_SIZE * 1.6)
    .padding(0)
    .on_press(Message::OldFileSelect);

    let new_button = button(
        text("Right")
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(BASE_SIZE * 7.2)
    .height(BASE_SIZE * 1.6)
    .padding(0)
    .on_press(Message::NewFileSelect);

    column![
        container(row![old_button, text(state.old_filepath.as_str()).size(20)]).width(Fill),
        container(row![new_button, text(state.new_filepath.as_str()).size(20)]).width(Fill),
    ]
    .spacing(10)
}

/// diff viewer
fn diff_viewer<'a>(
    state: &'a State,
    palette: &'a Extended,
    diff_font: &Font,
) -> Container<'a, Message> {
    if state.unified_diff.is_none() {
        return Container::new("").height(Fill);
    }

    let rows = diff_rows(state, palette, diff_font);
    let diff_content = Column::with_children(rows.into_iter().map(Element::from));
    let scrollable_diff_content = scrollable(diff_content);

    container(scrollable_diff_content).height(Fill)
}

/// footer
fn footer<'a>(state: &'a State, palette: &'a Extended) -> Container<'a, Message> {
    if state.unified_diff.is_none() {
        let footer_note = text(FOOTER_NOTE)
            .color(palette.secondary.weak.color)
            .size(BASE_SIZE * 0.75);

        return container(footer_note).align_right(Fill);
    }

    let clear_button = button(
        text("Clear")
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .style(|_, _| button::Style::default().with_background(palette.secondary.weak.color))
    .on_press(Message::Clear);

    let diff_to_clipboard_button = button(
        text(state.copy_to_clipboard_button_label.as_str())
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .style(|_, _| button::Style::default().with_background(palette.success.strong.color))
    .on_press(Message::DiffToClipboard);

    container(row![clear_button, diff_to_clipboard_button].spacing(10)).align_right(Fill)
}

/// generate diff lines
fn diff_rows<'a>(
    state: &'a State,
    palette: &'a Extended,
    diff_font: &Font,
) -> Vec<Row<'a, Message>> {
    let rows: Vec<Row<Message>> = if let Some(unified_diff) = &state.unified_diff {
        unified_diff
            .lines
            .iter()
            .map(|x| {
                let old_str = match x.kind {
                    UnifiedDiffKind::NewContent => "".to_owned(),
                    _ => format!("{}", x),
                };
                let new_str = match x.kind {
                    UnifiedDiffKind::OldContent => "".to_owned(),
                    _ => format!("{}", x),
                };

                let old_text = match x.kind {
                    UnifiedDiffKind::OldTitle | UnifiedDiffKind::NewTitle => {
                        text(old_str).color(palette.secondary.base.color)
                    }
                    UnifiedDiffKind::DiffPos => text(old_str).color(palette.secondary.strong.color),
                    UnifiedDiffKind::OldContent => text(old_str).color(palette.danger.strong.color),
                    _ => text(old_str),
                }
                .font(diff_font.clone());
                let new_text = match x.kind {
                    UnifiedDiffKind::OldTitle | UnifiedDiffKind::NewTitle => {
                        text(new_str).color(palette.secondary.base.color)
                    }
                    UnifiedDiffKind::DiffPos => text(new_str).color(palette.secondary.strong.color),
                    UnifiedDiffKind::NewContent => {
                        text(new_str).color(palette.success.strong.color)
                    }
                    _ => text(new_str),
                }
                .font(diff_font.clone());

                row![
                    column!(container(old_text).width(Fill)),
                    column!(container(new_text).width(Fill))
                ]
            })
            .collect()
    } else {
        vec![row![text(GUIDANCE)]]
    };
    rows
}
