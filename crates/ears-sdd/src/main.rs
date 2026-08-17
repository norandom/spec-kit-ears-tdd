use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use ears_sdd::report::Phase;
use ears_sdd::{render_human, validate, Request};

#[derive(Parser)]
#[command(
    name = "ears-sdd",
    version,
    about = "Deterministic EARS requirements and test-traceability gate"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install the policy into a Spec Kit project.
    Init(InitArgs),
    /// Run a gate and fail the process when it does not pass.
    Validate(GateArgs),
    /// Report the same result, always exiting zero.
    Status(GateArgs),
}

#[derive(clap::Args)]
struct InitArgs {
    #[arg(long, default_value = ".")]
    project: PathBuf,
    #[arg(long, default_value = "codex")]
    integration: String,
    /// Preset and extension resolution priority; lower wins.
    #[arg(long, default_value_t = 5)]
    priority: u32,
}

#[derive(clap::Args)]
struct GateArgs {
    #[arg(long, default_value = ".")]
    project: PathBuf,
    /// Evaluate one feature directory instead of the configured scope.
    #[arg(long)]
    feature: Option<String>,
    /// Evaluate every specification the glob matches, ignoring the active-feature pointer.
    #[arg(long)]
    all: bool,
    #[arg(long, value_enum, default_value = "final")]
    phase: PhaseArg,
    /// Emit the machine-readable report instead of the human summary.
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum PhaseArg {
    Spec,
    Plan,
    Tasks,
    Final,
}

impl From<PhaseArg> for Phase {
    fn from(value: PhaseArg) -> Self {
        match value {
            PhaseArg::Spec => Phase::Spec,
            PhaseArg::Plan => Phase::Plan,
            PhaseArg::Tasks => Phase::Tasks,
            PhaseArg::Final => Phase::Final,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let (args, status_only) = match cli.command {
        Command::Init(args) => {
            return match ears_sdd::init::run(&ears_sdd::init::Options {
                project: args.project,
                integration: args.integration,
                priority: args.priority,
            }) {
                Ok(()) => ExitCode::SUCCESS,
                Err(message) => {
                    eprintln!("{message}");
                    ExitCode::from(2)
                }
            }
        }
        Command::Validate(args) => (args, false),
        Command::Status(args) => (args, true),
    };

    let root = match args.project.canonicalize() {
        Ok(root) => root,
        Err(error) => {
            eprintln!("Project directory is unusable: {error}");
            return ExitCode::from(2);
        }
    };

    let report = validate(Request {
        root: &root,
        phase: args.phase.into(),
        feature: args.feature.as_deref(),
        all: args.all,
    });

    if args.json {
        match serde_json::to_string_pretty(&report) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("Failed to serialize the report: {error}");
                return ExitCode::from(2);
            }
        }
    } else {
        print!("{}", render_human(&report, status_only));
    }

    if status_only || report.ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
