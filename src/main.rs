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

   let new_years_message = r#"
   ======================================================================
   |   ______ _______  ______  _____  _______ __  __ __   _ _________   | 
   |    ____/ |______ |_____/ |     | |______  \_/   | \  | |           | 
   |   /______|______ |    \_ |_____|_______|   |    |  \_| |_____      | 
   |                                                                    | 
   ======================================================================
    _     _ _______  _____   _____  __   __      __   _ _______ _  _  _  
    |_____| |_____| |_____] |_____]   \_/        | \  | |______ |  |  |  
    |     | |     | |       |          |         |  \_| |______ |__|__|  
 
                    __   __ _______ _______  ______
                      \_/   |______ |_____| |_____/
                       |    |______ |     | |    \_
                
    "#;
   
   println!("{}", new_years_message);
   println!("CHAINSTATE DELIVERY FROM SPACE\n {}", serde_json::to_string_pretty(&chain_state.unwrap()).unwrap());

   Ok(())
}
