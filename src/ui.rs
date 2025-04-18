use chrono::DateTime;
use cleverlib::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        block::{self},
        Block, Borders, Clear, List, ListDirection, Paragraph, Row, Table,
    },
    Frame,
};
use std::vec;

struct Detail {
    timestap: String,
    message: String,
    level: String,
    exception: String,
    event_id: String,
}

use crate::app::{App, AppState};

pub fn render(app: &mut App, f: &mut Frame) {
    match app.app_state {
        AppState::ITERATING => {
            let widths = [Constraint::Length(30), Constraint::Percentage(100)];
            let mut clef_rows: Vec<(&Event, Row)> = vec![];
            let main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(f.area());

            let detail_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(79)])
                .split(main[1]);

            for line in app.event_collection.events.iter() {
                if !app.event_types.is_empty() {
                    let event_level = line.level.clone().unwrap_or_default().to_string();
                    if app
                        .event_types
                        .iter()
                        .any(|level| level.value == event_level && level.selected)
                    {
                        // TODO: Here I need to parse the line into a Ratatui Row
                        let date_time = DateTime::parse_from_rfc3339(
                            line.time.clone().unwrap_or_default().as_ref(),
                        );

                        let row = Row::new(vec![
                            format!(
                                "[{}] {}",
                                date_time.unwrap().format("%Y/%m/%d %H:%M:%S"),
                                event_level
                            ),
                            line.message.clone().unwrap_or_default().to_string(),
                        ]);
                        clef_rows.push((line, row));
                    }
                }
            }

            if !clef_rows.is_empty() {
                let mut selected_row_index = app.event_table_state.selected().unwrap();
                let selected_row: &Event = match clef_rows.get(selected_row_index) {
                    None => {
                        app.event_table_state.select(Some(0));
                        selected_row_index = 0;
                        clef_rows.first().unwrap().0
                    }
                    Some(val) => val.0,
                };
                let selection_text = format!("{}|{}", selected_row_index, clef_rows.len() - 1);

                let detail: Detail = Detail {
                    timestap: selected_row.time.clone().unwrap().to_string(),
                    message: selected_row.message.clone().unwrap().to_string(),
                    level: selected_row.level.clone().unwrap_or_default().to_string(),
                    exception: selected_row
                        .exception
                        .clone()
                        .unwrap_or_default()
                        .to_string(),
                    event_id: selected_row.eventid.clone().unwrap_or_default().to_string(),
                };

                let table = Table::new(clef_rows.iter().map(|v| v.1.clone()), widths)
                    .column_spacing(0)
                    .header(Row::new(vec!["Time|Level", "Message"]).style(Style::new().bold()))
                    .block(
                        Block::default()
                            .title("Clever")
                            .title_top(Line::from(app.file_path.as_str()).left_aligned())
                            .title_bottom(Line::from(selection_text.as_str()).centered())
                            .title_position(ratatui::widgets::block::Position::Top)
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Rounded)
                            .title_style(Style::default().fg(ratatui::style::Color::White)),
                    )
                    .style(Style::default().fg(ratatui::style::Color::White))
                    .row_highlight_style(Style::default().reversed());
                f.render_stateful_widget(table, main[0], &mut app.event_table_state);

                let status_details = Paragraph::new(format!(
                    " Message: {} \n Expcetion: {}\n EventID: {}",
                    detail.message, detail.exception, detail.event_id
                ))
                .style(Style::default().fg(ratatui::style::Color::White));

                f.render_widget(status_details, detail_area[1]);
                let empty_log_paragraph = Paragraph::new(String::from("Nothing to show..."))
                    .style(Style::new().fg(Color::White));
                f.render_widget(empty_log_paragraph, main[1]);
                let date_time = DateTime::parse_from_rfc3339(&detail.timestap.clone());

                let stats = Block::default()
                    .borders(Borders::ALL)
                    .title_top(Line::from(format!(
                        "{} | {}",
                        detail.level,
                        date_time.unwrap().format("%Y/%m/%d %H:%M:%S")
                    )))
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Move:'↓↑/jk' Quit:'q' Filter:'f' Jump:'Tap' ")
                    .title_position(ratatui::widgets::block::Position::Bottom)
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .title_alignment(ratatui::layout::Alignment::Left)
                    .title_style(Style::default().fg(ratatui::style::Color::White))
                    .border_style(Style::default().fg(ratatui::style::Color::White))
                    .style(Style::default());
                f.render_widget(stats, main[1]);
            }
        }
        AppState::FILTERING => {
            f.render_widget(Clear, f.area());
            let area = centered_rect(40, 30, f.area());
            let type_list: Vec<String> = app
                .event_types
                .iter()
                .map(|t| {
                    let text = if t.value.is_empty() {
                        if t.selected {
                            String::from("* Empty Log Level")
                        } else {
                            String::from("Empty Log Level")
                        }
                    } else {
                        t.to_string()
                    };
                    text.to_string()
                })
                .collect();
            let list = List::new(type_list)
                .block(
                    Block::default()
                        .title("Event Levels")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(block::BorderType::Rounded)
                        .title_bottom(Line::from("Select: Spc | Close: F").centered()),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .style(Style::default().fg(Color::White))
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);
            f.render_stateful_widget(list, area, &mut app.filter_list_state);
        }
    }

    // ANCHOR: centered_rect
    // helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}
