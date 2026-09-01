use iced::widget::{column, container, row, text};
use iced::{Element, Task};

#[derive(Debug, Clone)]
enum Message {}

struct State;

fn new() -> (State, Task<Message>) {
    (State, Task::none())
}

fn update(_state: &mut State, _message: Message) -> Task<Message> {
    Task::none()
}

fn view(_state: &State) -> Element<'_, Message> {
    let icon = |svg| {
        container(svg)
            .width(48)
            .height(48)
            .padding(8)
            .style(container::rounded_box)
    };

    let row = row![
        icon(lucide_iced::icon::heart().width(32).height(32)),
        icon(lucide_iced::icon::settings().width(32).height(32)),
        icon(lucide_iced::icon::trash_2().width(32).height(32)),
        icon(lucide_iced::icon::user().width(32).height(32)),
        icon(lucide_iced::icon::search().width(32).height(32)),
        icon(lucide_iced::icon::star().width(32).height(32)),
        icon(
            lucide_iced::mirror_svg(lucide_iced::bytes::HEART)
                .width(32)
                .height(32)
        ),
    ]
    .spacing(12);

    column![text("Lucide icons rendered via lucide-iced").size(24), row,]
        .spacing(24)
        .padding(24)
        .into()
}

pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .title("lucide-iced demo")
        .run()
}
