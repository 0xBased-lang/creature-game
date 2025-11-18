use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// Import from the creature crate
use creature::thought_viewer::ThoughtViewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading Thought Logger...");

    // Load thoughts
    let mut viewer = ThoughtViewer::load_from_disk()?;

    if viewer.thoughts.is_empty() {
        println!("\n⚠️  No thoughts found!");
        println!("\nTo use Thought Logger:");
        println!("1. Run the colony simulation to generate thoughts, OR");
        println!("2. Add thought JSON files to data/thoughts/\n");
        println!("Sample thoughts have been created for you in data/thoughts/");
        return Ok(());
    }

    println!("Loaded {} thoughts. Starting UI...\n", viewer.thoughts.len());

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear the screen
    terminal.clear()?;

    // Main loop
    let result = run_app(&mut terminal, &mut viewer);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    } else {
        println!("\nThanks for using Thought Logger!");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    viewer: &mut ThoughtViewer,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Render
        terminal.draw(|f| {
            viewer.render(f);
        })?;

        // Handle input (returns false to quit)
        if !viewer.handle_input()? {
            break;
        }
    }

    Ok(())
}
