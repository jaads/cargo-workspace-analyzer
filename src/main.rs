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
        simple_amount: amount_of_packages,
        total_packages: graph.get_node_count(),
        workspace_packages: filtered.get_node_count(),
        total_dependencies: graph.get_edge_count(),
        workspace_dependencies: filtered.get_edge_count(),
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    simple_amount: usize,
    total_packages: usize,
    workspace_packages: usize,
    total_dependencies: usize,
    workspace_dependencies: usize,
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
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let total_package_amount = Text::from(vec![
            Line::from(vec![
                "Simple count: ".into(),
                self.simple_amount.to_string().yellow(),
            ]),
            Line::from(vec![
                "Total packages: ".into(),
                self.total_packages.to_string().yellow(),
            ]),
            Line::from(vec![
                "Workspace packages: ".into(),
                self.workspace_packages.to_string().yellow(),
            ]),
            Line::from(vec![
                "Total dependencies: ".into(),
                self.total_dependencies.to_string().yellow(),
            ]),
            Line::from(vec![
                "Workspace dependencies: ".into(),
                self.workspace_dependencies.to_string().yellow(),
            ]),
        ]);

        Paragraph::new(total_package_amount)
            .centered()
            .block(block)
            .render(area, buf);
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

pub fn print_coupling(metrics: CouplingMetric) {
    if metrics.is_empty() {
        println!("No packages found in the graph.");
        return;
    }

    println!();
    println!("Metrics:");
    println!();
    println!(
        "{:<40} {:<20} {:<20} {:<10}",
        "Package", "Efferent Coupling", "Afferent Coupling", "Instability"
    );
    println!("{:-<95}", ""); // Divider line

    for (package, (ce, ca, instability)) in metrics {
        println!("{:<40} {:<20} {:<20} {:.2}", package, ce, ca, instability);
    }
}
