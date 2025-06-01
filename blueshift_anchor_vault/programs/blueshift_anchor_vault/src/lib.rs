use anchor_lang::prelude::*;

declare_id!("3ChhMDnr1v5FuT5tAgEur6KpuWpoGVwAQEw8pQonj3f4");

#[program]
pub mod blueshift_anchor_vault {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // Confirms the vault is empty - preventing double deposits
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyExists,
        );

        // Checks the amount clears the rent-exempt threshold
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount,
        );

        let _ = transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(), // signer is vault owner
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount, // deposit lamports amount
        );

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        let bindings = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault", bindings.as_ref(), &[ctx.bumps.vault]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]], // PDA act as signer for this CPI call
            ),
            ctx.accounts.vault.lamports(), // all vault lamports
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)] // Flag account as mutable
    pub signer: Signer<'info>, // Verifies the account signed the txn

    // Verifies account is a PDA generated from provided seeds + bump byte
    #[account(mut, seeds = [b"vault", signer.key().as_ref()], bump,)]
    pub vault: SystemAccount<'info>, // Confirms ownership of the account by System Program

    pub system_program: Program<'info, System>, // Ensures account is executable and matches System Program ID
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}
