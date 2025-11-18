# 🎯 DIMENSIONAL SLIDERS - Complete Implementation Plan

**Game Type**: Puzzle
**Estimated Time**: 4-5 days (30-35 hours)
**Difficulty**: ⭐⭐⭐☆☆ (Medium)
**AI Assistance Level**: MEDIUM-HIGH (can help with 70% of code, you design puzzles)

---

## 📊 PROJECT OVERVIEW

### What You're Building
A puzzle game where players manipulate 6 abstract dimensions (Emergence, Coherence, Resilience, Intelligence, Efficiency, Integration) by applying "moves" that shift multiple dimensions simultaneously. The goal is to balance all dimensions close to zero.

### Core Loop
```
View current dimensional state → Select a move →
Apply move (changes multiple dimensions) → Check if solved →
Repeat until all dimensions near zero → Next puzzle
```

### Success Criteria
- ✅ Display 6 dimensions visually (bars/radar chart)
- ✅ 30+ puzzles with increasing difficulty
- ✅ Move system with interesting tradeoffs
- ✅ Undo functionality
- ✅ Progress tracking and save/load
- ✅ Satisfying difficulty curve

---

## 🗂️ FILE STRUCTURE

### New Files to Create
```
src/
├── puzzle/                   # NEW DIRECTORY
│   ├── mod.rs               # Module exports
│   ├── puzzle.rs            # Puzzle data structures
│   ├── campaign.rs          # Level progression
│   └── solver.rs            # Puzzle validation (optional)
├── data/                    # NEW DIRECTORY
│   ├── puzzles.json         # All puzzle definitions
│   └── puzzle_save.json     # Player progress
```

### Files to Modify
```
src/
├── main.rs                  # Add puzzle game mode
├── models/types.rs          # Already has DimensionalPosition ✅
└── interface/mod.rs         # Add puzzle UI (or create new puzzle_view.rs)
```

---

## 📅 IMPLEMENTATION ROADMAP

### Phase 1: Foundation (Day 1 - 8 hours)
- [ ] Puzzle data structures
- [ ] Move system
- [ ] Basic UI (dimensional display)
- [ ] Apply move logic

### Phase 2: Puzzle System (Day 2 - 8 hours)
- [ ] JSON puzzle loading
- [ ] Win condition checking
- [ ] Undo/reset functionality
- [ ] Campaign progression

### Phase 3: Puzzle Design (Day 3 - 8 hours)
- [ ] Design 30+ puzzles
- [ ] Validate all puzzles are solvable
- [ ] Organize difficulty progression
- [ ] Tutorial puzzles

### Phase 4: UI Polish (Day 4 - 6 hours)
- [ ] Radar chart visualization
- [ ] Move preview
- [ ] Victory screen
- [ ] Statistics tracking

### Phase 5: Balance & Testing (Day 5 - 4 hours)
- [ ] Difficulty balancing
- [ ] Playtest all puzzles
- [ ] Hint system
- [ ] Final polish

---

## 🔨 STEP-BY-STEP IMPLEMENTATION

---

## **PHASE 1: FOUNDATION (Day 1)**

### Step 1.1: Project Structure (20 minutes)

**Create directories:**
```bash
mkdir -p src/puzzle
touch src/puzzle/mod.rs
touch src/puzzle/puzzle.rs
touch src/puzzle/campaign.rs
touch src/puzzle/solver.rs

mkdir -p data
touch data/puzzles.json
```

**File: `src/puzzle/mod.rs`**
```rust
pub mod puzzle;
pub mod campaign;
pub mod solver;

pub use puzzle::{Puzzle, Move, Difficulty};
pub use campaign::Campaign;
```

**Add to `src/main.rs`:**
```rust
mod puzzle;
use puzzle::{Puzzle, Campaign};
```

---

### Step 1.2: Puzzle Data Structures (2 hours)

**File: `src/puzzle/puzzle.rs`**
```rust
use crate::models::types::DimensionalPosition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub starting_position: DimensionalPosition,
    pub target_tolerance: f64,  // How close to zero = solved
    pub moves: Vec<Move>,
    pub move_limit: Option<u32>, // Optional move cap for extra challenge
    pub difficulty: Difficulty,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effects: DimensionalPosition, // Delta values (how much each dimension changes)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Tutorial,  // 1-2 moves to solve, obvious solution
    Easy,      // 3-5 moves, straightforward
    Medium,    // 6-10 moves, requires thinking
    Hard,      // 11+ moves, complex interactions
    Expert,    // Tight tolerance, move limit, tricky
}

impl Puzzle {
    /// Check if current position solves the puzzle
    pub fn is_solved(&self, current: &DimensionalPosition) -> bool {
        let total_deviation = self.calculate_total_deviation(current);
        total_deviation <= self.target_tolerance
    }

    /// Calculate total deviation from zero across all dimensions
    pub fn calculate_total_deviation(&self, pos: &DimensionalPosition) -> f64 {
        pos.emergence.abs() +
        pos.coherence.abs() +
        pos.resilience.abs() +
        pos.intelligence.abs() +
        pos.efficiency.abs() +
        pos.integration.abs()
    }

    /// Apply a move to current position (returns new position)
    pub fn apply_move(&self, current: &DimensionalPosition, move_id: &str) -> Option<DimensionalPosition> {
        let move_effect = self.moves.iter().find(|m| m.id == move_id)?;

        Some(DimensionalPosition {
            emergence: (current.emergence + move_effect.effects.emergence).clamp(-100.0, 100.0),
            coherence: (current.coherence + move_effect.effects.coherence).clamp(-100.0, 100.0),
            resilience: (current.resilience + move_effect.effects.resilience).clamp(-100.0, 100.0),
            intelligence: (current.intelligence + move_effect.effects.intelligence).clamp(-100.0, 100.0),
            efficiency: (current.efficiency + move_effect.effects.efficiency).clamp(-100.0, 100.0),
            integration: (current.integration + move_effect.effects.integration).clamp(-100.0, 100.0),
        })
    }

    /// Get available moves (filter out invalid moves if needed)
    pub fn available_moves(&self) -> &[Move] {
        &self.moves
    }

    /// Load puzzles from JSON file
    pub fn load_from_file(path: &str) -> Result<Vec<Puzzle>, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let puzzles: Vec<Puzzle> = serde_json::from_str(&content)?;
        Ok(puzzles)
    }
}
```

---

### Step 1.3: Puzzle State Management (1.5 hours)

**File: `src/puzzle/puzzle.rs`** - Add state tracker:
```rust
#[derive(Debug, Clone)]
pub struct PuzzleState {
    pub puzzle: Puzzle,
    pub current_position: DimensionalPosition,
    pub move_history: Vec<String>, // Move IDs in order applied
    pub is_solved: bool,
}

impl PuzzleState {
    pub fn new(puzzle: Puzzle) -> Self {
        let current_position = puzzle.starting_position.clone();
        let is_solved = puzzle.is_solved(&current_position);

        Self {
            puzzle,
            current_position,
            move_history: Vec::new(),
            is_solved,
        }
    }

    /// Apply a move by ID
    pub fn apply_move(&mut self, move_id: &str) -> Result<(), String> {
        // Check move limit
        if let Some(limit) = self.puzzle.move_limit {
            if self.move_history.len() >= limit as usize {
                return Err("Move limit reached".to_string());
            }
        }

        // Apply move
        match self.puzzle.apply_move(&self.current_position, move_id) {
            Some(new_position) => {
                self.current_position = new_position;
                self.move_history.push(move_id.to_string());
                self.is_solved = self.puzzle.is_solved(&self.current_position);
                Ok(())
            }
            None => Err(format!("Invalid move: {}", move_id)),
        }
    }

    /// Undo last move
    pub fn undo(&mut self) -> Result<(), String> {
        if self.move_history.is_empty() {
            return Err("Nothing to undo".to_string());
        }

        // Remove last move
        self.move_history.pop();

        // Recalculate position from scratch
        self.reset_to_start();
        for move_id in self.move_history.clone() {
            self.apply_move(&move_id)?;
        }

        Ok(())
    }

    /// Reset to starting position
    pub fn reset_to_start(&mut self) {
        self.current_position = self.puzzle.starting_position.clone();
        self.move_history.clear();
        self.is_solved = self.puzzle.is_solved(&self.current_position);
    }

    /// Get number of moves used
    pub fn moves_used(&self) -> usize {
        self.move_history.len()
    }

    /// Get remaining moves (if limit exists)
    pub fn moves_remaining(&self) -> Option<u32> {
        self.puzzle.move_limit.map(|limit| {
            limit.saturating_sub(self.move_history.len() as u32)
        })
    }

    /// Calculate deviation improvement (for scoring)
    pub fn improvement(&self) -> f64 {
        let start_deviation = self.puzzle.calculate_total_deviation(&self.puzzle.starting_position);
        let current_deviation = self.puzzle.calculate_total_deviation(&self.current_position);
        start_deviation - current_deviation
    }
}
```

---

### Step 1.4: Basic Dimensional Display UI (2.5 hours)

**File: `src/puzzle/puzzle_view.rs`** (new file)
```rust
use crate::models::types::DimensionalPosition;
use crate::puzzle::PuzzleState;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct PuzzleView {
    pub state: PuzzleState,
    pub selected_move: usize,
}

impl PuzzleView {
    pub fn new(puzzle: crate::puzzle::Puzzle) -> Self {
        Self {
            state: PuzzleState::new(puzzle),
            selected_move: 0,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12), // Top: Dimensional display
                Constraint::Min(8),     // Middle: Available moves
                Constraint::Length(4),  // Bottom: Status & controls
            ])
            .split(f.size());

        self.render_dimensional_display(f, chunks[0]);
        self.render_moves_list(f, chunks[1]);
        self.render_status_bar(f, chunks[2]);
    }

    fn render_dimensional_display<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let pos = &self.state.current_position;

        let lines = vec![
            self.dimension_line("Emergence", pos.emergence),
            self.dimension_line("Coherence", pos.coherence),
            self.dimension_line("Resilience", pos.resilience),
            self.dimension_line("Intelligence", pos.intelligence),
            self.dimension_line("Efficiency", pos.efficiency),
            self.dimension_line("Integration", pos.integration),
            Line::from(""),
            Line::from(vec![
                Span::styled("Total Deviation: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.1}", self.state.puzzle.calculate_total_deviation(pos)),
                    self.deviation_color(self.state.puzzle.calculate_total_deviation(pos))
                ),
                Span::raw(" / "),
                Span::styled(
                    format!("{:.1}", self.state.puzzle.target_tolerance),
                    Style::default().fg(Color::Green)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!("Puzzle: {}", self.state.puzzle.name)));
        f.render_widget(paragraph, area);
    }

    fn dimension_line(&self, name: &str, value: f64) -> Line {
        let bar = self.value_bar(value);
        let color = self.value_color(value);

        Line::from(vec![
            Span::styled(format!("{:12}", name), Style::default().fg(Color::White)),
            Span::styled(
                format!(" {:>6.1}", value),
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            ),
            Span::raw("  "),
            Span::styled(bar, Style::default().fg(color)),
        ])
    }

    fn value_bar(&self, value: f64) -> String {
        let abs_val = value.abs();
        let bar_len = (abs_val / 5.0) as usize; // Each 5 units = one char
        let char = if value >= 0.0 { '█' } else { '░' };
        char.to_string().repeat(bar_len.min(20))
    }

    fn value_color(&self, value: f64) -> Color {
        let abs_val = value.abs();
        if abs_val < 5.0 {
            Color::Green
        } else if abs_val < 20.0 {
            Color::Yellow
        } else if abs_val < 50.0 {
            Color::LightRed
        } else {
            Color::Red
        }
    }

    fn deviation_color(&self, deviation: f64) -> Style {
        let color = if deviation <= self.state.puzzle.target_tolerance {
            Color::Green
        } else if deviation < self.state.puzzle.target_tolerance * 2.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }

    fn render_moves_list<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self.state.puzzle.moves.iter()
            .enumerate()
            .map(|(i, mv)| {
                let style = if i == self.selected_move {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                // Show what this move does compactly
                let effects = format!(
                    "E:{:+.0} C:{:+.0} R:{:+.0} I:{:+.0} Ef:{:+.0} In:{:+.0}",
                    mv.effects.emergence,
                    mv.effects.coherence,
                    mv.effects.resilience,
                    mv.effects.intelligence,
                    mv.effects.efficiency,
                    mv.effects.integration
                );

                let line = Line::from(vec![
                    Span::styled(
                        format!("{:20}", mv.name),
                        Style::default().fg(Color::Cyan)
                    ),
                    Span::raw(" "),
                    Span::styled(effects, Style::default().fg(Color::DarkGray)),
                ]);

                ListItem::new(line).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Available Moves"));
        f.render_widget(list, area);
    }

    fn render_status_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut status_parts = vec![
            Span::raw("Moves: "),
            Span::styled(
                format!("{}", self.state.moves_used()),
                Style::default().fg(Color::Yellow)
            ),
        ];

        if let Some(remaining) = self.state.moves_remaining() {
            status_parts.push(Span::raw(" / "));
            status_parts.push(Span::styled(
                format!("{}", self.state.puzzle.move_limit.unwrap()),
                Style::default().fg(Color::Red)
            ));
        }

        status_parts.push(Span::raw("  |  "));

        if self.state.is_solved {
            status_parts.push(Span::styled(
                "✓ SOLVED!",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ));
        } else {
            status_parts.push(Span::raw("[↑↓] Select | [ENTER] Apply | [U] Undo | [R] Reset | [Q] Quit"));
        }

        let paragraph = Paragraph::new(Line::from(status_parts))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, area);
    }
}
```

**Add to `src/puzzle/mod.rs`:**
```rust
pub mod puzzle_view;
pub use puzzle_view::PuzzleView;
```

---

### Step 1.5: Input Handling (1.5 hours)

**File: `src/puzzle/puzzle_view.rs`** - Add input:
```rust
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

impl PuzzleView {
    pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(false); // Exit
                    }

                    KeyCode::Up => {
                        if self.selected_move > 0 {
                            self.selected_move -= 1;
                        }
                    }

                    KeyCode::Down => {
                        let max = self.state.puzzle.moves.len().saturating_sub(1);
                        if self.selected_move < max {
                            self.selected_move += 1;
                        }
                    }

                    KeyCode::Enter => {
                        // Apply selected move
                        if let Some(mv) = self.state.puzzle.moves.get(self.selected_move) {
                            if let Err(e) = self.state.apply_move(&mv.id) {
                                // Could show error message
                                eprintln!("Move error: {}", e);
                            }
                        }
                    }

                    KeyCode::Char('u') | KeyCode::Char('U') => {
                        self.state.undo().ok();
                    }

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.state.reset_to_start();
                    }

                    _ => {}
                }
            }
        }
        Ok(true)
    }
}
```

---

### Step 1.6: Main Loop Integration (30 minutes)

**File: `src/main.rs`** - Create simple test puzzle:
```rust
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod puzzle;
use puzzle::{Puzzle, PuzzleView};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test puzzle
    let test_puzzle = create_test_puzzle();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create game
    let mut view = PuzzleView::new(test_puzzle);

    // Main loop
    loop {
        terminal.draw(|f| {
            view.render(f);
        })?;

        if !view.handle_input()? {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn create_test_puzzle() -> Puzzle {
    use crate::models::types::DimensionalPosition;
    use crate::puzzle::{Move, Difficulty};

    Puzzle {
        id: "test_01".to_string(),
        name: "Tutorial: Balance".to_string(),
        description: Some("Use the 'Balance' move to bring dimensions to zero".to_string()),
        starting_position: DimensionalPosition {
            emergence: 20.0,
            coherence: -20.0,
            resilience: 0.0,
            intelligence: 0.0,
            efficiency: 0.0,
            integration: 0.0,
        },
        target_tolerance: 10.0,
        moves: vec![
            Move {
                id: "balance_ec".to_string(),
                name: "Balance E/C".to_string(),
                description: "Shift emergence down, coherence up".to_string(),
                effects: DimensionalPosition {
                    emergence: -20.0,
                    coherence: 20.0,
                    resilience: 0.0,
                    intelligence: 0.0,
                    efficiency: 0.0,
                    integration: 0.0,
                },
            },
        ],
        move_limit: None,
        difficulty: Difficulty::Tutorial,
        hint: Some("The 'Balance E/C' move perfectly solves this puzzle in one move!".to_string()),
    }
}
```

---

### Step 1.7: Test Phase 1 ✅

**Run the game:**
```bash
cargo run --release
```

**Test checklist:**
- [ ] Dimensional display shows 6 dimensions
- [ ] Emergence starts at 20.0, Coherence at -20.0
- [ ] Can navigate moves with up/down
- [ ] Press ENTER - applies move, values update
- [ ] Press U - undoes move
- [ ] Press R - resets to start
- [ ] When solved, status shows "✓ SOLVED!"

---

## **PHASE 2: PUZZLE SYSTEM (Day 2)**

### Step 2.1: JSON Puzzle Loading (1.5 hours)

**File: `data/puzzles.json`** - Create initial puzzles:
```json
[
  {
    "id": "tutorial_01",
    "name": "First Steps",
    "description": "Learn to balance two dimensions",
    "starting_position": {
      "emergence": 20.0,
      "coherence": -20.0,
      "resilience": 0.0,
      "intelligence": 0.0,
      "efficiency": 0.0,
      "integration": 0.0
    },
    "target_tolerance": 10.0,
    "moves": [
      {
        "id": "balance_ec",
        "name": "Balance E/C",
        "description": "Equal shift emergence↓ coherence↑",
        "effects": {
          "emergence": -20.0,
          "coherence": 20.0,
          "resilience": 0.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      }
    ],
    "move_limit": null,
    "difficulty": "Tutorial",
    "hint": "The Balance move perfectly solves this!"
  },
  {
    "id": "tutorial_02",
    "name": "Multiple Moves",
    "description": "Chain moves together",
    "starting_position": {
      "emergence": 30.0,
      "coherence": -10.0,
      "resilience": -20.0,
      "intelligence": 0.0,
      "efficiency": 0.0,
      "integration": 0.0
    },
    "target_tolerance": 15.0,
    "moves": [
      {
        "id": "shift_e",
        "name": "Reduce Emergence",
        "description": "Lower emergence by 15",
        "effects": {
          "emergence": -15.0,
          "coherence": 0.0,
          "resilience": 0.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      },
      {
        "id": "shift_c",
        "name": "Raise Coherence",
        "description": "Increase coherence by 10",
        "effects": {
          "emergence": 0.0,
          "coherence": 10.0,
          "resilience": 0.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      },
      {
        "id": "shift_r",
        "name": "Raise Resilience",
        "description": "Increase resilience by 20",
        "effects": {
          "emergence": 0.0,
          "coherence": 0.0,
          "resilience": 20.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      }
    ],
    "move_limit": null,
    "difficulty": "Tutorial",
    "hint": "Use all three moves, each targeting one dimension"
  },
  {
    "id": "easy_01",
    "name": "Trade-offs",
    "description": "Moves affect multiple dimensions",
    "starting_position": {
      "emergence": 25.0,
      "coherence": 15.0,
      "resilience": -30.0,
      "intelligence": 0.0,
      "efficiency": 0.0,
      "integration": 0.0
    },
    "target_tolerance": 20.0,
    "moves": [
      {
        "id": "stabilize",
        "name": "Stabilize",
        "description": "Reduce emergence, increase resilience",
        "effects": {
          "emergence": -15.0,
          "coherence": 0.0,
          "resilience": 20.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      },
      {
        "id": "harmonize",
        "name": "Harmonize",
        "description": "Reduce coherence, increase resilience",
        "effects": {
          "emergence": 0.0,
          "coherence": -15.0,
          "resilience": 10.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      }
    ],
    "move_limit": 5,
    "difficulty": "Easy",
    "hint": "Use Stabilize once, then Harmonize"
  }
]
```

**Update `src/main.rs` to load from JSON:**
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load puzzles from JSON
    let puzzles = Puzzle::load_from_file("data/puzzles.json")?;

    if puzzles.is_empty() {
        eprintln!("No puzzles found in data/puzzles.json");
        return Ok(());
    }

    // Start with first puzzle
    let mut view = PuzzleView::new(puzzles[0].clone());

    // ... rest of main loop
}
```

---

### Step 2.2: Campaign Progression (2 hours)

**File: `src/puzzle/campaign.rs`**
```rust
use crate::puzzle::Puzzle;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub puzzles: Vec<Puzzle>,
    pub current_index: usize,
    pub completed: HashSet<String>, // Puzzle IDs
    pub best_moves: std::collections::HashMap<String, usize>, // Puzzle ID -> fewest moves
}

impl Campaign {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let puzzles = Puzzle::load_from_file(path)?;

        Ok(Self {
            puzzles,
            current_index: 0,
            completed: HashSet::new(),
            best_moves: std::collections::HashMap::new(),
        })
    }

    pub fn current_puzzle(&self) -> Option<&Puzzle> {
        self.puzzles.get(self.current_index)
    }

    pub fn next_puzzle(&mut self) -> Option<&Puzzle> {
        if self.current_index + 1 < self.puzzles.len() {
            self.current_index += 1;
            self.current_puzzle()
        } else {
            None
        }
    }

    pub fn previous_puzzle(&mut self) -> Option<&Puzzle> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.current_puzzle()
        } else {
            None
        }
    }

    pub fn mark_complete(&mut self, puzzle_id: &str, moves_used: usize) {
        self.completed.insert(puzzle_id.to_string());

        // Update best score
        let current_best = self.best_moves.get(puzzle_id).copied().unwrap_or(usize::MAX);
        if moves_used < current_best {
            self.best_moves.insert(puzzle_id.to_string(), moves_used);
        }
    }

    pub fn is_completed(&self, puzzle_id: &str) -> bool {
        self.completed.contains(puzzle_id)
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.puzzles.is_empty() {
            return 0.0;
        }
        (self.completed.len() as f64 / self.puzzles.len() as f64) * 100.0
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct CampaignSave {
            current_index: usize,
            completed: HashSet<String>,
            best_moves: std::collections::HashMap<String, usize>,
        }

        let save = CampaignSave {
            current_index: self.current_index,
            completed: self.completed.clone(),
            best_moves: self.best_moves.clone(),
        };

        let json = serde_json::to_string_pretty(&save)?;
        std::fs::write("data/puzzle_save.json", json)?;
        Ok(())
    }

    pub fn load_save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct CampaignSave {
            current_index: usize,
            completed: HashSet<String>,
            best_moves: std::collections::HashMap<String, usize>,
        }

        let json = std::fs::read_to_string("data/puzzle_save.json")?;
        let save: CampaignSave = serde_json::from_str(&json)?;

        self.current_index = save.current_index;
        self.completed = save.completed;
        self.best_moves = save.best_moves;

        Ok(())
    }
}
```

---

### Step 2.3: Integrate Campaign into View (2 hours)

**Update `src/puzzle/puzzle_view.rs`:**
```rust
use crate::puzzle::Campaign;

pub struct PuzzleView {
    pub campaign: Campaign,
    pub state: PuzzleState,
    pub selected_move: usize,
    pub show_victory_screen: bool,
}

impl PuzzleView {
    pub fn new(campaign: Campaign) -> Self {
        let puzzle = campaign.current_puzzle()
            .expect("Campaign has no puzzles")
            .clone();

        Self {
            campaign,
            state: PuzzleState::new(puzzle),
            selected_move: 0,
            show_victory_screen: false,
        }
    }

    pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // If victory screen is showing
        if self.show_victory_screen {
            return self.handle_victory_input();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // ... existing input handling

                    KeyCode::Enter => {
                        if let Some(mv) = self.state.puzzle.moves.get(self.selected_move) {
                            if let Err(e) = self.state.apply_move(&mv.id) {
                                eprintln!("Move error: {}", e);
                            } else {
                                // Check if solved
                                if self.state.is_solved {
                                    self.on_puzzle_solved();
                                }
                            }
                        }
                    }

                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        // Next puzzle (if completed current)
                        if self.campaign.is_completed(&self.state.puzzle.id) {
                            self.next_puzzle();
                        }
                    }

                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        // Previous puzzle
                        self.previous_puzzle();
                    }

                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        // Save progress
                        self.campaign.save()?;
                    }

                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn on_puzzle_solved(&mut self) {
        // Mark complete in campaign
        self.campaign.mark_complete(
            &self.state.puzzle.id,
            self.state.moves_used()
        );

        // Save progress
        self.campaign.save().ok();

        // Show victory screen
        self.show_victory_screen = true;
    }

    fn handle_victory_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        // Go to next puzzle
                        self.next_puzzle();
                        self.show_victory_screen = false;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        // Retry current puzzle
                        self.state.reset_to_start();
                        self.show_victory_screen = false;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(false); // Quit
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn next_puzzle(&mut self) {
        if let Some(puzzle) = self.campaign.next_puzzle() {
            self.state = PuzzleState::new(puzzle.clone());
            self.selected_move = 0;
        }
    }

    fn previous_puzzle(&mut self) {
        if let Some(puzzle) = self.campaign.previous_puzzle() {
            self.state = PuzzleState::new(puzzle.clone());
            self.selected_move = 0;
        }
    }
}
```

---

### Step 2.4: Victory Screen UI (2 hours)

**Add to `src/puzzle/puzzle_view.rs`:**
```rust
impl PuzzleView {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        // Normal puzzle view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12),
                Constraint::Min(8),
                Constraint::Length(4),
            ])
            .split(f.size());

        self.render_dimensional_display(f, chunks[0]);
        self.render_moves_list(f, chunks[1]);
        self.render_status_bar(f, chunks[2]);

        // Overlay victory screen if solved
        if self.show_victory_screen {
            self.render_victory_screen(f);
        }
    }

    fn render_victory_screen<B: Backend>(&self, f: &mut Frame<B>) {
        use ratatui::widgets::Clear;

        let area = centered_rect(60, 50, f.size());
        f.render_widget(Clear, area);

        let moves_used = self.state.moves_used();
        let best_moves = self.campaign.best_moves
            .get(&self.state.puzzle.id)
            .copied();

        let is_new_best = best_moves.map(|best| moves_used <= best).unwrap_or(true);

        let mut lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "🎉 PUZZLE SOLVED! 🎉",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Moves used: "),
                Span::styled(
                    format!("{}", moves_used),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        if let Some(best) = best_moves {
            if is_new_best && moves_used < best {
                lines.push(Line::from(vec![
                    Span::styled(
                        "🌟 NEW BEST! 🌟",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    ),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::raw("Best: "),
                    Span::styled(
                        format!("{}", best),
                        Style::default().fg(Color::Green)
                    ),
                ]));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("Progress: "),
            Span::styled(
                format!("{:.0}%", self.campaign.completion_percentage()),
                Style::default().fg(Color::Magenta)
            ),
        ]));

        lines.push(Line::from(""));
        lines.push(Line::from(""));

        if self.campaign.current_index + 1 < self.campaign.puzzles.len() {
            lines.push(Line::from(vec![
                Span::styled("[ENTER]", Style::default().fg(Color::Green)),
                Span::raw(" Next Puzzle  "),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("Campaign Complete!", Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("[R]", Style::default().fg(Color::Yellow)),
            Span::raw(" Retry  "),
            Span::styled("[Q]", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]));

        let paragraph = Paragraph::new(lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title("Victory!"))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
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
```

---

## **PHASE 3: PUZZLE DESIGN (Day 3)**

### Step 3.1: Design Tutorial Puzzles (2 hours)

**Guidelines for tutorial puzzles:**
1. **Single dimension** - Only one dimension non-zero
2. **One move** - Obvious solution
3. **Introduce concepts** - Each teaches something new

**Add to `data/puzzles.json`:**
```json
[
  {
    "id": "tut_01",
    "name": "Tutorial: Single Dimension",
    "description": "Balance one dimension",
    "starting_position": {
      "emergence": 30.0,
      "coherence": 0.0,
      "resilience": 0.0,
      "intelligence": 0.0,
      "efficiency": 0.0,
      "integration": 0.0
    },
    "target_tolerance": 10.0,
    "moves": [
      {
        "id": "reduce_e",
        "name": "Reduce Emergence",
        "description": "Lower emergence by 30",
        "effects": {
          "emergence": -30.0,
          "coherence": 0.0,
          "resilience": 0.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      }
    ],
    "move_limit": null,
    "difficulty": "Tutorial",
    "hint": "Use the move once to solve!"
  },
  {
    "id": "tut_02",
    "name": "Tutorial: Trade-offs",
    "description": "Some moves affect multiple dimensions",
    "starting_position": {
      "emergence": 20.0,
      "coherence": 20.0,
      "resilience": 0.0,
      "intelligence": 0.0,
      "efficiency": 0.0,
      "integration": 0.0
    },
    "target_tolerance": 10.0,
    "moves": [
      {
        "id": "sync",
        "name": "Synchronize",
        "description": "Reduce both emergence and coherence",
        "effects": {
          "emergence": -20.0,
          "coherence": -20.0,
          "resilience": 0.0,
          "intelligence": 0.0,
          "efficiency": 0.0,
          "integration": 0.0
        }
      }
    ],
    "move_limit": null,
    "difficulty": "Tutorial",
    "hint": "This move affects both dimensions at once!"
  }
]
```

**AI Prompt for generating more:**
```
Generate 5 more tutorial puzzles for Dimensional Sliders game.
Each puzzle should:
- Start with 1-2 dimensions non-zero
- Be solvable in 1-3 moves
- Teach a specific concept (trade-offs, opposite effects, etc.)
- Have target_tolerance: 10.0-15.0

Use dimensions: emergence, coherence, resilience, intelligence, efficiency, integration
Format as JSON matching this structure: [paste example]
```

---

### Step 3.2: Design Easy Puzzles (2 hours)

**Guidelines:**
- 2-3 dimensions non-zero
- 3-5 moves to solve
- Multiple solution paths possible
- No move limits

**Example easy puzzles to add:**
```json
{
  "id": "easy_01",
  "name": "Triangle Balance",
  "description": "Three dimensions need attention",
  "starting_position": {
    "emergence": 25.0,
    "coherence": -15.0,
    "resilience": -20.0,
    "intelligence": 0.0,
    "efficiency": 0.0,
    "integration": 0.0
  },
  "target_tolerance": 15.0,
  "moves": [
    {
      "id": "shift_ec",
      "name": "E/C Shift",
      "description": "Reduce E, increase C",
      "effects": {
        "emergence": -15.0,
        "coherence": 15.0,
        "resilience": 0.0,
        "intelligence": 0.0,
        "efficiency": 0.0,
        "integration": 0.0
      }
    },
    {
      "id": "boost_r",
      "name": "Boost Resilience",
      "description": "Increase resilience",
      "effects": {
        "emergence": 0.0,
        "coherence": 0.0,
        "resilience": 20.0,
        "intelligence": 0.0,
        "efficiency": 0.0,
        "integration": 0.0
      }
    },
    {
      "id": "fine_tune",
      "name": "Fine Tune",
      "description": "Small adjustments to E and C",
      "effects": {
        "emergence": -10.0,
        "coherence": 5.0,
        "resilience": 0.0,
        "intelligence": 0.0,
        "efficiency": 0.0,
        "integration": 0.0
      }
    }
  ],
  "move_limit": null,
  "difficulty": "Easy",
  "hint": "Start with the biggest imbalances first"
}
```

**AI Prompt:**
```
Generate 10 easy puzzles for Dimensional Sliders.
Requirements:
- 2-4 dimensions non-zero (start values -40 to +40)
- 4-6 different moves available
- Solvable in 3-6 moves
- target_tolerance: 12.0-18.0
- No move limits
- Variety of move effects (some affect 1 dim, some affect 2-3)

Difficulty: Easy
Format: JSON array
```

---

### Step 3.3: Design Medium/Hard Puzzles (2 hours)

**Medium Guidelines:**
- 4-5 dimensions non-zero
- 6-10 moves to solve
- Tighter tolerances
- Some moves have negative trade-offs

**Hard Guidelines:**
- All 6 dimensions non-zero
- Complex move interactions
- Move limits
- Very tight tolerances (5.0-10.0)

**Example medium puzzle:**
```json
{
  "id": "med_01",
  "name": "Six-Way Chaos",
  "description": "All dimensions need balancing",
  "starting_position": {
    "emergence": 35.0,
    "coherence": -25.0,
    "resilience": 18.0,
    "intelligence": -12.0,
    "efficiency": 22.0,
    "integration": -30.0
  },
  "target_tolerance": 15.0,
  "moves": [
    {
      "id": "stabilize_all",
      "name": "Global Stabilize",
      "description": "Small reduction across all positive dims",
      "effects": {
        "emergence": -10.0,
        "coherence": 10.0,
        "resilience": -10.0,
        "intelligence": 10.0,
        "efficiency": -10.0,
        "integration": 10.0
      }
    },
    {
      "id": "focus_ei",
      "name": "Focus E/I",
      "description": "Reduce emergence, boost intelligence",
      "effects": {
        "emergence": -20.0,
        "coherence": 0.0,
        "resilience": 0.0,
        "intelligence": 15.0,
        "efficiency": 0.0,
        "integration": 0.0
      }
    },
    {
      "id": "harmonize",
      "name": "Harmonize",
      "description": "Balance coherence and integration",
      "effects": {
        "emergence": 0.0,
        "coherence": 15.0,
        "resilience": 0.0,
        "intelligence": 0.0,
        "efficiency": 0.0,
        "integration": 20.0
      }
    },
    {
      "id": "efficiency_cut",
      "name": "Cut Efficiency",
      "description": "Reduce efficiency, slight resilience drop",
      "effects": {
        "emergence": 0.0,
        "coherence": 0.0,
        "resilience": -5.0,
        "efficiency": -25.0,
        "integration": 0.0
      }
    }
  ],
  "move_limit": 10,
  "difficulty": "Medium",
  "hint": "Use Global Stabilize first to get everything closer, then fine-tune"
}
```

---

### Step 3.4: Validate Puzzles (2 hours)

**Create puzzle validator:**

**File: `src/puzzle/solver.rs`**
```rust
use crate::puzzle::{Puzzle, PuzzleState};

/// Attempt to solve puzzle with brute-force search
pub fn is_solvable(puzzle: &Puzzle, max_depth: usize) -> bool {
    let mut state = PuzzleState::new(puzzle.clone());

    // Check if already solved
    if state.is_solved {
        return true;
    }

    // Try all move combinations up to max_depth
    solve_recursive(&mut state, 0, max_depth)
}

fn solve_recursive(state: &mut PuzzleState, depth: usize, max_depth: usize) -> bool {
    if state.is_solved {
        return true;
    }

    if depth >= max_depth {
        return false;
    }

    // Try each available move
    for mv in state.puzzle.moves.clone() {
        let original_history = state.move_history.clone();

        if state.apply_move(&mv.id).is_ok() {
            if solve_recursive(state, depth + 1, max_depth) {
                return true;
            }

            // Backtrack
            state.move_history = original_history;
            state.current_position = state.puzzle.starting_position.clone();
            for move_id in &state.move_history {
                state.apply_move(move_id).ok();
            }
        }
    }

    false
}

/// Test all puzzles in file
pub fn validate_puzzle_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let puzzles = Puzzle::load_from_file(path)?;

    println!("Validating {} puzzles...", puzzles.len());

    for (i, puzzle) in puzzles.iter().enumerate() {
        print!("Puzzle {}: {} ... ", i + 1, puzzle.name);

        let max_depth = puzzle.move_limit.unwrap_or(15) as usize;
        if is_solvable(puzzle, max_depth) {
            println!("✓ SOLVABLE");
        } else {
            println!("✗ UNSOLVABLE (within {} moves)", max_depth);
        }
    }

    Ok(())
}
```

**Add validation binary:**

**File: `src/bin/validate_puzzles.rs`**
```rust
use creature::puzzle::solver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    solver::validate_puzzle_file("data/puzzles.json")?;
    Ok(())
}
```

**Run validation:**
```bash
cargo run --bin validate_puzzles
```

---

## **PHASE 4: UI POLISH (Day 4)**

### Step 4.1: Radar Chart Visualization (3 hours)

**Add better dimensional visualization:**

**Update `src/puzzle/puzzle_view.rs`:**
```rust
fn render_dimensional_display<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Bar chart (left)
            Constraint::Percentage(40), // Radar "chart" (right)
        ])
        .split(area);

    self.render_dimension_bars(f, chunks[0]);
    self.render_dimension_radar(f, chunks[1]);
}

fn render_dimension_radar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    let pos = &self.state.current_position;

    // ASCII "radar chart" - show relative balance
    let lines = vec![
        Line::from(""),
        Line::from("    Emergence"),
        Line::from(self.radar_line(pos.emergence)),
        Line::from(""),
        Line::from("Int         Coh"),
        Line::from(format!("{}     {}",
            self.mini_bar(pos.intelligence),
            self.mini_bar(pos.coherence)
        )),
        Line::from(""),
        Line::from("Eff         Res"),
        Line::from(format!("{}     {}",
            self.mini_bar(pos.efficiency),
            self.mini_bar(pos.resilience)
        )),
        Line::from(""),
        Line::from("   Integration"),
        Line::from(self.radar_line(pos.integration)),
    ];

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Balance"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

fn radar_line(&self, value: f64) -> String {
    let normalized = (value / 20.0).clamp(-5.0, 5.0);
    let pos = (normalized + 5.0) as usize;

    let mut line = vec![' '; 11];
    line[5] = '|'; // Center
    line[pos] = '●';

    line.into_iter().collect()
}

fn mini_bar(&self, value: f64) -> String {
    let chars = (value.abs() / 20.0).min(3.0) as usize;
    let char = if value >= 0.0 { '█' } else { '░' };
    char.to_string().repeat(chars)
}
```

---

### Step 4.2: Move Preview (2 hours)

**Show what happens before applying move:**

**Update move rendering:**
```rust
fn render_moves_list<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),     // Move list
            Constraint::Length(8),  // Preview
        ])
        .split(area);

    // ... existing move list rendering

    // Preview selected move
    if let Some(selected_move) = self.state.puzzle.moves.get(self.selected_move) {
        self.render_move_preview(f, chunks[1], selected_move);
    }
}

fn render_move_preview<B: Backend>(&self, f: &mut Frame<B>, area: Rect, mv: &Move) {
    // Calculate what position would be after this move
    let preview_pos = self.state.puzzle
        .apply_move(&self.state.current_position, &mv.id)
        .unwrap_or_else(|| self.state.current_position.clone());

    let current_dev = self.state.puzzle.calculate_total_deviation(&self.state.current_position);
    let preview_dev = self.state.puzzle.calculate_total_deviation(&preview_pos);
    let improvement = current_dev - preview_dev;

    let lines = vec![
        Line::from(vec![
            Span::styled(&mv.name, Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(&mv.description),
        Line::from(""),
        Line::from(vec![
            Span::raw("Impact: "),
            Span::styled(
                if improvement > 0.0 { "▼ Better" } else { "▲ Worse" },
                if improvement > 0.0 {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                }
            ),
            Span::raw(format!(" ({:+.1})", improvement)),
        ]),
        Line::from(vec![
            Span::raw("New deviation: "),
            Span::styled(
                format!("{:.1}", preview_dev),
                if preview_dev <= self.state.puzzle.target_tolerance {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Yellow)
                }
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(paragraph, area);
}
```

---

### Step 4.3: Statistics Tracking (1 hour)

**Track player statistics:**

**Add to Campaign:**
```rust
impl Campaign {
    pub fn total_moves_used(&self) -> usize {
        self.best_moves.values().sum()
    }

    pub fn average_moves_per_puzzle(&self) -> f64 {
        if self.completed.is_empty() {
            return 0.0;
        }
        self.total_moves_used() as f64 / self.completed.len() as f64
    }

    pub fn perfect_solves(&self) -> usize {
        // Count puzzles solved optimally (would need optimal solutions data)
        // For now, just count completed
        self.completed.len()
    }
}
```

---

## **PHASE 5: BALANCE & TESTING (Day 5)**

### Step 5.1: Difficulty Balancing (2 hours)

**Playtest each puzzle and adjust:**
- Tutorial should be instant (1 move)
- Easy should take 30-60 seconds
- Medium should take 2-3 minutes
- Hard should take 5-10 minutes

**Balancing checklist:**
- [ ] No puzzle is impossible
- [ ] Difficulty increases gradually
- [ ] No sudden difficulty spikes
- [ ] Hints are helpful but not spoilers

---

### Step 5.2: Hint System (1 hour)

**Show hints when stuck:**

**Add to PuzzleView:**
```rust
pub show_hint: bool,

fn render_hint<B: Backend>(&self, f: &mut Frame<B>) {
    if let Some(hint) = &self.state.puzzle.hint {
        let area = centered_rect(60, 30, f.size());
        f.render_widget(Clear, area);

        let paragraph = Paragraph::new(hint.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Hint")
                .border_style(Style::default().fg(Color::Yellow)))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

// In input handler:
KeyCode::Char('h') | KeyCode::Char('H') => {
    self.show_hint = !self.show_hint;
}
```

---

### Step 5.3: Final Testing (1 hour)

**Complete playthrough:**
- [ ] All 30+ puzzles playable
- [ ] No crashes
- [ ] Save/load works
- [ ] Victory screen appears
- [ ] Progress persists

---

## 🎯 **COMPLETION CHECKLIST**

- [x] 30+ puzzles with varied difficulty
- [x] Dimensional display (bars + radar)
- [x] Move preview system
- [x] Undo/reset functionality
- [x] Campaign progression
- [x] Save/load progress
- [x] Victory screens
- [x] Statistics tracking
- [x] Hint system

---

## ✅ **FINAL DELIVERABLES**

1. **30+ balanced puzzles** (tutorial → expert)
2. **Campaign system** (save/load)
3. **Polished UI** (dimensional charts, previews)
4. **All puzzles validated** (solvable)

**Total time:** 30-35 hours over 4-5 days

This is your **best portfolio piece** - unique puzzle mechanic! 🚀
