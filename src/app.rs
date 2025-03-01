use iced::theme::palette::Extended;
use iced::widget::{button, column, container, row, scrollable, text, Button, Column, Row};
use iced::{event, Element, Fill, Font, Length, Subscription};
use sheets_diff::core::diff::UnifiedDiffKind;

use crate::core::consts::{APP_THEME, BASE_SIZE, FOOTER_NOTE, GUIDANCE};
use crate::core::diff::diff;
use crate::core::font::app_font;
use crate::core::types::{Message, State};
use crate::core::utils::file_dialog;

/// iced update function
pub fn update(state: &mut State, message: Message) {
    let file_dialog = file_dialog(state, &message);

    match message {
        Message::EventOccurred(event) => match event {
            iced::Event::Window(event) => match event {
                iced::window::Event::FileDropped(path) => {
                    if !path.extension().is_some_and(|x| x.to_str() == Some("xlsx")) {
                        return;
                    }

                    if state.old_filepath.is_empty() {
                        state.old_filepath = path.to_string_lossy().to_string();
                    } else if state.new_filepath.is_empty() {
                        state.new_filepath = path.to_string_lossy().to_string();
                    } else {
                        state.old_filepath = path.to_string_lossy().to_string();
                        state.new_filepath = String::new();
                    }
                    diff(state);
                }
                _ => (),
            },
            _ => (),
        },
        Message::OldFileSelect => {
            let selected = file_dialog
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.old_filepath = selected;
                diff(state);
            }
        }
        Message::NewFileSelect => {
            let selected = file_dialog
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.new_filepath = selected;
                diff(state);
            }
        }
    }
}

/// iced view function
pub fn view(state: &State) -> Element<Message> {
    let palette = APP_THEME.extended_palette();
    let diff_text_font = Font::with_name(app_font());

    let old_button: Button<Message> = button(
        text("Left")
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(BASE_SIZE * 7.2)
    .height(BASE_SIZE * 1.6)
    .padding(0)
    // .style(|theme: &Theme, status| {
    //     let palette = theme.extended_palette();
    //     button::Style::default().with_background(palette.danger.strong.color)
    // })
    .on_press(Message::OldFileSelect);
    let new_button: Button<Message> = button(
        text("Right")
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(BASE_SIZE * 7.2)
    .height(BASE_SIZE * 1.6)
    .padding(0)
    .on_press(Message::NewFileSelect);

    let rows = diff_rows(&state, palette, diff_text_font);

    let diff_content = Column::with_children(rows.into_iter().map(Element::from));
    let scrollable_helper = scrollable(diff_content);
    let diff_viewer: container::Container<'_, Message> =
        container(scrollable_helper).height(Length::Fill);

    let bottom = container(
        text(FOOTER_NOTE)
            .color(palette.secondary.weak.color)
            .size(BASE_SIZE * 0.75),
    )
    .align_right(Fill);

    container(
        column![
            column![
                row![old_button, text(state.old_filepath.as_str()).size(20)],
                row![new_button, text(state.new_filepath.as_str()).size(20)],
            ]
            .spacing(10),
            diff_viewer,
            bottom,
        ]
        .spacing(10),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn subscription(_: &State) -> Subscription<Message> {
    event::listen().map(Message::EventOccurred)
}

fn diff_rows<'a>(
    state: &'a State,
    palette: &'a Extended,
    diff_text_font: Font,
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
                .font(diff_text_font);
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
                .font(diff_text_font);

                row![
                    column!(container(old_text).width(Fill)),
                    column!(container(new_text).width(Fill)) // .style(|_| {
                                                             //     Style {
                                                             //         text_color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
                                                             //         ..Default::default()
                                                             //     }
                                                             // })
                ]
            })
            .collect()
    } else {
        vec![row![text(GUIDANCE)]]
    };
    rows
}
