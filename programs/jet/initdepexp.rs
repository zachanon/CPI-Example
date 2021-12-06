pub mod init_deposit_account {
    use anchor_lang::prelude::*;
    use anchor_lang::Key;
    use crate::state::*;
    # [instruction (bump : u8)]
    pub struct InitializeDepositAccount<'info> {
        /// The relevant market this deposit is for
        # [account (has_one = market_authority)]
        pub market: Loader<'info, Market>,
        /// The market's authority account
        pub market_authority: AccountInfo<'info>,
        /// The reserve being deposited into
        # [account (has_one = market , has_one = deposit_note_mint)]
        pub reserve: Loader<'info, Reserve>,
        /// The mint for the deposit notes
        pub deposit_note_mint: AccountInfo<'info>,
        /// The user/authority that will own the deposits
        #[account(mut, signer)]
        pub depositor: AccountInfo<'info>,
        /// The account that will store the deposit notes
        # [account (init , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , depositor . key . as_ref ()] , bump = bump , token :: mint = deposit_note_mint , token :: authority = market_authority , payer = depositor)]
        pub deposit_account: AccountInfo<'info>,
        # [account (address = anchor_spl :: token :: ID)]
        pub token_program: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitializeDepositAccount<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let mut ix_data = ix_data;
            struct __Args {
                bump: u8,
            }
            impl borsh::ser::BorshSerialize for __Args
            where
                u8: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.bump, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for __Args
            where
                u8: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        bump: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let __Args { bump } = __Args::deserialize(&mut ix_data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let market: anchor_lang::Loader<Market> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market_authority: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let reserve: anchor_lang::Loader<Reserve> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let deposit_note_mint: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let depositor: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let deposit_account = &accounts[0];
            *accounts = &accounts[1..];
            let token_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let system_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let rent: Sysvar<Rent> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let __anchor_rent = Rent::get()?;
            let deposit_account: AccountInfo = {
                if !false
                    || deposit_account.to_account_info().owner
                        == &anchor_lang::solana_program::system_program::ID
                {
                    let payer = depositor.to_account_info();
                    let __current_lamports = deposit_account.to_account_info().lamports();
                    if __current_lamports == 0 {
                        let lamports =
                            __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                        anchor_lang::solana_program::program::invoke_signed(
                            &anchor_lang::solana_program::system_instruction::create_account(
                                payer.to_account_info().key,
                                deposit_account.to_account_info().key,
                                lamports,
                                anchor_spl::token::TokenAccount::LEN as u64,
                                token_program.to_account_info().key,
                            ),
                            &[
                                payer.to_account_info(),
                                deposit_account.to_account_info(),
                                system_program.to_account_info(),
                            ],
                            &[&[
                                b"deposits".as_ref(),
                                reserve.key().as_ref(),
                                depositor.key.as_ref(),
                                &[bump][..],
                            ][..]],
                        )?;
                    } else {
                        let required_lamports = __anchor_rent
                            .minimum_balance(anchor_spl::token::TokenAccount::LEN)
                            .max(1)
                            .saturating_sub(__current_lamports);
                        if required_lamports > 0 {
                            anchor_lang::solana_program::program::invoke(
                                &anchor_lang::solana_program::system_instruction::transfer(
                                    payer.to_account_info().key,
                                    deposit_account.to_account_info().key,
                                    required_lamports,
                                ),
                                &[
                                    payer.to_account_info(),
                                    deposit_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                            )?;
                        }
                        anchor_lang::solana_program::program::invoke_signed(
                            &anchor_lang::solana_program::system_instruction::allocate(
                                deposit_account.to_account_info().key,
                                anchor_spl::token::TokenAccount::LEN as u64,
                            ),
                            &[
                                deposit_account.to_account_info(),
                                system_program.to_account_info(),
                            ],
                            &[&[
                                b"deposits".as_ref(),
                                reserve.key().as_ref(),
                                depositor.key.as_ref(),
                                &[bump][..],
                            ][..]],
                        )?;
                        anchor_lang::solana_program::program::invoke_signed(
                            &anchor_lang::solana_program::system_instruction::assign(
                                deposit_account.to_account_info().key,
                                token_program.to_account_info().key,
                            ),
                            &[
                                deposit_account.to_account_info(),
                                system_program.to_account_info(),
                            ],
                            &[&[
                                b"deposits".as_ref(),
                                reserve.key().as_ref(),
                                depositor.key.as_ref(),
                                &[bump][..],
                            ][..]],
                        )?;
                    }
                    let cpi_program = token_program.to_account_info();
                    let accounts = anchor_spl::token::InitializeAccount {
                        account: deposit_account.to_account_info(),
                        mint: deposit_note_mint.to_account_info(),
                        authority: market_authority.to_account_info(),
                        rent: rent.to_account_info(),
                    };
                    let cpi_ctx = CpiContext::new(cpi_program, accounts);
                    anchor_spl::token::initialize_account(cpi_ctx)?;
                }
                let pa: AccountInfo = deposit_account.to_account_info();
                pa
            };
            let (__program_signer, __bump) =
                anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        depositor.key.as_ref(),
                    ],
                    program_id,
                );
            if deposit_account.to_account_info().key != &__program_signer {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
            }
            if __bump != bump {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
            }
            if !deposit_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !__anchor_rent.is_exempt(
                deposit_account.to_account_info().lamports(),
                deposit_account.to_account_info().try_data_len()?,
            ) {
                return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
            }
            if &market.load()?.market_authority != market_authority.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
            }
            if &reserve.load()?.market != market.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
            }
            if &reserve.load()?.deposit_note_mint != deposit_note_mint.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
            }
            if !depositor.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !depositor.is_signer {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
            }
            if token_program.to_account_info().key != &anchor_spl::token::ID {
                return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
            }
            Ok(InitializeDepositAccount {
                market,
                market_authority,
                reserve,
                deposit_note_mint,
                depositor,
                deposit_account,
                token_program,
                system_program,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeDepositAccount<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.market.to_account_infos());
            account_infos.extend(self.market_authority.to_account_infos());
            account_infos.extend(self.reserve.to_account_infos());
            account_infos.extend(self.deposit_note_mint.to_account_infos());
            account_infos.extend(self.depositor.to_account_infos());
            account_infos.extend(self.deposit_account.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeDepositAccount<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.extend(self.market_authority.to_account_metas(None));
            account_metas.extend(self.reserve.to_account_metas(None));
            account_metas.extend(self.deposit_note_mint.to_account_metas(None));
            account_metas.extend(self.depositor.to_account_metas(Some(true)));
            account_metas.extend(self.deposit_account.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeDepositAccount<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.depositor, program_id)?;
            anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
            Ok(())
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize_deposit_account {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct InitializeDepositAccount {
            pub market: anchor_lang::solana_program::pubkey::Pubkey,
            pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
            pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
            pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeDepositAccount
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.market, writer)?;
                borsh::BorshSerialize::serialize(&self.market_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeDepositAccount {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.market,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.market_authority,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.reserve,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.deposit_note_mint,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.depositor,
                    true,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.deposit_account,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.rent, false,
                    ),
                );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `cpi::accounts` module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize_deposit_account {
        use super::*;
        pub struct InitializeDepositAccount<'info> {
            pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub deposit_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeDepositAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.market_authority),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.depositor),
                    true,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.deposit_account),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.rent),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeDepositAccount<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.market_authority,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.deposit_note_mint,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.deposit_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.token_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.system_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                account_infos
            }
        }
    }
    /// Initialize an account that can be used to store deposit notes
    pub fn handler(_ctx: Context<InitializeDepositAccount>, _bump: u8) -> ProgramResult {
        ::solana_program::log::sol_log("initialized deposit account");
        Ok(())
    }
}
