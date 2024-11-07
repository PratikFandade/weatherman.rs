use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{display_weather_info, App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Welcome to Weather Station ☁️",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);
    let mut main_list_items = Vec::<ListItem>::new();
    let mut list_items = Vec::<ListItem>::new();

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);

    for (i, _row) in app.countries.iter().enumerate() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}, {}", app.cities[i], app.countries[i]),
            Style::default().fg(Color::Yellow),
        ))));
    }

    for (i, _row) in app.countries.iter().enumerate() {
        if let Some(weather) = app.weather.get(i) {
            let weather_info = display_weather_info(weather);
            main_list_items.push(ListItem::new(Line::from(Span::styled(
                weather_info,
                Style::default().fg(app.colors[i]),
            ))));
        }
    }

    let main_list = List::new(main_list_items).block(Block::default().borders(Borders::ALL));
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, body_chunks[0]);
    frame.render_widget(main_list, body_chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Country => {
                        Span::styled("Editing Country", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::City => {
                        Span::styled("Editing City", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to get weather of new city",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to get weather of new city",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(_) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new Country and City pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let countries_list: Vec<ListItem> = app
            .countries_list
            .iter()
            .enumerate()
            .map(|(i, &ref country)| {
                let content = if i == app.selected_country {
                    Span::styled(
                        country,
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(country)
                };
                ListItem::new(Line::from(content))
            })
            .collect();
        let list = List::new(countries_list).block(Block::default());
        frame.render_widget(list, popup_chunks[0]);

        let cities_list: Vec<ListItem> = app
            .cities_list
            .iter()
            .enumerate()
            .map(|(i, &ref city)| {
                let content = if i == app.selected_city {
                    Span::styled(
                        city,
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(city)
                };
                ListItem::new(Line::from(content))
            })
            .collect();
        let cities_list = List::new(cities_list).block(Block::default());
        frame.render_widget(cities_list, popup_chunks[1]);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area());
        let popup_block = Block::default()
            .title("Leaving? 😢")
            .borders(Borders::ALL)
            .style(Style::default());

        let exit_text = Text::styled(
            "Would you like to exit (y/n)",
            Style::default().fg(Color::White),
        );
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
