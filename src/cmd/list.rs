//! List all contracts
use crate::{util, Result, Storage};
use ceres_runtime::Metadata;

/// List all contracts
pub fn exec(store: &Storage) -> Result<()> {
    let mut contracts: Vec<Metadata> = Vec::new();
    for r in store.0.iter() {
        let (k, v) = r?;
        if k.len() == 32 {
            continue;
        }

        contracts.push(bincode::deserialize(&v)?);
    }

    let mut output: String = format!("\n\t{} {}\n", util::pad("contract", 20), "code-hash");
    output.push_str(&format!("\t{}\n", &"-".repeat(87)));
    contracts.iter().for_each(|c| {
        output.push_str(&format!(
            "\t{} {}\n",
            &util::pad(&c.contract.name, 20),
            &c.source.hash
        ))
    });

    println!("{}", output);
    Ok(())
}
