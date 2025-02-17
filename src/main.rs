use iced::widget::{button, column, container, row, scrollable, text, Button, Column};
use iced::{Element, Fill, Length, Theme};
use rfd::FileDialog;
use sheets_diff::core::diff::{Diff, UnifiedDiff, UnifiedDiffKind};

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

pub fn main() -> iced::Result {
    iced::run("Quick Diff Me", update, view)
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

    let old_rows: Vec<Element<Message>> = if let Some(unified_diff) = &state.unified_diff {
        unified_diff
            .lines
            .iter()
            .filter(|x| match x.kind {
                UnifiedDiffKind::NewTitle => false,
                _ => true,
            })
            .map(|x| {
                let str = match x.kind {
                    UnifiedDiffKind::NewContent => "".to_owned(),
                    _ => format!("{}", x),
                };
                let txt = text(str);
                let col = container(txt);
                let styled = match x.kind {
                    UnifiedDiffKind::OldTitle => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.secondary.base.color)
                    }),
                    UnifiedDiffKind::DiffPos => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.secondary.strong.color)
                    }),
                    UnifiedDiffKind::OldContent => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.danger.strong.color)
                    }),
                    _ => col,
                };
                styled.into()
            })
            .collect()
    } else {
        vec![container(text("(none)")).into()]
    };
    let new_rows: Vec<Element<Message>> = if let Some(unified_diff) = &state.unified_diff {
        unified_diff
            .lines
            .iter()
            .filter(|x| match x.kind {
                UnifiedDiffKind::OldTitle => false,
                _ => true,
            })
            .map(|x| {
                let str = match x.kind {
                    UnifiedDiffKind::OldContent => "".to_owned(),
                    _ => format!("{}", x),
                };
                let txt = text(str);
                let col = container(txt);
                let styled = match x.kind {
                    UnifiedDiffKind::NewTitle => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.secondary.base.color)
                    }),
                    UnifiedDiffKind::DiffPos => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.secondary.strong.color)
                    }),
                    UnifiedDiffKind::NewContent => col.style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style::default().color(palette.success.strong.color)
                    }),
                    _ => col,
                };
                styled.into()
            })
            .collect()
    } else {
        vec![container(text("(none)")).into()]
    };

    let left_pane = container(Column::with_children(old_rows))
        .width(Length::Fill)
        // .height(Length::Fill)
        // .style(|theme: &Theme| {
        //     let palette = theme.extended_palette();
        //     container::Style::default().background(palette.background.base.color)
        // })
        ;
    let right_pane = container(Column::with_children(new_rows))
        .width(Length::Fill)
        // .height(Length::Fill)
        // .style(|theme: &Theme| {
        //     let palette = theme.extended_palette();
        //     container::Style::default().background(palette.background.base.color)
        // })
        ;

    let scrollable_helper = scrollable(row![left_pane, right_pane].spacing(10));
    let diff_viewer: container::Container<'_, Message> =
        container(scrollable_helper).height(Length::Fill);

    let bottom =
        container("Thanks for using this app !! Repo: https://github.com/nabbisen/quick-diff-me")
            .align_right(Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style::default().color(palette.secondary.weak.color)
            });

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
