use crate::models::types::Thought;
use std::fs;

pub fn export_thoughts(
    thoughts: &[&Thought],
    filename: &str,
    format: ExportFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = match format {
        ExportFormat::Markdown => generate_markdown(thoughts),
        ExportFormat::PlainText => generate_plain_text(thoughts),
        ExportFormat::Json => generate_json(thoughts)?,
    };

    fs::write(filename, content)?;
    Ok(())
}

pub enum ExportFormat {
    Markdown,
    PlainText,
    Json,
}

fn generate_markdown(thoughts: &[&Thought]) -> String {
    let mut output = String::new();

    output.push_str("# CREATURE Thoughts Export\n\n");
    output.push_str(&format!("Generated: {}\n\n", chrono::Utc::now()));
    output.push_str(&format!("Total thoughts: {}\n\n", thoughts.len()));
    output.push_str("---\n\n");

    for (i, thought) in thoughts.iter().enumerate() {
        output.push_str(&format!("## Thought #{}\n\n", i + 1));

        output.push_str(&format!(
            "**Timestamp**: {}\n\n",
            thought.timestamp.format("%Y-%m-%d %H:%M:%S")
        ));

        output.push_str(&format!("**Relevance**: {:.2} | ", thought.relevance_score));
        output.push_str(&format!("**Confidence**: {:.2}\n\n", thought.confidence_score));

        if !thought.context_tags.is_empty() {
            output.push_str(&format!(
                "**Tags**: {}\n\n",
                thought.context_tags.join(", ")
            ));
        }

        output.push_str("### Content\n\n");
        output.push_str(&thought.content);
        output.push_str("\n\n");

        if let Some(ascii) = &thought.ascii_visualization {
            output.push_str("### Visualization\n\n```\n");
            output.push_str(ascii);
            output.push_str("\n```\n\n");
        }

        output.push_str("---\n\n");
    }

    output
}

fn generate_plain_text(thoughts: &[&Thought]) -> String {
    let mut output = String::new();

    output.push_str("CREATURE THOUGHTS EXPORT\n");
    output.push_str("========================\n\n");

    for (i, thought) in thoughts.iter().enumerate() {
        output.push_str(&format!("--- Thought #{} ---\n", i + 1));
        output.push_str(&format!("Time: {}\n", thought.timestamp));
        output.push_str(&format!("Relevance: {:.2} | Confidence: {:.2}\n",
            thought.relevance_score, thought.confidence_score));
        output.push_str(&format!("Tags: {}\n\n", thought.context_tags.join(", ")));
        output.push_str(&thought.content);
        output.push_str("\n\n");
    }

    output
}

fn generate_json(thoughts: &[&Thought]) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(thoughts)?;
    Ok(json)
}
