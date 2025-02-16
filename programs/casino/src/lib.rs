use anchor_lang::{prelude::*,
    system_program::{transfer, Transfer},
};
use rand::Rng;
use std::str::FromStr;

declare_id!("22XvUuqu3YWzAEf1sN5YxH24pnSDGYe6GsJQKKS2yjaz");

use anchor_lang::solana_program::sysvar::clock::Clock;
pub fn rand_num() -> u8 {
    let clock = Clock::get().unwrap();
    (clock.unix_timestamp % 100) as u8 // Generates a number between 0 and 99
}
#[program]
pub mod casino {
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

    use super::*;

    pub fn  initialize(ctx:Context<InitializeVault>) -> Result<()>{
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.user.key;
        vault.balance = 0;
        Ok(())

    }

   

    pub fn bet(ctx: Context<Vault>, bet_amount: u64) -> Result<()> {
        msg!("Bet amount: {}", bet_amount);

        let from = &ctx.accounts.user;
        let to = &ctx.accounts.vault_account;

        let cpi_accounts=  Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        };

        let cpi_context=CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_context, bet_amount*LAMPORTS_PER_SOL)?;


        Ok(())
    }

    pub fn betprocess(ctx: Context<VaultP>, picked_number: u8) -> Result<()> {
        msg!("Picked number: {}", picked_number);
        let random_number = rand_num() ; // Generates a number between 1 and 10

        if random_number == picked_number {
            msg!("You won the lottery");
            //let seed =ctx.accounts.user.key();

            let cpi_accounts =Transfer {
                from: ctx.accounts.vault_account.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            };
            let seed=ctx.accounts.user.key();
            let vault_seed = &[b"vault", seed.as_ref(), &[ctx.bumps.vault]];
            let vault_signer = &[&vault_seed[..]];


            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
                vault_signer, 
            );
            transfer(cpi_context, ctx.accounts.vault_account.lamports())?;

              


            Ok(())
        } else {
            msg!("You lost the lottery");

            let cpi_accounts =Transfer {
                from: ctx.accounts.vault_account.to_account_info(),
                to: ctx.accounts.owner_account.to_account_info(),
            };
            let seed=ctx.accounts.user.key();
            let vault_seed = &[b"vault", seed.as_ref(), &[ctx.bumps.vault]];
            let vault_signer = &[&vault_seed[..]];


            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
                vault_signer,
            );
            transfer(cpi_context, ctx.accounts.vault_account.lamports())?;


            Ok(())
        }
    }
}

#[account]
pub struct StoreA {
    pub owner: Pubkey,  // Owner of the vault
    pub balance: u64,   // SOL balance stored in vault
}



#[derive(Accounts)]
pub struct InitializeVault<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

        #[account(
        init,
        space = 8 + 32+8, // 8 (discriminator) + 32 (Pubkey) + 8 (u64),
        payer = user,
        seeds = [b"tluav", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, StoreA>,
    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault_account: SystemAccount<'info>,
    // #[account(mut)]
    // pub user_account :SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"tluav", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, StoreA>,

    #[account(mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct VaultP<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"tluav", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, StoreA>,

    #[account(mut)]
    pub owner_account: SystemAccount<'info>,

    #[account(mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}
