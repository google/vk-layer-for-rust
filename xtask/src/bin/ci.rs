// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    /// The path to the output json file.
    #[arg(long)]
    output_path: PathBuf,

    /// The left text of the coverage badge.
    #[arg(long)]
    label: String,

    /// The json field name and the path to the json coverage report separated by colon.
    coverage_report: PathBuf,
}

#[derive(Deserialize)]
struct CoverageStats {
    percent: f64,
}

#[derive(Deserialize)]
struct CoverageReportDataTotals {
    lines: CoverageStats,
}

#[derive(Deserialize)]
struct CoverageReportData {
    totals: CoverageReportDataTotals,
}

#[derive(Deserialize)]
struct CoverageReport {
    data: Vec<CoverageReportData>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShieldBadge {
    schema_version: u32,
    label: String,
    message: String,
    color: String,
}

fn create_shield_badge(label: String, coverage_stats: &CoverageStats) -> ShieldBadge {
    let percent = coverage_stats.percent;
    let color = if (0.0..75.0).contains(&percent) {
        "#e05d44"
    } else if (75.0..90.0).contains(&percent) {
        "#dfb317"
    } else if (90.0..95.0).contains(&percent) {
        "#a3c51c"
    } else if (95.0..100.0).contains(&percent) {
        "#4c1"
    } else {
        "#9f9f9f"
    };
    let message = if (0.0..=100.0).contains(&percent) {
        format!("{:.1}%", percent)
    } else {
        error!("Invalid percentage: {}", percent);
        "unknown".to_owned()
    };
    ShieldBadge {
        schema_version: 1,
        label,
        message,
        color: color.to_owned(),
    }
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let cli = Cli::parse();
    assert!(
        cli.coverage_report.exists(),
        "{} doesn't exist.",
        cli.coverage_report.display()
    );
    let file = File::open(&cli.coverage_report).unwrap_or_else(|e| {
        panic!(
            "Failed to open {} to read: {:?}",
            cli.coverage_report.display(),
            e
        )
    });
    let reader = BufReader::new(file);
    let report: CoverageReport = serde_json::from_reader(reader)
        .unwrap_or_else(|e| panic!("Failed to parse the report: {:?}", e));
    assert!(
        !report.data.is_empty(),
        "Unexpected data field length: {}",
        report.data.len()
    );
    let shield_badge = create_shield_badge(cli.label, &report.data[0].totals.lines);
    let mut out_file = File::create(&cli.output_path).unwrap_or_else(|e| {
        panic!(
            "Failed to open the output file {}: {:?}",
            cli.output_path.display(),
            e
        )
    });
    serde_json::to_writer(&mut out_file, &shield_badge)
        .unwrap_or_else(|e| panic!("Failed to write the shield badge: {:?}", e));
}
