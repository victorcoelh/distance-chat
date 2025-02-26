use iced::widget::container;
use iced::widget::text_input;
use iced::widget::Column;
use iced::widget::{button, column};
use iced::widget::{text, Row};
use iced::Element;
use iced::Length;

use super::state::{ChatData, UIMessage};

pub fn main_view(chat_data: &ChatData) -> Element<UIMessage> {
    let pos_label = text("Update Position:");
    let textbox = text_input("New Position", &chat_data.position_text)
        .on_input(UIMessage::PositionBoxChanged)
        .on_submit(UIMessage::UpdatePosition)
        .width(200.0);

    let spacing_a = iced::widget::Space::new(0.0, 40.0);
    let spacing_b = iced::widget::Space::new(0.0, 40.0);

    let clients_label = text("Available Clients");
    let mut row = Row::new()
        .spacing(20.0)
        .align_y(iced::Alignment::Center)
        .height(50.0);

    for available_client in chat_data.current_clients.iter() {
        if *available_client == chat_data.name {
            continue;
        }

        let button: button::Button<'_, UIMessage> =
            button(text(available_client)).on_press(UIMessage::FocusOn(available_client.clone()));

        row = row.push(button);
    }

    let column = column![
        pos_label,
        textbox,
        spacing_a,
        clients_label,
        row,
        spacing_b,
        chatbox_widget(chat_data)
    ]
    .width(400.0)
    .align_x(iced::Alignment::Center);

    container(column).center(Length::Fill).into()
}

fn chatbox_widget(chat_data: &ChatData) -> Element<UIMessage> {
    let mut messages_text = Column::new()
        .spacing(1.0)
        .width(Length::Fill)
        .align_x(iced::Alignment::Start);

    if let Some(target) = chat_data.current_target.clone() {
        for text_message in chat_data.chats[&target].iter() {
            messages_text = messages_text.push(text(text_message))
        }
    }

    let chat_messages_widget = container(messages_text)
        .height(250)
        .padding(10)
        .center_x(200.0)
        .width(Length::Fill)
        .style(container::rounded_box);

    let chatbox = text_input("Type here...", &chat_data.chatbox_text)
        .on_input(UIMessage::ChatBoxChanged)
        .on_submit(UIMessage::SendMsg);

    column![chat_messages_widget, chatbox]
        .align_x(iced::Alignment::Center)
        .into()
}
