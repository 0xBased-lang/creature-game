use crate::models::types::Thought;
use crate::thought_viewer::filters::{ThoughtFilter, FilterType};
use chrono::{DateTime, Utc, Duration};
use std::fs;
use std::path::Path;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration as StdDuration;

pub struct ThoughtViewer {
    pub thoughts: Vec<Thought>,
    pub selected_index: usize,
    pub filter: FilterType,
}

impl ThoughtViewer {
    /// Load all thoughts from data/thoughts directory
    pub fn load_from_disk() -> Result<Self, Box<dyn std::error::Error>> {
        let thought_dir = Path::new("data/thoughts");

        // Create directory if it doesn't exist
        if !thought_dir.exists() {
            fs::create_dir_all(thought_dir)?;
        }

        let mut thoughts = Vec::new();

        // Read all JSON files in the directory
        if thought_dir.is_dir() {
            for entry in fs::read_dir(thought_dir)? {
                let entry = entry?;
                let path = entry.path();

                // Only process .json files
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    match Self::load_thought_file(&path) {
                        Ok(thought) => thoughts.push(thought),
                        Err(e) => {
                            eprintln!("Failed to load {:?}: {}", path, e);
                            // Continue loading other files
                        }
                    }
                }
            }
        }

        // Sort by timestamp (newest first)
        thoughts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        println!("Loaded {} thoughts", thoughts.len());

        Ok(Self {
            thoughts,
            selected_index: 0,
            filter: FilterType::All,
        })
    }

    /// Load a single thought from JSON file
    fn load_thought_file(path: &Path) -> Result<Thought, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let thought: Thought = serde_json::from_str(&content)?;
        Ok(thought)
    }

    /// Get currently filtered thoughts
    pub fn filtered_thoughts(&self) -> Vec<&Thought> {
        self.thoughts.iter()
            .filter(|t| self.filter.matches(t))
            .collect()
    }

    /// Get currently selected thought
    pub fn selected_thought(&self) -> Option<&Thought> {
        self.filtered_thoughts().get(self.selected_index).copied()
    }

    /// Navigate up in the list
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Navigate down in the list
    pub fn move_down(&mut self) {
        let max_index = self.filtered_thoughts().len().saturating_sub(1);
        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }

    /// Reset selection when filter changes
    pub fn set_filter(&mut self, filter: FilterType) {
        self.filter = filter;
        self.selected_index = 0; // Reset to top
    }

    /// Get statistics
    pub fn stats(&self) -> ViewerStats {
        let total = self.thoughts.len();
        let filtered = self.filtered_thoughts().len();

        let avg_relevance = if !self.thoughts.is_empty() {
            self.thoughts.iter()
                .map(|t| t.relevance_score)
                .sum::<f64>() / total as f64
        } else {
            0.0
        };

        let unique_tags: std::collections::HashSet<_> = self.thoughts.iter()
            .flat_map(|t| &t.context_tags)
            .collect();

        ViewerStats {
            total_thoughts: total,
            filtered_count: filtered,
            avg_relevance,
            unique_tags: unique_tags.len(),
        }
    }

    /// Render the thought viewer UI
    pub fn render(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35), // Left: Thought list
                Constraint::Percentage(65), // Right: Detail view
            ])
            .split(f.size());

        self.render_thought_list(f, chunks[0]);
        self.render_thought_detail(f, chunks[1]);
    }

    fn render_thought_list(&self, f: &mut Frame, area: Rect) {
        let thoughts = self.filtered_thoughts();

        let items: Vec<ListItem> = thoughts.iter()
            .enumerate()
            .map(|(i, thought)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                // Color code by relevance
                let relevance_color = if thought.relevance_score >= 0.8 {
                    Color::Green
                } else if thought.relevance_score >= 0.5 {
                    Color::Yellow
                } else {
                    Color::Red
                };

                let time_str = thought.timestamp.format("%m/%d %H:%M").to_string();
                let preview = thought.content.chars().take(40).collect::<String>();
                let truncated = if thought.content.len() > 40 { "..." } else { "" };

                let line = Line::from(vec![
                    Span::styled(
                        format!("{} ", time_str),
                        Style::default().fg(Color::DarkGray)
                    ),
                    Span::styled(
                        format!("[{:.2}]", thought.relevance_score),
                        Style::default().fg(relevance_color)
                    ),
                    Span::raw(" "),
                    Span::raw(format!("{}{}", preview, truncated)),
                ]);

                ListItem::new(line).style(style)
            })
            .collect();

        let title = format!(
            "Thoughts ({}/{}) - [{}]",
            thoughts.len(),
            self.thoughts.len(),
            self.filter_description()
        );

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title));

        f.render_widget(list, area);
    }

    fn render_thought_detail(&self, f: &mut Frame, area: Rect) {
        if let Some(thought) = self.selected_thought() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(6),   // Metadata
                    Constraint::Min(10),     // Content
                    Constraint::Length(5),   // ASCII art (if exists)
                ])
                .split(area);

            // Metadata
            self.render_metadata(f, chunks[0], thought);

            // Content
            self.render_content(f, chunks[1], thought);

            // ASCII visualization (if exists)
            if thought.ascii_visualization.is_some() {
                self.render_ascii(f, chunks[2], thought);
            }
        } else {
            let text = vec![
                Line::from(""),
                Line::from("No thoughts to display."),
                Line::from(""),
                Line::from("Run the colony simulation to generate thoughts,"),
                Line::from("or add sample thought JSON files to data/thoughts/"),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Detail"))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(paragraph, area);
        }
    }

    fn render_metadata(&self, f: &mut Frame, area: Rect, thought: &Thought) {
        let lines = vec![
            Line::from(vec![
                Span::styled("Timestamp: ", Style::default().fg(Color::Gray)),
                Span::raw(thought.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()),
            ]),
            Line::from(vec![
                Span::styled("Relevance: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.2}", thought.relevance_score),
                    Style::default().fg(if thought.relevance_score >= 0.8 {
                        Color::Green
                    } else if thought.relevance_score >= 0.5 {
                        Color::Yellow
                    } else {
                        Color::Red
                    })
                ),
                Span::raw("  |  "),
                Span::styled("Confidence: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.2}", thought.confidence_score),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::styled("Tags: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    thought.context_tags.join(", "),
                    Style::default().fg(Color::Magenta)
                ),
            ]),
            Line::from(vec![
                Span::styled("Factors: ", Style::default().fg(Color::Gray)),
                Span::raw(thought.real_time_factors.join(", ")),
            ]),
        ];

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Metadata"));
        f.render_widget(paragraph, area);
    }

    fn render_content(&self, f: &mut Frame, area: Rect, thought: &Thought) {
        let text = vec![Line::from(thought.content.as_str())];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Content"))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }

    fn render_ascii(&self, f: &mut Frame, area: Rect, thought: &Thought) {
        if let Some(ascii) = &thought.ascii_visualization {
            let paragraph = Paragraph::new(ascii.as_str())
                .block(Block::default().borders(Borders::ALL).title("Visualization"))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(paragraph, area);
        }
    }

    fn filter_description(&self) -> String {
        match &self.filter {
            FilterType::All => "All".to_string(),
            FilterType::ByTag(tag) => format!("Tag: {}", tag),
            FilterType::HighRelevance(t) => format!("Rel >= {:.2}", t),
            FilterType::Recent(d) => format!("Last {}h", d.num_hours()),
            FilterType::HighConfidence(t) => format!("Conf >= {:.2}", t),
            FilterType::Search(q) => format!("Search: {}", q),
        }
    }

    /// Handle keyboard input
    pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(StdDuration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(false); // Exit
                    }

                    KeyCode::Up => {
                        self.move_up();
                    }

                    KeyCode::Down => {
                        self.move_down();
                    }

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        // Reload thoughts from disk
                        *self = Self::load_from_disk()?;
                    }

                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        // Export current filtered thoughts
                        self.export_current()?;
                    }

                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn export_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::thought_viewer::export::{export_thoughts, ExportFormat};
        use chrono::Utc;

        let thoughts = self.filtered_thoughts();

        if thoughts.is_empty() {
            return Ok(()); // Nothing to export
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("thoughts_export_{}.md", timestamp);

        export_thoughts(&thoughts, &filename, ExportFormat::Markdown)?;

        println!("✓ Exported {} thoughts to {}", thoughts.len(), filename);

        Ok(())
    }
}

pub struct ViewerStats {
    pub total_thoughts: usize,
    pub filtered_count: usize,
    pub avg_relevance: f64,
    pub unique_tags: usize,
}
