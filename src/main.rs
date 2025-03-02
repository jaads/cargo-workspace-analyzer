#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::arguments::get_args;
use crate::diagram_creation::create_diagram;
use crate::exporter::generate_mermaid_png;
use crate::manifests_collector::get_dependency_graph;
use crate::metrics::CouplingMetric;
use crate::package_counter::count_packages;
use std::io;
use std::path::Path;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

mod arguments;
mod dependency_filter;
mod diagram_creation;
mod exporter;
mod graph;
mod manifests_collector;
mod metrics;
mod package_counter;
mod types;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let args = get_args();
    let amount_of_packages = count_packages(&args.directory);

    // load filtered manifests
    let graph = get_dependency_graph(Path::new(&args.directory));

    // filter dependencies to only include references to workspace members
    let filtered = graph.filter_dependencies();

    let mut app = App {
        exit: false,
        simple_metrics: SimpleMetrics {
            simple_amount: amount_of_packages,
            total_packages: graph.get_node_count(),
            workspace_packages: filtered.get_node_count(),
            total_dependencies: graph.get_edge_count(),
            workspace_dependencies: filtered.get_edge_count(),
        },
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    simple_metrics: SimpleMetrics,
}

impl App {
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
                Constraint::Percentage(30), // Header
                Constraint::Percentage(40), // Main Content
                Constraint::Percentage(30), // Footer
            ])
            .split(inner_area); // Use inner area here!

        // Render inner widgets inside the main block
        frame.render_widget(&self.simple_metrics, chunks[0]);
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
            // KeyCode::Left => self.decrement_counter(),
            // KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Cargo Workspace Analyzer".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);
    }
}

#[derive(Debug)]
pub struct SimpleMetrics {
    simple_amount: usize,
    total_packages: usize,
    workspace_packages: usize,
    total_dependencies: usize,
    workspace_dependencies: usize,
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

// remaining parts from the old main
// fn main() {
//
//     // create diagram, incl. highlights of circular deps
//     let result = create_diagram(&filtered);
//
//     // save to file if needed
//     if args.no_file {
//         println!("{}", result);
//     } else {
//         generate_mermaid_png(&result);
//     }
//
//     // calculate and print the metrics
//     let metrics = filtered.calculate_coupling();
//     print_coupling(metrics);
// }
