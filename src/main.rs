use anyhow::{bail, Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use rand::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(
    name = "mastodon-toot-client",
    about = "Rust CLI bot to post scheduled toots to Mastodon from file inputs"
)]
struct Cli {
    /// Input file path (falls back to INPUT_FILE env var)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Max number of posts to emit in this run
    #[arg(short, long, default_value_t = 1)]
    limit: u32,

    /// Message template (falls back to TOOT_TEMPLATE env var)
    #[arg(long)]
    template: Option<String>,

    /// Deterministic seed (useful for tests)
    #[arg(long)]
    seed: Option<u64>,

    /// Print output without posting
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Enable verbose logs
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

#[derive(Serialize)]
struct StatusPayload<'a> {
    status: &'a str,
    visibility: &'a str,
}

fn main() -> Result<()> {
    // Load .env if present (safe in CI if missing)
    let _ = dotenv();

    let cli = Cli::parse();
    init_tracing(cli.verbose)?;

    if cli.limit == 0 {
        bail!("--limit must be >= 1");
    }

    let base_url = std::env::var("MASTODON_BASE_URL").context("MASTODON_BASE_URL is required")?;
    let token = std::env::var("MASTODON_ACCESS_TOKEN").context("MASTODON_ACCESS_TOKEN is required")?;

    let file_path = match cli.file {
        Some(p) => p,
        None => {
            let env_path = std::env::var("INPUT_FILE")
                .context("No --file provided. Set --file <PATH> or INPUT_FILE env var.",
            )?;
            PathBuf::from(env_path)
        }
    };

    let template = cli
        .template
        .or_else(|| std::env::var("TOOT_TEMPLATE").ok())
        .unwrap_or_else(|| "Today: {item}".to_string());

    let items = load_items(&file_path)?;
    if items.is_empty() {
        bail!("Input file is empty: {}", file_path.display());
    }

    let mut rng: StdRng = match cli.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    for _ in 0..cli.limit {
        let (idx, item) = pick_item(&items, &mut rng);
        let status = render_template(&template, item, idx, items.len());

        if cli.dry_run {
            println!("{}", status);
            continue;
        }

        post_status(&base_url, &token, &status)?;
        info!("Posted toot (idx={})", idx);
    }

    Ok(())
}

fn init_tracing(verbose: bool) -> Result<()> {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();

    Ok(())
}

fn load_items(path: &PathBuf) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file: {}", path.display()))?;

    Ok(content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect())
}

fn pick_item<'a>(items: &'a [String], rng: &'a mut StdRng) -> (usize, &'a str) {
    let idx = rng.gen_range(0..items.len());
    (idx, items[idx].as_str())
}

fn render_template(template: &str, item: &str, idx: usize, total: usize) -> String {
    template
        .replace("{item}", item)
        .replace("{idx}", &idx.to_string())
        .replace("{total}", &total.to_string())
}

fn post_status(base_url: &str, token: &str, status: &str) -> Result<()> {
    let url = format!("{}/api/v1/statuses", base_url.trim_end_matches('/'));
    debug!("POST {}", url);

    let payload = StatusPayload {
        status,
        visibility: "public",
    };

    let client = Client::new();
    let resp = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .context("Failed to call Mastodon API")?;

    if !resp.status().is_success() {
        let code = resp.status();
        let body = resp.text().unwrap_or_else(|_| "<unavailable>".to_string());
        warn!("Non-success response from Mastodon: {}", code);
        bail!("Mastodon API returned {}: {}", code, truncate(&body, 300));
    }

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut out = s[..max].to_string();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn template_renders_placeholders() {
        let t = "Hello {item} #{idx}/{total}";
        let out = render_template(t, "Cape Town", 3, 10);
        assert_eq!(out, "Hello Cape Town #3/10");
    }

    #[test]
    fn load_items_filters_empty_lines() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), "a\n\n b \n").unwrap();
        let items = load_items(&tmp.path().to_path_buf()).unwrap();
        assert_eq!(items, vec!["a".to_string(), "b".to_string()]);
    }
}
