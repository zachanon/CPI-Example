#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
extern crate static_assertions;
pub mod errors {
    use anchor_lang::error;
    /// Anchor generated Result to be used as the return type for the
    /// program.
    pub type Result<T> = std::result::Result<T, Error>;
    /// Anchor generated error allowing one to easily return a
    /// `ProgramError` or a custom, user defined error code by utilizing
    /// its `From` implementation.
    #[doc(hidden)]
    pub enum Error {
        #[error(transparent)]
        ProgramError(#[from] anchor_lang::solana_program::program_error::ProgramError),
        #[error(transparent)]
        ErrorCode(#[from] ErrorCode),
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for Error {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                Error::ProgramError { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::ErrorCode { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                Error::ProgramError(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::ErrorCode(_0) => std::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<anchor_lang::solana_program::program_error::ProgramError> for Error {
        #[allow(deprecated)]
        fn from(source: anchor_lang::solana_program::program_error::ProgramError) -> Self {
            Error::ProgramError { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<ErrorCode> for Error {
        #[allow(deprecated)]
        fn from(source: ErrorCode) -> Self {
            Error::ErrorCode { 0: source }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Error::ProgramError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ProgramError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::ErrorCode(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ErrorCode");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[repr(u32)]
    pub enum ErrorCode {
        ArithmeticError,
        InvalidOracle,
        NoFreeReserves,
        NoFreeObligation,
        UnregisteredPosition,
        InvalidOraclePrice,
        InsufficientCollateral,
        SimultaneousDepositAndBorrow,
        ObligationHealthy,
        ObligationUnhealthy,
        ExceptionalReserveState,
        InvalidAmountUnits,
        InvalidDexMarketMints,
        InvalidMarketAuthority,
        InvalidLiquidationQuoteTokenAccount,
        ObligationAccountMismatch,
        UnknownInstruction,
        Disallowed,
        LiquidationSwapSlipped,
        CollateralValueTooSmall,
        LiquidationLowCollateral,
        NotSupported,
        MarketHalted,
        InvalidParameter,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ErrorCode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ErrorCode::ArithmeticError,) => {
                    ::core::fmt::Formatter::write_str(f, "ArithmeticError")
                }
                (&ErrorCode::InvalidOracle,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidOracle")
                }
                (&ErrorCode::NoFreeReserves,) => {
                    ::core::fmt::Formatter::write_str(f, "NoFreeReserves")
                }
                (&ErrorCode::NoFreeObligation,) => {
                    ::core::fmt::Formatter::write_str(f, "NoFreeObligation")
                }
                (&ErrorCode::UnregisteredPosition,) => {
                    ::core::fmt::Formatter::write_str(f, "UnregisteredPosition")
                }
                (&ErrorCode::InvalidOraclePrice,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidOraclePrice")
                }
                (&ErrorCode::InsufficientCollateral,) => {
                    ::core::fmt::Formatter::write_str(f, "InsufficientCollateral")
                }
                (&ErrorCode::SimultaneousDepositAndBorrow,) => {
                    ::core::fmt::Formatter::write_str(f, "SimultaneousDepositAndBorrow")
                }
                (&ErrorCode::ObligationHealthy,) => {
                    ::core::fmt::Formatter::write_str(f, "ObligationHealthy")
                }
                (&ErrorCode::ObligationUnhealthy,) => {
                    ::core::fmt::Formatter::write_str(f, "ObligationUnhealthy")
                }
                (&ErrorCode::ExceptionalReserveState,) => {
                    ::core::fmt::Formatter::write_str(f, "ExceptionalReserveState")
                }
                (&ErrorCode::InvalidAmountUnits,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidAmountUnits")
                }
                (&ErrorCode::InvalidDexMarketMints,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidDexMarketMints")
                }
                (&ErrorCode::InvalidMarketAuthority,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidMarketAuthority")
                }
                (&ErrorCode::InvalidLiquidationQuoteTokenAccount,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidLiquidationQuoteTokenAccount")
                }
                (&ErrorCode::ObligationAccountMismatch,) => {
                    ::core::fmt::Formatter::write_str(f, "ObligationAccountMismatch")
                }
                (&ErrorCode::UnknownInstruction,) => {
                    ::core::fmt::Formatter::write_str(f, "UnknownInstruction")
                }
                (&ErrorCode::Disallowed,) => ::core::fmt::Formatter::write_str(f, "Disallowed"),
                (&ErrorCode::LiquidationSwapSlipped,) => {
                    ::core::fmt::Formatter::write_str(f, "LiquidationSwapSlipped")
                }
                (&ErrorCode::CollateralValueTooSmall,) => {
                    ::core::fmt::Formatter::write_str(f, "CollateralValueTooSmall")
                }
                (&ErrorCode::LiquidationLowCollateral,) => {
                    ::core::fmt::Formatter::write_str(f, "LiquidationLowCollateral")
                }
                (&ErrorCode::NotSupported,) => ::core::fmt::Formatter::write_str(f, "NotSupported"),
                (&ErrorCode::MarketHalted,) => ::core::fmt::Formatter::write_str(f, "MarketHalted"),
                (&ErrorCode::InvalidParameter,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidParameter")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ErrorCode {
        #[inline]
        fn clone(&self) -> ErrorCode {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ErrorCode {}
    impl std::fmt::Display for ErrorCode {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self { ErrorCode :: ArithmeticError => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["failed to perform some math operation safely"] , & match () { _args => [] , })) , ErrorCode :: InvalidOracle => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["oracle account provided is not valid"] , & match () { _args => [] , })) , ErrorCode :: NoFreeReserves => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["no free space left to add a new reserve in the market"] , & match () { _args => [] , })) , ErrorCode :: NoFreeObligation => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["no free space left to add the new loan or collateral in an obligation"] , & match () { _args => [] , })) , ErrorCode :: UnregisteredPosition => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the obligation account doesn\'t have any record of the loan or collateral account"] , & match () { _args => [] , })) , ErrorCode :: InvalidOraclePrice => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the oracle price account has an invalid price value"] , & match () { _args => [] , })) , ErrorCode :: InsufficientCollateral => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["there is not enough collateral deposited to borrow against"] , & match () { _args => [] , })) , ErrorCode :: SimultaneousDepositAndBorrow => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["cannot both deposit collateral to and borrow from the same reserve"] , & match () { _args => [] , })) , ErrorCode :: ObligationHealthy => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["cannot liquidate a healthy position"] , & match () { _args => [] , })) , ErrorCode :: ObligationUnhealthy => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["cannot perform an action that would leave the obligation unhealthy"] , & match () { _args => [] , })) , ErrorCode :: ExceptionalReserveState => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["reserve requires special action; call refresh_reserve until up to date"] , & match () { _args => [] , })) , ErrorCode :: InvalidAmountUnits => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the units provided in the amount are not valid for the instruction"] , & match () { _args => [] , })) , ErrorCode :: InvalidDexMarketMints => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the tokens in the DEX market don\'t match the reserve and lending market quote token"] , & match () { _args => [] , })) , ErrorCode :: InvalidMarketAuthority => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the market authority provided doesn\'t match the market account"] , & match () { _args => [] , })) , ErrorCode :: InvalidLiquidationQuoteTokenAccount => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the quote token account provided cannot be used for liquidations"] , & match () { _args => [] , })) , ErrorCode :: ObligationAccountMismatch => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the obligation account doesn\'t have the collateral/loan registered"] , & match () { _args => [] , })) , ErrorCode :: UnknownInstruction => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["unknown instruction"] , & match () { _args => [] , })) , ErrorCode :: Disallowed => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["current conditions prevent an action from being performed"] , & match () { _args => [] , })) , ErrorCode :: LiquidationSwapSlipped => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the actual slipped amount on the DEX trade exceeded the threshold configured"] , & match () { _args => [] , })) , ErrorCode :: CollateralValueTooSmall => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the collateral value is too small for a DEX trade"] , & match () { _args => [] , })) , ErrorCode :: LiquidationLowCollateral => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the collateral returned by the liquidation is smaller than requested"] , & match () { _args => [] , })) , ErrorCode :: NotSupported => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["this action is currently not supported by this version of the program"] , & match () { _args => [] , })) , ErrorCode :: MarketHalted => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["the market has currently halted this kind of operation"] , & match () { _args => [] , })) , ErrorCode :: InvalidParameter => fmt . write_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["a given parameter is not valid"] , & match () { _args => [] , })) , }
        }
    }
    impl std::error::Error for ErrorCode {}
    impl std::convert::From<Error> for anchor_lang::solana_program::program_error::ProgramError {
        fn from(e: Error) -> anchor_lang::solana_program::program_error::ProgramError {
            match e {
                Error::ProgramError(e) => e,
                Error::ErrorCode(c) => {
                    anchor_lang::solana_program::program_error::ProgramError::Custom(
                        c as u32 + anchor_lang::__private::ERROR_CODE_OFFSET,
                    )
                }
            }
        }
    }
    impl std::convert::From<ErrorCode> for anchor_lang::solana_program::program_error::ProgramError {
        fn from(e: ErrorCode) -> anchor_lang::solana_program::program_error::ProgramError {
            let err: Error = e.into();
            err.into()
        }
    }
    impl From<jet_math::Error> for ErrorCode {
        fn from(_: jet_math::Error) -> ErrorCode {
            ErrorCode::ArithmeticError
        }
    }
}
pub mod instructions {
    pub mod init_collateral_account {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use crate::state::*;
        # [instruction (bump : u8)]
        pub struct InitializeCollateralAccount<'info> {
            /// The relevant market this collateral is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation the collateral account is used for
            # [account (mut , has_one = market , has_one = owner)]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve that the collateral comes from
            # [account (has_one = market , has_one = deposit_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The mint for the deposit notes being used as collateral
            pub deposit_note_mint: AccountInfo<'info>,
            /// The user/authority that owns the collateral
            #[account(mut, signer)]
            pub owner: AccountInfo<'info>,
            /// The account that will store the deposit notes
            # [account (init , seeds = [b"collateral" . as_ref () , reserve . key () . as_ref () , obligation . key () . as_ref () , owner . key . as_ref ()] , bump = bump , token :: mint = deposit_note_mint , token :: authority = market_authority , payer = owner)]
            pub collateral_account: AccountInfo<'info>,
            # [account (address = anchor_spl :: token :: ID)]
            pub token_program: AccountInfo<'info>,
            pub system_program: AccountInfo<'info>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeCollateralAccount<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_account = &accounts[0];
                *accounts = &accounts[1..];
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let system_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let __anchor_rent = Rent::get()?;
                let collateral_account: AccountInfo = {
                    if !false
                        || collateral_account.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = owner.to_account_info();
                        let __current_lamports = collateral_account.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    collateral_account.to_account_info().key,
                                    lamports,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    payer.to_account_info(),
                                    collateral_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"collateral".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
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
                                        collateral_account.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        collateral_account.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    collateral_account.to_account_info().key,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                ),
                                &[
                                    collateral_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"collateral".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    collateral_account.to_account_info().key,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    collateral_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"collateral".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeAccount {
                            account: collateral_account.to_account_info(),
                            mint: deposit_note_mint.to_account_info(),
                            authority: market_authority.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_account(cpi_ctx)?;
                    }
                    let pa: AccountInfo = collateral_account.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[
                            b"collateral".as_ref(),
                            reserve.key().as_ref(),
                            obligation.key().as_ref(),
                            owner.key.as_ref(),
                        ],
                        program_id,
                    );
                if collateral_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !collateral_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    collateral_account.to_account_info().lamports(),
                    collateral_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &obligation.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.deposit_note_mint != deposit_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                if token_program.to_account_info().key != &anchor_spl::token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(InitializeCollateralAccount {
                    market,
                    market_authority,
                    obligation,
                    reserve,
                    deposit_note_mint,
                    owner,
                    collateral_account,
                    token_program,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeCollateralAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.collateral_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeCollateralAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas.extend(self.collateral_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeCollateralAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.owner, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_account, program_id)?;
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
        pub(crate) mod __client_accounts_initialize_collateral_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct InitializeCollateralAccount {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeCollateralAccount
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
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.market_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeCollateralAccount {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
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
                        self.owner, true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_account,
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
        pub(crate) mod __cpi_client_accounts_initialize_collateral_account {
            use super::*;
            pub struct InitializeCollateralAccount<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeCollateralAccount<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
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
                        anchor_lang::Key::key(&self.owner),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_account),
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
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeCollateralAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_account,
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
        /// Initialize an account that can be used to store deposit notes as collateral for an obligation
        pub fn handler(ctx: Context<InitializeCollateralAccount>, _bump: u8) -> ProgramResult {
            let mut obligation = ctx.accounts.obligation.load_mut()?;
            let reserve = ctx.accounts.reserve.load()?;
            let account = ctx.accounts.collateral_account.key();
            obligation.register_collateral(&account, reserve.index)?;
            ::solana_program::log::sol_log("initialized collateral account");
            Ok(())
        }
    }
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
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
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
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
    pub mod init_loan_account {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use crate::state::*;
        # [instruction (bump : u8)]
        pub struct InitializeLoanAccount<'info> {
            /// The relevant market this loan is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation the loan account is used for
            # [account (mut , has_one = market , has_one = owner)]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve that the loan comes from
            # [account (has_one = market , has_one = loan_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The mint for the loan notes being used as loan
            pub loan_note_mint: AccountInfo<'info>,
            /// The user/authority that owns the loan
            #[account(mut, signer)]
            pub owner: AccountInfo<'info>,
            /// The account that will store the loan notes
            # [account (init , seeds = [b"loan" . as_ref () , reserve . key () . as_ref () , obligation . key () . as_ref () , owner . key . as_ref ()] , bump = bump , token :: mint = loan_note_mint , token :: authority = market_authority , payer = owner)]
            pub loan_account: AccountInfo<'info>,
            # [account (address = anchor_spl :: token :: ID)]
            pub token_program: AccountInfo<'info>,
            pub system_program: AccountInfo<'info>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeLoanAccount<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_account = &accounts[0];
                *accounts = &accounts[1..];
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let system_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let __anchor_rent = Rent::get()?;
                let loan_account: AccountInfo = {
                    if !false
                        || loan_account.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = owner.to_account_info();
                        let __current_lamports = loan_account.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    loan_account.to_account_info().key,
                                    lamports,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    payer.to_account_info(),
                                    loan_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loan".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
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
                                        loan_account.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        loan_account.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    loan_account.to_account_info().key,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                ),
                                &[
                                    loan_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loan".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    loan_account.to_account_info().key,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    loan_account.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loan".as_ref(),
                                    reserve.key().as_ref(),
                                    obligation.key().as_ref(),
                                    owner.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeAccount {
                            account: loan_account.to_account_info(),
                            mint: loan_note_mint.to_account_info(),
                            authority: market_authority.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_account(cpi_ctx)?;
                    }
                    let pa: AccountInfo = loan_account.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[
                            b"loan".as_ref(),
                            reserve.key().as_ref(),
                            obligation.key().as_ref(),
                            owner.key.as_ref(),
                        ],
                        program_id,
                    );
                if loan_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !loan_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    loan_account.to_account_info().lamports(),
                    loan_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &obligation.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.loan_note_mint != loan_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                if token_program.to_account_info().key != &anchor_spl::token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(InitializeLoanAccount {
                    market,
                    market_authority,
                    obligation,
                    reserve,
                    loan_note_mint,
                    owner,
                    loan_account,
                    token_program,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeLoanAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.loan_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeLoanAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas.extend(self.loan_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeLoanAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.owner, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_account, program_id)?;
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
        pub(crate) mod __client_accounts_initialize_loan_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct InitializeLoanAccount {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeLoanAccount
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
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.market_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeLoanAccount {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.reserve,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.loan_note_mint,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.owner, true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_account,
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
        pub(crate) mod __cpi_client_accounts_initialize_loan_account {
            use super::*;
            pub struct InitializeLoanAccount<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeLoanAccount<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.reserve),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.loan_note_mint),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.owner),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_account),
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
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeLoanAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_account,
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
        /// Initialize an account that can be used to store loan notes to represent debt in an obligation
        pub fn handler(ctx: Context<InitializeLoanAccount>, _bump: u8) -> ProgramResult {
            let mut obligation = ctx.accounts.obligation.load_mut()?;
            let reserve = &ctx.accounts.reserve.load()?;
            let account = ctx.accounts.loan_account.key();
            obligation.register_loan(&account, reserve.index)?;
            ::solana_program::log::sol_log("initialized loan account");
            Ok(())
        }
    }
    pub mod init_market {
        use std::io::Write;
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use crate::state::*;
        pub struct InitializeMarket<'info> {
            #[account(zero)]
            pub market: Loader<'info, Market>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeMarket<'info>
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
                let market = &accounts[0];
                *accounts = &accounts[1..];
                let __anchor_rent = Rent::get()?;
                let market: anchor_lang::Loader<Market> = {
                    let mut __data: &[u8] = &market.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
                    }
                    anchor_lang::Loader::try_from_unchecked(program_id, &market)?
                };
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    market.to_account_info().lamports(),
                    market.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                Ok(InitializeMarket { market })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarket<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeMarket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeMarket<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
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
        pub(crate) mod __client_accounts_initialize_market {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct InitializeMarket {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeMarket
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeMarket {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
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
        pub(crate) mod __cpi_client_accounts_initialize_market {
            use super::*;
            pub struct InitializeMarket<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeMarket<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarket<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos
                }
            }
        }
        /// Initialize a new empty market with a given owner.
        pub fn handler(
            ctx: Context<InitializeMarket>,
            owner: Pubkey,
            quote_currency: String,
            quote_token_mint: Pubkey,
        ) -> ProgramResult {
            let market_address = ctx.accounts.market.key();
            let initial_seeds = &[ctx.accounts.market.to_account_info().key.as_ref()];
            let mut market = ctx.accounts.market.load_init()?;
            let (authority, authority_seed) =
                Pubkey::find_program_address(initial_seeds, ctx.program_id);
            market.version = 0;
            market.owner = owner;
            market.market_authority = authority;
            market.authority_seed = market_address;
            market.authority_bump_seed = [authority_seed];
            market.quote_token_mint = quote_token_mint;
            (&mut market.quote_currency[..]).write_all(quote_currency.as_bytes())?;
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["market initialized with currency "],
                    &match (&quote_currency,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            });
            Ok(())
        }
    }
    pub mod init_obligation {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use crate::state::*;
        # [instruction (bump : u8)]
        pub struct InitializeObligation<'info> {
            /// The relevant market
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The user/authority that is responsible for owning this obligation.
            #[account(mut, signer)]
            pub borrower: AccountInfo<'info>,
            /// The new account to track information about the borrower's loan,
            /// such as the collateral put up.
            # [account (init , seeds = [b"obligation" . as_ref () , market . key () . as_ref () , borrower . key . as_ref ()] , bump = bump , space = 8 + std :: mem :: size_of :: < Obligation > () , payer = borrower)]
            pub obligation: Loader<'info, Obligation>,
            pub token_program: AccountInfo<'info>,
            pub system_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeObligation<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let borrower: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let obligation = &accounts[0];
                *accounts = &accounts[1..];
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let system_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let __anchor_rent = Rent::get()?;
                let obligation = {
                    if !false
                        || obligation.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let space = 8 + std::mem::size_of::<Obligation>();
                        let payer = borrower.to_account_info();
                        let __current_lamports = obligation.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    obligation.to_account_info().key,
                                    lamports,
                                    space as u64,
                                    program_id,
                                ),
                                &[
                                    payer.to_account_info(),
                                    obligation.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"obligation".as_ref(),
                                    market.key().as_ref(),
                                    borrower.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                anchor_lang::solana_program::program::invoke(
                                    &anchor_lang::solana_program::system_instruction::transfer(
                                        payer.to_account_info().key,
                                        obligation.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        obligation.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    obligation.to_account_info().key,
                                    space as u64,
                                ),
                                &[
                                    obligation.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"obligation".as_ref(),
                                    market.key().as_ref(),
                                    borrower.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    obligation.to_account_info().key,
                                    program_id,
                                ),
                                &[
                                    obligation.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"obligation".as_ref(),
                                    market.key().as_ref(),
                                    borrower.key.as_ref(),
                                    &[bump][..],
                                ][..]],
                            )?;
                        }
                    }
                    let pa: anchor_lang::Loader<Obligation> =
                        anchor_lang::Loader::try_from_unchecked(program_id, &obligation)?;
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[
                            b"obligation".as_ref(),
                            market.key().as_ref(),
                            borrower.key.as_ref(),
                        ],
                        program_id,
                    );
                if obligation.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    obligation.to_account_info().lamports(),
                    obligation.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !borrower.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !borrower.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                Ok(InitializeObligation {
                    market,
                    market_authority,
                    borrower,
                    obligation,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeObligation<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.borrower.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeObligation<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.borrower.to_account_metas(Some(true)));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeObligation<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.borrower, program_id)?;
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
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
        pub(crate) mod __client_accounts_initialize_obligation {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct InitializeObligation {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub borrower: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeObligation
            where
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
                    borsh::BorshSerialize::serialize(&self.borrower, writer)?;
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeObligation {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.borrower,
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
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
        pub(crate) mod __cpi_client_accounts_initialize_obligation {
            use super::*;
            pub struct InitializeObligation<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub borrower: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeObligation<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.borrower),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
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
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeObligation<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.borrower));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos
                }
            }
        }
        /// Initialize an account that tracks a portfolio of collateral deposits and loans.
        pub fn handler(ctx: Context<InitializeObligation>, _bump: u8) -> ProgramResult {
            let mut obligation = ctx.accounts.obligation.load_init()?;
            obligation.market = ctx.accounts.market.key();
            obligation.owner = *ctx.accounts.borrower.key;
            ::solana_program::log::sol_log("initialized obligation account");
            Ok(())
        }
    }
    pub mod init_reserve {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::dex;
        use anchor_spl::dex::serum_dex::state::OpenOrders;
        use anchor_spl::dex::InitOpenOrders;
        use anchor_spl::token::{self, InitializeAccount, InitializeMint, Mint, TokenAccount};
        use pyth_client::Product;
        use crate::state::*;
        use crate::utils;
        pub struct InitReserveBumpSeeds {
            pub vault: u8,
            pub fee_note_vault: u8,
            pub dex_open_orders: u8,
            pub dex_swap_tokens: u8,
            pub deposit_note_mint: u8,
            pub loan_note_mint: u8,
        }
        impl borsh::ser::BorshSerialize for InitReserveBumpSeeds
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.vault, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_note_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.dex_open_orders, writer)?;
                borsh::BorshSerialize::serialize(&self.dex_swap_tokens, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for InitReserveBumpSeeds
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    vault: borsh::BorshDeserialize::deserialize(buf)?,
                    fee_note_vault: borsh::BorshDeserialize::deserialize(buf)?,
                    dex_open_orders: borsh::BorshDeserialize::deserialize(buf)?,
                    dex_swap_tokens: borsh::BorshDeserialize::deserialize(buf)?,
                    deposit_note_mint: borsh::BorshDeserialize::deserialize(buf)?,
                    loan_note_mint: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        # [instruction (bump : InitReserveBumpSeeds)]
        pub struct InitializeReserve<'info> {
            /// The market the new reserve is being added to.
            # [account (mut , has_one = owner , has_one = market_authority , has_one = quote_token_mint)]
            pub market: Loader<'info, Market>,
            /// The market's authority account, which owns the vault
            pub market_authority: AccountInfo<'info>,
            /// The new account to store data about the reserve
            #[account(zero)]
            pub reserve: Loader<'info, Reserve>,
            /// The account to hold custody of the tokens being
            /// controlled by this reserve.
            # [account (init , seeds = [b"vault" . as_ref () , reserve . key () . as_ref ()] , bump = bump . vault , token :: mint = token_mint , token :: authority = market_authority , payer = owner)]
            pub vault: AccountInfo<'info>,
            /// The account to hold the notes created from fees collected by the reserve
            # [account (init , seeds = [b"fee-vault" . as_ref () , reserve . key () . as_ref ()] , bump = bump . fee_note_vault , payer = owner , owner = token :: ID , space = TokenAccount :: LEN)]
            pub fee_note_vault: AccountInfo<'info>,
            /// The account for storing quote tokens during swaps
            # [account (init , seeds = [b"dex-swap-tokens" . as_ref () , reserve . key () . as_ref ()] , bump = bump . dex_swap_tokens , token :: mint = quote_token_mint , token :: authority = market_authority , payer = owner)]
            pub dex_swap_tokens: AccountInfo<'info>,
            /// The account to use for placing orders on the DEX
            # [account (init , seeds = [b"dex-open-orders" . as_ref () , reserve . key () . as_ref ()] , bump = bump . dex_open_orders , payer = owner , owner = dex :: ID , space = std :: mem :: size_of :: < OpenOrders > () + 12 , rent_exempt = skip)]
            pub dex_open_orders: AccountInfo<'info>,
            /// The DEX market that can be used to trade the reserve asset
            pub dex_market: AccountInfo<'info>,
            /// The mint for the token being stored in this reserve.
            pub token_mint: Account<'info, Mint>,
            /// The program for interacting with the token.
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
            /// The program for interacting with the DEX
            # [account (address = dex :: ID)]
            pub dex_program: AccountInfo<'info>,
            /// The account containing the price information for the token.
            pub oracle_price: AccountInfo<'info>,
            /// The account containing the metadata about the token being referenced
            pub oracle_product: AccountInfo<'info>,
            /// The mint for notes which will represent user deposits
            # [account (init , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , token_mint . key () . as_ref ()] , bump = bump . deposit_note_mint , payer = owner , owner = token :: ID , space = Mint :: LEN)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The mint for notes which will represent user loans
            # [account (init , seeds = [b"loans" . as_ref () , reserve . key () . as_ref () , token_mint . key () . as_ref ()] , bump = bump . loan_note_mint , payer = owner , owner = token :: ID , space = Mint :: LEN)]
            pub loan_note_mint: AccountInfo<'info>,
            /// The mint for the market quote tokens
            pub quote_token_mint: AccountInfo<'info>,
            /// The market owner, which must sign to make this change to the market.
            #[account(signer)]
            pub owner: AccountInfo<'info>,
            pub system_program: AccountInfo<'info>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeReserve<'info>
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
                    bump: InitReserveBumpSeeds,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    InitReserveBumpSeeds: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.bump, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    InitReserveBumpSeeds: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let reserve = &accounts[0];
                *accounts = &accounts[1..];
                let vault = &accounts[0];
                *accounts = &accounts[1..];
                let fee_note_vault = &accounts[0];
                *accounts = &accounts[1..];
                let dex_swap_tokens = &accounts[0];
                *accounts = &accounts[1..];
                let dex_open_orders = &accounts[0];
                *accounts = &accounts[1..];
                let dex_market: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_mint: anchor_lang::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let dex_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let oracle_price: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let oracle_product: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint = &accounts[0];
                *accounts = &accounts[1..];
                let loan_note_mint = &accounts[0];
                *accounts = &accounts[1..];
                let quote_token_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let system_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let __anchor_rent = Rent::get()?;
                let vault: AccountInfo = {
                    if !false
                        || vault.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = owner.to_account_info();
                        let __current_lamports = vault.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    vault.to_account_info().key,
                                    lamports,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    payer.to_account_info(),
                                    vault.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[
                                    &[b"vault".as_ref(), reserve.key().as_ref(), &[bump.vault][..]]
                                        [..],
                                ],
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
                                        vault.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        vault.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    vault.to_account_info().key,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                ),
                                &[vault.to_account_info(), system_program.to_account_info()],
                                &[
                                    &[b"vault".as_ref(), reserve.key().as_ref(), &[bump.vault][..]]
                                        [..],
                                ],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    vault.to_account_info().key,
                                    token_program.to_account_info().key,
                                ),
                                &[vault.to_account_info(), system_program.to_account_info()],
                                &[
                                    &[b"vault".as_ref(), reserve.key().as_ref(), &[bump.vault][..]]
                                        [..],
                                ],
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeAccount {
                            account: vault.to_account_info(),
                            mint: token_mint.to_account_info(),
                            authority: market_authority.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_account(cpi_ctx)?;
                    }
                    let pa: AccountInfo = vault.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[b"vault".as_ref(), reserve.key().as_ref()],
                        program_id,
                    );
                if vault.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.vault {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    vault.to_account_info().lamports(),
                    vault.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                let __anchor_rent = Rent::get()?;
                let fee_note_vault = {
                    if !false
                        || fee_note_vault.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let space = TokenAccount::LEN;
                        let payer = owner.to_account_info();
                        let __current_lamports = fee_note_vault.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    fee_note_vault.to_account_info().key,
                                    lamports,
                                    space as u64,
                                    &token::ID,
                                ),
                                &[
                                    payer.to_account_info(),
                                    fee_note_vault.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"fee-vault".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.fee_note_vault][..],
                                ][..]],
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                anchor_lang::solana_program::program::invoke(
                                    &anchor_lang::solana_program::system_instruction::transfer(
                                        payer.to_account_info().key,
                                        fee_note_vault.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        fee_note_vault.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    fee_note_vault.to_account_info().key,
                                    space as u64,
                                ),
                                &[
                                    fee_note_vault.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"fee-vault".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.fee_note_vault][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    fee_note_vault.to_account_info().key,
                                    &token::ID,
                                ),
                                &[
                                    fee_note_vault.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"fee-vault".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.fee_note_vault][..],
                                ][..]],
                            )?;
                        }
                    }
                    let pa: AccountInfo = fee_note_vault.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[b"fee-vault".as_ref(), reserve.key().as_ref()],
                        program_id,
                    );
                if fee_note_vault.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.fee_note_vault {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !fee_note_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if fee_note_vault.to_account_info().owner != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                if !__anchor_rent.is_exempt(
                    fee_note_vault.to_account_info().lamports(),
                    fee_note_vault.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                let __anchor_rent = Rent::get()?;
                let dex_swap_tokens: AccountInfo = {
                    if !false
                        || dex_swap_tokens.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = owner.to_account_info();
                        let __current_lamports = dex_swap_tokens.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    dex_swap_tokens.to_account_info().key,
                                    lamports,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    payer.to_account_info(),
                                    dex_swap_tokens.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-swap-tokens".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_swap_tokens][..],
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
                                        dex_swap_tokens.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        dex_swap_tokens.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    dex_swap_tokens.to_account_info().key,
                                    anchor_spl::token::TokenAccount::LEN as u64,
                                ),
                                &[
                                    dex_swap_tokens.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-swap-tokens".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_swap_tokens][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    dex_swap_tokens.to_account_info().key,
                                    token_program.to_account_info().key,
                                ),
                                &[
                                    dex_swap_tokens.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-swap-tokens".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_swap_tokens][..],
                                ][..]],
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeAccount {
                            account: dex_swap_tokens.to_account_info(),
                            mint: quote_token_mint.to_account_info(),
                            authority: market_authority.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_account(cpi_ctx)?;
                    }
                    let pa: AccountInfo = dex_swap_tokens.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[b"dex-swap-tokens".as_ref(), reserve.key().as_ref()],
                        program_id,
                    );
                if dex_swap_tokens.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.dex_swap_tokens {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !dex_swap_tokens.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    dex_swap_tokens.to_account_info().lamports(),
                    dex_swap_tokens.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                let dex_open_orders = {
                    if !false
                        || dex_open_orders.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let space = std::mem::size_of::<OpenOrders>() + 12;
                        let payer = owner.to_account_info();
                        let __current_lamports = dex_open_orders.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    dex_open_orders.to_account_info().key,
                                    lamports,
                                    space as u64,
                                    &dex::ID,
                                ),
                                &[
                                    payer.to_account_info(),
                                    dex_open_orders.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-open-orders".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_open_orders][..],
                                ][..]],
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                anchor_lang::solana_program::program::invoke(
                                    &anchor_lang::solana_program::system_instruction::transfer(
                                        payer.to_account_info().key,
                                        dex_open_orders.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        dex_open_orders.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    dex_open_orders.to_account_info().key,
                                    space as u64,
                                ),
                                &[
                                    dex_open_orders.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-open-orders".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_open_orders][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    dex_open_orders.to_account_info().key,
                                    &dex::ID,
                                ),
                                &[
                                    dex_open_orders.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"dex-open-orders".as_ref(),
                                    reserve.key().as_ref(),
                                    &[bump.dex_open_orders][..],
                                ][..]],
                            )?;
                        }
                    }
                    let pa: AccountInfo = dex_open_orders.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[b"dex-open-orders".as_ref(), reserve.key().as_ref()],
                        program_id,
                    );
                if dex_open_orders.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.dex_open_orders {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !dex_open_orders.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if dex_open_orders.to_account_info().owner != &dex::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                let __anchor_rent = Rent::get()?;
                let deposit_note_mint = {
                    if !false
                        || deposit_note_mint.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let space = Mint::LEN;
                        let payer = owner.to_account_info();
                        let __current_lamports = deposit_note_mint.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    deposit_note_mint.to_account_info().key,
                                    lamports,
                                    space as u64,
                                    &token::ID,
                                ),
                                &[
                                    payer.to_account_info(),
                                    deposit_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"deposits".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.deposit_note_mint][..],
                                ][..]],
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                anchor_lang::solana_program::program::invoke(
                                    &anchor_lang::solana_program::system_instruction::transfer(
                                        payer.to_account_info().key,
                                        deposit_note_mint.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        deposit_note_mint.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    deposit_note_mint.to_account_info().key,
                                    space as u64,
                                ),
                                &[
                                    deposit_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"deposits".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.deposit_note_mint][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    deposit_note_mint.to_account_info().key,
                                    &token::ID,
                                ),
                                &[
                                    deposit_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"deposits".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.deposit_note_mint][..],
                                ][..]],
                            )?;
                        }
                    }
                    let pa: AccountInfo = deposit_note_mint.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[
                            b"deposits".as_ref(),
                            reserve.key().as_ref(),
                            token_mint.key().as_ref(),
                        ],
                        program_id,
                    );
                if deposit_note_mint.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.deposit_note_mint {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if deposit_note_mint.to_account_info().owner != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                if !__anchor_rent.is_exempt(
                    deposit_note_mint.to_account_info().lamports(),
                    deposit_note_mint.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                let __anchor_rent = Rent::get()?;
                let loan_note_mint = {
                    if !false
                        || loan_note_mint.to_account_info().owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let space = Mint::LEN;
                        let payer = owner.to_account_info();
                        let __current_lamports = loan_note_mint.to_account_info().lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::create_account(
                                    payer.to_account_info().key,
                                    loan_note_mint.to_account_info().key,
                                    lamports,
                                    space as u64,
                                    &token::ID,
                                ),
                                &[
                                    payer.to_account_info(),
                                    loan_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loans".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.loan_note_mint][..],
                                ][..]],
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                anchor_lang::solana_program::program::invoke(
                                    &anchor_lang::solana_program::system_instruction::transfer(
                                        payer.to_account_info().key,
                                        loan_note_mint.to_account_info().key,
                                        required_lamports,
                                    ),
                                    &[
                                        payer.to_account_info(),
                                        loan_note_mint.to_account_info(),
                                        system_program.to_account_info(),
                                    ],
                                )?;
                            }
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::allocate(
                                    loan_note_mint.to_account_info().key,
                                    space as u64,
                                ),
                                &[
                                    loan_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loans".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.loan_note_mint][..],
                                ][..]],
                            )?;
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::assign(
                                    loan_note_mint.to_account_info().key,
                                    &token::ID,
                                ),
                                &[
                                    loan_note_mint.to_account_info(),
                                    system_program.to_account_info(),
                                ],
                                &[&[
                                    b"loans".as_ref(),
                                    reserve.key().as_ref(),
                                    token_mint.key().as_ref(),
                                    &[bump.loan_note_mint][..],
                                ][..]],
                            )?;
                        }
                    }
                    let pa: AccountInfo = loan_note_mint.to_account_info();
                    pa
                };
                let (__program_signer, __bump) =
                    anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                        &[
                            b"loans".as_ref(),
                            reserve.key().as_ref(),
                            token_mint.key().as_ref(),
                        ],
                        program_id,
                    );
                if loan_note_mint.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if __bump != bump.loan_note_mint {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !loan_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if loan_note_mint.to_account_info().owner != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                if !__anchor_rent.is_exempt(
                    loan_note_mint.to_account_info().lamports(),
                    loan_note_mint.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &market.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &market.load()?.quote_token_mint != quote_token_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                let __anchor_rent = Rent::get()?;
                let reserve: anchor_lang::Loader<Reserve> = {
                    let mut __data: &[u8] = &reserve.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
                    }
                    anchor_lang::Loader::try_from_unchecked(program_id, &reserve)?
                };
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !__anchor_rent.is_exempt(
                    reserve.to_account_info().lamports(),
                    reserve.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                if dex_program.to_account_info().key != &dex::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                Ok(InitializeReserve {
                    market,
                    market_authority,
                    reserve,
                    vault,
                    fee_note_vault,
                    dex_swap_tokens,
                    dex_open_orders,
                    dex_market,
                    token_mint,
                    token_program,
                    dex_program,
                    oracle_price,
                    oracle_product,
                    deposit_note_mint,
                    loan_note_mint,
                    quote_token_mint,
                    owner,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeReserve<'info>
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
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.fee_note_vault.to_account_infos());
                account_infos.extend(self.dex_swap_tokens.to_account_infos());
                account_infos.extend(self.dex_open_orders.to_account_infos());
                account_infos.extend(self.dex_market.to_account_infos());
                account_infos.extend(self.token_mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.dex_program.to_account_infos());
                account_infos.extend(self.oracle_price.to_account_infos());
                account_infos.extend(self.oracle_product.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.quote_token_mint.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeReserve<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.fee_note_vault.to_account_metas(None));
                account_metas.extend(self.dex_swap_tokens.to_account_metas(None));
                account_metas.extend(self.dex_open_orders.to_account_metas(None));
                account_metas.extend(self.dex_market.to_account_metas(None));
                account_metas.extend(self.token_mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.dex_program.to_account_metas(None));
                account_metas.extend(self.oracle_price.to_account_metas(None));
                account_metas.extend(self.oracle_product.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.quote_token_mint.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeReserve<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.fee_note_vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.dex_swap_tokens, program_id)?;
                anchor_lang::AccountsExit::exit(&self.dex_open_orders, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_note_mint, program_id)?;
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
        pub(crate) mod __client_accounts_initialize_reserve {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct InitializeReserve {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub fee_note_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_swap_tokens: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_open_orders: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_market: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub oracle_price: anchor_lang::solana_program::pubkey::Pubkey,
                pub oracle_product: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub quote_token_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeReserve
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
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.fee_note_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_swap_tokens, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_open_orders, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_market, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.oracle_price, writer)?;
                    borsh::BorshSerialize::serialize(&self.oracle_product, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.quote_token_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeReserve {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.market_authority,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.fee_note_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.dex_swap_tokens,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.dex_open_orders,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.dex_market,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_mint,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.dex_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.oracle_price,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.oracle_product,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_note_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.quote_token_mint,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
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
        pub(crate) mod __cpi_client_accounts_initialize_reserve {
            use super::*;
            pub struct InitializeReserve<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub fee_note_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_swap_tokens: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_open_orders: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub oracle_price: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub oracle_product: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub quote_token_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeReserve<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.market_authority),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.fee_note_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.dex_swap_tokens),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.dex_open_orders),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.dex_market),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_mint),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.dex_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.oracle_price),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.oracle_product),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_note_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.quote_token_mint),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
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
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeReserve<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.fee_note_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_swap_tokens,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_open_orders,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_market,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.oracle_price,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.oracle_product,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.quote_token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        impl<'info> InitializeReserve<'info> {
            fn init_deposit_mint_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    InitializeMint {
                        mint: self.deposit_note_mint.clone(),
                        rent: self.rent.to_account_info(),
                    },
                )
            }
            fn init_loan_mint_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    InitializeMint {
                        mint: self.loan_note_mint.clone(),
                        rent: self.rent.to_account_info(),
                    },
                )
            }
            fn init_fee_vault_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, InitializeAccount<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    InitializeAccount {
                        account: self.fee_note_vault.clone(),
                        authority: self.market_authority.clone(),
                        mint: self.deposit_note_mint.clone(),
                        rent: self.rent.to_account_info(),
                    },
                )
            }
            fn init_dex_open_orders_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, InitOpenOrders<'info>> {
                CpiContext::new(
                    self.dex_program.clone(),
                    InitOpenOrders {
                        open_orders: self.dex_open_orders.clone(),
                        authority: self.market_authority.clone(),
                        market: self.dex_market.clone(),
                        rent: self.rent.to_account_info(),
                    },
                )
            }
            fn init_accounts(&self) -> ProgramResult {
                token::initialize_mint(
                    self.init_deposit_mint_context(),
                    self.token_mint.decimals,
                    self.market_authority.key,
                    Some(self.market_authority.key),
                )?;
                token::initialize_mint(
                    self.init_loan_mint_context(),
                    self.token_mint.decimals,
                    self.market_authority.key,
                    Some(self.market_authority.key),
                )?;
                token::initialize_account(self.init_fee_vault_context())?;
                Ok(())
            }
            fn register_with_market(&mut self, config: ReserveConfig) -> ProgramResult {
                let mut market = self.market.load_mut()?;
                let mut reserve = self.reserve.load_init()?;
                let oracle_price = &self.oracle_price;
                let oracle_product = &self.oracle_product;
                let token_mint = &self.token_mint;
                reserve.version = 0;
                reserve.config = config;
                reserve.market = self.market.key();
                reserve.pyth_oracle_price = oracle_price.key();
                reserve.pyth_oracle_product = oracle_product.key();
                reserve.vault = self.vault.key();
                reserve.fee_note_vault = self.fee_note_vault.key();
                reserve.dex_swap_tokens = self.dex_swap_tokens.key();
                reserve.dex_open_orders = self.dex_open_orders.key();
                reserve.dex_market = self.dex_market.key();
                reserve.exponent = -(token_mint.decimals as i32);
                reserve.token_mint = token_mint.key();
                reserve.deposit_note_mint = *self.deposit_note_mint.key;
                reserve.loan_note_mint = *self.loan_note_mint.key;
                let clock = Clock::get()?;
                reserve.init(&clock);
                if token_mint.key() != market.quote_token_mint {
                    utils::verify_dex_market_tokens(
                        &self.dex_market,
                        self.dex_program.key,
                        &reserve.token_mint,
                        &market.quote_token_mint,
                    )?;
                    dex::init_open_orders(
                        self.init_dex_open_orders_context()
                            .with_signer(&[&market.authority_seeds()]),
                    )?;
                }
                let product_data = oracle_product.try_borrow_data()?;
                let product = pyth_client::cast::<Product>(&product_data);
                market.validate_oracle(product, oracle_price.key)?;
                let reserve_key = self.reserve.key();
                let market_reserves = market.reserves_mut();
                reserve.index = market_reserves.register(&reserve_key)?;
                ::solana_program::log::sol_log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["registered reserve #"],
                        &match (&{ reserve.index },) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                });
                Ok(())
            }
        }
        /// Initialize a new reserve in a market with some initial liquidity.
        pub fn handler(
            ctx: Context<InitializeReserve>,
            _bump: InitReserveBumpSeeds,
            config: ReserveConfig,
        ) -> ProgramResult {
            ctx.accounts.register_with_market(config)?;
            ctx.accounts.init_accounts()?;
            Ok(())
        }
    }
    pub mod set_market_flags {
        use anchor_lang::prelude::*;
        use crate::errors::ErrorCode;
        use crate::state::*;
        pub struct SetMarketFlags<'info> {
            # [account (mut , has_one = owner)]
            pub market: Loader<'info, Market>,
            #[account(signer)]
            pub owner: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SetMarketFlags<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &market.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                Ok(SetMarketFlags { market, owner })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SetMarketFlags<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SetMarketFlags<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SetMarketFlags<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
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
        pub(crate) mod __client_accounts_set_market_flags {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct SetMarketFlags {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SetMarketFlags
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SetMarketFlags {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
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
        pub(crate) mod __cpi_client_accounts_set_market_flags {
            use super::*;
            pub struct SetMarketFlags<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SetMarketFlags<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SetMarketFlags<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos
                }
            }
        }
        /// Change the flags on a market
        pub fn handler(ctx: Context<SetMarketFlags>, flags: u64) -> ProgramResult {
            let mut market = ctx.accounts.market.load_mut()?;
            let flags = match MarketFlags::from_bits(flags) {
                Some(f) => f,
                None => return Err(ErrorCode::InvalidParameter.into()),
            };
            market.reset_flags(flags);
            Ok(())
        }
    }
    pub mod set_market_owner {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct SetMarketOwner<'info> {
            # [account (mut , has_one = owner)]
            pub market: Loader<'info, Market>,
            #[account(signer)]
            pub owner: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SetMarketOwner<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &market.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                Ok(SetMarketOwner { market, owner })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SetMarketOwner<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SetMarketOwner<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SetMarketOwner<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
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
        pub(crate) mod __client_accounts_set_market_owner {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct SetMarketOwner {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SetMarketOwner
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SetMarketOwner {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
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
        pub(crate) mod __cpi_client_accounts_set_market_owner {
            use super::*;
            pub struct SetMarketOwner<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SetMarketOwner<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SetMarketOwner<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos
                }
            }
        }
        /// Change the owner on a market
        pub fn handler(ctx: Context<SetMarketOwner>, new_owner: Pubkey) -> ProgramResult {
            let mut market = ctx.accounts.market.load_mut()?;
            market.owner = new_owner;
            Ok(())
        }
    }
    pub mod close_deposit_account {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Burn, CloseAccount, Transfer};
        use crate::state::*;
        use crate::Rounding;
        # [instruction (bump : u8)]
        pub struct CloseDepositAccount<'info> {
            /// The relevant market this deposit is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve deposited into
            # [account (mut , has_one = market , has_one = vault , has_one = deposit_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault where any tokens to withdraw will be transferred from
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the deposit notes
            #[account(mut)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The user/authority that owns the deposits
            #[account(mut, signer)]
            pub depositor: AccountInfo<'info>,
            /// The account that stores the deposit notes, to be closed
            # [account (mut , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , depositor . key . as_ref ()] , bump = bump)]
            pub deposit_account: AccountInfo<'info>,
            /// The account to receive any remaining tokens still deposited
            #[account(mut)]
            pub receiver_account: AccountInfo<'info>,
            # [account (address = anchor_spl :: token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CloseDepositAccount<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let depositor: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let receiver_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.deposit_note_mint != deposit_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !depositor.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !depositor.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        depositor.key.as_ref(),
                        &[bump][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if deposit_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !receiver_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &anchor_spl::token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(CloseDepositAccount {
                    market,
                    market_authority,
                    reserve,
                    vault,
                    deposit_note_mint,
                    depositor,
                    deposit_account,
                    receiver_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDepositAccount<'info>
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
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.depositor.to_account_infos());
                account_infos.extend(self.deposit_account.to_account_infos());
                account_infos.extend(self.receiver_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CloseDepositAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.depositor.to_account_metas(Some(true)));
                account_metas.extend(self.deposit_account.to_account_metas(None));
                account_metas.extend(self.receiver_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CloseDepositAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.depositor, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.receiver_account, program_id)?;
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
        pub(crate) mod __client_accounts_close_deposit_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct CloseDepositAccount {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub receiver_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CloseDepositAccount
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
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.receiver_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CloseDepositAccount {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.depositor,
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.receiver_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_close_deposit_account {
            use super::*;
            pub struct CloseDepositAccount<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub receiver_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CloseDepositAccount<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.depositor),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.receiver_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDepositAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.receiver_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CloseDepositAccount<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.vault.to_account_info(),
                        to: self.receiver_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn note_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Burn {
                        to: self.deposit_account.to_account_info(),
                        mint: self.deposit_note_mint.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    CloseAccount {
                        account: self.deposit_account.to_account_info(),
                        destination: self.depositor.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Close an account that stores deposit notes
        pub fn handler(ctx: Context<CloseDepositAccount>, _bump: u8) -> ProgramResult {
            let market = ctx.accounts.market.load()?;
            let notes_remaining = token::accessor::amount(&ctx.accounts.deposit_account)?;
            if notes_remaining > 0 {
                market.verify_ability_deposit_withdraw()?;
                let mut reserve = ctx.accounts.reserve.load_mut()?;
                let clock = Clock::get()?;
                let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
                let tokens_to_withdraw =
                    reserve_info.deposit_notes_to_tokens(notes_remaining, Rounding::Down);
                reserve.withdraw(tokens_to_withdraw, notes_remaining);
                token::transfer(
                    ctx.accounts
                        .transfer_context()
                        .with_signer(&[&market.authority_seeds()]),
                    tokens_to_withdraw,
                )?;
                token::burn(
                    ctx.accounts
                        .note_burn_context()
                        .with_signer(&[&market.authority_seeds()]),
                    notes_remaining,
                )?;
            }
            token::close_account(
                ctx.accounts
                    .close_context()
                    .with_signer(&[&market.authority_seeds()]),
            )?;
            ::solana_program::log::sol_log("initialized deposit account");
            Ok(())
        }
    }
    pub mod borrow {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, MintTo, Transfer};
        use crate::state::*;
        use crate::{Amount, ErrorCode, Rounding};
        pub struct BorrowEvent {
            borrower: Pubkey,
            reserve: Pubkey,
            debt: u64,
        }
        impl borsh::ser::BorshSerialize for BorrowEvent
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.borrower, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.debt, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for BorrowEvent
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    borrower: borsh::BorshDeserialize::deserialize(buf)?,
                    reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    debt: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for BorrowEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [86, 8, 140, 206, 215, 179, 118, 201].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for BorrowEvent {
            fn discriminator() -> [u8; 8] {
                [86, 8, 140, 206, 215, 179, 118, 201]
            }
        }
        # [instruction (bump : u8)]
        pub struct Borrow<'info> {
            /// The relevant market this borrow is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation with collateral to borrow with
            #[account(mut)]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve being borrowed from
            # [account (mut , has_one = market , has_one = vault , has_one = loan_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the borrowed tokens will be transferred from
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the debt/loan notes
            #[account(mut)]
            pub loan_note_mint: AccountInfo<'info>,
            /// The user/authority that is borrowing
            #[account(signer)]
            pub borrower: AccountInfo<'info>,
            /// The account to track the borrower's balance to repay
            # [account (mut , seeds = [b"loan" . as_ref () , reserve . key () . as_ref () , obligation . key () . as_ref () , borrower . key . as_ref ()] , bump = bump)]
            pub loan_account: AccountInfo<'info>,
            /// The token account that the borrowed funds will be transferred to
            # [account (mut , constraint = receiver_account . key () != vault . key ())]
            pub receiver_account: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Borrow<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let borrower: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let receiver_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.loan_note_mint != loan_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !borrower.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"loan".as_ref(),
                        reserve.key().as_ref(),
                        obligation.key().as_ref(),
                        borrower.key.as_ref(),
                        &[bump][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if loan_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !loan_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !receiver_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !(receiver_account.key() != vault.key()) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(Borrow {
                    market,
                    market_authority,
                    obligation,
                    reserve,
                    vault,
                    loan_note_mint,
                    borrower,
                    loan_account,
                    receiver_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Borrow<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.borrower.to_account_infos());
                account_infos.extend(self.loan_account.to_account_infos());
                account_infos.extend(self.receiver_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Borrow<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.borrower.to_account_metas(Some(true)));
                account_metas.extend(self.loan_account.to_account_metas(None));
                account_metas.extend(self.receiver_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Borrow<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.receiver_account, program_id)?;
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
        pub(crate) mod __client_accounts_borrow {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct Borrow {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub borrower: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub receiver_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Borrow
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
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.market_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.borrower, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.receiver_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Borrow {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_note_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.borrower,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.receiver_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_borrow {
            use super::*;
            pub struct Borrow<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub borrower: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub receiver_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Borrow<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_note_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.borrower),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.receiver_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Borrow<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.borrower));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.receiver_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Borrow<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.vault.to_account_info(),
                        to: self.receiver_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn note_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    MintTo {
                        to: self.loan_account.to_account_info(),
                        mint: self.loan_note_mint.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Borrow tokens from a reserve
        pub fn handler(ctx: Context<Borrow>, _bump: u8, amount: Amount) -> ProgramResult {
            let market = ctx.accounts.market.load()?;
            let mut reserve = ctx.accounts.reserve.load_mut()?;
            let loan_account = &ctx.accounts.loan_account.key();
            market.verify_ability_borrow()?;
            let market_reserves = market.reserves();
            let clock = Clock::get().unwrap();
            let reserve_info = market_reserves.get_cached(reserve.index, clock.slot);
            let requested_tokens = amount.as_tokens(reserve_info, Rounding::Down);
            let fees = reserve.borrow_fee(requested_tokens);
            let total_token_debt = requested_tokens.checked_add(fees).expect(
                "Requested a debt that would exceed the maximum potential supply for a token.",
            );
            let new_notes = reserve_info.loan_notes_from_tokens(total_token_debt, Rounding::Up);
            reserve.borrow(clock.slot, requested_tokens, new_notes, fees);
            token::mint_to(
                ctx.accounts
                    .note_mint_context()
                    .with_signer(&[&market.authority_seeds()]),
                new_notes,
            )?;
            let obligation = &mut ctx.accounts.obligation.load_mut()?;
            obligation.borrow(loan_account, reserve.amount(new_notes))?;
            obligation.cache_calculations(market.reserves(), clock.slot);
            if !obligation.is_healthy(market_reserves, clock.slot) {
                return Err(ErrorCode::InsufficientCollateral.into());
            }
            token::transfer(
                ctx.accounts
                    .transfer_context()
                    .with_signer(&[&market.authority_seeds()]),
                requested_tokens,
            )?;
            {
                let data = anchor_lang::Event::data(&BorrowEvent {
                    borrower: ctx.accounts.borrower.key(),
                    reserve: ctx.accounts.reserve.key(),
                    debt: new_notes,
                });
                let msg_str = &anchor_lang::__private::base64::encode(data);
                ::solana_program::log::sol_log(msg_str);
            };
            Ok(())
        }
    }
    pub mod deposit {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, MintTo, Transfer};
        use crate::state::*;
        use crate::{Amount, Rounding};
        # [instruction (bump : u8)]
        pub struct Deposit<'info> {
            /// The relevant market this deposit is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve being deposited into
            # [account (mut , has_one = market , has_one = vault , has_one = deposit_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the deposited tokens will be transferred to
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the deposit notes
            #[account(mut)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The user/authority that owns the deposit
            #[account(signer)]
            pub depositor: AccountInfo<'info>,
            /// The account that will store the deposit notes
            # [account (mut , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , depositor . key . as_ref ()] , bump = bump)]
            pub deposit_account: AccountInfo<'info>,
            /// The token account with the tokens to be deposited
            #[account(mut)]
            pub deposit_source: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Deposit<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let depositor: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_source: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.deposit_note_mint != deposit_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !depositor.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        depositor.key.as_ref(),
                        &[bump][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if deposit_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_source.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(Deposit {
                    market,
                    market_authority,
                    reserve,
                    vault,
                    deposit_note_mint,
                    depositor,
                    deposit_account,
                    deposit_source,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info>
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
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.depositor.to_account_infos());
                account_infos.extend(self.deposit_account.to_account_infos());
                account_infos.extend(self.deposit_source.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.depositor.to_account_metas(Some(true)));
                account_metas.extend(self.deposit_account.to_account_metas(None));
                account_metas.extend(self.deposit_source.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Deposit<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_source, program_id)?;
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
        pub(crate) mod __client_accounts_deposit {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct Deposit {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_source: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Deposit
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
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_source, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Deposit {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.depositor,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_source,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_deposit {
            use super::*;
            pub struct Deposit<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_source: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.depositor),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_source),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_source,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Deposit<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.deposit_source.to_account_info(),
                        to: self.vault.to_account_info(),
                        authority: self.depositor.clone(),
                    },
                )
            }
            fn note_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    MintTo {
                        to: self.deposit_account.to_account_info(),
                        mint: self.deposit_note_mint.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Deposit tokens into a reserve
        pub fn handler(ctx: Context<Deposit>, _bump: u8, amount: Amount) -> ProgramResult {
            let market = ctx.accounts.market.load()?;
            let mut reserve = ctx.accounts.reserve.load_mut()?;
            let clock = Clock::get()?;
            let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
            market.verify_ability_deposit_withdraw()?;
            let token_amount = amount.as_tokens(reserve_info, Rounding::Up);
            let note_amount = amount.as_deposit_notes(reserve_info, Rounding::Down)?;
            reserve.deposit(token_amount, note_amount);
            token::transfer(ctx.accounts.transfer_context(), token_amount)?;
            token::mint_to(
                ctx.accounts
                    .note_mint_context()
                    .with_signer(&[&market.authority_seeds()]),
                note_amount,
            )?;
            Ok(())
        }
    }
    pub mod deposit_collateral {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Transfer};
        use crate::state::*;
        use crate::{Amount, Rounding};
        pub struct DepositCollateralEvent {
            depositor: Pubkey,
            reserve: Pubkey,
            amount: Amount,
        }
        impl borsh::ser::BorshSerialize for DepositCollateralEvent
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Amount: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for DepositCollateralEvent
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Amount: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    depositor: borsh::BorshDeserialize::deserialize(buf)?,
                    reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    amount: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for DepositCollateralEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [169, 14, 102, 148, 155, 137, 18, 235].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for DepositCollateralEvent {
            fn discriminator() -> [u8; 8] {
                [169, 14, 102, 148, 155, 137, 18, 235]
            }
        }
        pub struct DepositCollateralBumpSeeds {
            collateral_account: u8,
            deposit_account: u8,
        }
        impl borsh::de::BorshDeserialize for DepositCollateralBumpSeeds
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    collateral_account: borsh::BorshDeserialize::deserialize(buf)?,
                    deposit_account: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for DepositCollateralBumpSeeds
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                Ok(())
            }
        }
        # [instruction (bump : DepositCollateralBumpSeeds)]
        pub struct DepositCollateral<'info> {
            /// The relevant market this deposit is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve that the collateral comes from
            # [account (has_one = market)]
            pub reserve: Loader<'info, Reserve>,
            /// The obligation the collateral is being deposited toward
            # [account (mut , has_one = market , has_one = owner)]
            pub obligation: Loader<'info, Obligation>,
            /// The user/authority that owns the deposit
            #[account(signer)]
            pub owner: AccountInfo<'info>,
            /// The account that stores the user's deposit notes
            # [account (mut , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , owner . key . as_ref ()] , bump = bump . deposit_account)]
            pub deposit_account: AccountInfo<'info>,
            /// The account that will store the deposit notes as collateral
            # [account (mut , seeds = [b"collateral" . as_ref () , reserve . key () . as_ref () , obligation . key () . as_ref () , owner . key . as_ref ()] , bump = bump . collateral_account)]
            pub collateral_account: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for DepositCollateral<'info>
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
                    bump: DepositCollateralBumpSeeds,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    DepositCollateralBumpSeeds: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.bump, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    DepositCollateralBumpSeeds: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &obligation.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        owner.key.as_ref(),
                        &[bump.deposit_account][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if deposit_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"collateral".as_ref(),
                        reserve.key().as_ref(),
                        obligation.key().as_ref(),
                        owner.key.as_ref(),
                        &[bump.collateral_account][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if collateral_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !collateral_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(DepositCollateral {
                    market,
                    market_authority,
                    reserve,
                    obligation,
                    owner,
                    deposit_account,
                    collateral_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DepositCollateral<'info>
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
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.deposit_account.to_account_infos());
                account_infos.extend(self.collateral_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DepositCollateral<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas.extend(self.deposit_account.to_account_metas(None));
                account_metas.extend(self.collateral_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for DepositCollateral<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_account, program_id)?;
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
        pub(crate) mod __client_accounts_deposit_collateral {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct DepositCollateral {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for DepositCollateral
            where
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
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for DepositCollateral {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_deposit_collateral {
            use super::*;
            pub struct DepositCollateral<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for DepositCollateral<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for DepositCollateral<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> DepositCollateral<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.deposit_account.to_account_info(),
                        to: self.collateral_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Deposit reserve notes as collateral for an obligation
        pub fn handler(
            ctx: Context<DepositCollateral>,
            _bump: DepositCollateralBumpSeeds,
            amount: Amount,
        ) -> ProgramResult {
            let market = &ctx.accounts.market.load()?;
            let reserve = ctx.accounts.reserve.load()?;
            let clock = Clock::get()?;
            let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
            market.verify_ability_deposit_withdraw()?;
            let note_amount = amount.as_deposit_notes(reserve_info, Rounding::Down)?;
            token::transfer(
                ctx.accounts
                    .transfer_context()
                    .with_signer(&[&market.authority_seeds()]),
                note_amount,
            )?;
            let mut obligation = ctx.accounts.obligation.load_mut()?;
            let collateral_account = ctx.accounts.collateral_account.key();
            obligation.deposit_collateral(&collateral_account, reserve.amount(note_amount))?;
            {
                let data = anchor_lang::Event::data(&DepositCollateralEvent {
                    depositor: ctx.accounts.owner.key(),
                    reserve: ctx.accounts.reserve.key(),
                    amount,
                });
                let msg_str = &anchor_lang::__private::base64::encode(data);
                ::solana_program::log::sol_log(msg_str);
            };
            Ok(())
        }
    }
    pub mod liquidate {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Transfer};
        use crate::errors::ErrorCode;
        use crate::repay::{implement_repay_context, repay, RepayContext};
        use crate::state::*;
        use crate::{Amount, Rounding};
        pub struct LiquidateEvent {
            borrower: Pubkey,
            debt_reserve: Pubkey,
            collateral_reserve: Pubkey,
            paid_amount: Amount,
            collateral_amount: u64,
        }
        impl borsh::ser::BorshSerialize for LiquidateEvent
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Amount: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.borrower, writer)?;
                borsh::BorshSerialize::serialize(&self.debt_reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.collateral_reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.paid_amount, writer)?;
                borsh::BorshSerialize::serialize(&self.collateral_amount, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for LiquidateEvent
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Amount: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    borrower: borsh::BorshDeserialize::deserialize(buf)?,
                    debt_reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    collateral_reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    paid_amount: borsh::BorshDeserialize::deserialize(buf)?,
                    collateral_amount: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for LiquidateEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [158, 94, 144, 4, 147, 52, 5, 255].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for LiquidateEvent {
            fn discriminator() -> [u8; 8] {
                [158, 94, 144, 4, 147, 52, 5, 255]
            }
        }
        pub struct Liquidate<'info> {
            /// The relevant market this liquidation is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation with debt to be repaid
            # [account (mut , has_one = market , constraint = obligation . load () . unwrap () . has_collateral_custody (& collateral_account . key ()) , constraint = obligation . load () . unwrap () . has_loan_custody (& loan_account . key ()) , constraint = obligation . load () . unwrap () . is_collateral_reserve (& market . load () . unwrap () , & collateral_account . key () , & collateral_reserve . key ()))]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve that the debt is from
            # [account (mut , has_one = market , has_one = vault , has_one = loan_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve the collateral is from
            pub collateral_reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the payment will be transferred to
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the debt/loan notes
            #[account(mut)]
            pub loan_note_mint: AccountInfo<'info>,
            /// The account that holds the borrower's debt balance
            #[account(mut)]
            pub loan_account: AccountInfo<'info>,
            /// The account that holds the borrower's collateral
            #[account(mut)]
            pub collateral_account: AccountInfo<'info>,
            /// The token account that the payment funds will be transferred from
            #[account(mut)]
            pub payer_account: AccountInfo<'info>,
            /// The account that will receive a portion of the borrower's collateral
            #[account(mut)]
            pub receiver_account: AccountInfo<'info>,
            /// The account paying off the loan
            #[account(signer)]
            pub payer: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Liquidate<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let payer_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let receiver_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let payer: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !(obligation
                    .load()
                    .unwrap()
                    .has_collateral_custody(&collateral_account.key()))
                {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !(obligation
                    .load()
                    .unwrap()
                    .has_loan_custody(&loan_account.key()))
                {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !(obligation.load().unwrap().is_collateral_reserve(
                    &market.load().unwrap(),
                    &collateral_account.key(),
                    &collateral_reserve.key(),
                )) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.loan_note_mint != loan_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !collateral_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !payer_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !receiver_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !payer.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(Liquidate {
                    market,
                    market_authority,
                    obligation,
                    reserve,
                    collateral_reserve,
                    vault,
                    loan_note_mint,
                    loan_account,
                    collateral_account,
                    payer_account,
                    receiver_account,
                    payer,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Liquidate<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.collateral_reserve.to_account_infos());
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.loan_account.to_account_infos());
                account_infos.extend(self.collateral_account.to_account_infos());
                account_infos.extend(self.payer_account.to_account_infos());
                account_infos.extend(self.receiver_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Liquidate<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.collateral_reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.loan_account.to_account_metas(None));
                account_metas.extend(self.collateral_account.to_account_metas(None));
                account_metas.extend(self.payer_account.to_account_metas(None));
                account_metas.extend(self.receiver_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(Some(true)));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Liquidate<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.payer_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.receiver_account, program_id)?;
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
        pub(crate) mod __client_accounts_liquidate {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct Liquidate {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub receiver_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Liquidate
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
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.receiver_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Liquidate {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.collateral_reserve,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.receiver_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.payer, true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_liquidate {
            use super::*;
            pub struct Liquidate<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_reserve:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub receiver_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Liquidate<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.collateral_reserve),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.receiver_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.payer),
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Liquidate<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_reserve,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.payer_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.receiver_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Liquidate<'info> {
            fn transfer_collateral_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.collateral_account.to_account_info(),
                        to: self.receiver_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        impl<'info> RepayContext<'info> for Liquidate<'info> {
            fn market(&self) -> &Loader<'info, Market> {
                &self.market
            }
            fn market_authority(&self) -> &AccountInfo<'info> {
                &self.market_authority
            }
            fn obligation(&self) -> &Loader<'info, Obligation> {
                &self.obligation
            }
            fn reserve(&self) -> &Loader<'info, Reserve> {
                &self.reserve
            }
            fn vault(&self) -> &AccountInfo<'info> {
                &self.vault
            }
            fn loan_note_mint(&self) -> &AccountInfo<'info> {
                &self.loan_note_mint
            }
            fn payer(&self) -> &AccountInfo<'info> {
                &self.payer
            }
            fn loan_account(&self) -> &AccountInfo<'info> {
                &self.loan_account
            }
            fn payer_account(&self) -> &AccountInfo<'info> {
                &self.payer_account
            }
            fn token_program(&self) -> &AccountInfo<'info> {
                &self.token_program
            }
        }
        /// Liquidate a part of an obligation's debt
        /// amount: number of tokens being paid off from the debt
        pub fn handler(
            ctx: Context<Liquidate>,
            amount: Amount,
            min_collateral: u64,
        ) -> ProgramResult {
            let collateral_amount = transfer_collateral(ctx.accounts, amount, min_collateral)?;
            repay(&ctx, amount)?;
            {
                let data = anchor_lang::Event::data(&LiquidateEvent {
                    borrower: ctx.accounts.obligation.key(),
                    debt_reserve: ctx.accounts.reserve.key(),
                    collateral_reserve: ctx.accounts.reserve.key(),
                    paid_amount: amount,
                    collateral_amount,
                });
                let msg_str = &anchor_lang::__private::base64::encode(data);
                ::solana_program::log::sol_log(msg_str);
            };
            Ok(())
        }
        fn transfer_collateral(
            accounts: &mut Liquidate,
            amount: Amount,
            min_collateral: u64,
        ) -> Result<u64, ProgramError> {
            let market = accounts.market.load()?;
            let reserve = accounts.reserve.load()?;
            let collateral_reserve = accounts.collateral_reserve.load()?;
            let mut obligation = accounts.obligation.load_mut()?;
            let clock = Clock::get().unwrap();
            let market_reserves = market.reserves();
            let reserve_info = market_reserves.get_cached(reserve.index, clock.slot);
            obligation.cache_calculations(market.reserves(), clock.slot);
            if obligation.is_healthy(market_reserves, clock.slot) {
                return Err(ErrorCode::ObligationHealthy.into());
            }
            let repaid_notes_amount =
                reserve.amount(amount.as_loan_notes(reserve_info, Rounding::Down)?);
            let collateral_account = accounts.collateral_account.key();
            let loan_account = accounts.loan_account.key();
            let collateral_amount = obligation.liquidate(
                market_reserves,
                clock.slot,
                &collateral_account,
                &loan_account,
                repaid_notes_amount,
            )?;
            let collateral_amount = collateral_amount.as_u64_rounded(collateral_reserve.exponent);
            if collateral_amount < min_collateral {
                ::solana_program::log::sol_log("collateral below amount requested");
                return Err(ErrorCode::LiquidationLowCollateral.into());
            }
            token::transfer(
                accounts
                    .transfer_collateral_context()
                    .with_signer(&[&market.authority_seeds()]),
                collateral_amount,
            )?;
            Ok(collateral_amount)
        }
    }
    pub mod liquidate_dex {
        use std::num::NonZeroU64;
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::dex;
        use anchor_spl::dex::serum_dex::instruction::SelfTradeBehavior;
        use anchor_spl::dex::serum_dex::matching::{OrderType, Side};
        use anchor_spl::dex::serum_dex::state::MarketState as DexMarketState;
        use anchor_spl::token::Transfer;
        use anchor_spl::token::{self, Burn};
        use jet_math::Number;
        use crate::errors::ErrorCode;
        use crate::state::*;
        /// Accounts used to place orders on the DEX
        pub struct DexMarketAccounts<'info> {
            #[account(mut)]
            market: AccountInfo<'info>,
            #[account(mut)]
            open_orders: AccountInfo<'info>,
            #[account(mut)]
            request_queue: AccountInfo<'info>,
            #[account(mut)]
            event_queue: AccountInfo<'info>,
            #[account(mut)]
            bids: AccountInfo<'info>,
            #[account(mut)]
            asks: AccountInfo<'info>,
            /// The vault for the "base" currency
            #[account(mut)]
            coin_vault: AccountInfo<'info>,
            /// The vault for the "quote" currency
            #[account(mut)]
            pc_vault: AccountInfo<'info>,
            /// DEX owner
            vault_signer: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for DexMarketAccounts<'info>
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
                let market: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let open_orders: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let request_queue: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let event_queue: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let bids: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let asks: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let coin_vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let pc_vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let vault_signer: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !open_orders.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !request_queue.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !event_queue.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !bids.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !asks.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !coin_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !pc_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                Ok(DexMarketAccounts {
                    market,
                    open_orders,
                    request_queue,
                    event_queue,
                    bids,
                    asks,
                    coin_vault,
                    pc_vault,
                    vault_signer,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DexMarketAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.open_orders.to_account_infos());
                account_infos.extend(self.request_queue.to_account_infos());
                account_infos.extend(self.event_queue.to_account_infos());
                account_infos.extend(self.bids.to_account_infos());
                account_infos.extend(self.asks.to_account_infos());
                account_infos.extend(self.coin_vault.to_account_infos());
                account_infos.extend(self.pc_vault.to_account_infos());
                account_infos.extend(self.vault_signer.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DexMarketAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.open_orders.to_account_metas(None));
                account_metas.extend(self.request_queue.to_account_metas(None));
                account_metas.extend(self.event_queue.to_account_metas(None));
                account_metas.extend(self.bids.to_account_metas(None));
                account_metas.extend(self.asks.to_account_metas(None));
                account_metas.extend(self.coin_vault.to_account_metas(None));
                account_metas.extend(self.pc_vault.to_account_metas(None));
                account_metas.extend(self.vault_signer.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for DexMarketAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
                anchor_lang::AccountsExit::exit(&self.open_orders, program_id)?;
                anchor_lang::AccountsExit::exit(&self.request_queue, program_id)?;
                anchor_lang::AccountsExit::exit(&self.event_queue, program_id)?;
                anchor_lang::AccountsExit::exit(&self.bids, program_id)?;
                anchor_lang::AccountsExit::exit(&self.asks, program_id)?;
                anchor_lang::AccountsExit::exit(&self.coin_vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.pc_vault, program_id)?;
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
        pub(crate) mod __client_accounts_dex_market_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct DexMarketAccounts {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub open_orders: anchor_lang::solana_program::pubkey::Pubkey,
                pub request_queue: anchor_lang::solana_program::pubkey::Pubkey,
                pub event_queue: anchor_lang::solana_program::pubkey::Pubkey,
                pub bids: anchor_lang::solana_program::pubkey::Pubkey,
                pub asks: anchor_lang::solana_program::pubkey::Pubkey,
                pub coin_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub pc_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_signer: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for DexMarketAccounts
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
                    borsh::BorshSerialize::serialize(&self.open_orders, writer)?;
                    borsh::BorshSerialize::serialize(&self.request_queue, writer)?;
                    borsh::BorshSerialize::serialize(&self.event_queue, writer)?;
                    borsh::BorshSerialize::serialize(&self.bids, writer)?;
                    borsh::BorshSerialize::serialize(&self.asks, writer)?;
                    borsh::BorshSerialize::serialize(&self.coin_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.pc_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_signer, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for DexMarketAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.open_orders,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.request_queue,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.event_queue,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.bids, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.asks, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.coin_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.pc_vault,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_signer,
                            false,
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
        pub(crate) mod __cpi_client_accounts_dex_market_accounts {
            use super::*;
            pub struct DexMarketAccounts<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub open_orders: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub request_queue: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub event_queue: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub bids: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub asks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub coin_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub pc_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for DexMarketAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.open_orders),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.request_queue),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.event_queue),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.bids),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.asks),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.coin_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.pc_vault),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_signer),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for DexMarketAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.open_orders,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.request_queue,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.event_queue,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.bids));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.asks));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.coin_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pc_vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_signer,
                    ));
                    account_infos
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'info> ::core::clone::Clone for DexMarketAccounts<'info> {
            #[inline]
            fn clone(&self) -> DexMarketAccounts<'info> {
                match *self {
                    DexMarketAccounts {
                        market: ref __self_0_0,
                        open_orders: ref __self_0_1,
                        request_queue: ref __self_0_2,
                        event_queue: ref __self_0_3,
                        bids: ref __self_0_4,
                        asks: ref __self_0_5,
                        coin_vault: ref __self_0_6,
                        pc_vault: ref __self_0_7,
                        vault_signer: ref __self_0_8,
                    } => DexMarketAccounts {
                        market: ::core::clone::Clone::clone(&(*__self_0_0)),
                        open_orders: ::core::clone::Clone::clone(&(*__self_0_1)),
                        request_queue: ::core::clone::Clone::clone(&(*__self_0_2)),
                        event_queue: ::core::clone::Clone::clone(&(*__self_0_3)),
                        bids: ::core::clone::Clone::clone(&(*__self_0_4)),
                        asks: ::core::clone::Clone::clone(&(*__self_0_5)),
                        coin_vault: ::core::clone::Clone::clone(&(*__self_0_6)),
                        pc_vault: ::core::clone::Clone::clone(&(*__self_0_7)),
                        vault_signer: ::core::clone::Clone::clone(&(*__self_0_8)),
                    },
                }
            }
        }
        /// Client for interacting with the DEX program
        struct DexClient<'a, 'info> {
            market: &'a Market,
            market_authority: &'a AccountInfo<'info>,
            dex_market: &'a DexMarketAccounts<'info>,
            dex_program: &'a AccountInfo<'info>,
            order_payer_token_account: &'a AccountInfo<'info>,
            coin_wallet: &'a AccountInfo<'info>,
            pc_wallet: &'a AccountInfo<'info>,
            token_program: &'a AccountInfo<'info>,
            rent: &'a AccountInfo<'info>,
        }
        impl<'a, 'info> DexClient<'a, 'info> {
            fn price_lots(
                &self,
                price: Number,
                quote_expo: i32,
                coin_expo: i32,
            ) -> Result<u64, ProgramError> {
                let quote_decimals = (quote_expo * -1) as u32;
                let coin_decimals = (coin_expo * -1) as u32;
                let dex_market = DexMarketState::load(&self.dex_market.market, &dex::ID)?;
                Ok(
                    (price * Number::ten_pow(quote_decimals) * dex_market.coin_lot_size
                        / (Number::ten_pow(coin_decimals) * dex_market.pc_lot_size))
                        .as_u64_rounded(0),
                )
            }
            /// Buy as much of the base currency as possible with the given amount
            /// of quote tokens.
            fn buy(&self, limit_price: u64, quote_amount: u64) -> ProgramResult {
                let max_coin_qty = u64::MAX;
                let max_pc_qty = quote_amount;
                self.create_order(Side::Bid, limit_price, max_coin_qty, max_pc_qty)
            }
            /// Sell as much of the given base currency as possible.
            fn sell(&self, limit_price: u64, base_amount: u64) -> ProgramResult {
                let max_pc_qty = u64::MAX;
                let max_coin_qty = {
                    let dex_market = DexMarketState::load(&self.dex_market.market, &dex::ID)?;
                    base_amount.checked_div(dex_market.coin_lot_size).unwrap()
                };
                if max_coin_qty == 0 {
                    return Err(ErrorCode::CollateralValueTooSmall.into());
                }
                self.create_order(Side::Ask, limit_price, max_coin_qty, max_pc_qty)
            }
            /// Create a new order to trade on the DEX
            fn create_order(
                &self,
                side: Side,
                limit_price: u64,
                max_coin_qty: u64,
                max_pc_qty: u64,
            ) -> ProgramResult {
                let dex_accs = dex::NewOrderV3 {
                    market: self.dex_market.market.clone(),
                    open_orders: self.dex_market.open_orders.clone(),
                    request_queue: self.dex_market.request_queue.clone(),
                    event_queue: self.dex_market.event_queue.clone(),
                    market_bids: self.dex_market.bids.clone(),
                    market_asks: self.dex_market.asks.clone(),
                    order_payer_token_account: self.order_payer_token_account.clone(),
                    open_orders_authority: self.market_authority.clone(),
                    coin_vault: self.dex_market.coin_vault.clone(),
                    pc_vault: self.dex_market.pc_vault.clone(),
                    token_program: self.token_program.clone(),
                    rent: self.rent.clone(),
                };
                let ctx = CpiContext::new(self.dex_program.clone(), dex_accs);
                dex::new_order_v3(
                    ctx.with_signer(&[&self.market.authority_seeds()]),
                    side,
                    NonZeroU64::new(limit_price).unwrap(),
                    NonZeroU64::new(max_coin_qty).unwrap(),
                    NonZeroU64::new(max_pc_qty).unwrap(),
                    SelfTradeBehavior::DecrementTake,
                    OrderType::ImmediateOrCancel,
                    0,
                    65535,
                )
            }
            /// Settle funds from a trade
            fn settle(&self) -> ProgramResult {
                let settle_accs = dex::SettleFunds {
                    market: self.dex_market.market.clone(),
                    open_orders: self.dex_market.open_orders.clone(),
                    open_orders_authority: self.market_authority.clone(),
                    coin_vault: self.dex_market.coin_vault.clone(),
                    pc_vault: self.dex_market.pc_vault.clone(),
                    coin_wallet: self.coin_wallet.clone(),
                    pc_wallet: self.pc_wallet.clone(),
                    vault_signer: self.dex_market.vault_signer.clone(),
                    token_program: self.token_program.clone(),
                };
                let ctx = CpiContext::new(self.dex_program.clone(), settle_accs);
                dex::settle_funds(ctx.with_signer(&[&self.market.authority_seeds()]))
            }
        }
        pub enum DexSide {
            Bid,
            Ask,
        }
        impl borsh::de::BorshDeserialize for DexSide {
            fn deserialize(
                buf: &mut &[u8],
            ) -> core::result::Result<Self, borsh::maybestd::io::Error> {
                let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
                let return_value = match variant_idx {
                    0u8 => DexSide::Bid,
                    1u8 => DexSide::Ask,
                    _ => {
                        let msg = {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Unexpected variant index: "],
                                &match (&variant_idx,) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                            ));
                            res
                        };
                        return Err(borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            msg,
                        ));
                    }
                };
                Ok(return_value)
            }
        }
        impl borsh::ser::BorshSerialize for DexSide {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> core::result::Result<(), borsh::maybestd::io::Error> {
                match self {
                    DexSide::Bid => {
                        let variant_idx: u8 = 0u8;
                        writer.write_all(&variant_idx.to_le_bytes())?;
                    }
                    DexSide::Ask => {
                        let variant_idx: u8 = 1u8;
                        writer.write_all(&variant_idx.to_le_bytes())?;
                    }
                }
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for DexSide {
            #[inline]
            fn clone(&self) -> DexSide {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for DexSide {}
        pub struct LiquidateDex<'info> {
            /// The relevant market this liquidation is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation with debt to be repaid
            # [account (mut , has_one = market)]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve that the debt is from
            # [account (mut , has_one = market , has_one = loan_note_mint , has_one = dex_swap_tokens , constraint = loan_reserve . load () . unwrap () . vault == loan_reserve_vault . key ())]
            pub loan_reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the debt repayment should go
            #[account(mut)]
            pub loan_reserve_vault: AccountInfo<'info>,
            /// The mint for the debt/loan notes
            #[account(mut)]
            pub loan_note_mint: AccountInfo<'info>,
            /// The account that holds the borrower's debt balance
            #[account(mut)]
            pub loan_account: AccountInfo<'info>,
            /// The reserve that the collateral is from
            # [account (has_one = market , has_one = deposit_note_mint , constraint = collateral_reserve . load () . unwrap () . vault == collateral_reserve_vault . key ())]
            pub collateral_reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the collateral will be withdrawn from
            #[account(mut)]
            pub collateral_reserve_vault: AccountInfo<'info>,
            /// The mint for the collateral's deposit notes
            #[account(mut)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The account that holds the borrower's collateral balance
            #[account(mut)]
            pub collateral_account: AccountInfo<'info>,
            /// The account for temporarily storing any quote tokens during
            /// the swap between collateral and loaned assets.
            #[account(mut)]
            pub dex_swap_tokens: AccountInfo<'info>,
            /// The DEX program
            # [account (address = dex :: ID)]
            pub dex_program: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
            pub rent: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for LiquidateDex<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_reserve_vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_reserve_vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let dex_swap_tokens: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let dex_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let rent: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !loan_reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &loan_reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &loan_reserve.load()?.loan_note_mint != loan_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &loan_reserve.load()?.dex_swap_tokens != dex_swap_tokens.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !(loan_reserve.load().unwrap().vault == loan_reserve_vault.key()) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !loan_reserve_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &collateral_reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &collateral_reserve.load()?.deposit_note_mint
                    != deposit_note_mint.to_account_info().key
                {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !(collateral_reserve.load().unwrap().vault == collateral_reserve_vault.key()) {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !collateral_reserve_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !collateral_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !dex_swap_tokens.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if dex_program.to_account_info().key != &dex::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(LiquidateDex {
                    market,
                    market_authority,
                    obligation,
                    loan_reserve,
                    loan_reserve_vault,
                    loan_note_mint,
                    loan_account,
                    collateral_reserve,
                    collateral_reserve_vault,
                    deposit_note_mint,
                    collateral_account,
                    dex_swap_tokens,
                    dex_program,
                    token_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for LiquidateDex<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.loan_reserve.to_account_infos());
                account_infos.extend(self.loan_reserve_vault.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.loan_account.to_account_infos());
                account_infos.extend(self.collateral_reserve.to_account_infos());
                account_infos.extend(self.collateral_reserve_vault.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.collateral_account.to_account_infos());
                account_infos.extend(self.dex_swap_tokens.to_account_infos());
                account_infos.extend(self.dex_program.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for LiquidateDex<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.loan_reserve.to_account_metas(None));
                account_metas.extend(self.loan_reserve_vault.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.loan_account.to_account_metas(None));
                account_metas.extend(self.collateral_reserve.to_account_metas(None));
                account_metas.extend(self.collateral_reserve_vault.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.collateral_account.to_account_metas(None));
                account_metas.extend(self.dex_swap_tokens.to_account_metas(None));
                account_metas.extend(self.dex_program.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for LiquidateDex<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_reserve_vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_reserve_vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.dex_swap_tokens, program_id)?;
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
        pub(crate) mod __client_accounts_liquidate_dex {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct LiquidateDex {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_reserve_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_reserve_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_swap_tokens: anchor_lang::solana_program::pubkey::Pubkey,
                pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for LiquidateDex
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
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_reserve_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_reserve_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_swap_tokens, writer)?;
                    borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for LiquidateDex {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_reserve_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.collateral_reserve,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_reserve_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.dex_swap_tokens,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.dex_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
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
        pub(crate) mod __cpi_client_accounts_liquidate_dex {
            use super::*;
            pub struct LiquidateDex<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_reserve_vault:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_reserve:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_reserve_vault:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_swap_tokens: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dex_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for LiquidateDex<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_reserve_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.collateral_reserve),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_reserve_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.dex_swap_tokens),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.dex_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
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
            impl<'info> anchor_lang::ToAccountInfos<'info> for LiquidateDex<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_reserve,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_reserve_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_reserve,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_reserve_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_swap_tokens,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dex_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        impl<'info> LiquidateDex<'info> {
            fn loan_note_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Burn {
                        to: self.loan_account.clone(),
                        mint: self.loan_note_mint.clone(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn collateral_note_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Burn {
                        to: self.collateral_account.clone(),
                        mint: self.deposit_note_mint.clone(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn _transfer_swapped_token_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.dex_swap_tokens.clone(),
                        to: self.loan_reserve_vault.clone(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            /// Check that the loan/collateral accounts are registered with the obligation
            fn verify_obligation_accounts(&self) -> Result<(), ProgramError> {
                let obligation = self.obligation.load()?;
                if !obligation.has_collateral_custody(self.collateral_account.key)
                    || !obligation.has_loan_custody(self.loan_account.key)
                {
                    ::solana_program::log::sol_log("note accounts don't match the obligation");
                    return Err(ErrorCode::ObligationAccountMismatch.into());
                }
                Ok(())
            }
            /// Ensure an obligation has an unhealthy debt position to allow liquidation
            fn verify_unhealthy(&self) -> Result<(), ProgramError> {
                let mut obligation = self.obligation.load_mut()?;
                let market = self.market.load()?;
                let clock = Clock::get()?;
                obligation.cache_calculations(market.reserves(), clock.slot);
                if obligation.is_healthy(market.reserves(), clock.slot) {
                    ::solana_program::log::sol_log("cannot liquidate a healthy position");
                    return Err(ErrorCode::ObligationHealthy.into());
                }
                Ok(())
            }
        }
        enum SwapKind {
            Buy,
            Sell,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SwapKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&SwapKind::Buy,) => ::core::fmt::Formatter::write_str(f, "Buy"),
                    (&SwapKind::Sell,) => ::core::fmt::Formatter::write_str(f, "Sell"),
                }
            }
        }
        struct SwapPlan {
            /// The total value of collateral that can be sold to bring the
            /// loan back into a healthy position.
            collateral_sellable_value: Number,
            /// The total value that would be repaid to cover the loan position,
            /// which may be less than the total collateral sold due to fees.
            loan_repay_value: Number,
            /// The _actual_ amount of collateral tokens that can be used in
            /// the trade to buy back loaned tokens. This can be less than the
            /// total sellable amount when:
            ///
            ///     * the collateral account being liquidated is of a lesser value
            ///       compared to the overall collateral available within the account
            ///
            ///     * the loan account being liquidated is of a lesser value compared
            ///       to the overall loans on the account.
            collateral_tokens_tradable: Number,
            /// The worst price to accept for the trade
            limit_price: Number,
            /// The kind of trade that should be executed
            kind: SwapKind,
            /// The acceptable slippage for the trade
            slippage: Number,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SwapPlan {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    SwapPlan {
                        collateral_sellable_value: ref __self_0_0,
                        loan_repay_value: ref __self_0_1,
                        collateral_tokens_tradable: ref __self_0_2,
                        limit_price: ref __self_0_3,
                        kind: ref __self_0_4,
                        slippage: ref __self_0_5,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SwapPlan");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "collateral_sellable_value",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "loan_repay_value",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "collateral_tokens_tradable",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "limit_price",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "kind",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "slippage",
                            &&(*__self_0_5),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        struct SwapCalculator<'a, 'info> {
            market: &'a Market,
            loan_reserve: &'a Reserve,
            collateral_reserve: &'a Reserve,
            loan_account: &'a AccountInfo<'info>,
            collateral_account: &'a AccountInfo<'info>,
            obligation: &'a Obligation,
            collateral_reserve_info: &'a CachedReserveInfo,
            loan_reserve_info: &'a CachedReserveInfo,
        }
        impl<'a, 'info> SwapCalculator<'a, 'info> {
            fn new(
                market: &'a Market,
                loan_reserve: &'a Reserve,
                collateral_reserve: &'a Reserve,
                loan_account: &'a AccountInfo<'info>,
                collateral_account: &'a AccountInfo<'info>,
                obligation: &'a Obligation,
            ) -> SwapCalculator<'a, 'info> {
                let clock = Clock::get().unwrap();
                let collateral_reserve_info = market
                    .reserves()
                    .get_cached(collateral_reserve.index, clock.slot);
                let loan_reserve_info =
                    market.reserves().get_cached(loan_reserve.index, clock.slot);
                SwapCalculator {
                    market,
                    loan_reserve,
                    collateral_reserve,
                    loan_account,
                    collateral_account,
                    obligation,
                    collateral_reserve_info,
                    loan_reserve_info,
                }
            }
            fn max_collateral_tradable(
                &self,
                sellable_value: Number,
            ) -> Result<Number, ProgramError> {
                let liquidation_fee =
                    Number::from_bps(self.collateral_reserve.config.liquidation_premium);
                let max_collateral_tokens = sellable_value / self.collateral_reserve_info.price;
                let cur_collateral_tokens = self
                    .collateral_reserve
                    .amount(token::accessor::amount(&self.collateral_account)?)
                    * self.collateral_reserve_info.deposit_note_exchange_rate;
                let loan_value = self
                    .loan_reserve
                    .amount(token::accessor::amount(&self.loan_account)?)
                    * self.loan_reserve_info.loan_note_exchange_rate
                    * self.loan_reserve_info.price;
                let max_sellable_tokens = loan_value * (Number::ONE + liquidation_fee)
                    / self.collateral_reserve_info.price;
                let reserve_sell_limit =
                    match self.collateral_reserve.config.liquidation_dex_trade_max {
                        0 => self.collateral_reserve.amount(std::u64::MAX),
                        n => Number::from(n),
                    };
                let tradable_tokens = [
                    max_collateral_tokens,
                    cur_collateral_tokens,
                    max_sellable_tokens,
                    reserve_sell_limit,
                ]
                .iter()
                .min()
                .cloned()
                .unwrap();
                Ok(tradable_tokens)
            }
            /// Calculate the plan for swapping the collateral for debt
            fn plan(&self) -> Result<SwapPlan, ProgramError> {
                let clock = Clock::get()?;
                let min_c_ratio = Number::from_bps(self.loan_reserve.config.min_collateral_ratio);
                let liquidation_fee =
                    Number::from_bps(self.collateral_reserve.config.liquidation_premium);
                let slippage = liquidation_fee / (Number::ONE + liquidation_fee);
                let collateral_value = self
                    .obligation
                    .collateral_value(self.market.reserves(), clock.slot);
                let loan_value = self
                    .obligation
                    .loan_value(self.market.reserves(), clock.slot);
                let loan_to_value = loan_value / collateral_value;
                let c_ratio_ltv = min_c_ratio * loan_to_value;
                if c_ratio_ltv <= Number::ONE {
                    ::solana_program::log::sol_log(
                        "c_ratio_ltv < 1 implies this cannot be liquidated",
                    );
                    return Err(ErrorCode::ObligationHealthy.into());
                } else if c_ratio_ltv > min_c_ratio {
                    return Err(ErrorCode::Disallowed.into());
                }
                let limit_fraction = (c_ratio_ltv - Number::ONE)
                    / (min_c_ratio * (Number::ONE - liquidation_fee) - Number::ONE);
                let collateral_sellable_value = limit_fraction * collateral_value;
                let loan_repay_value = collateral_sellable_value / (Number::ONE + liquidation_fee);
                let normal_limit_price = (Number::ONE - slippage)
                    * (self.collateral_reserve_info.price / self.loan_reserve_info.price);
                let (kind, limit_price) =
                    if self.loan_reserve.token_mint == self.market.quote_token_mint {
                        (SwapKind::Sell, normal_limit_price)
                    } else if self.collateral_reserve.token_mint == self.market.quote_token_mint {
                        (SwapKind::Buy, Number::ONE / normal_limit_price)
                    } else {
                        ::solana_program::log::sol_log("cannot liquidate these pairs");
                        return Err(ErrorCode::Disallowed.into());
                    };
                let collateral_tokens_tradable =
                    self.max_collateral_tradable(collateral_sellable_value)?;
                Ok(SwapPlan {
                    collateral_sellable_value,
                    collateral_tokens_tradable,
                    loan_repay_value,
                    limit_price,
                    kind,
                    slippage,
                })
            }
        }
        /// Calculate the estimates for swap values
        fn calculate_collateral_swap_plan(
            internal: &LiquidateDex,
        ) -> Result<SwapPlan, ProgramError> {
            let loan_reserve = internal.loan_reserve.load()?;
            let collateral_reserve = internal.collateral_reserve.load()?;
            let obligation = internal.obligation.load()?;
            let market = internal.market.load()?;
            let calculator = SwapCalculator::new(
                &market,
                &loan_reserve,
                &collateral_reserve,
                &internal.loan_account,
                &internal.collateral_account,
                &obligation,
            );
            calculator.plan()
        }
        /// Execute the calculated plan to swap the collateral.
        ///
        /// Returns the number of collateral tokens swapped.
        fn execute_plan<'info>(
            internal: &LiquidateDex<'info>,
            source_dex_market: &DexMarketAccounts<'info>,
            target_dex_market: &DexMarketAccounts<'info>,
            plan: &SwapPlan,
        ) -> Result<(), ProgramError> {
            let market = internal.market.load()?;
            let collateral_reserve = internal.collateral_reserve.load()?;
            let loan_reserve = internal.loan_reserve.load()?;
            let get_dex_client = |dex_market, coin_wallet, pc_wallet| DexClient {
                market: &market,
                market_authority: &internal.market_authority,
                dex_program: &internal.dex_program,
                dex_market,
                order_payer_token_account: &internal.collateral_reserve_vault,
                token_program: &internal.token_program,
                rent: &internal.rent,
                coin_wallet,
                pc_wallet,
            };
            match plan.kind {
                SwapKind::Sell => {
                    let dex_client = get_dex_client(
                        source_dex_market,
                        &internal.collateral_reserve_vault,
                        &internal.loan_reserve_vault,
                    );
                    let limit_price = dex_client.price_lots(
                        plan.limit_price,
                        loan_reserve.exponent,
                        collateral_reserve.exponent,
                    )?;
                    dex_client.sell(
                        limit_price,
                        plan.collateral_tokens_tradable
                            .as_u64_rounded(collateral_reserve.exponent),
                    )?;
                    dex_client.settle()?;
                }
                SwapKind::Buy => {
                    let dex_client = get_dex_client(
                        target_dex_market,
                        &internal.loan_reserve_vault,
                        &internal.collateral_reserve_vault,
                    );
                    let limit_price = dex_client.price_lots(
                        plan.limit_price,
                        collateral_reserve.exponent,
                        loan_reserve.exponent,
                    )?;
                    dex_client.buy(
                        limit_price,
                        plan.collateral_tokens_tradable
                            .as_u64_rounded(collateral_reserve.exponent),
                    )?;
                    dex_client.settle()?;
                }
            }
            Ok(())
        }
        /// Verify that the amount of tokens we received for selling some collateral is acceptable
        fn verify_proceeds(
            internal: &LiquidateDex,
            proceeds: u64,
            collateral_tokens_sold: Number,
            slippage: Number,
        ) -> Result<(), ProgramError> {
            let clock = Clock::get()?;
            let market = internal.market.load()?;
            let collateral_reserve = internal.collateral_reserve.load()?;
            let loan_reserve = internal.loan_reserve.load()?;
            let collateral_info = market
                .reserves()
                .get_cached(collateral_reserve.index, clock.slot);
            let loan_info = market.reserves().get_cached(loan_reserve.index, clock.slot);
            let proceeds_value = loan_info.price * loan_reserve.amount(proceeds);
            let collateral_value = collateral_info.price * collateral_tokens_sold;
            let min_value = collateral_value * (Number::ONE - slippage);
            if proceeds_value < min_value {
                ::solana_program::log::sol_log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["proceeds = ", ", minimum = "],
                        &match (&proceeds_value, &min_value) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ));
                    res
                });
                return Err(ErrorCode::LiquidationSwapSlipped.into());
            }
            Ok(())
        }
        /// Update the internal accounting to reflect the changes in the debt an
        /// collateral positions in the obligation and reserves.
        fn update_accounting(
            internal: &LiquidateDex,
            plan: &SwapPlan,
            proceeds: u64,
            collateral_tokens_sold: Number,
        ) -> Result<(), ProgramError> {
            let clock = Clock::get()?;
            let market = internal.market.load()?;
            let collateral_reserve = internal.collateral_reserve.load()?;
            let mut loan_reserve = internal.loan_reserve.load_mut()?;
            let mut obligation = internal.obligation.load_mut()?;
            let loan_info = market.reserves().get_cached(loan_reserve.index, clock.slot);
            let collateral_info = market
                .reserves()
                .get_cached(collateral_reserve.index, clock.slot);
            let collateral_sell_expected = plan.collateral_sellable_value / collateral_info.price;
            let collateral_repaid_ratio_actual = collateral_tokens_sold / collateral_sell_expected;
            let loan_repaid_value = collateral_repaid_ratio_actual * plan.loan_repay_value;
            let loan_repaid_tokens = loan_repaid_value / loan_info.price;
            let loan_repaid_tokens_u64 = loan_repaid_tokens.as_u64(loan_reserve.exponent);
            let loan_repaid_notes = loan_repaid_tokens / loan_info.deposit_note_exchange_rate;
            let loan_repaid_notes_u64 = loan_repaid_notes.as_u64(loan_reserve.exponent);
            loan_reserve.repay(clock.slot, loan_repaid_tokens_u64, loan_repaid_notes_u64);
            let collateral_notes_sold =
                collateral_tokens_sold / collateral_info.deposit_note_exchange_rate;
            obligation
                .withdraw_collateral(internal.collateral_account.key, collateral_notes_sold)?;
            obligation.repay(internal.loan_account.key, loan_repaid_notes)?;
            token::burn(
                internal
                    .loan_note_burn_context()
                    .with_signer(&[&market.authority_seeds()]),
                loan_repaid_notes.as_u64(loan_reserve.exponent),
            )?;
            token::burn(
                internal
                    .collateral_note_burn_context()
                    .with_signer(&[&market.authority_seeds()]),
                collateral_notes_sold.as_u64(collateral_reserve.exponent),
            )?;
            let fee_proceeds = proceeds.saturating_sub(loan_repaid_tokens_u64);
            loan_reserve.add_uncollected_fees(clock.slot, fee_proceeds);
            Ok(())
        }
        #[inline(never)]
        fn handler<'info>(
            source_market: &DexMarketAccounts<'info>,
            target_market: &DexMarketAccounts<'info>,
            internal: &LiquidateDex<'info>,
        ) -> ProgramResult {
            internal.verify_unhealthy()?;
            internal.verify_obligation_accounts()?;
            ::solana_program::log::sol_log("ready to liquidate");
            let loan_reserve_tokens = token::accessor::amount(&internal.loan_reserve_vault)?;
            let plan = calculate_collateral_swap_plan(internal)?;
            execute_plan(internal, source_market, target_market, &plan)?;
            ::solana_program::log::sol_log("collateral sold");
            let loan_reserve_proceeds = token::accessor::amount(&internal.loan_reserve_vault)?
                .saturating_sub(loan_reserve_tokens);
            verify_proceeds(
                internal,
                loan_reserve_proceeds,
                plan.collateral_tokens_tradable,
                plan.slippage,
            )?;
            ::solana_program::log::sol_log("swap is ok");
            update_accounting(
                internal,
                &plan,
                loan_reserve_proceeds,
                plan.collateral_tokens_tradable,
            )?;
            ::solana_program::log::sol_log("liquidation complete!");
            Ok(())
        }
        /// Somewhat custom handler for the `liquidate_dex` instruction, where we do some setup
        /// work manually that anchor normally would generate automatically. In this case the
        /// generated code has some issues fitting within the stack frame limit, so to workaround
        /// that we just implement it here explicitly for now to ensure it fits within the frame.
        pub fn handler_raw<'info>(
            program_id: &Pubkey,
            accounts: &[AccountInfo<'info>],
            data: &[u8],
        ) -> ProgramResult {
            let mut account_list = accounts;
            ::solana_program::log::sol_log("attempting liquidation");
            let source_market =
                DexMarketAccounts::try_accounts(program_id, &mut account_list, data)?;
            let target_market =
                DexMarketAccounts::try_accounts(program_id, &mut account_list, data)?;
            let liquidation = LiquidateDex::try_accounts(program_id, &mut account_list, data)?;
            handler(&source_market, &target_market, &liquidation)?;
            Ok(())
        }
        pub struct MockLiquidateDex<'info> {
            source_market: DexMarketAccounts<'info>,
            target_market: DexMarketAccounts<'info>,
            to_liquidate: LiquidateDex<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for MockLiquidateDex<'info>
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
                let source_market: DexMarketAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let target_market: DexMarketAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let to_liquidate: LiquidateDex<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                Ok(MockLiquidateDex {
                    source_market,
                    target_market,
                    to_liquidate,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for MockLiquidateDex<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.source_market.to_account_infos());
                account_infos.extend(self.target_market.to_account_infos());
                account_infos.extend(self.to_liquidate.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for MockLiquidateDex<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.source_market.to_account_metas(None));
                account_metas.extend(self.target_market.to_account_metas(None));
                account_metas.extend(self.to_liquidate.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for MockLiquidateDex<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.source_market, program_id)?;
                anchor_lang::AccountsExit::exit(&self.target_market, program_id)?;
                anchor_lang::AccountsExit::exit(&self.to_liquidate, program_id)?;
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
        pub(crate) mod __client_accounts_mock_liquidate_dex {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_liquidate_dex::LiquidateDex;
            pub use __client_accounts_dex_market_accounts::DexMarketAccounts;
            pub struct MockLiquidateDex {
                pub source_market: __client_accounts_dex_market_accounts::DexMarketAccounts,
                pub target_market: __client_accounts_dex_market_accounts::DexMarketAccounts,
                pub to_liquidate: __client_accounts_liquidate_dex::LiquidateDex,
            }
            impl borsh::ser::BorshSerialize for MockLiquidateDex
            where
                __client_accounts_dex_market_accounts::DexMarketAccounts:
                    borsh::ser::BorshSerialize,
                __client_accounts_dex_market_accounts::DexMarketAccounts:
                    borsh::ser::BorshSerialize,
                __client_accounts_liquidate_dex::LiquidateDex: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.source_market, writer)?;
                    borsh::BorshSerialize::serialize(&self.target_market, writer)?;
                    borsh::BorshSerialize::serialize(&self.to_liquidate, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for MockLiquidateDex {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.source_market.to_account_metas(None));
                    account_metas.extend(self.target_market.to_account_metas(None));
                    account_metas.extend(self.to_liquidate.to_account_metas(None));
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
        pub(crate) mod __cpi_client_accounts_mock_liquidate_dex {
            use super::*;
            pub use __cpi_client_accounts_dex_market_accounts::DexMarketAccounts;
            pub use __cpi_client_accounts_liquidate_dex::LiquidateDex;
            pub struct MockLiquidateDex<'info> {
                pub source_market:
                    __cpi_client_accounts_dex_market_accounts::DexMarketAccounts<'info>,
                pub target_market:
                    __cpi_client_accounts_dex_market_accounts::DexMarketAccounts<'info>,
                pub to_liquidate: __cpi_client_accounts_liquidate_dex::LiquidateDex<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for MockLiquidateDex<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.source_market.to_account_metas(None));
                    account_metas.extend(self.target_market.to_account_metas(None));
                    account_metas.extend(self.to_liquidate.to_account_metas(None));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for MockLiquidateDex<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.source_market,
                    ));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.target_market,
                    ));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.to_liquidate,
                    ));
                    account_infos
                }
            }
        }
    }
    pub mod refresh_reserve {
        use anchor_lang::prelude::*;
        use anchor_spl::token::{self, MintTo};
        use jet_math::Number;
        use pyth_client::Price;
        use crate::utils::JobCompletion;
        use crate::{errors::ErrorCode, state::*};
        pub struct RefreshReserve<'info> {
            /// The relevant market this refresh is for
            # [account (mut , has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve being refreshed
            # [account (mut , has_one = market , has_one = fee_note_vault , has_one = pyth_oracle_price)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault for storing collected fees
            #[account(mut)]
            pub fee_note_vault: AccountInfo<'info>,
            /// The reserve's mint for deposit notes
            #[account(mut)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The account containing the price information for the token.
            pub pyth_oracle_price: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for RefreshReserve<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let fee_note_vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let pyth_oracle_price: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if !market.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.fee_note_vault != fee_note_vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.pyth_oracle_price != pyth_oracle_price.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !fee_note_vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(RefreshReserve {
                    market,
                    market_authority,
                    reserve,
                    fee_note_vault,
                    deposit_note_mint,
                    pyth_oracle_price,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for RefreshReserve<'info>
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
                account_infos.extend(self.fee_note_vault.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.pyth_oracle_price.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for RefreshReserve<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.fee_note_vault.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.pyth_oracle_price.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for RefreshReserve<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.market, program_id)?;
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.fee_note_vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
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
        pub(crate) mod __client_accounts_refresh_reserve {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct RefreshReserve {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub fee_note_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub pyth_oracle_price: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for RefreshReserve
            where
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
                    borsh::BorshSerialize::serialize(&self.fee_note_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.pyth_oracle_price, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for RefreshReserve {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.market,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.market_authority,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.fee_note_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pyth_oracle_price,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_refresh_reserve {
            use super::*;
            pub struct RefreshReserve<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub fee_note_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub pyth_oracle_price:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for RefreshReserve<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.market_authority),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.fee_note_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pyth_oracle_price),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for RefreshReserve<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.fee_note_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.pyth_oracle_price,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> RefreshReserve<'info> {
            fn fee_note_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    MintTo {
                        to: self.fee_note_vault.clone(),
                        mint: self.deposit_note_mint.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        pub fn handler(ctx: Context<RefreshReserve>) -> ProgramResult {
            let mut market = ctx.accounts.market.load_mut()?;
            let mut reserve = ctx.accounts.reserve.load_mut()?;
            let oracle_data = ctx.accounts.pyth_oracle_price.try_borrow_data()?;
            let oracle = pyth_client::cast::<Price>(&oracle_data);
            if oracle.agg.price < 0 {
                return Err(ErrorCode::InvalidOraclePrice.into());
            }
            let threshold = Number::from_bps(reserve.config.confidence_threshold)
                * Number::from_decimal(oracle.twap.val, oracle.expo);
            if oracle.agg.conf > threshold.as_u64_ceil(oracle.expo) {
                ::solana_program::log::sol_log("pyth confidence range outside threshold");
                return Err(ErrorCode::InvalidOraclePrice.into());
            }
            let market_reserves = market.reserves_mut();
            let reserve_info = market_reserves.get_mut(reserve.index);
            let clock = Clock::get()?;
            let vault_amount = reserve.total_deposits();
            let deposit_note_mint_supply = reserve.total_deposit_notes();
            let loan_note_mint_supply = reserve.total_loan_notes();
            match reserve.try_accrue_interest(vault_amount, clock.unix_timestamp, clock.slot) {
                JobCompletion::Partial => {
                    ::solana_program::log::sol_log(
                        "performing partial reserve refresh: additional iterations required",
                    );
                    reserve_info.invalidate();
                }
                JobCompletion::Full => {
                    let reserve_cache = reserve_info.get_stale_mut();
                    let deposit_note_exchange_rate = reserve.deposit_note_exchange_rate(
                        clock.slot,
                        vault_amount,
                        deposit_note_mint_supply,
                    );
                    let loan_note_exchange_rate =
                        reserve.loan_note_exchange_rate(clock.slot, loan_note_mint_supply);
                    reserve_cache.price = Number::from_decimal(oracle.agg.price, oracle.expo);
                    reserve_cache.deposit_note_exchange_rate = deposit_note_exchange_rate;
                    reserve_cache.loan_note_exchange_rate = loan_note_exchange_rate;
                    reserve_cache.min_collateral_ratio =
                        Number::from_bps(reserve.config.min_collateral_ratio);
                    reserve_cache.liquidation_bonus = reserve.config.liquidation_premium;
                    reserve_info.refresh_to(clock.slot);
                    let notes_to_mint =
                        reserve.collect_accrued_fees(clock.slot, deposit_note_exchange_rate);
                    if notes_to_mint > 0 {
                        token::mint_to(
                            ctx.accounts
                                .fee_note_mint_context()
                                .with_signer(&[&market.authority_seeds()]),
                            notes_to_mint,
                        )?;
                    }
                    ::solana_program::log::sol_log("reserve refreshed");
                }
            }
            Ok(())
        }
    }
    pub mod repay {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Burn, Transfer};
        use crate::state::*;
        use crate::{Amount, Rounding};
        pub struct RepayEvent {
            borrower: Pubkey,
            reserve: Pubkey,
            amount: Amount,
        }
        impl borsh::ser::BorshSerialize for RepayEvent
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Amount: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.borrower, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for RepayEvent
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Amount: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    borrower: borsh::BorshDeserialize::deserialize(buf)?,
                    reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    amount: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for RepayEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [129, 213, 0, 108, 218, 108, 82, 140].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for RepayEvent {
            fn discriminator() -> [u8; 8] {
                [129, 213, 0, 108, 218, 108, 82, 140]
            }
        }
        pub struct Repay<'info> {
            /// The relevant market this repayment is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The obligation with debt to be repaid
            # [account (mut , has_one = market , constraint = obligation . load () . unwrap () . has_loan_custody (& loan_account . key ()))]
            pub obligation: Loader<'info, Obligation>,
            /// The reserve that the debt is from
            # [account (mut , has_one = market , has_one = vault , has_one = loan_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the payment will be transferred to
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the debt/loan notes
            #[account(mut)]
            pub loan_note_mint: AccountInfo<'info>,
            /// The account that holds the borrower's debt balance
            #[account(mut)]
            pub loan_account: AccountInfo<'info>,
            /// The token account that the payment funds will be transferred from
            #[account(mut)]
            pub payer_account: AccountInfo<'info>,
            /// The account repaying the loan
            #[account(signer)]
            pub payer: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Repay<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let loan_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let payer_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let payer: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !(obligation
                    .load()
                    .unwrap()
                    .has_loan_custody(&loan_account.key()))
                {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.loan_note_mint != loan_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !loan_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !payer_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !payer.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(Repay {
                    market,
                    market_authority,
                    obligation,
                    reserve,
                    vault,
                    loan_note_mint,
                    loan_account,
                    payer_account,
                    payer,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Repay<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.market_authority.to_account_infos());
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.loan_note_mint.to_account_infos());
                account_infos.extend(self.loan_account.to_account_infos());
                account_infos.extend(self.payer_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Repay<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.loan_note_mint.to_account_metas(None));
                account_metas.extend(self.loan_account.to_account_metas(None));
                account_metas.extend(self.payer_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(Some(true)));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Repay<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.loan_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.payer_account, program_id)?;
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
        pub(crate) mod __client_accounts_repay {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct Repay {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub loan_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Repay
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
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.market_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.loan_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Repay {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_note_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.loan_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.payer, true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_repay {
            use super::*;
            pub struct Repay<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_note_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub loan_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Repay<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_note_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.loan_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.payer),
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Repay<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_note_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.loan_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.payer_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        pub trait RepayContext<'info> {
            fn market(&self) -> &Loader<'info, Market>;
            fn market_authority(&self) -> &AccountInfo<'info>;
            fn obligation(&self) -> &Loader<'info, Obligation>;
            fn reserve(&self) -> &Loader<'info, Reserve>;
            fn vault(&self) -> &AccountInfo<'info>;
            fn loan_note_mint(&self) -> &AccountInfo<'info>;
            fn payer(&self) -> &AccountInfo<'info>;
            fn loan_account(&self) -> &AccountInfo<'info>;
            fn payer_account(&self) -> &AccountInfo<'info>;
            fn token_program(&self) -> &AccountInfo<'info>;
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program().clone(),
                    Transfer {
                        from: self.payer_account().to_account_info(),
                        to: self.vault().to_account_info(),
                        authority: self.payer().clone(),
                    },
                )
            }
            fn note_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program().clone(),
                    Burn {
                        to: self.loan_account().to_account_info(),
                        mint: self.loan_note_mint().to_account_info(),
                        authority: self.market_authority().clone(),
                    },
                )
            }
        }
        pub(crate) use implement_repay_context;
        impl<'info> RepayContext<'info> for Repay<'info> {
            fn market(&self) -> &Loader<'info, Market> {
                &self.market
            }
            fn market_authority(&self) -> &AccountInfo<'info> {
                &self.market_authority
            }
            fn obligation(&self) -> &Loader<'info, Obligation> {
                &self.obligation
            }
            fn reserve(&self) -> &Loader<'info, Reserve> {
                &self.reserve
            }
            fn vault(&self) -> &AccountInfo<'info> {
                &self.vault
            }
            fn loan_note_mint(&self) -> &AccountInfo<'info> {
                &self.loan_note_mint
            }
            fn payer(&self) -> &AccountInfo<'info> {
                &self.payer
            }
            fn loan_account(&self) -> &AccountInfo<'info> {
                &self.loan_account
            }
            fn payer_account(&self) -> &AccountInfo<'info> {
                &self.payer_account
            }
            fn token_program(&self) -> &AccountInfo<'info> {
                &self.token_program
            }
        }
        /// Repay tokens for a loan
        pub fn handler(ctx: Context<Repay>, amount: Amount) -> ProgramResult {
            repay(&ctx, amount)?;
            Ok(())
        }
        pub fn repay<'info, T: RepayContext<'info>>(
            ctx: &Context<T>,
            amount: Amount,
        ) -> Result<(), ProgramError> {
            let clock = Clock::get().unwrap();
            let market = ctx.accounts.market().load()?;
            let mut reserve = ctx.accounts.reserve().load_mut()?;
            let mut obligation = ctx.accounts.obligation().load_mut()?;
            let loan_account = ctx.accounts.loan_account();
            let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
            market.verify_ability_repay()?;
            let payoff_notes = amount.as_loan_notes(reserve_info, Rounding::Down)?;
            let payoff_notes = std::cmp::min(payoff_notes, token::accessor::amount(loan_account)?);
            let payoff_tokens = std::cmp::min(
                reserve_info.loan_notes_to_tokens(payoff_notes, Rounding::Up),
                reserve.unwrap_outstanding_debt(clock.slot).as_u64(0),
            );
            token::burn(
                ctx.accounts
                    .note_burn_context()
                    .with_signer(&[&market.authority_seeds()]),
                payoff_notes,
            )?;
            token::transfer(ctx.accounts.transfer_context(), payoff_tokens)?;
            reserve.repay(clock.slot, payoff_tokens, payoff_notes);
            obligation.repay(&loan_account.key(), reserve.amount(payoff_notes))?;
            {
                let data = anchor_lang::Event::data(&RepayEvent {
                    borrower: ctx.accounts.payer().key(),
                    reserve: ctx.accounts.reserve().key(),
                    amount,
                });
                let msg_str = &anchor_lang::__private::base64::encode(data);
                ::solana_program::log::sol_log(msg_str);
            };
            Ok(())
        }
    }
    pub mod update_reserve_config {
        use crate::state::*;
        use anchor_lang::prelude::*;
        pub struct UpdateReserveConfig<'info> {
            # [account (has_one = owner)]
            pub market: Loader<'info, Market>,
            # [account (mut , has_one = market)]
            pub reserve: Loader<'info, Reserve>,
            #[account(signer)]
            pub owner: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for UpdateReserveConfig<'info>
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
                let market: anchor_lang::Loader<Market> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let reserve: anchor_lang::Loader<Reserve> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                Ok(UpdateReserveConfig {
                    market,
                    reserve,
                    owner,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateReserveConfig<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.market.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for UpdateReserveConfig<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for UpdateReserveConfig<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
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
        pub(crate) mod __client_accounts_update_reserve_config {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct UpdateReserveConfig {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for UpdateReserveConfig
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.market, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for UpdateReserveConfig {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
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
        pub(crate) mod __cpi_client_accounts_update_reserve_config {
            use super::*;
            pub struct UpdateReserveConfig<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for UpdateReserveConfig<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateReserveConfig<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos
                }
            }
        }
        pub fn handler(
            ctx: Context<UpdateReserveConfig>,
            new_config: ReserveConfig,
        ) -> ProgramResult {
            let mut reserve = ctx.accounts.reserve.load_mut()?;
            reserve.config = new_config;
            Ok(())
        }
    }
    pub mod withdraw {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Burn, Transfer};
        use crate::state::*;
        use crate::{Amount, Rounding};
        # [instruction (bump : u8)]
        pub struct Withdraw<'info> {
            /// The relevant market this withdraw is for
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve being withdrawn from
            # [account (mut , has_one = market , has_one = vault , has_one = deposit_note_mint)]
            pub reserve: Loader<'info, Reserve>,
            /// The reserve's vault where the withdrawn tokens will be transferred from
            #[account(mut)]
            pub vault: AccountInfo<'info>,
            /// The mint for the deposit notes
            #[account(mut)]
            pub deposit_note_mint: AccountInfo<'info>,
            /// The user/authority that owns the deposit
            #[account(signer)]
            pub depositor: AccountInfo<'info>,
            /// The account that stores the deposit notes
            # [account (mut , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , depositor . key . as_ref ()] , bump = bump)]
            pub deposit_account: AccountInfo<'info>,
            /// The token account where to transfer withdrawn tokens to
            #[account(mut)]
            pub withdraw_account: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Withdraw<'info>
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
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
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
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let vault: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_note_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let depositor: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let withdraw_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !reserve.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.vault != vault.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.deposit_note_mint != deposit_note_mint.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !deposit_note_mint.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !depositor.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        depositor.key.as_ref(),
                        &[bump][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if deposit_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if !withdraw_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(Withdraw {
                    market,
                    market_authority,
                    reserve,
                    vault,
                    deposit_note_mint,
                    depositor,
                    deposit_account,
                    withdraw_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
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
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.deposit_note_mint.to_account_infos());
                account_infos.extend(self.depositor.to_account_infos());
                account_infos.extend(self.deposit_account.to_account_infos());
                account_infos.extend(self.withdraw_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.deposit_note_mint.to_account_metas(None));
                account_metas.extend(self.depositor.to_account_metas(Some(true)));
                account_metas.extend(self.deposit_account.to_account_metas(None));
                account_metas.extend(self.withdraw_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.reserve, program_id)?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_note_mint, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.withdraw_account, program_id)?;
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
        pub(crate) mod __client_accounts_withdraw {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct Withdraw {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_note_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub withdraw_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Withdraw
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
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_note_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.withdraw_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Withdraw {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reserve,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_note_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.depositor,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.withdraw_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_withdraw {
            use super::*;
            pub struct Withdraw<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_note_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub withdraw_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reserve),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_note_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.depositor),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.withdraw_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_note_mint,
                    ));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.withdraw_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Withdraw<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.vault.to_account_info(),
                        to: self.withdraw_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
            fn note_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Burn {
                        to: self.deposit_account.to_account_info(),
                        mint: self.deposit_note_mint.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Withdraw tokens from a reserve
        pub fn handler(ctx: Context<Withdraw>, _bump: u8, amount: Amount) -> ProgramResult {
            let market = ctx.accounts.market.load()?;
            let mut reserve = ctx.accounts.reserve.load_mut()?;
            let clock = Clock::get().unwrap();
            let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
            market.verify_ability_deposit_withdraw()?;
            let token_amount = amount.as_tokens(reserve_info, Rounding::Down);
            let note_amount = amount.as_deposit_notes(reserve_info, Rounding::Up)?;
            reserve.withdraw(token_amount, note_amount);
            token::transfer(
                ctx.accounts
                    .transfer_context()
                    .with_signer(&[&market.authority_seeds()]),
                token_amount,
            )?;
            token::burn(
                ctx.accounts
                    .note_burn_context()
                    .with_signer(&[&market.authority_seeds()]),
                note_amount,
            )?;
            Ok(())
        }
    }
    pub mod withdraw_collateral {
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use anchor_spl::token::{self, Transfer};
        use crate::errors::ErrorCode;
        use crate::state::*;
        use crate::{Amount, Rounding};
        pub struct WithdrawCollateralEvent {
            depositor: Pubkey,
            reserve: Pubkey,
            amount: Amount,
        }
        impl borsh::ser::BorshSerialize for WithdrawCollateralEvent
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Amount: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.depositor, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for WithdrawCollateralEvent
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Amount: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    depositor: borsh::BorshDeserialize::deserialize(buf)?,
                    reserve: borsh::BorshDeserialize::deserialize(buf)?,
                    amount: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for WithdrawCollateralEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [145, 38, 46, 87, 190, 149, 253, 191].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for WithdrawCollateralEvent {
            fn discriminator() -> [u8; 8] {
                [145, 38, 46, 87, 190, 149, 253, 191]
            }
        }
        pub struct WithdrawCollateralBumpSeeds {
            collateral_account: u8,
            deposit_account: u8,
        }
        impl borsh::de::BorshDeserialize for WithdrawCollateralBumpSeeds
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    collateral_account: borsh::BorshDeserialize::deserialize(buf)?,
                    deposit_account: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for WithdrawCollateralBumpSeeds
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                Ok(())
            }
        }
        # [instruction (bump : WithdrawCollateralBumpSeeds)]
        pub struct WithdrawCollateral<'info> {
            /// The relevant market the collateral is in
            # [account (has_one = market_authority)]
            pub market: Loader<'info, Market>,
            /// The market's authority account
            pub market_authority: AccountInfo<'info>,
            /// The reserve associated with the c-tokens that are being withdrawn
            # [account (has_one = market)]
            pub reserve: Loader<'info, Reserve>,
            /// The obligation the collateral is being withdrawn from
            /// todo verify depositor?
            # [account (mut , has_one = market , has_one = owner)]
            pub obligation: Loader<'info, Obligation>,
            /// The user/authority that owns the deposited collateral (depositor)
            #[account(signer)]
            pub owner: AccountInfo<'info>,
            /// The account that stores the user's deposit notes, where
            /// the collateral will be returned to.
            # [account (mut , seeds = [b"deposits" . as_ref () , reserve . key () . as_ref () , owner . key . as_ref ()] , bump = bump . deposit_account)]
            pub deposit_account: AccountInfo<'info>,
            /// The account that contains the collateral to be withdrawn
            # [account (mut , seeds = [b"collateral" . as_ref () , reserve . key () . as_ref () , obligation . key () . as_ref () , owner . key . as_ref ()] , bump = bump . collateral_account)]
            pub collateral_account: AccountInfo<'info>,
            # [account (address = token :: ID)]
            pub token_program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for WithdrawCollateral<'info>
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
                    bump: WithdrawCollateralBumpSeeds,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    WithdrawCollateralBumpSeeds: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.bump, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    WithdrawCollateralBumpSeeds: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
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
                let obligation: anchor_lang::Loader<Obligation> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let deposit_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let collateral_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                let token_program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
                if &market.load()?.market_authority != market_authority.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &reserve.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !obligation.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if &obligation.load()?.market != market.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if &obligation.load()?.owner != owner.to_account_info().key {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
                }
                if !owner.is_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"deposits".as_ref(),
                        reserve.key().as_ref(),
                        owner.key.as_ref(),
                        &[bump.deposit_account][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if deposit_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !deposit_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                let __program_signer = Pubkey::create_program_address(
                    &[
                        b"collateral".as_ref(),
                        reserve.key().as_ref(),
                        obligation.key().as_ref(),
                        owner.key.as_ref(),
                        &[bump.collateral_account][..],
                    ][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if collateral_account.to_account_info().key != &__program_signer {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
                if !collateral_account.to_account_info().is_writable {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
                }
                if token_program.to_account_info().key != &token::ID {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
                }
                Ok(WithdrawCollateral {
                    market,
                    market_authority,
                    reserve,
                    obligation,
                    owner,
                    deposit_account,
                    collateral_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawCollateral<'info>
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
                account_infos.extend(self.obligation.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.deposit_account.to_account_infos());
                account_infos.extend(self.collateral_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawCollateral<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.market.to_account_metas(None));
                account_metas.extend(self.market_authority.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(self.obligation.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(Some(true)));
                account_metas.extend(self.deposit_account.to_account_metas(None));
                account_metas.extend(self.collateral_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for WithdrawCollateral<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
                anchor_lang::AccountsExit::exit(&self.obligation, program_id)?;
                anchor_lang::AccountsExit::exit(&self.deposit_account, program_id)?;
                anchor_lang::AccountsExit::exit(&self.collateral_account, program_id)?;
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
        pub(crate) mod __client_accounts_withdraw_collateral {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub struct WithdrawCollateral {
                pub market: anchor_lang::solana_program::pubkey::Pubkey,
                pub market_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub obligation: anchor_lang::solana_program::pubkey::Pubkey,
                pub owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub deposit_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub collateral_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for WithdrawCollateral
            where
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
                    borsh::BorshSerialize::serialize(&self.obligation, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.deposit_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.collateral_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for WithdrawCollateral {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.obligation,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.owner, true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.deposit_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.collateral_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
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
        pub(crate) mod __cpi_client_accounts_withdraw_collateral {
            use super::*;
            pub struct WithdrawCollateral<'info> {
                pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub market_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub obligation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub deposit_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub collateral_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for WithdrawCollateral<'info> {
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
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.obligation),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.owner),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.deposit_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.collateral_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawCollateral<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.obligation,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.deposit_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.collateral_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> WithdrawCollateral<'info> {
            fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.clone(),
                    Transfer {
                        from: self.collateral_account.to_account_info(),
                        to: self.deposit_account.to_account_info(),
                        authority: self.market_authority.clone(),
                    },
                )
            }
        }
        /// Withdraw reserve notes previously deposited as collateral for an obligation
        pub fn handler(
            ctx: Context<WithdrawCollateral>,
            _bump: WithdrawCollateralBumpSeeds,
            amount: Amount,
        ) -> ProgramResult {
            let market = ctx.accounts.market.load()?;
            let reserve = ctx.accounts.reserve.load()?;
            let clock = Clock::get()?;
            let reserve_info = market.reserves().get_cached(reserve.index, clock.slot);
            market.verify_ability_borrow()?;
            let note_amount = amount.as_deposit_notes(reserve_info, Rounding::Up)?;
            token::transfer(
                ctx.accounts
                    .transfer_context()
                    .with_signer(&[&market.authority_seeds()]),
                note_amount,
            )?;
            let mut obligation = ctx.accounts.obligation.load_mut()?;
            let collateral_account = ctx.accounts.collateral_account.key();
            obligation.withdraw_collateral(&collateral_account, reserve.amount(note_amount))?;
            let clock = Clock::get().unwrap();
            let market_info = market.reserves();
            obligation.cache_calculations(market.reserves(), clock.slot);
            if !obligation.is_healthy(market_info, clock.slot) {
                return Err(ErrorCode::ObligationUnhealthy.into());
            }
            {
                let data = anchor_lang::Event::data(&WithdrawCollateralEvent {
                    depositor: ctx.accounts.owner.key(),
                    reserve: ctx.accounts.reserve.key(),
                    amount,
                });
                let msg_str = &anchor_lang::__private::base64::encode(data);
                ::solana_program::log::sol_log(msg_str);
            };
            Ok(())
        }
    }
    pub use borrow::*;
    pub use close_deposit_account::*;
    pub use deposit::*;
    pub use deposit_collateral::*;
    pub use init_collateral_account::*;
    pub use init_deposit_account::*;
    pub use init_loan_account::*;
    pub use init_market::*;
    pub use init_obligation::*;
    pub use init_reserve::*;
    pub use liquidate::*;
    pub use liquidate_dex::*;
    pub use refresh_reserve::*;
    pub use repay::*;
    pub use set_market_flags::*;
    pub use set_market_owner::*;
    pub use update_reserve_config::*;
    pub use withdraw::*;
    pub use withdraw_collateral::*;
}
pub mod state {
    mod cache {
        use bytemuck::{Pod, Zeroable};
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 16 == std::mem::size_of::<Cache<[u8; 0], 0>>();
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<Cache<[u8; 0], 0>>() % 8;
            ASSERT
        } as usize] = [];
        #[repr(C)]
        pub struct Cache<T, const TTL: u64> {
            /// The value being cached
            value: T,
            /// The last slot that this information was updated in
            last_updated: u64,
            /// Whether the value has been manually invalidated
            invalidated: u8,
            _reserved: [u8; 7],
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::clone::Clone, const TTL: u64> ::core::clone::Clone for Cache<T, TTL> {
            #[inline]
            fn clone(&self) -> Cache<T, TTL> {
                match *self {
                    Cache {
                        value: ref __self_0_0,
                        last_updated: ref __self_0_1,
                        invalidated: ref __self_0_2,
                        _reserved: ref __self_0_3,
                    } => Cache {
                        value: ::core::clone::Clone::clone(&(*__self_0_0)),
                        last_updated: ::core::clone::Clone::clone(&(*__self_0_1)),
                        invalidated: ::core::clone::Clone::clone(&(*__self_0_2)),
                        _reserved: ::core::clone::Clone::clone(&(*__self_0_3)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::marker::Copy, const TTL: u64> ::core::marker::Copy for Cache<T, TTL> {}
        unsafe impl<T, const TTL: u64> Pod for Cache<T, TTL> where T: Pod {}
        unsafe impl<T, const TTL: u64> Zeroable for Cache<T, TTL> where T: Zeroable {}
        /// Store calculated values that can be manually invalidated or expire after some number of slots
        /// Methods expect a "current_slot" argument which should indicate which slot the calculation is
        /// relevant for. This is usually the actual current slot but may be an older slot if the value is
        /// used or calculated for a previous slot, for example a partial refresh of the reserve.
        impl<T, const TTL: u64> Cache<T, TTL> {
            pub fn new(value: T, current_slot: u64) -> Self {
                Self {
                    value,
                    invalidated: 0,
                    last_updated: current_slot,
                    _reserved: [0; 7],
                }
            }
            pub fn validate_fresh(&self, current_slot: u64) -> Result<(), CacheInvalidError> {
                if current_slot - self.last_updated > TTL {
                    return Err(CacheInvalidError::Expired {
                        msg: self.time_msg(current_slot),
                    });
                }
                if current_slot < self.last_updated {
                    return Err(CacheInvalidError::TooNew {
                        msg: self.time_msg(current_slot),
                    });
                }
                if self.invalidated != 0 {
                    return Err(CacheInvalidError::Invalidated);
                }
                Ok(())
            }
            fn time_msg(&self, current_slot: u64) -> String {
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["last_updated = ", ", time_to_live = ", ", current_slot = "],
                        &match (&self.last_updated, &TTL, &current_slot) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Display::fmt),
                            ],
                        },
                    ));
                    res
                }
            }
            /// If the cache is neither expired nor marked invalid, return the value,
            /// otherwise return an error indicating why it is stale
            pub fn try_get(&self, current_slot: u64) -> Result<&T, CacheInvalidError> {
                self.validate_fresh(current_slot)?;
                Ok(&self.value)
            }
            /// If the cache is neither expired nor marked invalid, return the value mutably,
            /// otherwise return an error indicating why it is stale
            pub fn try_get_mut(&mut self, current_slot: u64) -> Result<&mut T, CacheInvalidError> {
                self.validate_fresh(current_slot)?;
                Ok(&mut self.value)
            }
            /// If the cache is neither expired nor marked invalid, return the value,
            /// otherwise panic with an error message describing the item and why it is stale
            pub fn expect(&self, current_slot: u64, description: &str) -> &T {
                self.try_get(current_slot).expect(description)
            }
            /// If the cache is neither expired nor marked invalid, return the value mutably,
            /// otherwise panic with an error message describing the item and why it is stale
            pub fn expect_mut(&mut self, current_slot: u64, description: &str) -> &mut T {
                self.try_get_mut(current_slot).expect(description)
            }
            /// Returns the current value, regardless of whether or not it is stale
            pub fn get_stale(&self) -> &T {
                &self.value
            }
            /// Returns the current value mutably, regardless of whether or not it is stale.
            pub fn get_stale_mut(&mut self) -> &mut T {
                &mut self.value
            }
            /// Replace the current value and reset the state to the current slot
            pub fn refresh_as(&mut self, value: T, current_slot: u64) {
                self.value = value;
                self.invalidated = 0;
                self.last_updated = current_slot;
            }
            /// Marks the data as stale
            pub fn invalidate(&mut self) {
                self.invalidated = 1;
            }
            /// Returns the slot when this data was last updated
            pub fn last_updated(&self) -> u64 {
                self.last_updated
            }
            /// Updates the cache to be valid and at current_slot without mutating the value.
            pub fn refresh(&mut self, current_slot: u64) {
                self.invalidated = 0;
                self.last_updated = current_slot;
            }
            /// Updates the cache to be valid and increments the laste updated slot
            pub fn refresh_additional(&mut self, additional_slots: u64) {
                self.invalidated = 0;
                self.last_updated += additional_slots;
            }
            /// Updates the cache to be valid and increments the laste updated slot
            pub fn refresh_to(&mut self, current_slot: u64) {
                self.invalidated = 0;
                self.last_updated = current_slot;
            }
        }
        pub enum CacheInvalidError {
            /// The cache is too old to use for the current slot.
            Expired { msg: String },
            /// A calculation was attempted for a slot that is too old to use the cache,
            /// since the cache was created more recently than the relevant slot.
            TooNew { msg: String },
            /// The cache has been manually invalidated and may no longer be used.
            Invalidated,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CacheInvalidError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&CacheInvalidError::Expired { msg: ref __self_0 },) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Expired");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "msg",
                            &&(*__self_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                    (&CacheInvalidError::TooNew { msg: ref __self_0 },) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "TooNew");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "msg",
                            &&(*__self_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                    (&CacheInvalidError::Invalidated,) => {
                        ::core::fmt::Formatter::write_str(f, "Invalidated")
                    }
                }
            }
        }
    }
    mod market {
        use std::ops::{Deref, DerefMut};
        use anchor_lang::prelude::*;
        use bytemuck::{Pod, Zeroable};
        use pyth_client::Product;
        use jet_math::Number;
        use jet_proc_macros::assert_size;
        use crate::errors::ErrorCode;
        use crate::utils::{FixedBuf, StoredPubkey};
        use crate::Rounding;
        use super::Cache;
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 12800usize == std::mem::size_of::<Market>();
            ASSERT
        } as usize] = [];
        #[repr(packed)]
        /// Lending market account
        pub struct Market {
            pub version: u32,
            /// The exponent used for quote prices
            pub quote_exponent: i32,
            /// The currency used for quote prices
            pub quote_currency: [u8; 15],
            /// The bump seed value for generating the authority address.
            pub authority_bump_seed: [u8; 1],
            /// The address used as the seed for generating the market authority
            /// address. Typically this is the market account's own address.
            pub authority_seed: Pubkey,
            /// The account derived by the program, which has authority over all
            /// assets in the market.
            pub market_authority: Pubkey,
            /// The account that has authority to make changes to the market
            pub owner: Pubkey,
            /// The mint for the token used to quote the value for reserve assets.
            pub quote_token_mint: Pubkey,
            /// Storage for flags that can be set on the market.
            pub flags: u64,
            /// Unused space before start of reserve list
            _reserved: [u8; 352],
            /// The storage for information on reserves in the market
            reserves: [u8; 12288],
        }
        #[automatically_derived]
        impl Market {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Market {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Market {
            #[inline]
            fn clone(&self) -> Market {
                {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    let _: ::core::clone::AssertParamIsClone<i32>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 15]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 1]>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 352]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 12288]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for Market {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for Market {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for Market {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for Market {
            fn discriminator() -> [u8; 8] {
                [219, 190, 213, 55, 0, 227, 198, 154]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Market {
            fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
                if buf.len() < [219, 190, 213, 55, 0, 227, 198, 154].len() {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[219, 190, 213, 55, 0, 227, 198, 154] != given_disc {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into(),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, ProgramError> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Market {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl Market {
            /// Validate that the oracles for a reserve match
            ///
            /// 1) Checks that the product quote currency matches the one set on
            /// the market.
            ///
            /// 2) Checks that the price account address is the same mentioned in
            /// the product.
            pub fn validate_oracle(
                &self,
                product: &Product,
                price: &Pubkey,
            ) -> Result<(), ErrorCode> {
                let quote_currency = match crate::utils::read_pyth_product_attribute(
                    &product.attr,
                    b"quote_currency",
                ) {
                    None => return Err(ErrorCode::InvalidOracle),
                    Some(name) => std::str::from_utf8(name).unwrap(),
                };
                if self.quote_currency() != quote_currency {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "oracle quote currency doesn\'t match the market\'s: \'",
                                "\' vs \'",
                                "\'",
                            ],
                            &match (&quote_currency, &self.quote_currency()) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    ),
                                    ::core::fmt::ArgumentV1::new(
                                        _args.1,
                                        ::core::fmt::Display::fmt,
                                    ),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(ErrorCode::InvalidOracle);
                }
                if product.px_acc.val[..] != price.to_bytes() {
                    ::solana_program::log::sol_log("oracle product and price accounts don't match");
                    return Err(ErrorCode::InvalidOracle);
                }
                Ok(())
            }
            /// Gets the name of the currency used for quotes
            pub fn quote_currency(&self) -> &str {
                let end = self.quote_currency.iter().position(|c| *c == 0).unwrap();
                std::str::from_utf8(&self.quote_currency[..end]).unwrap()
            }
            /// Gets the authority seeds for signing requests with the
            /// market authority address.
            pub fn authority_seeds(&self) -> [&[u8]; 2] {
                [self.authority_seed.as_ref(), &self.authority_bump_seed]
            }
            pub fn reserves_mut(&mut self) -> &mut MarketReserves {
                bytemuck::from_bytes_mut(&mut self.reserves)
            }
            pub fn reserves(&self) -> &MarketReserves {
                bytemuck::from_bytes(&self.reserves)
            }
            /// Get the current flags set on the market
            pub fn flags(&self) -> MarketFlags {
                MarketFlags::from_bits(self.flags).unwrap()
            }
            /// Set new flags on the market
            pub fn reset_flags(&mut self, flags: MarketFlags) {
                self.flags = flags.bits();
            }
            /// Verify that the market is currently allowing deposits and withdrawls
            pub fn verify_ability_deposit_withdraw(&self) -> Result<(), ErrorCode> {
                if self.flags().contains(MarketFlags::HALT_DEPOSITS) {
                    ::solana_program::log::sol_log(
                        "the market is currently not allowing deposits/withdrawls",
                    );
                    return Err(ErrorCode::MarketHalted);
                }
                Ok(())
            }
            /// Verify that the market is currently allowing changes to borrows
            pub fn verify_ability_borrow(&self) -> Result<(), ErrorCode> {
                if self.flags().contains(MarketFlags::HALT_BORROWS) {
                    ::solana_program::log::sol_log("the market is currently not allowing borrows");
                    return Err(ErrorCode::MarketHalted);
                }
                Ok(())
            }
            /// Verify that the market is currently allowing repayments to loans
            pub fn verify_ability_repay(&self) -> Result<(), ErrorCode> {
                if self.flags().contains(MarketFlags::HALT_REPAYS) {
                    ::solana_program::log::sol_log("the market is currently not allowing borrows");
                    return Err(ErrorCode::MarketHalted);
                }
                Ok(())
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<MarketReserves>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 12288usize == std::mem::size_of::<MarketReserves>();
            ASSERT
        } as usize] = [];
        #[repr(C)]
        pub struct MarketReserves {
            /// Tracks the current prices of the tokens in reserve accounts
            reserve_info: [ReserveInfo; 32],
        }
        const _: fn() = || {
            struct TypeWithoutPadding([u8; ::core::mem::size_of::<[ReserveInfo; 32]>()]);
            let _ = ::core::mem::transmute::<MarketReserves, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<[ReserveInfo; 32]>();
            }
        };
        unsafe impl ::bytemuck::Pod for MarketReserves {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<[ReserveInfo; 32]>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for MarketReserves {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for MarketReserves {
            #[inline]
            fn clone(&self) -> MarketReserves {
                {
                    let _: ::core::clone::AssertParamIsClone<[ReserveInfo; 32]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for MarketReserves {}
        pub type ReserveIndex = u16;
        impl MarketReserves {
            pub fn register(&mut self, reserve: &Pubkey) -> Result<ReserveIndex, ErrorCode> {
                for (index, entry) in self.reserve_info.iter_mut().enumerate() {
                    if entry.reserve != Pubkey::default() {
                        continue;
                    }
                    *entry.reserve = *reserve;
                    return Ok(index as ReserveIndex);
                }
                Err(ErrorCode::NoFreeReserves)
            }
            pub fn remove(&mut self, index: ReserveIndex) {
                self.reserve_info[index as usize] = ReserveInfo::zeroed();
            }
            pub fn get_mut(&mut self, index: ReserveIndex) -> &mut ReserveInfo {
                &mut self.reserve_info[index as usize]
            }
            pub fn get(&self, index: ReserveIndex) -> &ReserveInfo {
                &self.reserve_info[index as usize]
            }
            pub fn get_cached(&self, index: ReserveIndex, current_slot: u64) -> &CachedReserveInfo {
                let entry = self.get(index);
                match entry.cache.try_get(current_slot) {
                    Ok(info) => info,
                    Err(_) => {
                        ::solana_program::log::sol_log(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["reserve ", " is stale in market"],
                                &match (&entry.reserve,) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ));
                            res
                        });
                        ::solana_program::log::sol_log(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["cached_slot = ", ", current_slot = "],
                                &match (&entry.cache.last_updated(), &current_slot) {
                                    _args => [
                                        ::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        });
                        {
                            ::std::rt::begin_panic("market reserve is stale")
                        };
                    }
                }
            }
            pub fn get_cached_mut(
                &mut self,
                index: ReserveIndex,
                current_slot: u64,
            ) -> &mut CachedReserveInfo {
                self.get_mut(index)
                    .cache
                    .expect_mut(current_slot, "reserve market data is stale")
            }
            pub fn iter(&self) -> impl Iterator<Item = &ReserveInfo> {
                self.reserve_info
                    .iter()
                    .take_while(|r| r.reserve != Pubkey::default())
            }
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ReserveInfo> {
                self.reserve_info
                    .iter_mut()
                    .take_while(|r| r.reserve != Pubkey::default())
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<ReserveInfo>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 384usize == std::mem::size_of::<ReserveInfo>();
            ASSERT
        } as usize] = [];
        #[repr(C)]
        pub struct ReserveInfo {
            /// The related reserve account
            pub reserve: StoredPubkey,
            /// Unused space
            _reserved: FixedBuf<80>,
            /// Cached values for the portion of the reserve info that is calculated dynamically
            pub cache: Cache<CachedReserveInfo, 1>,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<StoredPubkey>()
                    + ::core::mem::size_of::<FixedBuf<80>>()
                    + ::core::mem::size_of::<Cache<CachedReserveInfo, 1>>()],
            );
            let _ = ::core::mem::transmute::<ReserveInfo, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<StoredPubkey>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<FixedBuf<80>>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Cache<CachedReserveInfo, 1>>();
            }
        };
        unsafe impl ::bytemuck::Pod for ReserveInfo {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<StoredPubkey>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<FixedBuf<80>>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Cache<CachedReserveInfo, 1>>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for ReserveInfo {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ReserveInfo {
            #[inline]
            fn clone(&self) -> ReserveInfo {
                {
                    let _: ::core::clone::AssertParamIsClone<StoredPubkey>;
                    let _: ::core::clone::AssertParamIsClone<FixedBuf<80>>;
                    let _: ::core::clone::AssertParamIsClone<Cache<CachedReserveInfo, 1>>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ReserveInfo {}
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<CachedReserveInfo>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 256usize == std::mem::size_of::<CachedReserveInfo>();
            ASSERT
        } as usize] = [];
        #[repr(C)]
        pub struct CachedReserveInfo {
            /// The price of the asset being stored in the reserve account.
            /// USD per smallest unit (1u64) of a token
            pub price: Number,
            /// The value of the deposit note (unit: reserve tokens per note token)
            pub deposit_note_exchange_rate: Number,
            /// The value of the loan note (unit: reserve tokens per note token)
            pub loan_note_exchange_rate: Number,
            /// The minimum allowable collateralization ratio for a loan on this reserve
            pub min_collateral_ratio: Number,
            /// The bonus awarded to liquidators when repaying a loan in exchange for a
            /// collateral asset.
            pub liquidation_bonus: u16,
            /// Unused space
            _reserved: FixedBuf<158>,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<FixedBuf<158>>()],
            );
            let _ = ::core::mem::transmute::<CachedReserveInfo, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<FixedBuf<158>>();
            }
        };
        unsafe impl ::bytemuck::Pod for CachedReserveInfo {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<FixedBuf<158>>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for CachedReserveInfo {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CachedReserveInfo {
            #[inline]
            fn clone(&self) -> CachedReserveInfo {
                {
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<FixedBuf<158>>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for CachedReserveInfo {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CachedReserveInfo {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    CachedReserveInfo {
                        price: ref __self_0_0,
                        deposit_note_exchange_rate: ref __self_0_1,
                        loan_note_exchange_rate: ref __self_0_2,
                        min_collateral_ratio: ref __self_0_3,
                        liquidation_bonus: ref __self_0_4,
                        _reserved: ref __self_0_5,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "CachedReserveInfo");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "price",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "deposit_note_exchange_rate",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "loan_note_exchange_rate",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "min_collateral_ratio",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "liquidation_bonus",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "_reserved",
                            &&(*__self_0_5),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl CachedReserveInfo {
            /// USD per smallest unit (1u64) of the deposit note
            pub fn deposit_note_price(&self) -> Number {
                self.deposit_note_exchange_rate * self.price
            }
            /// USD per smallest unit (1u64) of the loan note
            pub fn loan_note_price(&self) -> Number {
                self.loan_note_exchange_rate * self.price
            }
            /// Convert loan notes into the equivalent value of tokens
            pub fn loan_notes_to_tokens(&self, notes: u64, rounding: Rounding) -> u64 {
                let tokens = self.loan_note_exchange_rate * Number::from(notes);
                match rounding {
                    Rounding::Up => tokens.as_u64_ceil(0),
                    Rounding::Down => tokens.as_u64(0),
                }
            }
            /// Convert a token amount into the equivalent value of loan notes
            pub fn loan_notes_from_tokens(&self, tokens: u64, rounding: Rounding) -> u64 {
                let loan_notes = Number::from(tokens) / self.loan_note_exchange_rate;
                match rounding {
                    Rounding::Up => loan_notes.as_u64_ceil(0),
                    Rounding::Down => loan_notes.as_u64(0),
                }
            }
            /// Convert deposit notes into the equivalent value of tokens
            pub fn deposit_notes_to_tokens(&self, notes: u64, rounding: Rounding) -> u64 {
                let tokens = self.deposit_note_exchange_rate * Number::from(notes);
                match rounding {
                    Rounding::Up => tokens.as_u64_ceil(0),
                    Rounding::Down => tokens.as_u64(0),
                }
            }
            /// Convert a token amount into the equivalent value of deposit notes
            pub fn deposit_notes_from_tokens(&self, tokens: u64, rounding: Rounding) -> u64 {
                let deposit_notes = Number::from(tokens) / self.deposit_note_exchange_rate;
                match rounding {
                    Rounding::Up => deposit_notes.as_u64_ceil(0),
                    Rounding::Down => deposit_notes.as_u64(0),
                }
            }
        }
        impl Deref for ReserveInfo {
            type Target = Cache<CachedReserveInfo, 1>;
            fn deref(&self) -> &Self::Target {
                &self.cache
            }
        }
        impl DerefMut for ReserveInfo {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.cache
            }
        }
        impl ReserveInfo {
            pub fn unwrap(&self, current_slot: u64) -> &CachedReserveInfo {
                self.try_get(current_slot).unwrap_or_else(|_| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["reserve "],
                        &match (&self.reserve,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
            pub fn unwrap_mut(&mut self, current_slot: u64) -> &mut CachedReserveInfo {
                let key = self.reserve;
                self.try_get_mut(current_slot).unwrap_or_else(|_| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["reserve "],
                        &match (&key,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
        }
        pub struct MarketFlags {
            bits: u64,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for MarketFlags {}
        impl ::core::marker::StructuralPartialEq for MarketFlags {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for MarketFlags {
            #[inline]
            fn eq(&self, other: &MarketFlags) -> bool {
                match *other {
                    MarketFlags {
                        bits: ref __self_1_0,
                    } => match *self {
                        MarketFlags {
                            bits: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &MarketFlags) -> bool {
                match *other {
                    MarketFlags {
                        bits: ref __self_1_0,
                    } => match *self {
                        MarketFlags {
                            bits: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for MarketFlags {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for MarketFlags {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u64>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for MarketFlags {
            #[inline]
            fn clone(&self) -> MarketFlags {
                {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for MarketFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &MarketFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    MarketFlags {
                        bits: ref __self_1_0,
                    } => match *self {
                        MarketFlags {
                            bits: ref __self_0_0,
                        } => match ::core::cmp::PartialOrd::partial_cmp(
                            &(*__self_0_0),
                            &(*__self_1_0),
                        ) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        },
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for MarketFlags {
            #[inline]
            fn cmp(&self, other: &MarketFlags) -> ::core::cmp::Ordering {
                match *other {
                    MarketFlags {
                        bits: ref __self_1_0,
                    } => match *self {
                        MarketFlags {
                            bits: ref __self_0_0,
                        } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for MarketFlags {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    MarketFlags {
                        bits: ref __self_0_0,
                    } => ::core::hash::Hash::hash(&(*__self_0_0), state),
                }
            }
        }
        impl ::bitflags::_core::fmt::Debug for MarketFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::_core::fmt::Formatter,
            ) -> ::bitflags::_core::fmt::Result {
                #[allow(non_snake_case)]
                trait __BitFlags {
                    #[inline]
                    fn HALT_BORROWS(&self) -> bool {
                        false
                    }
                    #[inline]
                    fn HALT_REPAYS(&self) -> bool {
                        false
                    }
                    #[inline]
                    fn HALT_DEPOSITS(&self) -> bool {
                        false
                    }
                    #[inline]
                    fn HALT_ALL(&self) -> bool {
                        false
                    }
                }
                #[allow(non_snake_case)]
                impl __BitFlags for MarketFlags {
                    #[allow(deprecated)]
                    #[inline]
                    fn HALT_BORROWS(&self) -> bool {
                        if Self::HALT_BORROWS.bits == 0 && self.bits != 0 {
                            false
                        } else {
                            self.bits & Self::HALT_BORROWS.bits == Self::HALT_BORROWS.bits
                        }
                    }
                    #[allow(deprecated)]
                    #[inline]
                    fn HALT_REPAYS(&self) -> bool {
                        if Self::HALT_REPAYS.bits == 0 && self.bits != 0 {
                            false
                        } else {
                            self.bits & Self::HALT_REPAYS.bits == Self::HALT_REPAYS.bits
                        }
                    }
                    #[allow(deprecated)]
                    #[inline]
                    fn HALT_DEPOSITS(&self) -> bool {
                        if Self::HALT_DEPOSITS.bits == 0 && self.bits != 0 {
                            false
                        } else {
                            self.bits & Self::HALT_DEPOSITS.bits == Self::HALT_DEPOSITS.bits
                        }
                    }
                    #[allow(deprecated)]
                    #[inline]
                    fn HALT_ALL(&self) -> bool {
                        if Self::HALT_ALL.bits == 0 && self.bits != 0 {
                            false
                        } else {
                            self.bits & Self::HALT_ALL.bits == Self::HALT_ALL.bits
                        }
                    }
                }
                let mut first = true;
                if <Self as __BitFlags>::HALT_BORROWS(self) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("HALT_BORROWS")?;
                }
                if <Self as __BitFlags>::HALT_REPAYS(self) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("HALT_REPAYS")?;
                }
                if <Self as __BitFlags>::HALT_DEPOSITS(self) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("HALT_DEPOSITS")?;
                }
                if <Self as __BitFlags>::HALT_ALL(self) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("HALT_ALL")?;
                }
                let extra_bits = self.bits & !Self::all().bits();
                if extra_bits != 0 {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("0x")?;
                    ::bitflags::_core::fmt::LowerHex::fmt(&extra_bits, f)?;
                }
                if first {
                    f.write_str("(empty)")?;
                }
                Ok(())
            }
        }
        impl ::bitflags::_core::fmt::Binary for MarketFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::_core::fmt::Formatter,
            ) -> ::bitflags::_core::fmt::Result {
                ::bitflags::_core::fmt::Binary::fmt(&self.bits, f)
            }
        }
        impl ::bitflags::_core::fmt::Octal for MarketFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::_core::fmt::Formatter,
            ) -> ::bitflags::_core::fmt::Result {
                ::bitflags::_core::fmt::Octal::fmt(&self.bits, f)
            }
        }
        impl ::bitflags::_core::fmt::LowerHex for MarketFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::_core::fmt::Formatter,
            ) -> ::bitflags::_core::fmt::Result {
                ::bitflags::_core::fmt::LowerHex::fmt(&self.bits, f)
            }
        }
        impl ::bitflags::_core::fmt::UpperHex for MarketFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::_core::fmt::Formatter,
            ) -> ::bitflags::_core::fmt::Result {
                ::bitflags::_core::fmt::UpperHex::fmt(&self.bits, f)
            }
        }
        #[allow(dead_code)]
        impl MarketFlags {
            /// Disable all borrowing and collateral withdrawls
            pub const HALT_BORROWS: Self = Self { bits: 1 << 0 };
            /// Disable repaying loans
            pub const HALT_REPAYS: Self = Self { bits: 1 << 1 };
            /// Disable deposits + withdrawls
            pub const HALT_DEPOSITS: Self = Self { bits: 1 << 2 };
            /// Disable all operations
            pub const HALT_ALL: Self = Self {
                bits: Self::HALT_BORROWS.bits | Self::HALT_REPAYS.bits | Self::HALT_DEPOSITS.bits,
            };
            /// Returns an empty set of flags.
            #[inline]
            pub const fn empty() -> Self {
                Self { bits: 0 }
            }
            /// Returns the set containing all flags.
            #[inline]
            pub const fn all() -> Self {
                #[allow(non_snake_case)]
                trait __BitFlags {
                    const HALT_BORROWS: u64 = 0;
                    const HALT_REPAYS: u64 = 0;
                    const HALT_DEPOSITS: u64 = 0;
                    const HALT_ALL: u64 = 0;
                }
                #[allow(non_snake_case)]
                impl __BitFlags for MarketFlags {
                    #[allow(deprecated)]
                    const HALT_BORROWS: u64 = Self::HALT_BORROWS.bits;
                    #[allow(deprecated)]
                    const HALT_REPAYS: u64 = Self::HALT_REPAYS.bits;
                    #[allow(deprecated)]
                    const HALT_DEPOSITS: u64 = Self::HALT_DEPOSITS.bits;
                    #[allow(deprecated)]
                    const HALT_ALL: u64 = Self::HALT_ALL.bits;
                }
                Self {
                    bits: <Self as __BitFlags>::HALT_BORROWS
                        | <Self as __BitFlags>::HALT_REPAYS
                        | <Self as __BitFlags>::HALT_DEPOSITS
                        | <Self as __BitFlags>::HALT_ALL,
                }
            }
            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub const fn bits(&self) -> u64 {
                self.bits
            }
            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub const fn from_bits(bits: u64) -> ::bitflags::_core::option::Option<Self> {
                if (bits & !Self::all().bits()) == 0 {
                    ::bitflags::_core::option::Option::Some(Self { bits })
                } else {
                    ::bitflags::_core::option::Option::None
                }
            }
            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub const fn from_bits_truncate(bits: u64) -> Self {
                Self {
                    bits: bits & Self::all().bits,
                }
            }
            /// Convert from underlying bit representation, preserving all
            /// bits (even those not corresponding to a defined flag).
            ///
            /// # Safety
            ///
            /// The caller of the `bitflags!` macro can chose to allow or
            /// disallow extra bits for their bitflags type.
            ///
            /// The caller of `from_bits_unchecked()` has to ensure that
            /// all bits correspond to a defined flag or that extra bits
            /// are valid for this bitflags type.
            #[inline]
            pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
                Self { bits }
            }
            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.bits() == Self::empty().bits()
            }
            /// Returns `true` if all flags are currently set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().bits | self.bits == self.bits
            }
            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                !(Self {
                    bits: self.bits & other.bits,
                })
                .is_empty()
            }
            /// Returns `true` if all of the flags in `other` are contained within `self`.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }
            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.bits |= other.bits;
            }
            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.bits &= !other.bits;
            }
            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
            /// Returns the intersection between the flags in `self` and
            /// `other`.
            ///
            /// Specifically, the returned set contains only the flags which are
            /// present in *both* `self` *and* `other`.
            ///
            /// This is equivalent to using the `&` operator (e.g.
            /// [`ops::BitAnd`]), as in `flags & other`.
            ///
            /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self {
                    bits: self.bits & other.bits,
                }
            }
            /// Returns the union of between the flags in `self` and `other`.
            ///
            /// Specifically, the returned set contains all flags which are
            /// present in *either* `self` *or* `other`, including any which are
            /// present in both (see [`Self::symmetric_difference`] if that
            /// is undesirable).
            ///
            /// This is equivalent to using the `|` operator (e.g.
            /// [`ops::BitOr`]), as in `flags | other`.
            ///
            /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self {
                    bits: self.bits | other.bits,
                }
            }
            /// Returns the difference between the flags in `self` and `other`.
            ///
            /// Specifically, the returned set contains all flags present in
            /// `self`, except for the ones present in `other`.
            ///
            /// It is also conceptually equivalent to the "bit-clear" operation:
            /// `flags & !other` (and this syntax is also supported).
            ///
            /// This is equivalent to using the `-` operator (e.g.
            /// [`ops::Sub`]), as in `flags - other`.
            ///
            /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self {
                    bits: self.bits & !other.bits,
                }
            }
            /// Returns the [symmetric difference][sym-diff] between the flags
            /// in `self` and `other`.
            ///
            /// Specifically, the returned set contains the flags present which
            /// are present in `self` or `other`, but that are not present in
            /// both. Equivalently, it contains the flags present in *exactly
            /// one* of the sets `self` and `other`.
            ///
            /// This is equivalent to using the `^` operator (e.g.
            /// [`ops::BitXor`]), as in `flags ^ other`.
            ///
            /// [sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
            /// [`ops::BitXor`]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self {
                    bits: self.bits ^ other.bits,
                }
            }
            /// Returns the complement of this set of flags.
            ///
            /// Specifically, the returned set contains all the flags which are
            /// not set in `self`, but which are allowed for this type.
            ///
            /// Alternatively, it can be thought of as the set difference
            /// between [`Self::all()`] and `self` (e.g. `Self::all() - self`)
            ///
            /// This is equivalent to using the `!` operator (e.g.
            /// [`ops::Not`]), as in `!flags`.
            ///
            /// [`Self::all()`]: Self::all
            /// [`ops::Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.bits)
            }
        }
        impl ::bitflags::_core::ops::BitOr for MarketFlags {
            type Output = Self;
            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: MarketFlags) -> Self {
                Self {
                    bits: self.bits | other.bits,
                }
            }
        }
        impl ::bitflags::_core::ops::BitOrAssign for MarketFlags {
            /// Adds the set of flags.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.bits |= other.bits;
            }
        }
        impl ::bitflags::_core::ops::BitXor for MarketFlags {
            type Output = Self;
            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                Self {
                    bits: self.bits ^ other.bits,
                }
            }
        }
        impl ::bitflags::_core::ops::BitXorAssign for MarketFlags {
            /// Toggles the set of flags.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
        }
        impl ::bitflags::_core::ops::BitAnd for MarketFlags {
            type Output = Self;
            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self {
                    bits: self.bits & other.bits,
                }
            }
        }
        impl ::bitflags::_core::ops::BitAndAssign for MarketFlags {
            /// Disables all flags disabled in the set.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.bits &= other.bits;
            }
        }
        impl ::bitflags::_core::ops::Sub for MarketFlags {
            type Output = Self;
            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: Self) -> Self {
                Self {
                    bits: self.bits & !other.bits,
                }
            }
        }
        impl ::bitflags::_core::ops::SubAssign for MarketFlags {
            /// Disables all flags enabled in the set.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.bits &= !other.bits;
            }
        }
        impl ::bitflags::_core::ops::Not for MarketFlags {
            type Output = Self;
            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> Self {
                Self { bits: !self.bits } & Self::all()
            }
        }
        impl ::bitflags::_core::iter::Extend<MarketFlags> for MarketFlags {
            fn extend<T: ::bitflags::_core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::_core::iter::FromIterator<MarketFlags> for MarketFlags {
            fn from_iter<T: ::bitflags::_core::iter::IntoIterator<Item = Self>>(
                iterator: T,
            ) -> Self {
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl std::fmt::Debug for ReserveInfo {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let cached = self.cache.get_stale();
                f.debug_struct("Reserve")
                    .field("address", &self.reserve)
                    .field("price", &cached.price.to_string())
                    .field(
                        "deposit_note_exchange_rate",
                        &cached.deposit_note_exchange_rate.to_string(),
                    )
                    .field(
                        "loan_note_exchange_rate",
                        &cached.loan_note_exchange_rate.to_string(),
                    )
                    .finish()
            }
        }
        impl std::fmt::Debug for Market {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct("Market")
                    .field("quote_currency", &self.quote_currency())
                    .field("quote_mint", &self.quote_token_mint)
                    .field("market_authority", &self.market_authority)
                    .field("market_owner", &self.owner)
                    .field("reserves", &self.reserves().iter().collect::<Vec<_>>())
                    .finish()
            }
        }
    }
    mod obligation {
        use std::fmt::Debug;
        use anchor_lang::prelude::*;
        use anchor_lang::Key;
        use bytemuck::{Contiguous, Pod, Zeroable};
        use jet_math::Number;
        use jet_proc_macros::assert_size;
        use crate::errors::ErrorCode;
        use crate::state::{CachedReserveInfo, ReserveIndex};
        use crate::utils::{FixedBuf, StoredPubkey};
        use super::Cache;
        use super::Market;
        use super::MarketReserves;
        /// Limit the total positions that can be registered on an obligation
        const MAX_OBLIGATION_POSITIONS: usize = 11;
        /// The minimum quote value for an obligation to require partial liquidations,
        /// if an obligation has a lower value then it can be fully liquidated when below
        /// the minimum collateralization ratio.
        const MIN_PARTIAL_LIQUIDATION_VALUE: u64 = 10;
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 4608usize == std::mem::size_of::<Obligation>();
            ASSERT
        } as usize] = [];
        #[repr(packed)]
        /// Tracks information about a user's obligation to repay a borrowed position.
        pub struct Obligation {
            pub version: u32,
            pub _reserved0: u32,
            /// The market this obligation is a part of
            pub market: Pubkey,
            /// The address that owns the debt/assets as a part of this obligation
            pub owner: Pubkey,
            /// Unused space before start of collateral info
            pub _reserved1: [u8; 184],
            /// The storage for cached calculations
            pub cached: [u8; 256],
            /// The storage for the information on positions owed by this obligation
            pub collateral: [u8; 2048],
            /// The storage for the information on positions owed by this obligation
            pub loans: [u8; 2048],
        }
        #[automatically_derived]
        impl Obligation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Obligation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Obligation {
            #[inline]
            fn clone(&self) -> Obligation {
                {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 184]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 256]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 2048]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 2048]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for Obligation {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for Obligation {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for Obligation {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for Obligation {
            fn discriminator() -> [u8; 8] {
                [168, 206, 141, 106, 88, 76, 172, 167]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Obligation {
            fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
                if buf.len() < [168, 206, 141, 106, 88, 76, 172, 167].len() {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[168, 206, 141, 106, 88, 76, 172, 167] != given_disc {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into(),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, ProgramError> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Obligation {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl Obligation {
            pub fn register_collateral(
                &mut self,
                account: &Pubkey,
                reserve_index: ReserveIndex,
            ) -> Result<(), ErrorCode> {
                if self.position_count() >= MAX_OBLIGATION_POSITIONS {
                    return Err(ErrorCode::NoFreeObligation);
                }
                self.collateral_mut().register(Position::new(
                    Side::Collateral,
                    *account,
                    reserve_index,
                ))
            }
            pub fn register_loan(
                &mut self,
                account: &Pubkey,
                reserve_index: ReserveIndex,
            ) -> Result<(), ErrorCode> {
                if self.position_count() >= MAX_OBLIGATION_POSITIONS {
                    return Err(ErrorCode::NoFreeObligation);
                }
                self.loans_mut()
                    .register(Position::new(Side::Loan, *account, reserve_index))
            }
            /// Record the collateral deposited for an obligation
            pub fn deposit_collateral(
                &mut self,
                collateral_account: &Pubkey,
                deposit_notes_amount: Number,
            ) -> ProgramResult {
                self.cached_mut().invalidate();
                self.collateral_mut()
                    .add(collateral_account, deposit_notes_amount)
            }
            /// Record the collateral withdrawn for an obligation
            pub fn withdraw_collateral(
                &mut self,
                collateral_account: &Pubkey,
                deposit_notes_amount: Number,
            ) -> ProgramResult {
                self.cached_mut().invalidate();
                self.collateral_mut()
                    .subtract(collateral_account, deposit_notes_amount)
            }
            /// Record the loan borrowed from an obligation (borrow notes deposited)
            pub fn borrow(
                &mut self,
                loan_account: &Pubkey,
                loan_notes_amount: Number,
            ) -> ProgramResult {
                self.cached_mut().invalidate();
                self.loans_mut().add(loan_account, loan_notes_amount)
            }
            /// Record the loan repaid from an obligation (borrow notes burned)
            pub fn repay(
                &mut self,
                loan_account: &Pubkey,
                loan_notes_amount: Number,
            ) -> ProgramResult {
                self.cached_mut().invalidate();
                self.loans_mut().subtract(loan_account, loan_notes_amount)
            }
            /// Be smarter about compute
            pub fn cache_calculations(&mut self, market: &MarketReserves, current_slot: u64) {
                let loans: &ObligationSide = bytemuck::from_bytes(&self.loans);
                let collateral: &ObligationSide = bytemuck::from_bytes(&self.collateral);
                let cached: &mut CalculationCache = bytemuck::from_bytes_mut(&mut self.cached);
                cached.refresh(current_slot);
                let values = cached.get_stale_mut();
                values.loan_value = loans._market_value(market, current_slot);
                values.collateral_value = collateral._market_value(market, current_slot);
            }
            /// Determine if the obligation is healthy, or otherwise unhealthy and
            /// at risk of liquidation.
            pub fn is_healthy(&self, market: &MarketReserves, current_slot: u64) -> bool {
                let max_min_c_ratio: Number;
                let _max_min_c_ratio = self
                    .loans()
                    .positions
                    .iter()
                    .take_while(|p| p.account != Pubkey::default())
                    .map(|p| {
                        market
                            .get_cached(p.reserve_index, current_slot)
                            .min_collateral_ratio
                    })
                    .max();
                if let Some(c) = _max_min_c_ratio {
                    max_min_c_ratio = c;
                } else {
                    return true;
                }
                let cached: &CalculationCache = bytemuck::from_bytes(&self.cached);
                let cache_values = cached.expect(current_slot, "calculations not performed");
                let min_collateral_value = cache_values.loan_value * max_min_c_ratio;
                min_collateral_value <= cache_values.collateral_value
            }
            /// Liquidate a loan on this obligation
            ///
            /// Returns the number of collateral notes that the liquidator should
            /// receive in return for paying off the loan.
            pub fn liquidate(
                &mut self,
                market: &MarketReserves,
                current_slot: u64,
                collateral_account: &Pubkey,
                loan_account: &Pubkey,
                repay_notes_amount: Number,
            ) -> Result<Number, ErrorCode> {
                let loan_total = self.loan_value(market, current_slot);
                let loan = self.loans().position(loan_account)?;
                let loan_reserve = market.get_cached(loan.reserve_index, current_slot);
                let collateral_total = self.collateral_value(market, current_slot);
                let collateral = self.collateral_mut().position(collateral_account)?;
                let collateral_reserve = market.get_cached(collateral.reserve_index, current_slot);
                let repaid_value =
                    repay_notes_amount * loan_reserve.loan_note_exchange_rate * loan_reserve.price;
                let repaid_ratio = repaid_value / loan_total;
                let min_c_ratio = loan_reserve.min_collateral_ratio;
                let liquidation_bonus = Number::from_bps(collateral_reserve.liquidation_bonus);
                let loan_to_value = loan_total / collateral_total;
                let c_ratio_ltv = min_c_ratio * loan_to_value;
                let collateral_max_value = if c_ratio_ltv < Number::ONE {
                    ::solana_program::log::sol_log(
                        "c_ratio_ltv < 1 implies this cannot be liquidated",
                    );
                    return Err(ErrorCode::ObligationHealthy);
                } else {
                    collateral_total * repaid_ratio
                };
                let limit_fraction = (c_ratio_ltv - Number::ONE)
                    / (min_c_ratio * (Number::ONE - liquidation_bonus) - Number::ONE);
                let collateral_sellable_value = std::cmp::min(
                    (Number::ONE + liquidation_bonus) * repaid_value,
                    limit_fraction * collateral_total,
                );
                let collateral_sellable_value = std::cmp::max(
                    collateral_sellable_value,
                    Number::from(MIN_PARTIAL_LIQUIDATION_VALUE),
                );
                let collateral_max_value =
                    std::cmp::min(collateral_max_value, collateral_sellable_value);
                let collateral_max_notes = collateral_max_value
                    / collateral_reserve.price
                    / collateral_reserve.deposit_note_exchange_rate;
                let collateral_max_notes = std::cmp::min(collateral_max_notes, collateral.amount);
                let collateral = self.collateral_mut().position_mut(collateral_account)?;
                collateral.amount = collateral.amount.saturating_sub(collateral_max_notes);
                Ok(collateral_max_notes)
            }
            /// Determine if this obligation has a custody over some account,
            /// by checking if its in the list of registered accounts.
            pub fn has_collateral_custody(&self, account: &Pubkey) -> bool {
                self.collateral()
                    .positions
                    .iter()
                    .any(|p| p.account.as_ref() == account)
            }
            /// Determine if this obligation has a custody over some account,
            /// by checking if its in the list of registered accounts.
            pub fn has_loan_custody(&self, account: &Pubkey) -> bool {
                self.loans()
                    .positions
                    .iter()
                    .any(|p| p.account.as_ref() == account)
            }
            /// Determine if the reserve matches the collateral
            pub fn is_collateral_reserve(
                &self,
                market: &Market,
                collateral: &Pubkey,
                reserve: &Pubkey,
            ) -> bool {
                self.collateral().positions.iter().any(|p| {
                    p.account.as_ref() == collateral
                        && market.reserves().iter().enumerate().any(|(index, r)| {
                            *r.reserve == *reserve && (index as u16) == p.reserve_index
                        })
                })
            }
            pub fn collateral_value(&self, market: &MarketReserves, current_slot: u64) -> Number {
                if let Ok(values) = self.cached().try_get(current_slot) {
                    return values.collateral_value;
                }
                self.collateral()._market_value(market, current_slot)
            }
            pub fn loan_value(&self, market: &MarketReserves, current_slot: u64) -> Number {
                if let Ok(values) = self.cached().try_get(current_slot) {
                    return values.loan_value;
                }
                self.loans()._market_value(market, current_slot)
            }
            fn position_count(&self) -> usize {
                let collaterals = self
                    .collateral()
                    .positions
                    .iter()
                    .take_while(|p| p.account != Pubkey::default())
                    .count();
                let loans = self
                    .loans()
                    .positions
                    .iter()
                    .take_while(|p| p.account != Pubkey::default())
                    .count();
                loans + collaterals
            }
            fn cached(&self) -> &CalculationCache {
                bytemuck::from_bytes(&self.cached)
            }
            fn cached_mut(&mut self) -> &mut CalculationCache {
                bytemuck::from_bytes_mut(&mut self.cached)
            }
            pub fn collateral(&self) -> &ObligationSide {
                bytemuck::from_bytes(&self.collateral)
            }
            pub fn loans(&self) -> &ObligationSide {
                bytemuck::from_bytes(&self.loans)
            }
            fn collateral_mut(&mut self) -> &mut ObligationSide {
                bytemuck::from_bytes_mut(&mut self.collateral)
            }
            fn loans_mut(&mut self) -> &mut ObligationSide {
                bytemuck::from_bytes_mut(&mut self.loans)
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 240usize == std::mem::size_of::<CalculationCacheInner>();
            ASSERT
        } as usize] = [];
        #[repr(C)]
        struct CalculationCacheInner {
            collateral_value: Number,
            loan_value: Number,
            _reserved: FixedBuf<192>,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<FixedBuf<192>>()],
            );
            let _ = ::core::mem::transmute::<CalculationCacheInner, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<FixedBuf<192>>();
            }
        };
        unsafe impl ::bytemuck::Pod for CalculationCacheInner {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<FixedBuf<192>>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for CalculationCacheInner {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CalculationCacheInner {
            #[inline]
            fn clone(&self) -> CalculationCacheInner {
                {
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<FixedBuf<192>>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for CalculationCacheInner {}
        type CalculationCache = Cache<CalculationCacheInner, 0>;
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 4usize == std::mem::size_of::<Side>();
            ASSERT
        } as usize] = [];
        #[repr(u32)]
        enum Side {
            Collateral,
            Loan,
        }
        unsafe impl ::bytemuck::Contiguous for Side {
            type Int = u32;
            const MIN_VALUE: u32 = 0;
            const MAX_VALUE: u32 = 1;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Side {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Side::Collateral,) => ::core::fmt::Formatter::write_str(f, "Collateral"),
                    (&Side::Loan,) => ::core::fmt::Formatter::write_str(f, "Loan"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Side {
            #[inline]
            fn clone(&self) -> Side {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Side {}
        impl ::core::marker::StructuralEq for Side {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for Side {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        impl ::core::marker::StructuralPartialEq for Side {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Side {
            #[inline]
            fn eq(&self, other: &Side) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<ObligationSide>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 2048usize == std::mem::size_of::<ObligationSide>();
            ASSERT
        } as usize] = [];
        /// Tracks information about the collateral deposited towards an obligation
        #[repr(C)]
        pub struct ObligationSide {
            positions: [Position; 16],
        }
        const _: fn() = || {
            struct TypeWithoutPadding([u8; ::core::mem::size_of::<[Position; 16]>()]);
            let _ = ::core::mem::transmute::<ObligationSide, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<[Position; 16]>();
            }
        };
        unsafe impl ::bytemuck::Pod for ObligationSide {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<[Position; 16]>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for ObligationSide {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ObligationSide {
            #[inline]
            fn clone(&self) -> ObligationSide {
                {
                    let _: ::core::clone::AssertParamIsClone<[Position; 16]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ObligationSide {}
        impl ObligationSide {
            /// Register a position for this obligation (account which holds loan or collateral notes)
            fn register(&mut self, new: Position) -> Result<(), ErrorCode> {
                for position in self.positions.iter_mut() {
                    if position.account == new.account.key() {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                &[
                                    "Cannot register account ",
                                    " as ",
                                    " for reserve index ",
                                    " since it is already registered with ",
                                    " for this obligation",
                                ],
                                &match (&new.account, &new.side, &position.reserve_index, &position)
                                {
                                    _args => [
                                        ::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.1,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.2,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.3,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                    ],
                                },
                            ))
                        };
                    }
                    if position.reserve_index == new.reserve_index
                        && position.account != Pubkey::default()
                    {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                &[
                                    "Cannot register account ",
                                    " as ",
                                    " for reserve index ",
                                    " since the reserve index is already registered with ",
                                    " for this obligation",
                                ],
                                &match (&new.account, &new.side, &position.reserve_index, &position)
                                {
                                    _args => [
                                        ::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.1,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.2,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.3,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                    ],
                                },
                            ))
                        };
                    }
                    if position.account != Pubkey::default() {
                        continue;
                    }
                    *position = new;
                    return Ok(());
                }
                Err(ErrorCode::NoFreeObligation)
            }
            /// Record the loan borrowed from an obligation (borrow notes deposited)
            fn add(&mut self, account: &Pubkey, notes_amount: Number) -> ProgramResult {
                let position = self.position_mut(account)?;
                position.amount += notes_amount;
                Ok(())
            }
            /// Record the loan repaid from an obligation (borrow notes burned)
            fn subtract(&mut self, account: &Pubkey, notes_amount: Number) -> ProgramResult {
                let position = self.position_mut(account)?;
                position.amount = position.amount.saturating_sub(notes_amount);
                Ok(())
            }
            pub fn position(&self, account: &Pubkey) -> Result<&Position, ErrorCode> {
                let position = self
                    .positions
                    .iter()
                    .find(|p| p.account == *account)
                    .ok_or(ErrorCode::UnregisteredPosition)?;
                Ok(position)
            }
            fn position_mut(&mut self, account: &Pubkey) -> Result<&mut Position, ErrorCode> {
                let position = self
                    .positions
                    .iter_mut()
                    .find(|p| p.account == *account)
                    .ok_or(ErrorCode::UnregisteredPosition)?;
                Ok(position)
            }
            pub fn market_value(
                &self,
                market_info: &MarketReserves,
                current_slot: u64,
            ) -> PositionValue {
                let mut value = PositionValue::zeroed();
                for position in self.positions.iter() {
                    if position.account == Pubkey::default() {
                        break;
                    }
                    let reserve = market_info.get(position.reserve_index).unwrap(current_slot);
                    let position_value = position.market_value(reserve);
                    value.market_value += position_value.market_value;
                    value.complementary_limit += position_value.complementary_limit;
                }
                value
            }
            fn _market_value(&self, market: &MarketReserves, current_slot: u64) -> Number {
                let mut value = Number::ZERO;
                for pos in &self.positions {
                    if pos.account == Pubkey::default() {
                        break;
                    }
                    let reserve = market.get_cached(pos.reserve_index, current_slot);
                    value += pos._market_value(reserve);
                }
                value
            }
            pub fn iter(&self) -> impl Iterator<Item = &Position> {
                self.positions
                    .iter()
                    .take_while(|p| p.account != Pubkey::default())
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<Position>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 128usize == std::mem::size_of::<Position>();
            ASSERT
        } as usize] = [];
        /// Information about a single collateral or loan account registered with an obligation
        #[repr(C)]
        pub struct Position {
            /// The token account holding the bank notes
            pub account: StoredPubkey,
            /// Non-authoritative number of bank notes placed in the account
            pub amount: Number,
            pub side: u32,
            /// The index of the reserve that this position's assets are from
            pub reserve_index: ReserveIndex,
            _reserved: FixedBuf<66>,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<StoredPubkey>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<u32>()
                    + ::core::mem::size_of::<ReserveIndex>()
                    + ::core::mem::size_of::<FixedBuf<66>>()],
            );
            let _ = ::core::mem::transmute::<Position, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<StoredPubkey>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u32>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<ReserveIndex>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<FixedBuf<66>>();
            }
        };
        unsafe impl ::bytemuck::Pod for Position {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<StoredPubkey>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u32>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<ReserveIndex>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<FixedBuf<66>>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for Position {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Position {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Position {
                        account: ref __self_0_0,
                        amount: ref __self_0_1,
                        side: ref __self_0_2,
                        reserve_index: ref __self_0_3,
                        _reserved: ref __self_0_4,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Position");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "amount",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "side",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reserve_index",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "_reserved",
                            &&(*__self_0_4),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Position {
            #[inline]
            fn clone(&self) -> Position {
                {
                    let _: ::core::clone::AssertParamIsClone<StoredPubkey>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    let _: ::core::clone::AssertParamIsClone<ReserveIndex>;
                    let _: ::core::clone::AssertParamIsClone<FixedBuf<66>>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Position {}
        /// The value of a collateral or loan position within an obligation
        #[repr(C)]
        pub struct PositionValue {
            /// The market value in USD of the assets that were either deposited or borrowed.
            pub market_value: Number,
            /// For loans, this is the minimum collateral requirement in USD.
            /// For collateral, this is the maximum in USD that can be borrowed against it.
            pub complementary_limit: Number,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<Number>() + ::core::mem::size_of::<Number>()],
            );
            let _ = ::core::mem::transmute::<PositionValue, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        unsafe impl ::bytemuck::Pod for PositionValue {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for PositionValue {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for PositionValue {
            #[inline]
            fn clone(&self) -> PositionValue {
                {
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for PositionValue {}
        impl Position {
            fn new(side: Side, account: Pubkey, reserve_index: ReserveIndex) -> Position {
                Position {
                    account: account.into(),
                    side: side.into_integer(),
                    amount: Number::ZERO,
                    reserve_index,
                    _reserved: FixedBuf::zeroed(),
                }
            }
            pub fn market_value(&self, reserve: &CachedReserveInfo) -> PositionValue {
                let market_value = self._market_value(reserve);
                PositionValue {
                    market_value,
                    complementary_limit: self.complementary_limit(reserve, market_value),
                }
            }
            fn _market_value(&self, reserve: &CachedReserveInfo) -> Number {
                self.amount * self.note_exchange_rate(reserve) * reserve.price
            }
            fn complementary_limit(
                &self,
                reserve: &CachedReserveInfo,
                market_value: Number,
            ) -> Number {
                match Side::from_integer(self.side).expect("invalid side value") {
                    Side::Collateral => market_value / reserve.min_collateral_ratio,
                    Side::Loan => market_value * reserve.min_collateral_ratio,
                }
            }
            fn note_exchange_rate(&self, reserve: &CachedReserveInfo) -> Number {
                match Side::from_integer(self.side).expect("invalid side value") {
                    Side::Collateral => reserve.deposit_note_exchange_rate,
                    Side::Loan => reserve.loan_note_exchange_rate,
                }
            }
        }
        impl Debug for Obligation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let collateral = self.collateral().iter().collect::<Vec<_>>();
                let loans = self.loans().iter().collect::<Vec<_>>();
                f.debug_struct("Obligation")
                    .field("version", &{ self.version })
                    .field("market", &self.market)
                    .field("owner", &self.owner)
                    .field("collateral", &collateral)
                    .field("loans", &loans)
                    .finish()
            }
        }
    }
    mod reserve {
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::clock::UnixTimestamp;
        use bytemuck::{Pod, Zeroable};
        use std::cmp::Ordering;
        use jet_math::Number;
        use jet_proc_macros::assert_size;
        use crate::state::Cache;
        use crate::utils::FixedBuf;
        use crate::utils::JobCompletion;
        const SECONDS_PER_HOUR: UnixTimestamp = 3600;
        const SECONDS_PER_2H: UnixTimestamp = SECONDS_PER_HOUR * 2;
        const SECONDS_PER_12H: UnixTimestamp = SECONDS_PER_HOUR * 12;
        const SECONDS_PER_DAY: UnixTimestamp = SECONDS_PER_HOUR * 24;
        const SECONDS_PER_WEEK: UnixTimestamp = SECONDS_PER_DAY * 7;
        const SECONDS_PER_YEAR: UnixTimestamp = 31_536_000;
        const MAX_ACCRUAL_SECONDS: UnixTimestamp = SECONDS_PER_WEEK;
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_HOUR == 60 * 60;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_2H == 60 * 60 * 2;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_12H == 60 * 60 * 12;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_DAY == 60 * 60 * 24;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_WEEK == 60 * 60 * 24 * 7;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = SECONDS_PER_YEAR == 60 * 60 * 24 * 365;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<ReserveConfig>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 64usize == std::mem::size_of::<ReserveConfig>();
            ASSERT
        } as usize] = [];
        /// We have three interest rate regimes. The rate is described by a continuous,
        /// piecewise-linear function of the utilization rate:
        /// 1. zero to [utilization_rate_1]: borrow rate increases linearly from
        ///     [borrow_rate_0] to [borrow_rate_1].
        /// 2. [utilization_rate_1] to [utilization_rate_2]: borrow rate increases linearly
        ///     from [borrow_rate_1] to [borrow_rate_2].
        /// 3. [utilization_rate_2] to one: borrow rate increases linearly from
        ///     [borrow_rate_2] to [borrow_rate_3].
        ///
        /// Interest rates are nominal annual amounts, compounded continuously with
        /// a day-count convention of actual-over-365. The accrual period is determined
        /// by counting slots, and comparing against the number of slots per year.
        #[repr(C)]
        pub struct ReserveConfig {
            /// The utilization rate at which we switch from the first to second regime.
            pub utilization_rate_1: u16,
            /// The utilization rate at which we switch from the second to third regime.
            pub utilization_rate_2: u16,
            /// The lowest borrow rate in the first regime. Essentially the minimum
            /// borrow rate possible for the reserve.
            pub borrow_rate_0: u16,
            /// The borrow rate at the transition point from the first to second regime.
            pub borrow_rate_1: u16,
            /// The borrow rate at the transition point from the second to thirs regime.
            pub borrow_rate_2: u16,
            /// The highest borrow rate in the third regime. Essentially the maximum
            /// borrow rate possible for the reserve.
            pub borrow_rate_3: u16,
            /// The minimum allowable collateralization ratio for an obligation
            pub min_collateral_ratio: u16,
            /// The amount given as a bonus to a liquidator
            pub liquidation_premium: u16,
            /// The threshold at which to collect the fees accumulated from interest into
            /// real deposit notes.
            pub manage_fee_collection_threshold: u64,
            /// The fee rate applied to the interest payments collected
            pub manage_fee_rate: u16,
            /// The fee rate applied as interest owed on new loans
            pub loan_origination_fee: u16,
            /// unused
            pub _reserved0: u16,
            /// Represented as a percentage of the Price
            /// confidence values above this will not be accepted
            pub confidence_threshold: u16,
            /// The maximum token amount to allow in a single DEX trade when
            /// liquidating assetr from this reserve as collateral.
            pub liquidation_dex_trade_max: u64,
            pub _reserved1: [u8; 24],
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u64>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u16>()
                    + ::core::mem::size_of::<u64>()
                    + ::core::mem::size_of::<[u8; 24]>()],
            );
            let _ = ::core::mem::transmute::<ReserveConfig, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<[u8; 24]>();
            }
        };
        unsafe impl ::bytemuck::Pod for ReserveConfig {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u16>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<[u8; 24]>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for ReserveConfig {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ReserveConfig {
            #[inline]
            fn clone(&self) -> ReserveConfig {
                {
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 24]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ReserveConfig {}
        impl borsh::de::BorshDeserialize for ReserveConfig
        where
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            [u8; 24]: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    utilization_rate_1: borsh::BorshDeserialize::deserialize(buf)?,
                    utilization_rate_2: borsh::BorshDeserialize::deserialize(buf)?,
                    borrow_rate_0: borsh::BorshDeserialize::deserialize(buf)?,
                    borrow_rate_1: borsh::BorshDeserialize::deserialize(buf)?,
                    borrow_rate_2: borsh::BorshDeserialize::deserialize(buf)?,
                    borrow_rate_3: borsh::BorshDeserialize::deserialize(buf)?,
                    min_collateral_ratio: borsh::BorshDeserialize::deserialize(buf)?,
                    liquidation_premium: borsh::BorshDeserialize::deserialize(buf)?,
                    manage_fee_collection_threshold: borsh::BorshDeserialize::deserialize(buf)?,
                    manage_fee_rate: borsh::BorshDeserialize::deserialize(buf)?,
                    loan_origination_fee: borsh::BorshDeserialize::deserialize(buf)?,
                    _reserved0: borsh::BorshDeserialize::deserialize(buf)?,
                    confidence_threshold: borsh::BorshDeserialize::deserialize(buf)?,
                    liquidation_dex_trade_max: borsh::BorshDeserialize::deserialize(buf)?,
                    _reserved1: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for ReserveConfig
        where
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            [u8; 24]: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.utilization_rate_1, writer)?;
                borsh::BorshSerialize::serialize(&self.utilization_rate_2, writer)?;
                borsh::BorshSerialize::serialize(&self.borrow_rate_0, writer)?;
                borsh::BorshSerialize::serialize(&self.borrow_rate_1, writer)?;
                borsh::BorshSerialize::serialize(&self.borrow_rate_2, writer)?;
                borsh::BorshSerialize::serialize(&self.borrow_rate_3, writer)?;
                borsh::BorshSerialize::serialize(&self.min_collateral_ratio, writer)?;
                borsh::BorshSerialize::serialize(&self.liquidation_premium, writer)?;
                borsh::BorshSerialize::serialize(&self.manage_fee_collection_threshold, writer)?;
                borsh::BorshSerialize::serialize(&self.manage_fee_rate, writer)?;
                borsh::BorshSerialize::serialize(&self.loan_origination_fee, writer)?;
                borsh::BorshSerialize::serialize(&self._reserved0, writer)?;
                borsh::BorshSerialize::serialize(&self.confidence_threshold, writer)?;
                borsh::BorshSerialize::serialize(&self.liquidation_dex_trade_max, writer)?;
                borsh::BorshSerialize::serialize(&self._reserved1, writer)?;
                Ok(())
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 2048usize == std::mem::size_of::<Reserve>();
            ASSERT
        } as usize] = [];
        #[repr(packed)]
        pub struct Reserve {
            pub version: u16,
            /// The unique id for this reserve within the market
            pub index: u16,
            /// The base 10 decimals used for token values
            pub exponent: i32,
            /// The market this reserve is a part of.
            pub market: Pubkey,
            /// The account where a Pyth oracle keeps the updated price of the token.
            pub pyth_oracle_price: Pubkey,
            /// The account where a Pyth oracle keeps metadata about the token.
            pub pyth_oracle_product: Pubkey,
            /// The mint for the token being held in this reserve
            pub token_mint: Pubkey,
            /// The mint for this reserve's deposit notes
            pub deposit_note_mint: Pubkey,
            /// The mint for this reserve's loan notes
            pub loan_note_mint: Pubkey,
            /// The account with custody over the reserve's tokens.
            pub vault: Pubkey,
            /// The account with custody of the notes generated from collected fees
            pub fee_note_vault: Pubkey,
            /// The account for storing quote tokens during a swap
            pub dex_swap_tokens: Pubkey,
            /// The account used for trading with the DEX
            pub dex_open_orders: Pubkey,
            /// The DEX market account that this reserve can trade in
            pub dex_market: Pubkey,
            pub _reserved0: [u8; 408],
            pub config: ReserveConfig,
            _reserved1: [u8; 704],
            state: [u8; 512],
        }
        #[automatically_derived]
        impl Reserve {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Reserve {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Reserve {
            #[inline]
            fn clone(&self) -> Reserve {
                {
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<i32>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 408]>;
                    let _: ::core::clone::AssertParamIsClone<ReserveConfig>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 704]>;
                    let _: ::core::clone::AssertParamIsClone<[u8; 512]>;
                    *self
                }
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for Reserve {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for Reserve {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for Reserve {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for Reserve {
            fn discriminator() -> [u8; 8] {
                [43, 242, 204, 202, 26, 247, 59, 127]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Reserve {
            fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
                if buf.len() < [43, 242, 204, 202, 26, 247, 59, 127].len() {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[43, 242, 204, 202, 26, 247, 59, 127] != given_disc {
                    return Err(
                        anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into(),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, ProgramError> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Reserve {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl Reserve {
            pub(crate) fn init(&mut self, clock: &Clock) {
                self.state_mut().get_stale_mut().accrued_until = clock.unix_timestamp;
            }
            pub(crate) fn amount(&self, value: u64) -> Number {
                Number::from_decimal(value, self.exponent)
            }
            pub fn total_deposits(&self) -> u64 {
                self.state().get_stale().total_deposits
            }
            pub fn total_deposit_notes(&self) -> u64 {
                self.state().get_stale().total_deposit_notes
            }
            pub fn total_loan_notes(&self) -> u64 {
                self.state().get_stale().total_loan_notes
            }
            pub fn unwrap_outstanding_debt(&self, current_slot: u64) -> &Number {
                &self.unwrap_state(current_slot).outstanding_debt
            }
            fn state(&self) -> &Cache<ReserveState, 1> {
                bytemuck::from_bytes(&self.state)
            }
            fn state_mut(&mut self) -> &mut Cache<ReserveState, 1> {
                bytemuck::from_bytes_mut(&mut self.state)
            }
            fn unwrap_state(&self, current_slot: u64) -> &ReserveState {
                self.state()
                    .expect(current_slot, "Reserve needs to be refreshed")
            }
            fn unwrap_state_mut(&mut self, current_slot: u64) -> &mut ReserveState {
                self.state_mut()
                    .expect_mut(current_slot, "Reserve needs to be refreshed")
            }
            /// Record an amount of tokens deposited into the reserve
            pub fn deposit(&mut self, token_amount: u64, note_amount: u64) {
                let state = self.state_mut().get_stale_mut();
                state.total_deposits = state.total_deposits.checked_add(token_amount).unwrap();
                state.total_deposit_notes =
                    state.total_deposit_notes.checked_add(note_amount).unwrap();
            }
            /// Record an amount of tokens withdrawn from the reserve
            pub fn withdraw(&mut self, token_amount: u64, note_amount: u64) {
                let state = self.state_mut().get_stale_mut();
                state.total_deposits = state.total_deposits.checked_sub(token_amount).unwrap();
                state.total_deposit_notes =
                    state.total_deposit_notes.checked_sub(note_amount).unwrap();
            }
            /// Calculates the borrow fee token amount for
            /// an amount of tokens to be borrowed from the reserve.
            pub fn borrow_fee(&self, token_amount: u64) -> u64 {
                let origination_fee = Number::from_bps(self.config.loan_origination_fee);
                let fee_owed = origination_fee * token_amount;
                fee_owed.as_u64_ceil(0)
            }
            /// Record an amount of tokens to be borrowed from the reserve.
            pub fn borrow(
                &mut self,
                current_slot: u64,
                token_amount: u64,
                note_amount: u64,
                fees: u64,
            ) {
                let borrowed_amount = Number::from(token_amount);
                let state = self.unwrap_state_mut(current_slot);
                let fees = Number::from_decimal(fees, 0);
                state.uncollected_fees += fees;
                state.outstanding_debt += borrowed_amount + fees;
                state.total_deposits = state.total_deposits.checked_sub(token_amount).unwrap();
                state.total_loan_notes = state.total_loan_notes.checked_add(note_amount).unwrap();
            }
            /// Record an amount of tokens repaid back to the reserve.
            pub fn repay(&mut self, current_slot: u64, token_amount: u64, note_amount: u64) {
                let state = self.unwrap_state_mut(current_slot);
                state.outstanding_debt -= Number::from(token_amount);
                state.total_loan_notes = state.total_loan_notes.checked_sub(note_amount).unwrap();
                state.total_deposits = state.total_deposits.checked_add(token_amount).unwrap();
                if state.total_loan_notes == 0 && state.outstanding_debt < Number::ONE {
                    state.outstanding_debt = Number::ZERO;
                }
            }
            /// Record an amount of tokens added to the vault which need
            /// to be collected as fees later.
            pub fn add_uncollected_fees(&mut self, current_slot: u64, amount: u64) {
                let state = self.unwrap_state_mut(current_slot);
                state.uncollected_fees += Number::from(amount);
                state.total_deposits = state.total_deposits.checked_add(amount).unwrap();
            }
            /// Calculate the exchange rate for deposit notes (tokens per note)
            pub fn deposit_note_exchange_rate(
                &self,
                current_slot: u64,
                vault_total: u64,
                mint_supply: u64,
            ) -> Number {
                let state = self.unwrap_state(current_slot);
                let calc = DepositNoteCalculator {
                    outstanding_debt: state.outstanding_debt,
                    uncollected_fees: state.uncollected_fees,
                    vault_total,
                    mint_supply,
                };
                calc.exchange_rate()
            }
            /// Calculate the exchange rate for loan notes (tokens per note)
            pub fn loan_note_exchange_rate(&self, current_slot: u64, mint_supply: u64) -> Number {
                let state = self.unwrap_state(current_slot);
                let calc = LoanNoteCalculator {
                    outstanding_debt: state.outstanding_debt,
                    mint_supply,
                };
                calc.exchange_rate()
            }
            /// Accrue the interest charges for outstanding borrows
            pub fn try_accrue_interest(
                &mut self,
                vault_total: u64,
                target_time: UnixTimestamp,
                target_slot: u64,
            ) -> JobCompletion {
                let ReserveState {
                    outstanding_debt,
                    accrued_until,
                    ..
                } = *self.state().get_stale();
                let time_behind = target_time - accrued_until;
                let time_to_accrue = std::cmp::min(time_behind, MAX_ACCRUAL_SECONDS);
                let interest_rate = self.interest_rate(outstanding_debt, vault_total);
                let state_cache: &mut Cache<ReserveState, 0> =
                    bytemuck::from_bytes_mut(&mut self.state);
                match time_to_accrue.cmp(&0) {
                    Ordering::Less => {
                        {
                            ::std::rt::begin_panic(
                                "Interest may not be accrued over a negative time period.",
                            )
                        };
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        let compound_rate =
                            Reserve::compound_interest(interest_rate, time_to_accrue);
                        let interest_fee_rate = Number::from_bps(self.config.manage_fee_rate);
                        let state = state_cache.get_stale_mut();
                        let new_interest_accrued = state.outstanding_debt * compound_rate;
                        let fee_to_collect = new_interest_accrued * interest_fee_rate;
                        state.outstanding_debt += new_interest_accrued;
                        state.uncollected_fees += fee_to_collect;
                        state.accrued_until =
                            state.accrued_until.checked_add(time_to_accrue).unwrap();
                    }
                }
                if time_behind == time_to_accrue {
                    state_cache.refresh_to(target_slot);
                    JobCompletion::Full
                } else {
                    state_cache.invalidate();
                    JobCompletion::Partial
                }
            }
            /// Collect any fees that were accumulated
            ///
            /// Returns the number of notes to mint to represent the fees collected
            pub fn collect_accrued_fees(
                &mut self,
                current_slot: u64,
                exchange_rate: Number,
            ) -> u64 {
                let threshold = Number::from(self.config.manage_fee_collection_threshold);
                let state = self.unwrap_state_mut(current_slot);
                if threshold > state.uncollected_fees {
                    return 0;
                }
                let fee_notes = (state.uncollected_fees / exchange_rate).as_u64(0);
                state.uncollected_fees = Number::ZERO;
                state.total_deposit_notes =
                    state.total_deposit_notes.checked_add(fee_notes).unwrap();
                fee_notes
            }
            /// Computes the effective applicable interest rate assuming continuous
            /// compounding for the given number of slots.
            ///
            /// Uses an approximation calibrated for accuracy to twenty decimals places,
            /// though the current configuration of Number does not support that. (TODO)
            fn compound_interest(rate: Number, seconds: UnixTimestamp) -> Number {
                if rate > Number::ONE * 2 {
                    {
                        ::std::rt::begin_panic(
                            "Not implemented; interest rate too large for compound_interest()",
                        )
                    };
                }
                let terms = match seconds {
                    _ if seconds <= SECONDS_PER_2H => 5,
                    _ if seconds <= SECONDS_PER_12H => 6,
                    _ if seconds <= SECONDS_PER_DAY => 7,
                    _ if seconds <= SECONDS_PER_WEEK => 10,
                    _ => ::std::rt::begin_panic(
                        "Not implemented; too many seconds in compound_interest()",
                    ),
                };
                let x = rate * seconds / SECONDS_PER_YEAR;
                jet_math::expm1_approx(x, terms)
            }
            /// Get the interest rate charged to borrowers for the given inputs
            pub fn interest_rate(&self, outstanding_debt: Number, vault_total: u64) -> Number {
                let borrow_1 = Number::from_bps(self.config.borrow_rate_1);
                if vault_total == 0 && outstanding_debt == Number::ZERO {
                    return borrow_1;
                }
                let util_rate = utilization_rate(outstanding_debt, vault_total);
                let util_1 = Number::from_bps(self.config.utilization_rate_1);
                if util_rate <= util_1 {
                    let borrow_0 = Number::from_bps(self.config.borrow_rate_0);
                    return Reserve::interpolate(
                        util_rate,
                        Number::ZERO,
                        util_1,
                        borrow_0,
                        borrow_1,
                    );
                }
                let util_2 = Number::from_bps(self.config.utilization_rate_2);
                let borrow_2 = Number::from_bps(self.config.borrow_rate_2);
                if util_rate <= util_2 {
                    let borrow_1 = Number::from_bps(self.config.borrow_rate_1);
                    return Reserve::interpolate(util_rate, util_1, util_2, borrow_1, borrow_2);
                }
                let borrow_3 = Number::from_bps(self.config.borrow_rate_3);
                if util_rate < Number::ONE {
                    return Reserve::interpolate(
                        util_rate,
                        util_2,
                        Number::ONE,
                        borrow_2,
                        borrow_3,
                    );
                }
                borrow_3
            }
            /// Linear interpolation between (x0, y0) and (x1, y1).
            fn interpolate(x: Number, x0: Number, x1: Number, y0: Number, y1: Number) -> Number {
                if !(x >= x0) {
                    ::core::panicking::panic("assertion failed: x >= x0")
                };
                if !(x <= x1) {
                    ::core::panicking::panic("assertion failed: x <= x1")
                };
                y0 + ((x - x0) * (y1 - y0)) / (x1 - x0)
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 0 == std::mem::size_of::<ReserveState>() % 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = 496usize == std::mem::size_of::<ReserveState>();
            ASSERT
        } as usize] = [];
        /// Information about a single collateral or loan account registered with an obligation
        #[repr(C)]
        struct ReserveState {
            accrued_until: i64,
            outstanding_debt: Number,
            uncollected_fees: Number,
            total_deposits: u64,
            total_deposit_notes: u64,
            total_loan_notes: u64,
            _reserved: FixedBuf<416>,
        }
        const _: fn() = || {
            struct TypeWithoutPadding(
                [u8; ::core::mem::size_of::<i64>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<Number>()
                    + ::core::mem::size_of::<u64>()
                    + ::core::mem::size_of::<u64>()
                    + ::core::mem::size_of::<u64>()
                    + ::core::mem::size_of::<FixedBuf<416>>()],
            );
            let _ = ::core::mem::transmute::<ReserveState, TypeWithoutPadding>;
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<i64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<FixedBuf<416>>();
            }
        };
        unsafe impl ::bytemuck::Pod for ReserveState {}
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<i64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Number>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<u64>();
            }
        };
        const _: fn() = || {
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<FixedBuf<416>>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for ReserveState {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ReserveState {
            #[inline]
            fn clone(&self) -> ReserveState {
                {
                    let _: ::core::clone::AssertParamIsClone<i64>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<Number>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<FixedBuf<416>>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ReserveState {}
        /// Get the current utilization rate (borrowed / deposited)
        pub fn utilization_rate(outstanding_debt: Number, vault_total: u64) -> Number {
            outstanding_debt / (outstanding_debt + Number::from(vault_total))
        }
        struct DepositNoteCalculator {
            outstanding_debt: Number,
            uncollected_fees: Number,
            vault_total: u64,
            mint_supply: u64,
        }
        impl<'a> NoteCalculator for DepositNoteCalculator {
            fn note_supply(&self) -> Number {
                Number::from(self.mint_supply)
            }
            fn token_supply(&self) -> Number {
                self.outstanding_debt + Number::from(self.vault_total) - self.uncollected_fees
            }
        }
        struct LoanNoteCalculator {
            outstanding_debt: Number,
            mint_supply: u64,
        }
        impl NoteCalculator for LoanNoteCalculator {
            fn note_supply(&self) -> Number {
                Number::from(self.mint_supply)
            }
            fn token_supply(&self) -> Number {
                self.outstanding_debt
            }
        }
        pub trait NoteCalculator {
            fn note_supply(&self) -> Number;
            fn token_supply(&self) -> Number;
            /// Number of tokens each bank note is worth.
            /// Ratio in terms of the smallest transferable units of each token.
            fn exchange_rate(&self) -> Number {
                let note_supply = match self.note_supply() {
                    Number::ZERO => Number::ONE,
                    n => n,
                };
                let token_supply = match self.token_supply() {
                    Number::ZERO => Number::ONE,
                    n => n,
                };
                token_supply / note_supply
            }
            /// Returns the quantity of notes that represent the provided number of tokens
            fn notes_from_tokens(&self, tokens: u64) -> u64 {
                let tokens = Number::from(tokens);
                let notes = tokens / self.exchange_rate();
                notes.as_u64(0)
            }
            /// Returns the quantity of tokens that are represented by the provided number of notes
            fn tokens_from_notes(&self, notes: u64) -> u64 {
                let notes = Number::from(notes);
                let tokens = notes * self.exchange_rate();
                tokens.as_u64(0)
            }
        }
        impl std::fmt::Debug for ReserveState {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct("ReserveState")
                    .field("accrued_until", &self.accrued_until)
                    .field("outstanding_debt", &self.outstanding_debt.to_string())
                    .field("uncollected_fees", &self.uncollected_fees.to_string())
                    .field("total_deposits", &self.total_deposits.to_string())
                    .field("total_deposit_notes", &self.total_deposit_notes.to_string())
                    .field("total_loan_notes", &self.total_loan_notes.to_string())
                    .finish()
            }
        }
        impl std::fmt::Debug for Reserve {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let state = self.state().get_stale();
                f.debug_struct("Reserve")
                    .field("index", &{ self.index })
                    .field("market", &self.market)
                    .field("token_mint", &self.token_mint)
                    .field("vault", &self.vault)
                    .field("fee_vault", &self.fee_note_vault)
                    .field("exponent", &{ self.exponent })
                    .field("state", state)
                    .finish()
            }
        }
    }
    pub use cache::*;
    pub use market::*;
    pub use obligation::*;
    pub use reserve::*;
}
pub mod utils {
    use std::{
        cell::RefMut,
        fmt::Display,
        ops::{Deref, DerefMut},
    };
    use anchor_lang::prelude::*;
    use anchor_spl::dex::serum_dex::state::{Market as DexMarket, ToAlignedBytes};
    use bytemuck::{Pod, Zeroable};
    use crate::errors::ErrorCode;
    pub fn read_pyth_product_attribute<'d>(data: &'d [u8], attribute: &[u8]) -> Option<&'d [u8]> {
        let mut idx = 0;
        while idx < data.len() {
            let key_len = data[idx] as usize;
            idx += 1;
            if key_len == 0 {
                continue;
            }
            let key = &data[idx..idx + key_len];
            idx += key_len;
            let val_len = data[idx] as usize;
            idx += 1;
            let value = &data[idx..idx + val_len];
            idx += val_len;
            if key == attribute {
                return Some(value);
            }
        }
        None
    }
    pub fn verify_dex_market_tokens(
        market: &AccountInfo,
        program: &Pubkey,
        expected_base_mint: &Pubkey,
        expected_quote_mint: &Pubkey,
    ) -> ProgramResult {
        let market_state = DexMarket::load(market, program)?;
        let market_v1 = match market_state {
            DexMarket::V1(v1) => v1,
            DexMarket::V2(v2) => RefMut::map(v2, |m| &mut m.inner),
        };
        let expected_base_mint = expected_base_mint.to_aligned_bytes();
        let expected_quote_mint = expected_quote_mint.to_aligned_bytes();
        if { market_v1.coin_mint } != expected_base_mint || { market_v1.pc_mint }
            != expected_quote_mint
        {
            return Err(ErrorCode::InvalidDexMarketMints.into());
        }
        Ok(())
    }
    /// Workaround for the fact that `Pubkey` doesn't implement the
    /// `Pod` trait (even though it meets the requirements), and there
    /// isn't really a way for us to extend the original type, so we wrap
    /// it in a new one.
    #[repr(transparent)]
    pub struct StoredPubkey(Pubkey);
    impl ::core::marker::StructuralEq for StoredPubkey {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for StoredPubkey {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Pubkey>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for StoredPubkey {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for StoredPubkey {
        #[inline]
        fn eq(&self, other: &StoredPubkey) -> bool {
            match *other {
                StoredPubkey(ref __self_1_0) => match *self {
                    StoredPubkey(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &StoredPubkey) -> bool {
            match *other {
                StoredPubkey(ref __self_1_0) => match *self {
                    StoredPubkey(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for StoredPubkey {
        #[inline]
        fn clone(&self) -> StoredPubkey {
            {
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for StoredPubkey {}
    #[allow(unknown_lints, eq_op)]
    const _: [(); 0 - !{
        const ASSERT: bool = 32 == std::mem::size_of::<StoredPubkey>();
        ASSERT
    } as usize] = [];
    unsafe impl Pod for StoredPubkey {}
    unsafe impl Zeroable for StoredPubkey {}
    impl AsRef<Pubkey> for StoredPubkey {
        fn as_ref(&self) -> &Pubkey {
            &self.0
        }
    }
    impl From<StoredPubkey> for Pubkey {
        fn from(key: StoredPubkey) -> Self {
            key.0
        }
    }
    impl From<Pubkey> for StoredPubkey {
        fn from(key: Pubkey) -> Self {
            Self(key)
        }
    }
    impl Deref for StoredPubkey {
        type Target = Pubkey;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for StoredPubkey {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl PartialEq<Pubkey> for StoredPubkey {
        fn eq(&self, other: &Pubkey) -> bool {
            self.0.eq(other)
        }
    }
    impl std::fmt::Debug for StoredPubkey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (&self.0 as &dyn std::fmt::Display).fmt(f)
        }
    }
    impl Display for StoredPubkey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /// A fixed-size byte array
    #[repr(transparent)]
    pub struct FixedBuf<const SIZE: usize> {
        data: [u8; SIZE],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const SIZE: usize> ::core::clone::Clone for FixedBuf<SIZE> {
        #[inline]
        fn clone(&self) -> FixedBuf<SIZE> {
            {
                let _: ::core::clone::AssertParamIsClone<[u8; SIZE]>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const SIZE: usize> ::core::marker::Copy for FixedBuf<SIZE> {}
    #[allow(unknown_lints, eq_op)]
    const _: [(); 0 - !{
        const ASSERT: bool = 0 == std::mem::size_of::<FixedBuf<0>>();
        ASSERT
    } as usize] = [];
    #[allow(unknown_lints, eq_op)]
    const _: [(); 0 - !{
        const ASSERT: bool = 61 == std::mem::size_of::<FixedBuf<61>>();
        ASSERT
    } as usize] = [];
    unsafe impl<const SIZE: usize> Pod for FixedBuf<SIZE> {}
    unsafe impl<const SIZE: usize> Zeroable for FixedBuf<SIZE> {}
    impl<const SIZE: usize> std::fmt::Debug for FixedBuf<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["FixedBuf<", ">"],
                &match (&SIZE,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
        }
    }
    impl<const SIZE: usize> AsRef<[u8]> for FixedBuf<SIZE> {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }
    impl<const SIZE: usize> AsMut<[u8]> for FixedBuf<SIZE> {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }
    impl<const SIZE: usize> Deref for FixedBuf<SIZE> {
        type Target = [u8];
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    impl<const SIZE: usize> DerefMut for FixedBuf<SIZE> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }
    impl<const SIZE: usize> borsh::BorshDeserialize for FixedBuf<SIZE> {
        fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
            let mut data = [0u8; SIZE];
            data.copy_from_slice(buf);
            Ok(FixedBuf { data })
        }
    }
    impl<const SIZE: usize> borsh::BorshSerialize for FixedBuf<SIZE> {
        fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            let _ = writer.write(&self.data)?;
            Ok(())
        }
    }
    pub enum JobCompletion {
        Partial,
        Full,
    }
}
use errors::ErrorCode;
use instructions::*;
use state::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        4u8, 116u8, 200u8, 98u8, 42u8, 73u8, 33u8, 206u8, 151u8, 77u8, 137u8, 13u8, 40u8, 43u8,
        122u8, 29u8, 37u8, 239u8, 25u8, 145u8, 25u8, 142u8, 166u8, 76u8, 188u8, 96u8, 220u8, 154u8,
        114u8, 126u8, 8u8, 77u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
pub const LIQUIDATE_DEX_INSTR_ID: [u8; 8] = [28, 129, 253, 125, 243, 52, 11, 162];
use jet::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < 8 {
        return jet::default(program_id, accounts, data);
    }
    dispatch(program_id, accounts, data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct Jet;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Jet {
        #[inline]
        fn clone(&self) -> Jet {
            match *self {
                Jet => Jet,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for Jet {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Jet::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(Jet)
        }
    }
    impl anchor_lang::Id for Jet {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [33, 253, 15, 116, 89, 25, 127, 236] => {
            __private::__global::init_market(program_id, accounts, ix_data)
        }
        [138, 245, 71, 225, 153, 4, 3, 43] => {
            __private::__global::init_reserve(program_id, accounts, ix_data)
        }
        [61, 148, 100, 70, 143, 107, 17, 13] => {
            __private::__global::update_reserve_config(program_id, accounts, ix_data)
        }
        [136, 79, 202, 206, 211, 146, 182, 158] => {
            __private::__global::init_deposit_account(program_id, accounts, ix_data)
        }
        [255, 145, 182, 44, 246, 213, 160, 56] => {
            __private::__global::init_collateral_account(program_id, accounts, ix_data)
        }
        [194, 102, 166, 130, 91, 74, 188, 81] => {
            __private::__global::init_loan_account(program_id, accounts, ix_data)
        }
        [251, 10, 231, 76, 27, 11, 159, 96] => {
            __private::__global::init_obligation(program_id, accounts, ix_data)
        }
        [166, 195, 167, 232, 32, 198, 184, 182] => {
            __private::__global::set_market_owner(program_id, accounts, ix_data)
        }
        [73, 138, 236, 76, 82, 179, 94, 155] => {
            __private::__global::set_market_flags(program_id, accounts, ix_data)
        }
        [152, 6, 13, 164, 50, 219, 225, 43] => {
            __private::__global::close_deposit_account(program_id, accounts, ix_data)
        }
        [242, 35, 198, 137, 82, 225, 242, 182] => {
            __private::__global::deposit(program_id, accounts, ix_data)
        }
        [183, 18, 70, 156, 148, 109, 161, 34] => {
            __private::__global::withdraw(program_id, accounts, ix_data)
        }
        [156, 131, 142, 116, 146, 247, 162, 120] => {
            __private::__global::deposit_collateral(program_id, accounts, ix_data)
        }
        [115, 135, 168, 106, 139, 214, 138, 150] => {
            __private::__global::withdraw_collateral(program_id, accounts, ix_data)
        }
        [228, 253, 131, 202, 207, 116, 89, 18] => {
            __private::__global::borrow(program_id, accounts, ix_data)
        }
        [234, 103, 67, 82, 208, 234, 219, 166] => {
            __private::__global::repay(program_id, accounts, ix_data)
        }
        [223, 179, 226, 125, 48, 46, 39, 74] => {
            __private::__global::liquidate(program_id, accounts, ix_data)
        }
        [247, 195, 172, 177, 64, 18, 23, 209] => {
            __private::__global::mock_liquidate_dex(program_id, accounts, ix_data)
        }
        [2, 218, 138, 235, 79, 201, 25, 102] => {
            __private::__global::refresh_reserve(program_id, accounts, ix_data)
        }
        _ => jet::default(program_id, accounts, data),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn init_market(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitMarket::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitMarket {
                owner,
                quote_currency,
                quote_token_mint,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                InitializeMarket::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::init_market(
                Context::new(program_id, &mut accounts, remaining_accounts),
                owner,
                quote_currency,
                quote_token_mint,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn init_reserve(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitReserve::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitReserve { bump, config } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                InitializeReserve::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::init_reserve(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                config,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_reserve_config(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::UpdateReserveConfig::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateReserveConfig { new_config } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                UpdateReserveConfig::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::update_reserve_config(
                Context::new(program_id, &mut accounts, remaining_accounts),
                new_config,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn init_deposit_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitDepositAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitDepositAccount { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitializeDepositAccount::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
            )?;
            jet::init_deposit_account(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn init_collateral_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitCollateralAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitCollateralAccount { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitializeCollateralAccount::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
            )?;
            jet::init_collateral_account(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn init_loan_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitLoanAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitLoanAccount { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                InitializeLoanAccount::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::init_loan_account(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn init_obligation(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitObligation::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitObligation { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                InitializeObligation::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::init_obligation(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn set_market_owner(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SetMarketOwner::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetMarketOwner { new_owner } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SetMarketOwner::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::set_market_owner(
                Context::new(program_id, &mut accounts, remaining_accounts),
                new_owner,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn set_market_flags(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SetMarketFlags::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetMarketFlags { flags } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SetMarketFlags::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::set_market_flags(
                Context::new(program_id, &mut accounts, remaining_accounts),
                flags,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn close_deposit_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::CloseDepositAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CloseDepositAccount { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CloseDepositAccount::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::close_deposit_account(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Deposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Deposit { bump, amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Deposit::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::deposit(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Withdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Withdraw { bump, amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Withdraw::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::withdraw(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit_collateral(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::DepositCollateral::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DepositCollateral { bump, amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DepositCollateral::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::deposit_collateral(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw_collateral(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::WithdrawCollateral::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WithdrawCollateral { bump, amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WithdrawCollateral::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::withdraw_collateral(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn borrow(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Borrow::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Borrow { bump, amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Borrow::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::borrow(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn repay(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Repay::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Repay { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Repay::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::repay(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn liquidate(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Liquidate::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Liquidate {
                amount,
                min_collateral,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Liquidate::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::liquidate(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
                min_collateral,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn mock_liquidate_dex(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::MockLiquidateDex::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::MockLiquidateDex = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                MockLiquidateDex::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::mock_liquidate_dex(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn refresh_reserve(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::RefreshReserve::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::RefreshReserve = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                RefreshReserve::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            jet::refresh_reserve(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
    }
}
mod jet {
    use super::*;
    /// Initialize a new empty market with a given owner.
    pub fn init_market(
        ctx: Context<InitializeMarket>,
        owner: Pubkey,
        quote_currency: String,
        quote_token_mint: Pubkey,
    ) -> ProgramResult {
        instructions::init_market::handler(ctx, owner, quote_currency, quote_token_mint)
    }
    /// Initialize a new reserve in a market with some initial liquidity.
    pub fn init_reserve(
        ctx: Context<InitializeReserve>,
        bump: InitReserveBumpSeeds,
        config: ReserveConfig,
    ) -> ProgramResult {
        instructions::init_reserve::handler(ctx, bump, config)
    }
    /// Replace an existing reserve config
    pub fn update_reserve_config(
        ctx: Context<UpdateReserveConfig>,
        new_config: ReserveConfig,
    ) -> ProgramResult {
        instructions::update_reserve_config::handler(ctx, new_config)
    }
    /// Initialize an account that can be used to store deposit notes
    pub fn init_deposit_account(ctx: Context<InitializeDepositAccount>, bump: u8) -> ProgramResult {
        instructions::init_deposit_account::handler(ctx, bump)
    }
    /// Initialize an account that can be used to store deposit notes as collateral
    pub fn init_collateral_account(
        ctx: Context<InitializeCollateralAccount>,
        bump: u8,
    ) -> ProgramResult {
        instructions::init_collateral_account::handler(ctx, bump)
    }
    /// Initialize an account that can be used to store deposit notes as collateral
    pub fn init_loan_account(ctx: Context<InitializeLoanAccount>, bump: u8) -> ProgramResult {
        instructions::init_loan_account::handler(ctx, bump)
    }
    /// Initialize an account that can be used to borrow from a reserve
    pub fn init_obligation(ctx: Context<InitializeObligation>, bump: u8) -> ProgramResult {
        instructions::init_obligation::handler(ctx, bump)
    }
    /// Change the owner on a market
    pub fn set_market_owner(ctx: Context<SetMarketOwner>, new_owner: Pubkey) -> ProgramResult {
        instructions::set_market_owner::handler(ctx, new_owner)
    }
    /// Change the flags on a market
    pub fn set_market_flags(ctx: Context<SetMarketFlags>, flags: u64) -> ProgramResult {
        instructions::set_market_flags::handler(ctx, flags)
    }
    /// Close a deposit account
    pub fn close_deposit_account(ctx: Context<CloseDepositAccount>, bump: u8) -> ProgramResult {
        instructions::close_deposit_account::handler(ctx, bump)
    }
    /// Deposit tokens into a reserve
    pub fn deposit(ctx: Context<Deposit>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::deposit::handler(ctx, bump, amount)
    }
    /// Deposit tokens from a reserve
    pub fn withdraw(ctx: Context<Withdraw>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::withdraw::handler(ctx, bump, amount)
    }
    /// Deposit notes as collateral in an obligation
    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        bump: DepositCollateralBumpSeeds,
        amount: Amount,
    ) -> ProgramResult {
        instructions::deposit_collateral::handler(ctx, bump, amount)
    }
    /// Withdraw notes previously deposited as collateral in an obligation
    pub fn withdraw_collateral(
        ctx: Context<WithdrawCollateral>,
        bump: WithdrawCollateralBumpSeeds,
        amount: Amount,
    ) -> ProgramResult {
        instructions::withdraw_collateral::handler(ctx, bump, amount)
    }
    /// Borrow tokens from a reserve
    pub fn borrow(ctx: Context<Borrow>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::borrow::handler(ctx, bump, amount)
    }
    /// Repay a loan
    pub fn repay(ctx: Context<Repay>, amount: Amount) -> ProgramResult {
        instructions::repay::handler(ctx, amount)
    }
    /// Liquidate an unhealthy loan
    pub fn liquidate(
        ctx: Context<Liquidate>,
        amount: Amount,
        min_collateral: u64,
    ) -> ProgramResult {
        instructions::liquidate::handler(ctx, amount, min_collateral)
    }
    /// Liquidate an unhealthy loan
    pub fn mock_liquidate_dex(_ctx: Context<MockLiquidateDex>) -> ProgramResult {
        {
            ::std::rt::begin_panic("not supported")
        }
    }
    /// Refresh a reserve's market price and interest owed
    ///
    /// If the reserve is extremely stale, only a partial update will be
    /// performed. It may be necessary to call refresh_reserve multiple
    /// times to get the reserve up to date.
    pub fn refresh_reserve(ctx: Context<RefreshReserve>) -> ProgramResult {
        instructions::refresh_reserve::handler(ctx)
    }
    /// Route super special instructions
    pub fn default<'info>(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'info>],
        ix_data: &[u8],
    ) -> ProgramResult {
        if ix_data[..8] == LIQUIDATE_DEX_INSTR_ID {
            instructions::liquidate_dex::handler_raw(program_id, accounts, &ix_data[8..])?;
        } else {
            return Err(ErrorCode::UnknownInstruction.into());
        }
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct InitMarket {
        pub owner: Pubkey,
        pub quote_currency: String,
        pub quote_token_mint: Pubkey,
    }
    impl borsh::ser::BorshSerialize for InitMarket
    where
        Pubkey: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.owner, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_currency, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_token_mint, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitMarket
    where
        Pubkey: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                owner: borsh::BorshDeserialize::deserialize(buf)?,
                quote_currency: borsh::BorshDeserialize::deserialize(buf)?,
                quote_token_mint: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for InitMarket {
        fn data(&self) -> Vec<u8> {
            let mut d = [33, 253, 15, 116, 89, 25, 127, 236].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitReserve {
        pub bump: InitReserveBumpSeeds,
        pub config: ReserveConfig,
    }
    impl borsh::ser::BorshSerialize for InitReserve
    where
        InitReserveBumpSeeds: borsh::ser::BorshSerialize,
        ReserveConfig: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitReserve
    where
        InitReserveBumpSeeds: borsh::BorshDeserialize,
        ReserveConfig: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                config: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for InitReserve {
        fn data(&self) -> Vec<u8> {
            let mut d = [138, 245, 71, 225, 153, 4, 3, 43].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateReserveConfig {
        pub new_config: ReserveConfig,
    }
    impl borsh::ser::BorshSerialize for UpdateReserveConfig
    where
        ReserveConfig: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.new_config, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateReserveConfig
    where
        ReserveConfig: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                new_config: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateReserveConfig {
        fn data(&self) -> Vec<u8> {
            let mut d = [61, 148, 100, 70, 143, 107, 17, 13].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitDepositAccount {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for InitDepositAccount
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
    impl borsh::de::BorshDeserialize for InitDepositAccount
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
    impl anchor_lang::InstructionData for InitDepositAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [136, 79, 202, 206, 211, 146, 182, 158].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitCollateralAccount {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for InitCollateralAccount
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
    impl borsh::de::BorshDeserialize for InitCollateralAccount
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
    impl anchor_lang::InstructionData for InitCollateralAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [255, 145, 182, 44, 246, 213, 160, 56].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitLoanAccount {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for InitLoanAccount
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
    impl borsh::de::BorshDeserialize for InitLoanAccount
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
    impl anchor_lang::InstructionData for InitLoanAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [194, 102, 166, 130, 91, 74, 188, 81].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitObligation {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for InitObligation
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
    impl borsh::de::BorshDeserialize for InitObligation
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
    impl anchor_lang::InstructionData for InitObligation {
        fn data(&self) -> Vec<u8> {
            let mut d = [251, 10, 231, 76, 27, 11, 159, 96].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SetMarketOwner {
        pub new_owner: Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetMarketOwner
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.new_owner, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetMarketOwner
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                new_owner: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetMarketOwner {
        fn data(&self) -> Vec<u8> {
            let mut d = [166, 195, 167, 232, 32, 198, 184, 182].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SetMarketFlags {
        pub flags: u64,
    }
    impl borsh::ser::BorshSerialize for SetMarketFlags
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.flags, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetMarketFlags
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                flags: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetMarketFlags {
        fn data(&self) -> Vec<u8> {
            let mut d = [73, 138, 236, 76, 82, 179, 94, 155].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CloseDepositAccount {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for CloseDepositAccount
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
    impl borsh::de::BorshDeserialize for CloseDepositAccount
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
    impl anchor_lang::InstructionData for CloseDepositAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [152, 6, 13, 164, 50, 219, 225, 43].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Deposit {
        pub bump: u8,
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for Deposit
    where
        u8: borsh::ser::BorshSerialize,
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Deposit
    where
        u8: borsh::BorshDeserialize,
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Deposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Withdraw {
        pub bump: u8,
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for Withdraw
    where
        u8: borsh::ser::BorshSerialize,
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        u8: borsh::BorshDeserialize,
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Withdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [183, 18, 70, 156, 148, 109, 161, 34].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DepositCollateral {
        pub bump: DepositCollateralBumpSeeds,
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for DepositCollateral
    where
        DepositCollateralBumpSeeds: borsh::ser::BorshSerialize,
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DepositCollateral
    where
        DepositCollateralBumpSeeds: borsh::BorshDeserialize,
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for DepositCollateral {
        fn data(&self) -> Vec<u8> {
            let mut d = [156, 131, 142, 116, 146, 247, 162, 120].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WithdrawCollateral {
        pub bump: WithdrawCollateralBumpSeeds,
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for WithdrawCollateral
    where
        WithdrawCollateralBumpSeeds: borsh::ser::BorshSerialize,
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawCollateral
    where
        WithdrawCollateralBumpSeeds: borsh::BorshDeserialize,
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WithdrawCollateral {
        fn data(&self) -> Vec<u8> {
            let mut d = [115, 135, 168, 106, 139, 214, 138, 150].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Borrow {
        pub bump: u8,
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for Borrow
    where
        u8: borsh::ser::BorshSerialize,
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Borrow
    where
        u8: borsh::BorshDeserialize,
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Borrow {
        fn data(&self) -> Vec<u8> {
            let mut d = [228, 253, 131, 202, 207, 116, 89, 18].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Repay {
        pub amount: Amount,
    }
    impl borsh::ser::BorshSerialize for Repay
    where
        Amount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Repay
    where
        Amount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Repay {
        fn data(&self) -> Vec<u8> {
            let mut d = [234, 103, 67, 82, 208, 234, 219, 166].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Liquidate {
        pub amount: Amount,
        pub min_collateral: u64,
    }
    impl borsh::ser::BorshSerialize for Liquidate
    where
        Amount: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.min_collateral, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Liquidate
    where
        Amount: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
                min_collateral: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Liquidate {
        fn data(&self) -> Vec<u8> {
            let mut d = [223, 179, 226, 125, 48, 46, 39, 74].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct MockLiquidateDex;
    impl borsh::ser::BorshSerialize for MockLiquidateDex {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for MockLiquidateDex {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for MockLiquidateDex {
        fn data(&self) -> Vec<u8> {
            let mut d = [247, 195, 172, 177, 64, 18, 23, 209].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct RefreshReserve;
    impl borsh::ser::BorshSerialize for RefreshReserve {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RefreshReserve {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for RefreshReserve {
        fn data(&self) -> Vec<u8> {
            let mut d = [2, 218, 138, 235, 79, 201, 25, 102].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_initialize_reserve::*;
    pub use crate::__client_accounts_mock_liquidate_dex::*;
    pub use crate::__client_accounts_deposit_collateral::*;
    pub use crate::__client_accounts_initialize_market::*;
    pub use crate::__client_accounts_deposit::*;
    pub use crate::__client_accounts_set_market_owner::*;
    pub use crate::__client_accounts_withdraw_collateral::*;
    pub use crate::__client_accounts_close_deposit_account::*;
    pub use crate::__client_accounts_refresh_reserve::*;
    pub use crate::__client_accounts_initialize_deposit_account::*;
    pub use crate::__client_accounts_withdraw::*;
    pub use crate::__client_accounts_update_reserve_config::*;
    pub use crate::__client_accounts_initialize_collateral_account::*;
    pub use crate::__client_accounts_initialize_obligation::*;
    pub use crate::__client_accounts_repay::*;
    pub use crate::__client_accounts_borrow::*;
    pub use crate::__client_accounts_liquidate::*;
    pub use crate::__client_accounts_initialize_loan_account::*;
    pub use crate::__client_accounts_set_market_flags::*;
}
/// Specifies the units of some amount of value
pub enum AmountUnits {
    Tokens,
    DepositNotes,
    LoanNotes,
}
impl borsh::de::BorshDeserialize for AmountUnits {
    fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
        let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
        let return_value = match variant_idx {
            0u8 => AmountUnits::Tokens,
            1u8 => AmountUnits::DepositNotes,
            2u8 => AmountUnits::LoanNotes,
            _ => {
                let msg = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Unexpected variant index: "],
                        &match (&variant_idx,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                };
                return Err(borsh::maybestd::io::Error::new(
                    borsh::maybestd::io::ErrorKind::InvalidInput,
                    msg,
                ));
            }
        };
        Ok(return_value)
    }
}
impl borsh::ser::BorshSerialize for AmountUnits {
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> core::result::Result<(), borsh::maybestd::io::Error> {
        match self {
            AmountUnits::Tokens => {
                let variant_idx: u8 = 0u8;
                writer.write_all(&variant_idx.to_le_bytes())?;
            }
            AmountUnits::DepositNotes => {
                let variant_idx: u8 = 1u8;
                writer.write_all(&variant_idx.to_le_bytes())?;
            }
            AmountUnits::LoanNotes => {
                let variant_idx: u8 = 2u8;
                writer.write_all(&variant_idx.to_le_bytes())?;
            }
        }
        Ok(())
    }
}
impl ::core::marker::StructuralEq for AmountUnits {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for AmountUnits {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for AmountUnits {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for AmountUnits {
    #[inline]
    fn eq(&self, other: &AmountUnits) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for AmountUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&AmountUnits::Tokens,) => ::core::fmt::Formatter::write_str(f, "Tokens"),
            (&AmountUnits::DepositNotes,) => ::core::fmt::Formatter::write_str(f, "DepositNotes"),
            (&AmountUnits::LoanNotes,) => ::core::fmt::Formatter::write_str(f, "LoanNotes"),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for AmountUnits {
    #[inline]
    fn clone(&self) -> AmountUnits {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for AmountUnits {}
/// Represent an amount of some value (like tokens, or notes)
pub struct Amount {
    pub units: AmountUnits,
    pub value: u64,
}
impl borsh::de::BorshDeserialize for Amount
where
    AmountUnits: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            units: borsh::BorshDeserialize::deserialize(buf)?,
            value: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl borsh::ser::BorshSerialize for Amount
where
    AmountUnits: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.units, writer)?;
        borsh::BorshSerialize::serialize(&self.value, writer)?;
        Ok(())
    }
}
impl ::core::marker::StructuralEq for Amount {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Amount {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AmountUnits>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
}
impl ::core::marker::StructuralPartialEq for Amount {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Amount {
    #[inline]
    fn eq(&self, other: &Amount) -> bool {
        match *other {
            Amount {
                units: ref __self_1_0,
                value: ref __self_1_1,
            } => match *self {
                Amount {
                    units: ref __self_0_0,
                    value: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Amount) -> bool {
        match *other {
            Amount {
                units: ref __self_1_0,
                value: ref __self_1_1,
            } => match *self {
                Amount {
                    units: ref __self_0_0,
                    value: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Amount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Amount {
                units: ref __self_0_0,
                value: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Amount");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "units", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "value", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Amount {
    #[inline]
    fn clone(&self) -> Amount {
        {
            let _: ::core::clone::AssertParamIsClone<AmountUnits>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Amount {}
/// Specifies rounding integers up or down
pub enum Rounding {
    Up,
    Down,
}
impl Amount {
    /// Get the amount represented in tokens
    pub fn as_tokens(&self, reserve_info: &CachedReserveInfo, rounding: Rounding) -> u64 {
        match self.units {
            AmountUnits::Tokens => self.value,
            AmountUnits::DepositNotes => reserve_info.deposit_notes_to_tokens(self.value, rounding),
            AmountUnits::LoanNotes => reserve_info.loan_notes_to_tokens(self.value, rounding),
        }
    }
    /// Get the amount represented in deposit notes
    pub fn as_deposit_notes(
        &self,
        reserve_info: &CachedReserveInfo,
        rounding: Rounding,
    ) -> Result<u64, ErrorCode> {
        match self.units {
            AmountUnits::Tokens => Ok(reserve_info.deposit_notes_from_tokens(self.value, rounding)),
            AmountUnits::DepositNotes => Ok(self.value),
            AmountUnits::LoanNotes => Err(ErrorCode::InvalidAmountUnits),
        }
    }
    /// Get the amount represented in loan notes
    pub fn as_loan_notes(
        &self,
        reserve_info: &CachedReserveInfo,
        rounding: Rounding,
    ) -> Result<u64, ErrorCode> {
        match self.units {
            AmountUnits::Tokens => Ok(reserve_info.loan_notes_from_tokens(self.value, rounding)),
            AmountUnits::LoanNotes => Ok(self.value),
            AmountUnits::DepositNotes => Err(ErrorCode::InvalidAmountUnits),
        }
    }
    pub fn from_tokens(value: u64) -> Amount {
        Amount {
            units: AmountUnits::Tokens,
            value,
        }
    }
    pub fn from_deposit_notes(value: u64) -> Amount {
        Amount {
            units: AmountUnits::DepositNotes,
            value,
        }
    }
    pub fn from_loan_notes(value: u64) -> Amount {
        Amount {
            units: AmountUnits::LoanNotes,
            value,
        }
    }
}
