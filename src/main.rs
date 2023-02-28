use anyhow::Result;
use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use owo_colors::OwoColorize;

use hex::decode;

#[derive(FromArgs)]
/// A little utility to print hex strings as bits
struct Args {}

fn main() -> Result<()> {
    let _args: Args = argh::from_env();

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Gimme a hex string")
        .interact_text()?;

    let items = vec![8, 16, 32, 64];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bits per line")
        .items(&items)
        .default(0)
        .interact()?;

    let bits_per_line = items[selection];
    // let bits_per_line = 32;

    let print_hex_lines = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Display hex values on their own line?")
        .default(true)
        .interact()?;

    // let input = "308191060C2A8648";
    // let input = "308191060C2A8648308191060C2A8648";
    let bytes = decode(input)?;
    // println!("Input: 0x{input}");

    let mut bits_processed = 0;
    let mut bits = vec![];
    let mut nibbles = vec![];

    for byte in bytes {
        let nibble1 = byte >> 4;
        nibbles.push(nibble1);
        let nibble2 = byte & 0b00001111;
        nibbles.push(nibble2);

        for i in (0..8).rev() {
            let mask = 1 << i;
            let bit_is_set = (mask & byte) > 0;
            let bit = u8::from(bit_is_set);
            bits.push(bit);
        }
    }

    let bit_lines = bits.chunks(bits_per_line);
    let nibbles_per_line = bits_per_line / 4;
    let nibble_lines = nibbles.chunks(nibbles_per_line);

    let zipped = bit_lines.zip(nibble_lines);

    println!();
    for (bit_line, nibble_line) in zipped {
        if print_hex_lines {
            for nibble in nibble_line {
                let formatted = nibble_to_string(*nibble);
                print!("{}    ", formatted.green())
            }
            println!();
        }

        for bit in bit_line {
            if bit == &1 {
                print!("{}", bit.blue());
            } else {
                print!("{bit}",);
            }
            bits_processed += 1;

            if (bits_processed % 4) == 0 {
                print!(" ");
            }
        }

        println!();
    }

    Ok(())
}

fn nibble_to_string(nibble: u8) -> String {
    let binding = hex::encode_upper([nibble]);
    let mut chars = binding.chars();
    // skip first char, hex::encode always adds a 0
    chars.next();
    chars.as_str().to_string()
}
