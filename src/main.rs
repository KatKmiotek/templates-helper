use clap::{Parser, Subcommand};
use lambda::create_lambda_project;
use terraform::create_terraform_project;
mod lambda;
mod runtime;
mod template_helper;
mod terraform;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/lambda_templates"]
struct LambdaTemplates;

#[derive(RustEmbed)]
#[folder = "src/terraform_templates"]
struct TerraformTemplates;

#[derive(RustEmbed)]
#[folder = "src/terraform_module_templates"]
struct TerraformModuleTemplates;

#[derive(Parser)]
#[command(name = "template")]
#[clap(about = "Project template generator", long_about = None, version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        about = "Generates new lambda project (TS, .NET, Python) with optional terraform for it"
    )]
    Lambda {},
    #[clap(about = "Generates new blank terraform module")]
    Terraform {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Lambda {} => {
            create_lambda_project().expect("Creating lambda project failed");
        }
        Commands::Terraform {} => {
            create_terraform_project().expect("Creating terraform project failed");
        }
    }
}
