use crate::tui::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, chunks[0], app);

    match app.current_view {
        View::MarketList => render_market_list(f, chunks[1], app),
        View::OrderBook => render_split_view(f, chunks[1], app),
    }

    render_footer(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let title = match app.current_view {
        View::MarketList => " Hyperliquid Market Data ",
        View::OrderBook => " Order Book ",
    };

    let header = Block::default()
        .borders(Borders::ALL)
        .title(title);

    f.render_widget(header, area);
}

fn render_market_list(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if let Some(ref mids) = app.all_mids {
        let rows: Vec<Row> = mids
            .iter()
            .enumerate()
            .map(|(idx, (coin, price))| {
                let style = if idx == app.market_list_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(coin.as_str()),
                    Cell::from(price.to_string()),
                ]).style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [Constraint::Percentage(30), Constraint::Percentage(70)],
        )
        .header(
            Row::new(vec!["Coin", "Mid Price"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title(" Markets "));

        f.render_widget(table, area);
    } else {
        let loading = Block::default()
            .borders(Borders::ALL)
            .title(" Loading... ");
        f.render_widget(loading, area);
    }
}

fn render_split_view(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    render_market_list(f, chunks[0], app);
    render_orderbook(f, chunks[1], app);
}

fn render_orderbook(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if let Some(ref book) = app.orderbook {
        let title = if let Some(ref coin) = app.selected_coin {
            format!(" {} Order Book ", coin)
        } else {
            " Order Book ".to_string()
        };

        // Split into asks and bids
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        // Render asks (top 10)
        let ask_rows: Vec<Row> = book.levels.1.iter()
            .filter(|level| level.n > 0)
            .take(10)
            .map(|level| {
                Row::new(vec![
                    Cell::from(level.px.to_string()).style(Style::default().fg(Color::Red)),
                    Cell::from(level.sz.to_string()),
                ])
            })
            .collect();

        let asks_table = Table::new(
            ask_rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(
            Row::new(vec!["Ask Price", "Size"])
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
.block(Block::default().borders(Borders::ALL).title(title.as_str()));

        f.render_widget(asks_table, chunks[0]);

        // Render bids (top 10)
        let bid_rows: Vec<Row> = book.levels.0.iter()
            .filter(|level| level.n < 0)
            .take(10)
            .map(|level| {
                Row::new(vec![
                    Cell::from(level.px.to_string()).style(Style::default().fg(Color::Green)),
                    Cell::from(level.sz.to_string()),
                ])
            })
            .collect();

        let bids_table = Table::new(
            bid_rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(
            Row::new(vec!["Bid Price", "Size"])
                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(bids_table, chunks[1]);
    } else {
        let loading = Block::default()
            .borders(Borders::ALL)
            .title(" Select a coin and press Enter ");
        f.render_widget(loading, area);
    }
}

fn render_footer(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let footer_text = if let Some(ref error) = app.error_message {
        vec![
            Span::styled("Error: ", Style::default().fg(Color::Red)),
            Span::raw(error),
        ]
    } else {
        vec![
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" quit | "),
            Span::styled("↑↓/jk", Style::default().fg(Color::Yellow)),
            Span::raw(" navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" view book | "),
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::raw(" switch view | "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(" refresh"),
        ]
    };

    let footer = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(footer_text));

    f.render_widget(footer, area);
}
