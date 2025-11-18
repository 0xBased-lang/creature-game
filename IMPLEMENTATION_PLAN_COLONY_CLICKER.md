# 🎮 COLONY CLICKER - Complete Implementation Plan

**Game Type**: Idle/Incremental
**Estimated Time**: 5-7 days (40-50 hours)
**Difficulty**: ⭐⭐☆☆☆ (Easy-Medium)
**AI Assistance Level**: HIGH (can help with 80% of code)

---

## 📊 PROJECT OVERVIEW

### What You're Building
An incremental clicker game where players manually spawn cells, watch them evolve and reproduce, then use earned energy to purchase upgrades that automate and accelerate growth. Includes prestige system for long-term progression.

### Core Loop
```
Click to spawn cell → Cell generates energy → Buy upgrades →
Cells spawn automatically → Reach prestige goal → Reset with bonuses
```

### Success Criteria
- ✅ Players can click to spawn cells
- ✅ Cells generate passive energy
- ✅ 10+ upgrades available
- ✅ Prestige system unlocks at specific milestone
- ✅ Save/load progress
- ✅ Balanced progression (fun for 30+ minutes)

---

## 🗂️ FILE STRUCTURE

### New Files to Create
```
src/
├── game/                     # NEW DIRECTORY
│   ├── mod.rs               # Module exports
│   ├── player.rs            # Player state & resources
│   ├── upgrades.rs          # Upgrade system
│   └── clicker_mode.rs      # Main game loop for clicker
├── models/
│   └── player_state.rs      # NEW: Player save data
└── data/                    # NEW DIRECTORY
    ├── upgrades.json        # Upgrade definitions
    └── player_save.json     # Auto-generated save file
```

### Files to Modify
```
src/
├── main.rs                  # Add game mode switching
├── interface/mod.rs         # Add clicker UI panels
└── systems/colony.rs        # Add manual spawn method
```

---

## 📅 IMPLEMENTATION ROADMAP

### Phase 1: Foundation (Day 1 - 8 hours)
- [ ] Create project structure
- [ ] Add player resource system
- [ ] Implement click-to-spawn
- [ ] Basic UI display

### Phase 2: Upgrade System (Day 2-3 - 12 hours)
- [ ] Create upgrade data structures
- [ ] Load upgrades from JSON
- [ ] Implement upgrade purchase logic
- [ ] Upgrade effects application

### Phase 3: UI & Polish (Day 4 - 8 hours)
- [ ] Upgrade panel UI
- [ ] Keybindings
- [ ] Visual feedback
- [ ] Tutorial tooltips

### Phase 4: Prestige System (Day 5 - 6 hours)
- [ ] Prestige logic
- [ ] Bonus calculations
- [ ] Reset mechanics
- [ ] Prestige UI

### Phase 5: Balance & Testing (Day 6-7 - 10 hours)
- [ ] Number balancing
- [ ] Playtest sessions
- [ ] Bug fixes
- [ ] Save/load testing

---

## 🔨 STEP-BY-STEP IMPLEMENTATION

---

## **PHASE 1: FOUNDATION (Day 1)**

### Step 1.1: Create Project Structure (30 minutes)

**Create new directory:**
```bash
mkdir -p src/game
touch src/game/mod.rs
touch src/game/player.rs
touch src/game/upgrades.rs
touch src/game/clicker_mode.rs

mkdir -p data
touch data/upgrades.json
```

**File: `src/game/mod.rs`**
```rust
pub mod player;
pub mod upgrades;
pub mod clicker_mode;

pub use player::Player;
pub use upgrades::{Upgrade, UpgradeEffect, UpgradeManager};
pub use clicker_mode::ClickerGame;
```

**Add to `src/main.rs`** (at the top):
```rust
mod game;
use game::{Player, ClickerGame};
```

---

### Step 1.2: Player Resource System (2 hours)

**File: `src/game/player.rs`**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub energy: f64,
    pub total_energy_earned: f64,
    pub total_cells_spawned: u64,
    pub cells_spawned_this_run: u64,
    pub upgrades: HashMap<String, u32>, // upgrade_id -> level
    pub prestige_level: u32,
    pub prestige_points: f64,
    pub stats: PlayerStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_clicks: u64,
    pub total_playtime_seconds: u64,
    pub highest_cell_count: u64,
    pub fastest_prestige_seconds: Option<u64>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            energy: 100.0,
            total_energy_earned: 0.0,
            total_cells_spawned: 0,
            cells_spawned_this_run: 0,
            upgrades: HashMap::new(),
            prestige_level: 0,
            prestige_points: 0.0,
            stats: PlayerStats::default(),
        }
    }

    /// Called every game tick to generate passive energy
    pub fn tick(&mut self, colony_size: usize, delta_time: f64) {
        let base_rate = 0.1; // Energy per cell per second
        let prestige_multiplier = self.prestige_multiplier();
        let upgrade_multiplier = self.get_passive_income_multiplier();

        let energy_gain = (colony_size as f64)
            * base_rate
            * prestige_multiplier
            * upgrade_multiplier
            * delta_time;

        self.energy += energy_gain;
        self.total_energy_earned += energy_gain;

        // Cap energy to prevent overflow
        self.energy = self.energy.min(1_000_000.0);
    }

    /// Calculate spawn cost (can be reduced by upgrades)
    pub fn spawn_cost(&self) -> f64 {
        let base_cost = 10.0;
        let cost_reduction = self.get_upgrade_value("spawn_cost_reduction");
        (base_cost - cost_reduction).max(1.0)
    }

    /// Try to spawn a cell (returns true if successful)
    pub fn try_spawn(&mut self) -> bool {
        let cost = self.spawn_cost();
        if self.energy >= cost {
            self.energy -= cost;
            self.total_cells_spawned += 1;
            self.cells_spawned_this_run += 1;
            self.stats.total_clicks += 1;
            true
        } else {
            false
        }
    }

    /// Prestige multiplier based on prestige points
    pub fn prestige_multiplier(&self) -> f64 {
        1.0 + (self.prestige_points * 0.1)
    }

    /// Get total value from all levels of an upgrade
    pub fn get_upgrade_value(&self, upgrade_id: &str) -> f64 {
        // This will be implemented when we add upgrade effects
        self.upgrades.get(upgrade_id).copied().unwrap_or(0) as f64
    }

    /// Get passive income multiplier from upgrades
    pub fn get_passive_income_multiplier(&self) -> f64 {
        1.0 + (self.get_upgrade_value("passive_income") * 0.1)
    }

    /// Check if player can prestige
    pub fn can_prestige(&self) -> bool {
        self.total_cells_spawned >= 1000
    }

    /// Save player state to JSON
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("data/player_save.json", json)?;
        Ok(())
    }

    /// Load player state from JSON
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string("data/player_save.json")?;
        let player: Player = serde_json::from_str(&json)?;
        Ok(player)
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            total_clicks: 0,
            total_playtime_seconds: 0,
            highest_cell_count: 0,
            fastest_prestige_seconds: None,
        }
    }
}
```

**Test this step:**
```rust
// In main.rs, temporarily add:
let mut player = Player::new();
println!("Starting energy: {}", player.energy);
player.tick(10, 1.0); // 10 cells, 1 second
println!("After 1 tick: {}", player.energy);
player.try_spawn();
println!("After spawn: {}", player.energy);
```

**Expected output:**
```
Starting energy: 100
After 1 tick: 101
After spawn: 91
```

---

### Step 1.3: Click-to-Spawn Implementation (2 hours)

**File: `src/systems/colony.rs`** - Add this method:
```rust
impl Colony {
    /// Create empty colony (no auto-spawn)
    pub fn new_empty(name: String, mission: String) -> Self {
        Self {
            name,
            mission,
            cells: HashMap::new(),
            cycle: 0,
            lenia_world: LeniaWorld::new(LeniaParams::default()),
            leaderboard: Leaderboard::new(),
        }
    }

    /// Spawn a single cell at random position (for clicker mode)
    pub fn spawn_random_cell(&mut self) -> Uuid {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let position = Coordinates {
            x: rng.gen_range(-10.0..10.0),
            y: rng.gen_range(-10.0..10.0),
            z: rng.gen_range(-10.0..10.0),
        };

        let cell = Cell::new(position);
        let id = cell.id;
        self.cells.insert(id, cell);
        id
    }

    /// Get current cell count
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}
```

---

### Step 1.4: Basic UI Display (3.5 hours)

**File: `src/game/clicker_mode.rs`**
```rust
use crate::game::{Player, UpgradeManager};
use crate::systems::Colony;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

pub struct ClickerGame {
    pub player: Player,
    pub colony: Colony,
    pub upgrade_manager: UpgradeManager,
    pub last_tick: Instant,
    pub selected_upgrade: usize,
}

impl ClickerGame {
    pub fn new() -> Self {
        // Try to load save, or create new
        let player = Player::load().unwrap_or_else(|_| Player::new());
        let colony = Colony::new_empty(
            "Colony Clicker".to_string(),
            "Grow and evolve".to_string(),
        );
        let upgrade_manager = UpgradeManager::load_from_file("data/upgrades.json")
            .expect("Failed to load upgrades.json");

        Self {
            player,
            colony,
            upgrade_manager,
            last_tick: Instant::now(),
            selected_upgrade: 0,
        }
    }

    /// Update game state
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick).as_secs_f64();

        if delta >= 0.1 {
            // Update player energy (passive income)
            self.player.tick(self.colony.cell_count(), delta);

            // Auto-spawn if upgrade unlocked
            if self.can_auto_spawn() {
                let auto_spawn_rate = self.get_auto_spawn_rate();
                let spawns_this_tick = (auto_spawn_rate * delta) as u32;

                for _ in 0..spawns_this_tick {
                    if self.player.try_spawn() {
                        self.colony.spawn_random_cell();
                    }
                }
            }

            // Update colony (cell evolution, reproduction)
            // Note: We'll keep this minimal for clicker mode
            self.colony.cycle += 1;

            self.last_tick = now;
        }
    }

    /// Render the game UI
    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),   // Top bar (energy, stats)
                Constraint::Min(10),     // Main area (colony view + upgrades)
                Constraint::Length(3),   // Bottom bar (controls help)
            ])
            .split(f.size());

        // Top bar
        self.render_top_bar(f, chunks[0]);

        // Main area - split into colony view and upgrade panel
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Colony view
                Constraint::Percentage(50), // Upgrade panel
            ])
            .split(chunks[1]);

        self.render_colony_view(f, main_chunks[0]);
        self.render_upgrade_panel(f, main_chunks[1]);

        // Bottom bar
        self.render_controls(f, chunks[2]);
    }

    fn render_top_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Line::from(vec![
                Span::styled("Energy: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{:.0}", self.player.energy),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("Cells: "),
                Span::styled(
                    format!("{}", self.colony.cell_count()),
                    Style::default().fg(Color::Cyan)
                ),
                Span::raw(" | Total Spawned: "),
                Span::styled(
                    format!("{}", self.player.total_cells_spawned),
                    Style::default().fg(Color::Green)
                ),
            ]),
            Line::from(vec![
                Span::raw("Prestige Level: "),
                Span::styled(
                    format!("{}", self.player.prestige_level),
                    Style::default().fg(Color::Magenta)
                ),
                Span::raw(" ("),
                Span::styled(
                    format!("+{:.0}%", self.player.prestige_multiplier() * 100.0 - 100.0),
                    Style::default().fg(Color::Green)
                ),
                Span::raw(" bonus)"),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Stats"));
        f.render_widget(paragraph, area);
    }

    fn render_colony_view<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // Simple text-based colony display for now
        let lines = vec![
            Line::from(format!("Colony Size: {}", self.colony.cell_count())),
            Line::from(format!("Cycle: {}", self.colony.cycle)),
            Line::from(""),
            Line::from("Press SPACE to spawn a cell!"),
            Line::from(format!("Cost: {:.0} energy", self.player.spawn_cost())),
        ];

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Colony"))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }

    fn render_upgrade_panel<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // Placeholder - we'll implement this in Phase 2
        let text = vec![
            Line::from("Upgrades coming soon!"),
            Line::from(""),
            Line::from("Phase 2 will add:"),
            Line::from("- Passive income"),
            Line::from("- Auto-spawn"),
            Line::from("- Cost reduction"),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Upgrades"));
        f.render_widget(paragraph, area);
    }

    fn render_controls<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let text = Line::from(vec![
            Span::raw("[SPACE] Spawn | "),
            Span::raw("[Q] Quit | "),
            Span::raw("[S] Save | "),
            Span::raw("[P] Prestige ("),
            Span::styled(
                if self.player.can_prestige() { "READY" } else { "NOT YET" },
                if self.player.can_prestige() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                }
            ),
            Span::raw(")"),
        ]);

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, area);
    }

    /// Handle input
    pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.player.save()?;
                        return Ok(false); // Exit
                    }
                    KeyCode::Char(' ') => {
                        // Spawn cell
                        if self.player.try_spawn() {
                            self.colony.spawn_random_cell();
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.player.save()?;
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        if self.player.can_prestige() {
                            self.prestige();
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn can_auto_spawn(&self) -> bool {
        self.player.upgrades.get("auto_spawn").copied().unwrap_or(0) > 0
    }

    fn get_auto_spawn_rate(&self) -> f64 {
        // 1 spawn per second per level
        self.player.upgrades.get("auto_spawn").copied().unwrap_or(0) as f64
    }

    fn prestige(&mut self) {
        // Calculate prestige points
        let points = (self.player.total_cells_spawned as f64 / 100.0).sqrt();

        // Reset progress
        self.player.prestige_level += 1;
        self.player.prestige_points += points;
        self.player.energy = 100.0;
        self.player.cells_spawned_this_run = 0;
        self.player.upgrades.clear();
        self.colony = Colony::new_empty(
            "Colony Clicker".to_string(),
            "Grow and evolve".to_string(),
        );

        // Save after prestige
        self.player.save().ok();
    }
}
```

---

### Step 1.5: Integrate into Main Loop (1 hour)

**File: `src/main.rs`** - Replace main function:
```rust
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod game;
mod systems;
mod models;
mod interface;
mod utils;
mod api;

use game::ClickerGame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create game
    let mut game = ClickerGame::new();

    // Main game loop
    loop {
        // Update game state
        game.tick();

        // Render
        terminal.draw(|f| {
            game.render(f);
        })?;

        // Handle input (returns false to quit)
        if !game.handle_input()? {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("Thanks for playing Colony Clicker!");
    Ok(())
}
```

---

### Step 1.6: Test Phase 1 ✅

**Run the game:**
```bash
cargo run --release
```

**What you should see:**
- Top bar showing energy, cell count, prestige info
- Colony view showing current size
- Placeholder upgrade panel
- Bottom controls

**Test checklist:**
- [ ] Press SPACE - energy decreases, cell count increases
- [ ] Energy regenerates over time (watch the number go up)
- [ ] Press S - creates `data/player_save.json`
- [ ] Restart game - your progress loads
- [ ] Press Q - exits cleanly

**If it doesn't work:**
- Check that `data/` directory exists
- Make sure all imports are correct
- Run `cargo check` to see compiler errors
- AI prompt: "I'm getting error X in Colony Clicker Phase 1, here's my code: [paste error]"

---

## **PHASE 2: UPGRADE SYSTEM (Day 2-3)**

### Step 2.1: Upgrade Data Structures (2 hours)

**File: `src/game/upgrades.rs`**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub id: String,
    pub name: String,
    pub description: String,
    pub base_cost: f64,
    pub cost_multiplier: f64,
    pub max_level: u32,
    pub effect: UpgradeEffect,
    #[serde(default)]
    pub category: UpgradeCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeEffect {
    /// Increase passive energy generation (value = % increase per level)
    PassiveIncome(f64),

    /// Reduce spawn cost (value = flat reduction per level)
    SpawnCostReduction(f64),

    /// Enable auto-spawning (value = spawns per second per level)
    AutoSpawn(f64),

    /// Increase cell reproduction chance (value = % increase per level)
    ReproductionBonus(f64),

    /// Start each prestige with bonus energy (value = flat bonus per level)
    StartingEnergy(f64),

    /// Increase prestige point multiplier (value = % increase per level)
    PrestigeBonus(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpgradeCategory {
    Production,  // Passive income
    Spawning,    // Spawn cost, auto-spawn
    Evolution,   // Cell mechanics
    Prestige,    // Prestige bonuses
}

impl Default for UpgradeCategory {
    fn default() -> Self {
        Self::Production
    }
}

impl Upgrade {
    /// Calculate cost for next level
    pub fn cost_for_level(&self, current_level: u32) -> f64 {
        if current_level >= self.max_level {
            return f64::INFINITY; // Can't buy
        }
        self.base_cost * self.cost_multiplier.powi(current_level as i32)
    }

    /// Get current effect value at given level
    pub fn value_at_level(&self, level: u32) -> f64 {
        let base_value = match &self.effect {
            UpgradeEffect::PassiveIncome(v) => *v,
            UpgradeEffect::SpawnCostReduction(v) => *v,
            UpgradeEffect::AutoSpawn(v) => *v,
            UpgradeEffect::ReproductionBonus(v) => *v,
            UpgradeEffect::StartingEnergy(v) => *v,
            UpgradeEffect::PrestigeBonus(v) => *v,
        };
        base_value * (level as f64)
    }

    /// Check if this upgrade is available (no prerequisites for now)
    pub fn is_available(&self, _player_level: u32) -> bool {
        true // Could add unlock conditions later
    }
}

pub struct UpgradeManager {
    pub upgrades: Vec<Upgrade>,
}

impl UpgradeManager {
    /// Load upgrades from JSON file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let upgrades: Vec<Upgrade> = serde_json::from_str(&json)?;
        Ok(Self { upgrades })
    }

    /// Get upgrade by ID
    pub fn get_upgrade(&self, id: &str) -> Option<&Upgrade> {
        self.upgrades.iter().find(|u| u.id == id)
    }

    /// Get all upgrades in a category
    pub fn upgrades_by_category(&self, category: UpgradeCategory) -> Vec<&Upgrade> {
        self.upgrades.iter()
            .filter(|u| u.category == category)
            .collect()
    }
}
```

---

### Step 2.2: Create Upgrades JSON (2 hours)

**File: `data/upgrades.json`**
```json
[
  {
    "id": "passive_income",
    "name": "Solar Panels",
    "description": "Cells generate +10% more energy",
    "base_cost": 50.0,
    "cost_multiplier": 1.5,
    "max_level": 20,
    "effect": { "PassiveIncome": 0.1 },
    "category": "Production"
  },
  {
    "id": "spawn_cost_reduction",
    "name": "Efficient Cloning",
    "description": "Reduce spawn cost by 1 energy",
    "base_cost": 100.0,
    "cost_multiplier": 2.0,
    "max_level": 9,
    "effect": { "SpawnCostReduction": 1.0 },
    "category": "Spawning"
  },
  {
    "id": "auto_spawn",
    "name": "Automated Spawner",
    "description": "Spawn 1 cell per second automatically",
    "base_cost": 500.0,
    "cost_multiplier": 3.0,
    "max_level": 10,
    "effect": { "AutoSpawn": 1.0 },
    "category": "Spawning"
  },
  {
    "id": "reproduction_bonus",
    "name": "Fertility Boost",
    "description": "Cells reproduce +5% more often",
    "base_cost": 200.0,
    "cost_multiplier": 1.8,
    "max_level": 15,
    "effect": { "ReproductionBonus": 0.05 },
    "category": "Evolution"
  },
  {
    "id": "starting_energy",
    "name": "Energy Reserves",
    "description": "Start with +100 bonus energy",
    "base_cost": 1000.0,
    "cost_multiplier": 2.5,
    "max_level": 5,
    "effect": { "StartingEnergy": 100.0 },
    "category": "Prestige"
  },
  {
    "id": "prestige_bonus",
    "name": "Prestige Power",
    "description": "Gain +10% more prestige points",
    "base_cost": 5000.0,
    "cost_multiplier": 3.0,
    "max_level": 10,
    "effect": { "PrestigeBonus": 0.1 },
    "category": "Prestige"
  },
  {
    "id": "bulk_spawn",
    "name": "Mass Production",
    "description": "SPACE spawns 5 cells at once",
    "base_cost": 2000.0,
    "cost_multiplier": 5.0,
    "max_level": 3,
    "effect": { "AutoSpawn": 0.0 },
    "category": "Spawning"
  },
  {
    "id": "energy_cap",
    "name": "Energy Storage",
    "description": "Increase max energy capacity",
    "base_cost": 300.0,
    "cost_multiplier": 2.0,
    "max_level": 10,
    "effect": { "PassiveIncome": 0.0 },
    "category": "Production"
  },
  {
    "id": "faster_evolution",
    "name": "Rapid Evolution",
    "description": "Cells evolve faster",
    "base_cost": 800.0,
    "cost_multiplier": 2.2,
    "max_level": 5,
    "effect": { "ReproductionBonus": 0.1 },
    "category": "Evolution"
  },
  {
    "id": "prestige_cost_reduction",
    "name": "Quick Rebirth",
    "description": "Reduce prestige requirement by 100 cells",
    "base_cost": 10000.0,
    "cost_multiplier": 4.0,
    "max_level": 5,
    "effect": { "PrestigeBonus": 0.0 },
    "category": "Prestige"
  }
]
```

**Test loading:**
```rust
// In main.rs temporarily:
let upgrades = UpgradeManager::load_from_file("data/upgrades.json")?;
println!("Loaded {} upgrades", upgrades.upgrades.len());
for upgrade in &upgrades.upgrades {
    println!("- {}: {}", upgrade.name, upgrade.description);
}
```

---

### Step 2.3: Purchase Logic (3 hours)

**Add to `src/game/player.rs`:**
```rust
impl Player {
    /// Try to purchase an upgrade
    pub fn try_purchase_upgrade(
        &mut self,
        upgrade: &Upgrade,
    ) -> Result<(), PurchaseError> {
        let current_level = self.upgrades.get(&upgrade.id).copied().unwrap_or(0);

        // Check max level
        if current_level >= upgrade.max_level {
            return Err(PurchaseError::MaxLevel);
        }

        // Check cost
        let cost = upgrade.cost_for_level(current_level);
        if self.energy < cost {
            return Err(PurchaseError::NotEnoughEnergy);
        }

        // Purchase
        self.energy -= cost;
        *self.upgrades.entry(upgrade.id.clone()).or_insert(0) += 1;

        Ok(())
    }

    /// Get current level of upgrade
    pub fn upgrade_level(&self, upgrade_id: &str) -> u32 {
        self.upgrades.get(upgrade_id).copied().unwrap_or(0)
    }

    /// Apply all upgrade effects (called during initialization and prestige)
    pub fn apply_upgrade_effects(&mut self, upgrade_manager: &UpgradeManager) {
        // Reset calculated values
        // (This is called when loading or after prestige)

        // Starting energy bonus
        if let Some(upgrade) = upgrade_manager.get_upgrade("starting_energy") {
            let level = self.upgrade_level("starting_energy");
            let bonus = upgrade.value_at_level(level);
            self.energy = 100.0 + bonus;
        }
    }

    /// Get passive income multiplier from all upgrades
    pub fn get_passive_income_multiplier(&self) -> f64 {
        let passive_levels = self.upgrade_level("passive_income");
        1.0 + (passive_levels as f64 * 0.1) // 10% per level
    }

    /// Get spawn cost with reductions
    pub fn spawn_cost(&self) -> f64 {
        let base_cost = 10.0;
        let reduction = self.upgrade_level("spawn_cost_reduction") as f64;
        (base_cost - reduction).max(1.0)
    }

    /// Get cells per click (bulk spawn upgrade)
    pub fn cells_per_click(&self) -> u32 {
        if self.upgrade_level("bulk_spawn") > 0 {
            5 * self.upgrade_level("bulk_spawn")
        } else {
            1
        }
    }
}

#[derive(Debug)]
pub enum PurchaseError {
    NotEnoughEnergy,
    MaxLevel,
}
```

---

### Step 2.4: Upgrade Effects Application (2 hours)

**Update `src/game/clicker_mode.rs`:**
```rust
impl ClickerGame {
    pub fn new() -> Self {
        let mut player = Player::load().unwrap_or_else(|_| Player::new());
        let upgrade_manager = UpgradeManager::load_from_file("data/upgrades.json")
            .expect("Failed to load upgrades.json");

        // Apply upgrade effects on load
        player.apply_upgrade_effects(&upgrade_manager);

        // ... rest of constructor
    }

    /// Handle upgrade purchase
    pub fn buy_selected_upgrade(&mut self) -> Result<(), PurchaseError> {
        if let Some(upgrade) = self.upgrade_manager.upgrades.get(self.selected_upgrade) {
            self.player.try_purchase_upgrade(upgrade)?;
        }
        Ok(())
    }

    /// Updated spawn to handle bulk spawning
    pub fn spawn_cells(&mut self) {
        let cells_to_spawn = self.player.cells_per_click();

        for _ in 0..cells_to_spawn {
            if self.player.try_spawn() {
                self.colony.spawn_random_cell();
            } else {
                break; // Not enough energy
            }
        }
    }

    /// Get auto-spawn rate from upgrades
    fn get_auto_spawn_rate(&self) -> f64 {
        let level = self.player.upgrade_level("auto_spawn");
        level as f64 // 1 spawn/sec per level
    }

    fn can_auto_spawn(&self) -> bool {
        self.player.upgrade_level("auto_spawn") > 0
    }
}
```

---

### Step 2.5: Test Phase 2 ✅

**Create a test save file with energy:**
```json
{
  "energy": 10000.0,
  "total_energy_earned": 0.0,
  "total_cells_spawned": 0,
  "cells_spawned_this_run": 0,
  "upgrades": {},
  "prestige_level": 0,
  "prestige_points": 0.0,
  "stats": {
    "total_clicks": 0,
    "total_playtime_seconds": 0,
    "highest_cell_count": 0,
    "fastest_prestige_seconds": null
  }
}
```

**Test checklist:**
- [ ] Upgrades load from JSON without errors
- [ ] Can calculate costs correctly (check with println!)
- [ ] Purchasing reduces energy
- [ ] Upgrade levels increase
- [ ] Effects apply (check passive income increases)

---

## **PHASE 3: UI & POLISH (Day 4)**

### Step 3.1: Upgrade Panel UI (4 hours)

**Update `src/game/clicker_mode.rs` - `render_upgrade_panel`:**
```rust
fn render_upgrade_panel<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    // Create scrollable list of upgrades
    use ratatui::widgets::{List, ListItem};

    let mut items = Vec::new();

    for (i, upgrade) in self.upgrade_manager.upgrades.iter().enumerate() {
        let current_level = self.player.upgrade_level(&upgrade.id);
        let cost = upgrade.cost_for_level(current_level);
        let can_afford = self.player.energy >= cost;
        let maxed = current_level >= upgrade.max_level;

        // Style based on affordability
        let style = if i == self.selected_upgrade {
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else if maxed {
            Style::default().fg(Color::DarkGray)
        } else if can_afford {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        // Format upgrade info
        let level_text = if maxed {
            format!("[MAX]")
        } else {
            format!("[Lv {}]", current_level)
        };

        let cost_text = if maxed {
            "MAXED".to_string()
        } else {
            format!("{:.0} energy", cost)
        };

        let line = format!(
            "{} {} - {}",
            level_text,
            upgrade.name,
            cost_text
        );

        items.push(ListItem::new(line).style(style));
    }

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Upgrades [↑↓ to select, ENTER to buy]"));

    f.render_widget(list, area);

    // Show selected upgrade details below if space
    // (We'll add this in Step 3.2)
}
```

---

### Step 3.2: Upgrade Details View (2 hours)

**Update `render_upgrade_panel` to show details:**
```rust
fn render_upgrade_panel<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    // Split area into list and detail
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70), // Upgrade list
            Constraint::Percentage(30), // Selected detail
        ])
        .split(area);

    // Render list (same as before)
    // ... list rendering code ...
    f.render_widget(list, chunks[0]);

    // Render selected upgrade detail
    if let Some(upgrade) = self.upgrade_manager.upgrades.get(self.selected_upgrade) {
        let current_level = self.player.upgrade_level(&upgrade.id);
        let current_value = upgrade.value_at_level(current_level);
        let next_value = upgrade.value_at_level(current_level + 1);

        let detail_text = vec![
            Line::from(vec![
                Span::styled(&upgrade.name, Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Line::from(&upgrade.description),
            Line::from(""),
            Line::from(vec![
                Span::raw("Current: "),
                Span::styled(
                    format!("{:.1}", current_value),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Next: "),
                Span::styled(
                    format!("{:.1}", next_value),
                    Style::default().fg(Color::Green)
                ),
                Span::raw(format!(" (+{:.1})", next_value - current_value)),
            ]),
        ];

        let detail = Paragraph::new(detail_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });
        f.render_widget(detail, chunks[1]);
    }
}
```

---

### Step 3.3: Keybindings & Input (1 hour)

**Update `handle_input` in `clicker_mode.rs`:**
```rust
pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.player.save()?;
                    return Ok(false);
                }
                KeyCode::Char(' ') => {
                    self.spawn_cells();
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    self.player.save()?;
                }
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    if self.player.can_prestige() {
                        self.prestige();
                    }
                }
                KeyCode::Up => {
                    if self.selected_upgrade > 0 {
                        self.selected_upgrade -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected_upgrade < self.upgrade_manager.upgrades.len() - 1 {
                        self.selected_upgrade += 1;
                    }
                }
                KeyCode::Enter => {
                    // Try to buy selected upgrade
                    if let Err(e) = self.buy_selected_upgrade() {
                        // Could show error message in UI later
                        // For now, just ignore
                    }
                }
                _ => {}
            }
        }
    }
    Ok(true)
}
```

---

### Step 3.4: Visual Feedback (1 hour)

**Add purchase feedback animation:**
```rust
// Add to ClickerGame struct:
pub struct ClickerGame {
    // ... existing fields
    pub last_purchase: Option<(String, Instant)>, // (upgrade_name, when)
}

impl ClickerGame {
    pub fn buy_selected_upgrade(&mut self) -> Result<(), PurchaseError> {
        if let Some(upgrade) = self.upgrade_manager.upgrades.get(self.selected_upgrade) {
            self.player.try_purchase_upgrade(upgrade)?;
            self.last_purchase = Some((upgrade.name.clone(), Instant::now()));
        }
        Ok(())
    }
}

// Update render_top_bar to show recent purchase:
fn render_top_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    let mut lines = vec![
        // ... existing lines
    ];

    // Show recent purchase for 2 seconds
    if let Some((name, when)) = &self.last_purchase {
        if when.elapsed().as_secs() < 2 {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("✓ Purchased: {}", name),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Stats"));
    f.render_widget(paragraph, area);
}
```

---

## **PHASE 4: PRESTIGE SYSTEM (Day 5)**

### Step 4.1: Prestige Calculation (2 hours)

**Update `src/game/player.rs`:**
```rust
impl Player {
    /// Calculate how many prestige points would be earned
    pub fn calculate_prestige_points(&self) -> f64 {
        let base_points = (self.total_cells_spawned as f64 / 100.0).sqrt();
        let bonus_multiplier = 1.0 + (self.upgrade_level("prestige_bonus") as f64 * 0.1);
        base_points * bonus_multiplier
    }

    /// Get prestige requirement (can be reduced by upgrades)
    pub fn prestige_requirement(&self) -> u64 {
        let base_requirement = 1000;
        let reduction = self.upgrade_level("prestige_cost_reduction") as u64 * 100;
        base_requirement.saturating_sub(reduction).max(100)
    }

    /// Check if can prestige
    pub fn can_prestige(&self) -> bool {
        self.total_cells_spawned >= self.prestige_requirement()
    }

    /// Perform prestige
    pub fn do_prestige(&mut self, upgrade_manager: &UpgradeManager) {
        let points = self.calculate_prestige_points();

        // Update stats
        self.prestige_level += 1;
        self.prestige_points += points;

        // Reset run-specific progress
        self.energy = 100.0;
        self.cells_spawned_this_run = 0;
        self.upgrades.clear();

        // Apply starting bonuses
        self.apply_upgrade_effects(upgrade_manager);
    }

    /// Get prestige multiplier
    pub fn prestige_multiplier(&self) -> f64 {
        1.0 + (self.prestige_points * 0.1)
    }
}
```

---

### Step 4.2: Prestige UI (2 hours)

**Add prestige confirmation screen:**
```rust
// Add to ClickerGame:
pub struct ClickerGame {
    // ... existing
    pub show_prestige_modal: bool,
}

impl ClickerGame {
    pub fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check if in prestige modal
        if self.show_prestige_modal {
            return self.handle_prestige_modal_input();
        }

        // ... normal input handling

        KeyCode::Char('p') | KeyCode::Char('P') => {
            if self.player.can_prestige() {
                self.show_prestige_modal = true;
            }
        }
    }

    fn handle_prestige_modal_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.prestige();
                        self.show_prestige_modal = false;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        self.show_prestige_modal = false;
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn prestige(&mut self) {
        self.player.do_prestige(&self.upgrade_manager);
        self.colony = Colony::new_empty(
            "Colony Clicker".to_string(),
            "Grow and evolve".to_string(),
        );
        self.player.save().ok();
    }
}

// Add rendering for prestige modal:
pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
    // ... normal rendering

    // Overlay prestige modal if active
    if self.show_prestige_modal {
        self.render_prestige_modal(f);
    }
}

fn render_prestige_modal<B: Backend>(&self, f: &mut Frame<B>) {
    use ratatui::widgets::Clear;

    let points = self.player.calculate_prestige_points();
    let new_multiplier = 1.0 + ((self.player.prestige_points + points) * 0.1);

    // Center modal
    let area = centered_rect(60, 40, f.size());

    // Clear background
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "🌟 PRESTIGE 🌟",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("You will gain "),
            Span::styled(
                format!("{:.2}", points),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ),
            Span::raw(" prestige points"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("New multiplier: "),
            Span::styled(
                format!("{:.1}x", new_multiplier),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(""),
        Line::from("This will reset:"),
        Line::from("  • Energy"),
        Line::from("  • Cells"),
        Line::from("  • All upgrades"),
        Line::from(""),
        Line::from("But you keep:"),
        Line::from("  • Prestige points"),
        Line::from("  • Total cells spawned"),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Y]", Style::default().fg(Color::Green)),
            Span::raw(" Prestige  "),
            Span::styled("[N]", Style::default().fg(Color::Red)),
            Span::raw(" Cancel"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title("Prestige?"))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}

// Helper function to center a rect
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

### Step 4.3: Prestige Upgrades (2 hours)

These upgrades only make sense after first prestige. Add unlock conditions:

**Update `src/game/upgrades.rs`:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    // ... existing fields
    #[serde(default)]
    pub requires_prestige: u32, // Minimum prestige level to unlock
}

impl Upgrade {
    pub fn is_available(&self, prestige_level: u32) -> bool {
        prestige_level >= self.requires_prestige
    }
}
```

**Update upgrades.json to add requirements:**
```json
{
  "id": "prestige_bonus",
  "name": "Prestige Power",
  "description": "Gain +10% more prestige points",
  "base_cost": 5000.0,
  "cost_multiplier": 3.0,
  "max_level": 10,
  "effect": { "PrestigeBonus": 0.1 },
  "category": "Prestige",
  "requires_prestige": 1
}
```

**Filter upgrades in UI:**
```rust
fn render_upgrade_panel<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
    let available_upgrades: Vec<_> = self.upgrade_manager.upgrades.iter()
        .filter(|u| u.is_available(self.player.prestige_level))
        .collect();

    // ... render available_upgrades instead of all upgrades
}
```

---

## **PHASE 5: BALANCE & TESTING (Day 6-7)**

### Step 5.1: Balancing Numbers (4 hours)

**Create a balance testing spreadsheet or script:**
```rust
// test_balance.rs (separate binary for testing)
fn main() {
    // Simulate game progression
    let mut energy = 100.0;
    let mut cells = 0;

    println!("=== Balance Test ===");
    println!("Time (s) | Energy | Cells | Income/s");

    for time in 0..3600 { // 1 hour simulation
        // Passive income
        energy += cells as f64 * 0.1;

        // Auto-buy spawn if affordable
        if energy >= 10.0 {
            energy -= 10.0;
            cells += 1;
        }

        if time % 60 == 0 {
            println!("{:8} | {:6.0} | {:5} | {:8.2}",
                time, energy, cells, cells as f64 * 0.1);
        }
    }
}
```

**Run simulations and adjust:**
- Base spawn cost
- Passive income rate
- Upgrade costs and effects
- Prestige requirement and rewards

**Target feel:**
- First 5 minutes: Clicking feels impactful
- 5-15 minutes: Upgrades unlock, automation begins
- 15-30 minutes: Idle income takes over
- 30+ minutes: Ready to prestige

---

### Step 5.2: Playtesting (4 hours)

**Test checklist:**
- [ ] Can reach first upgrade in < 2 minutes
- [ ] Auto-spawn unlocks around 5-10 minutes
- [ ] First prestige happens around 20-30 minutes
- [ ] Second run is noticeably faster
- [ ] No upgrade feels useless
- [ ] No upgrade is mandatory
- [ ] UI is readable
- [ ] Controls are intuitive
- [ ] Saving/loading works perfectly

**Get feedback:**
- Have 3-5 people playtest
- Watch them play (don't help!)
- Note where they get confused
- Adjust based on feedback

---

### Step 5.3: Bug Fixes (2 hours)

**Common bugs to watch for:**
- [ ] Energy going negative
- [ ] Upgrade costs not updating
- [ ] Prestige not resetting correctly
- [ ] Save file corruption
- [ ] UI overflow with large numbers
- [ ] Keybindings not working

**Test edge cases:**
- Max level upgrades
- Prestige with 0 upgrades
- Rapid clicking (check for double-spawns)
- Loading old save files

---

## 🎯 **COMPLETION CHECKLIST**

### Core Features
- [x] Click to spawn cells
- [x] Passive energy generation
- [x] 10+ upgrades with different effects
- [x] Upgrade purchase system
- [x] Auto-spawn automation
- [x] Prestige system with bonuses
- [x] Save/load functionality
- [x] Responsive UI

### Polish
- [ ] Tutorial/help screen (optional)
- [ ] Statistics screen (total clicks, etc.)
- [ ] Achievement system (optional)
- [ ] Sound effects (optional, requires audio library)
- [ ] Particle effects for spawns (optional)

### Balance
- [ ] Numbers feel good (not too fast/slow)
- [ ] Progression curve is satisfying
- [ ] Upgrades are balanced
- [ ] Prestige is rewarding

---

## 🐛 **TROUBLESHOOTING GUIDE**

### "Failed to load upgrades.json"
- Check file exists in `data/` directory
- Validate JSON syntax (use jsonlint.com)
- Check file permissions

### "Energy going negative"
- Add bounds checking: `energy = energy.max(0.0)`
- Check spawn cost calculation

### "UI looks broken"
- Terminal too small? Resize window
- Try different terminal (some don't support colors)
- Check ratatui version compatibility

### "Upgrades don't do anything"
- Verify effects are applied in tick()
- Check upgrade_level() returns correct value
- Add debug prints to trace effect application

### "Game runs too fast/slow"
- Adjust tick rate in main loop
- Change delta_time calculations
- Add frame limiter

---

## 🚀 **NEXT STEPS AFTER COMPLETION**

### Version 1.1 Ideas
- Add more upgrade categories (research, mutations, etc.)
- Implement challenges (reach X cells in Y time)
- Add achievements with rewards
- Create upgrade presets (build orders)
- Add offline progress (time since last save)

### Version 2.0 Ideas
- Multiple colony types (different starting bonuses)
- Web UI version (replace TUI with HTML/JS)
- Leaderboards (local or online)
- Export/import save codes
- Modding support (load custom upgrades)

---

## 📚 **AI ASSISTANT PROMPTS**

Use these prompts when you get stuck:

**For debugging:**
```
I'm working on Colony Clicker (Rust + Ratatui). I'm getting error:
[paste error]

Here's my code:
[paste relevant code]

What's wrong and how do I fix it?
```

**For new features:**
```
I'm adding a new upgrade type to Colony Clicker that should [describe effect].
Can you help me implement the UpgradeEffect enum variant and application logic?

Current UpgradeEffect enum:
[paste enum]
```

**For balancing:**
```
I need help balancing Colony Clicker. Current progression:
- Spawn cost: 10 energy
- Passive income: 0.1 energy/cell/sec
- First upgrade: 50 energy

Players say it's too [slow/fast]. What should I adjust?
```

---

## ✅ **FINAL DELIVERABLES**

When you're done, you should have:
1. **Working game** (can play for 30+ min without getting bored)
2. **10+ upgrades** (with clear effects)
3. **Prestige system** (with meaningful bonuses)
4. **Save/load** (persistent progress)
5. **Polished UI** (readable, intuitive)
6. **Balanced progression** (tested by real players)

**Estimated total time:** 40-50 hours over 5-7 days

Good luck! This is your most achievable game. Ship it! 🚀
