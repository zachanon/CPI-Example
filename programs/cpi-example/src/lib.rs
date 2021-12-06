use anchor_lang::prelude::*;

declare_id!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
mod jet_cpi_example {

    use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

    use super::*;

    #[derive(Accounts)]
    pub struct DepositAccountInterfaceInit<'info> {
        // jet accounts
        pub jet: AccountInfo<'info>,
        pub market: AccountInfo<'info>,
        pub market_authority: AccountInfo<'info>,
        pub reserve: AccountInfo<'info>,
        pub deposit_note_mint: AccountInfo<'info>,
        pub deposit_account: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,

        pub payer: AccountInfo<'info>,
    }

    pub fn init_deposit_account_through_holder(
        ctx: Context<DepositAccountInterfaceInit>,
        bump: u8,
    ) -> ProgramResult {
        // sighash for init_deposit_account + bump
        let ix_data: Vec<u8> = vec![136, 79, 202, 206, 211, 146, 182, 158, bump];

        let jet_account_metas = vec![
            AccountMeta::new(ctx.accounts.deposit_account.key(), false),
            AccountMeta::new(ctx.accounts.market.key(), false),
            AccountMeta::new(ctx.accounts.market_authority.key(), false),
            AccountMeta::new(ctx.accounts.reserve.key(), false),
            AccountMeta::new(ctx.accounts.deposit_note_mint.key(), false),
            AccountMeta::new(ctx.program_id.clone(), true),
            AccountMeta::new(ctx.accounts.token_program.key(), false),
            AccountMeta::new(ctx.accounts.system_program.key(), false),
            AccountMeta::new(ctx.accounts.rent.key(), false),
        ];
        let jet_instruction = Instruction {
            program_id: ctx.accounts.jet.key(),
            accounts: jet_account_metas,
            data: ix_data,
        };
        let jet_account_infos = &[ctx.accounts.market_authority.clone()];
        let signer_seeds = {
            let initial_seed = ctx.accounts.payer.key.as_ref();
            let (_, authority_seed) =
                Pubkey::find_program_address(&[initial_seed], ctx.program_id);
            let seed = initial_seed;
            [seed, &[authority_seed]]
        };
        invoke_signed(
            &jet_instruction,
            jet_account_infos,
            &[&signer_seeds],
        )?;

        Ok(())
    }
}
