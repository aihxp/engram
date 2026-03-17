use anyhow::Result;
use clap::Parser;
use engram_sync::{LanceDBBackend, BatchImporter};
use std::sync::Arc;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    dir: String,
    
    #[arg(short, long, default_value = "./engram_memory")]
    db: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Bootstrapping Engram memory from {}...", args.dir);
    
    let backend = Arc::new(LanceDBBackend::new(&args.db).await?);
    let importer = BatchImporter::new(backend);
    
    let count = importer.import_directory(&args.dir).await?;
    
    println!("Imported {} facts successfully.", count);
    
    Ok(())
}
