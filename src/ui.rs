use ratatui::{prelude::*, widgets::*};
use crate::{App, app::CurrentMode, LENGTH};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(LENGTH),
            Constraint::Min(1),
            Constraint::Length(LENGTH),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Editor",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let editing_block = Block::default().title("Editing").borders(Borders::ALL);
    let editing_text = Paragraph::new(app.input.clone())
        .block(editing_block)
        .wrap(Wrap { trim: true });
    f.render_widget(editing_text, chunks[1]);

    if let CurrentMode::Exiting = app.current_mode {
       let popup_block = Block::default()
           .title("Y/N")
           .borders(Borders::ALL)
           .style(Style::default().bg(Color::DarkGray));

       let exit_text = Text::styled(
           "Are you sure you want to quit? (y/n)",
           Style::default().fg(Color::Red),
       );
       // the `trim: false` will stop the text from being cut off when over the edge of the block
       let exit_paragraph = Paragraph::new(exit_text)
           .block(popup_block)
           .wrap(Wrap { trim: false });

       let area = centered_rect(60, 25, f.size());
       f.render_widget(Clear, area); //this clears what is directly under the popup
       f.render_widget(exit_paragraph, area);
    }

    //let text_origin = (chunks[1].left() + 1, chunks[1].top() + 1);
    f.set_cursor(app.cursorpos.0, app.cursorpos.1);
}

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
