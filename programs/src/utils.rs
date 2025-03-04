use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

pub const LAMPORTS_ONE_SOL: u64 = 1_000_000_000;
pub fn get_price(supply: u64, amount: u64) -> u64 {
    let sum1 = if supply == 0 {
        0
    } else {
        (supply - 1) * (supply) * (2 * (supply - 1) + 1) / 6
    };
    let sum2 = if supply == 0 && amount == 1 {
        0
    } else {
        (supply - 1 + amount) * (supply + amount) * (2 * (supply - 1 + amount) + 1) / 6
    };
    (sum2 - sum1) * LAMPORTS_ONE_SOL / 16000
}

pub fn sol_transfer<'info>(
    system_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_context = CpiContext::new(system_program, Transfer { from, to });
    transfer(cpi_context, amount)?;
    Ok(())
}

// fn sol_transfer_with_signer<'a, 'b, 'c, 'info>(
//     system_program: AccountInfo<'info>,
//     from: AccountInfo<'info>,
//     to: AccountInfo<'info>,
//     seeds: &'a [&'b [&'c [u8]]],
//     amount: u64,
// ) -> Result<()> {
//     let cpi_ctx = CpiContext::new_with_signer(
//         system_program.to_account_info(),
//         Transfer { from, to },
//         seeds,
//     );

//     transfer(cpi_ctx, amount)?;
//     Ok(())
// }
