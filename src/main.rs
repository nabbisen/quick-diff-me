use iced::widget::{button, column, container, row, scrollable, text, Button, Column, Row};
use iced::{Element, Fill, Font, Length, Theme};
use rfd::FileDialog;
use sheets_diff::core::diff::{Diff, UnifiedDiff, UnifiedDiffKind};

mod font;
use font::app_font;

#[derive(Debug, Clone)]
enum Message {
    OldFileSelect,
    NewFileSelect,
}

#[derive(Default)]
struct State {
    old_filepath: String,
    new_filepath: String,
    unified_diff: Option<UnifiedDiff>,
}

const BASE_SIZE: f32 = 16.0;

const APP_THEME: Theme = Theme::Dark;

pub fn main() -> iced::Result {
    let app = iced::application("Quick Diff Me", update, view)
        .default_font(Font::with_name(app_font()))
        .theme(|_state| APP_THEME);
    app.run()
}

fn diff(state: &mut State) {
    if state.old_filepath.is_empty()
        || state.new_filepath.is_empty()
        || state.old_filepath == state.new_filepath
    {
        return;
    }
    let unified_diff =
        Diff::new(state.old_filepath.as_str(), state.new_filepath.as_str()).unified_diff();
    state.unified_diff = Some(unified_diff)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::OldFileSelect => {
            let selected = FileDialog::new()
                .add_filter("Excel", &["xlsx"])
                // todo: file dialog default path
                .set_directory("~/Desktop")
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.old_filepath = selected;
                diff(state);
            }
        }
        Message::NewFileSelect => {
            let selected = FileDialog::new()
                .add_filter("Excel", &["xlsx"])
                // todo: file dialog default path
                .set_directory("~/Desktop")
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.new_filepath = selected;
                diff(state);
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let palette = APP_THEME.extended_palette();

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

    let rows: Vec<Row<Message>> = if let Some(unified_diff) = &state.unified_diff {
        unified_diff
            .lines
            .iter()
            .map(|x| {
                let old_str = match x.kind {
                    UnifiedDiffKind::OldContent => "".to_owned(),
                    _ => format!("{}", x),
                };
                let new_str = match x.kind {
                    UnifiedDiffKind::NewContent => "".to_owned(),
                    _ => format!("{}", x),
                };

                let old_text = match x.kind {
                    UnifiedDiffKind::OldTitle | UnifiedDiffKind::NewTitle => {
                        text(old_str).color(palette.secondary.base.color)
                    }
                    UnifiedDiffKind::DiffPos => text(old_str).color(palette.secondary.strong.color),
                    UnifiedDiffKind::OldContent => text(old_str).color(palette.danger.strong.color),
                    _ => text(old_str),
                };
                let new_text = match x.kind {
                    UnifiedDiffKind::OldTitle | UnifiedDiffKind::NewTitle => {
                        text(new_str).color(palette.secondary.base.color)
                    }
                    UnifiedDiffKind::DiffPos => text(new_str).color(palette.secondary.strong.color),
                    UnifiedDiffKind::NewContent => {
                        text(new_str).color(palette.success.strong.color)
                    }
                    _ => text(new_str),
                };

                row![
                    column!(container(old_text).width(Fill)),
                    column!(container(new_text).width(Fill))
                ]
            })
            .collect()
    } else {
        vec![row![text("(none)")]]
    };

    let diff_content = Column::with_children(rows.into_iter().map(Element::from));
    let scrollable_helper = scrollable(diff_content);
    let diff_viewer: container::Container<'_, Message> =
        container(scrollable_helper).height(Length::Fill);

    let bottom = container(
        text("Thanks for using this app !! Repo: https://github.com/nabbisen/quick-diff-me")
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
