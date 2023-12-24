use zerosync_verifier::verify;
use std::fs::{File, read};
use flate2::read::GzDecoder;
use tar::Archive;
use clap::Parser;

#[derive(Parser)]
struct Cli {
   proof_dir_name: std::path::PathBuf,
   path: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
   let args = Cli::parse();
   // Unpack archive
   let tar_gz = File::open(args.path)?;
   let tar = GzDecoder::new(tar_gz);
   let mut archive = Archive::new(tar);
   _ = std::fs::create_dir_all(&args.proof_dir_name)?;
   archive.unpack(&args.proof_dir_name)?;

   // Read proof
   let public_input_bytes = read(format!("{}/air-public-input.json", args.proof_dir_name.display())).unwrap();
   let proof_bytes = read(format!("{}/aggregated_proof.bin", args.proof_dir_name.display())).unwrap();
   
   // Verify proof
   let chain_state = verify(public_input_bytes, proof_bytes);

   println!("ChainState: {}", serde_json::to_string(&chain_state.unwrap()).unwrap());

   Ok(())
}
