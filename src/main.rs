use anyhow::Result;
use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use hex::decode;

#[derive(FromArgs)]
/// A little utility to print hex strings as bits
struct Args {}

fn main() -> Result<()> {
    let _args: Args = argh::from_env();

    let input: String = Input::new()
        .with_prompt("Gimme a hex string")
        .interact_text()?;

    let items = vec![8, 16, 32, 64];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bits per line")
        .items(&items)
        .default(0)
        .interact()?;

    let bits_per_line = items[selection];
    // let bytes_per_line = 8;

    // let input = "308191060C2A8648";
    // let input = "308191060C2A8648308191060C2A8648";
    let bytes = decode(input)?;
    // println!("Input: 0x{input}");

    let mut bits_processed = 0;
    let mut _bytes_processed = 0;

    for byte in bytes {
        let mut bits = vec![];
        for i in (0..8).rev() {
            let mask = 1 << i;
            let bit_is_set = (mask & byte) > 0;
            let bit = if bit_is_set { 1u8 } else { 0u8 };
            bits.push(bit);
        }

        for bit in bits {
            print!("{bit}");
            bits_processed += 1;

            if (bits_processed % 4) == 0 {
                print!(" ");
            }
        }

        _bytes_processed += 1;

        if (bits_processed % bits_per_line) == 0 {
            println!();
        }
    }

    Ok(())
}
