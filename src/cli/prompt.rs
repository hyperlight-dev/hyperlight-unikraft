// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The questions `hluk init` asks when a terminal is attached.  Prompts go
//! to stderr so stdout stays clean for scripts.

use std::io::{BufRead, IsTerminal, Write};

/// Whether asking makes sense: a person is on both ends.
pub fn interactive() -> bool {
    std::io::stdin().is_terminal() && std::io::stderr().is_terminal()
}

/// Pick one of `items` by number or by the name `label` renders.
pub fn select<'a, T>(
    title: &str,
    items: &'a [T],
    label: impl Fn(&T) -> String,
) -> Result<&'a T, String> {
    let labels: Vec<String> = items.iter().map(&label).collect();
    eprintln!("{title}");
    for (i, l) in labels.iter().enumerate() {
        eprintln!("  {:>2}) {l}", i + 1);
    }
    loop {
        let answer = ask(&format!("Choice [1-{}]", items.len()), None)?;
        if let Ok(n) = answer.parse::<usize>()
            && (1..=items.len()).contains(&n)
        {
            return Ok(&items[n - 1]);
        }
        if let Some(i) = labels
            .iter()
            .position(|l| l == &answer || l.split_whitespace().next() == Some(answer.as_str()))
        {
            return Ok(&items[i]);
        }
        eprintln!("  not a choice: {answer:?}");
    }
}

/// Ask a line; an empty answer takes `default` when there is one.
pub fn ask(question: &str, default: Option<&str>) -> Result<String, String> {
    loop {
        match default {
            Some(d) => eprint!("{question} [{d}]: "),
            None => eprint!("{question}: "),
        }
        let _ = std::io::stderr().flush();
        let mut line = String::new();
        let n = std::io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|e| format!("reading the answer: {e}"))?;
        if n == 0 {
            return Err("stdin closed".into());
        }
        let answer = line.trim();
        if !answer.is_empty() {
            return Ok(answer.to_string());
        }
        if let Some(d) = default {
            return Ok(d.to_string());
        }
    }
}
