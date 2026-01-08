pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

// 256-color ANSI palette (works in most terminals)
pub const NEON_PINK: &str = "\x1b[38;5;205m";
pub const MINT_GREEN: &str = "\x1b[38;5;121m";
pub const NEON_GREEN: &str = "\x1b[38;5;82m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const LIGHT_BLUE: &str = "\x1b[38;5;117m";
pub const PURPLE: &str = "\x1b[38;5;141m";
pub const YELLOW: &str = "\x1b[38;5;226m";
pub const CYAN: &str = "\x1b[38;5;51m";
pub const RED: &str = "\x1b[38;5;196m";
pub const DIM: &str = "\x1b[2m";

pub fn wrap(color: &str, s: &str) -> String {
    format!("{color}{s}{RESET}")
}

// Backwards-compatible helpers you already use elsewhere
pub fn pink(s: &str) -> String {
    wrap(NEON_PINK, s)
}
pub fn green(s: &str) -> String {
    wrap(NEON_GREEN, s)
}
pub fn yellow(s: &str) -> String {
    wrap(YELLOW, s)
}
pub fn cyan(s: &str) -> String {
    wrap(CYAN, s)
}
pub fn red(s: &str) -> String {
    wrap(RED, s)
}
pub fn dim(s: &str) -> String {
    wrap(DIM, s)
}

// Mint formatting (always mint green, bold)
pub fn mint(s: &str) -> String {
    format!("{BOLD}{MINT_GREEN}{s}{RESET}")
}

// Score formatting (tweak bands however you want)
pub fn score_fmt(score: i32) -> String {
    let c = if score >= 75 {
        NEON_GREEN
    } else if score >= 55 {
        LIGHT_BLUE
    } else if score >= 35 {
        PURPLE
    } else if score >= 20 {
        YELLOW
    } else {
        DIM
    };
    format!("{c}{score}{RESET}")
}

// FDV band colors:
// <20k orange, 20-50k light blue, 50-100k purple, 100k+ neon green
pub fn fdv_band(fdv: f64) -> &'static str {
    if fdv >= 100_000.0 {
        NEON_GREEN
    } else if fdv >= 50_000.0 {
        PURPLE
    } else if fdv >= 20_000.0 {
        LIGHT_BLUE
    } else {
        ORANGE
    }
}

pub fn fdv_fmt(fdv: f64) -> String {
    let c = fdv_band(fdv);
    format!("{c}${:.0}{RESET}", fdv)
}

// Used by scoring/engine.rs
pub fn active_line(mint_addr: &str, score: i32) -> String {
    format!(
        "✅ ACTIVE: {} (score={})",
        mint(mint_addr),
        score_fmt(score)
    )
}

// Used by scoring/engine.rs
pub fn call_line(
    mint_addr: &str,
    fdv: f64,
    score: i32,
    tx_5m: u64,
    signers: usize,
    events: usize,
) -> String {
    // CALL label in neon pink, mint in mint green, fdv band-colored, score band-colored
    format!(
        "{NEON_PINK}📣 CALL:{RESET} {} fdv={} score={} tx_5m={} signers={} events={}",
        mint(mint_addr),
        fdv_fmt(fdv),
        score_fmt(score),
        tx_5m,
        signers,
        events
    )
}
