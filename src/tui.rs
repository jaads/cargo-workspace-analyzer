use crate::metrics::CouplingMetric;
use crate::{CouplingMetricsWidget, SimpleMetrics};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::{Line, Modifier, Style, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug)]
pub struct TUI {
    pub(crate) exit: bool,
    pub(crate) simple_metrics: SimpleMetrics,
    pub(crate) coupling_metrics: CouplingMetric,
    pub(crate) scroll_offset: usize,
}

impl TUI {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        let inner_area = frame.area().inner(Margin {
            horizontal: 2,
            vertical: 2,
        });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20), // Header
                Constraint::Percentage(80), // Main Content
            ])
            .split(inner_area); // Use inner area here!

        // Render inner widgets inside the main block
        frame.render_widget(&self.simple_metrics, chunks[0]);

        // Render Coupling Metrics
        let coupling_widget = CouplingMetricsWidget {
            metrics: &self.coupling_metrics,
            scroll_offset: self.scroll_offset,
        };

        frame.render_widget(&coupling_widget, chunks[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            _ => {}
        }
    }

    fn scroll_down(&mut self) {
        let row_count = self.coupling_metrics.len();
        let max_visible_rows = 10; // Change this based on available space

        if self.scroll_offset + max_visible_rows < row_count {
            self.scroll_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &TUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Cargo Workspace Analyzer".bold());
        let instructions = Line::from(vec![
            " Quit: ".into(),
            "<Q> ".blue().bold(),
            " | Scroll: ".into(),
            "↑ ↓ ".yellow().bold(),
        ]);
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);
    }
}

impl Widget for &SimpleMetrics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Min(30)])
            .split(area);

        let package_block = Block::bordered()
            .title(" 📦 Package Count ")
            .border_set(border::PLAIN);

        Paragraph::new(vec![
            Line::from(vec![
                "Total: ".into(),
                self.total_packages.to_string().yellow(),
            ]),
            Line::from(vec![
                "Workspace: ".into(),
                self.workspace_packages.to_string().yellow(),
            ]),
        ])
        .block(package_block)
        .alignment(ratatui::layout::Alignment::Right)
        .render(chunks[0], buf);

        // Dependency Count Block (Right)
        let dep_block = Block::bordered()
            .title(" 🔗 Dependency Count ")
            .border_set(border::PLAIN);

        Paragraph::new(vec![
            Line::from(vec![
                "Total: ".into(),
                self.total_dependencies.to_string().yellow(),
            ]),
            Line::from(vec![
                "Workspace: ".into(),
                self.workspace_dependencies.to_string().yellow(),
            ]),
        ])
        .block(dep_block)
        .alignment(ratatui::layout::Alignment::Right)
        .render(chunks[1], buf);
    }
}

impl<'a> Widget for &CouplingMetricsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(" 📊 Coupling Metrics ")
            .borders(Borders::ALL)
            .render(area, buf);

        // Define column constraints for table widths
        let column_constraints = [
            Constraint::Percentage(40), // Package Name
            Constraint::Percentage(20), // Ce
            Constraint::Percentage(20), // Ca
            Constraint::Percentage(20), // Instability
        ];

        // Define the table headers
        let header_cells = [
            "Package",
            "Efferent Coupling",
            "Afferent Coupling",
            "Instability",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        // Limit rows based on scroll_offset
        let max_visible_rows = (area.height as usize).saturating_sub(3); // Header + padding
        let rows: Vec<Row> = self
            .metrics
            .iter()
            .skip(self.scroll_offset) // Scroll offset determines starting row
            .take(max_visible_rows) // Limit number of visible rows
            .map(|(package, (ce, ca, instability))| {
                Row::new(vec![
                    Cell::from(package.as_str()),
                    Cell::from(ce.to_string()),
                    Cell::from(ca.to_string()),
                    Cell::from(format!("{:.2}", instability)),
                ])
            })
            .collect();

        Table::new(rows, &column_constraints)
            .header(header)
            .block(Block::bordered().borders(Borders::ALL))
            .column_spacing(1)
            .render(area, buf);
    }
}
