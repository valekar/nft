use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        max_accounts: u16,
        go_live_date: String,
        nft_config_data: NftConfigData,
    ) -> ProgramResult {
        let nft_account = &mut ctx.accounts.nft_account;

        let admin = &mut ctx.accounts.admin;

        nft_account.admin = *admin.key;

        //nft_account.nft_config_data.price = nft_config_data.price;
        //nft_account.nft_config_data.items_available = nft_config_data.items_available;

        let parsed_go_live_date = go_live_date.parse::<i64>();
        // nft_account
        //     .buyers
        //     .resize(max_accounts as usize, Pubkey::default());
        // let def_nft_config_data = &mut ctx.accounts.nft_account.nft_config_data;
        // def_nft_config_data.uuid = nft_config_data.uuid;

        match parsed_go_live_date {
            Ok(result) => {
                // nft_account.nft_config_data.go_live_date = Some(123);
            }
            Err(er) => {
                msg!("Parse error {}", er);
            }
        }

        Ok(())
    }

    pub fn update_nft_config(
        ctx: Context<UpdateNftConfig>,
        nft_config_data: NftConfigData,
    ) -> ProgramResult {
        msg!("updating");
        let nft_account = &mut ctx.accounts.nft_account;
        // nft_account.test = nft_config_data.uuid.clone();
        let def_nft_config_data = &mut nft_account.clone().nft_config_data;
        def_nft_config_data.uuid = nft_config_data.uuid;
        let admin = nft_account.admin;
        nft_account.buyers.push(Pubkey::default());

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(max_accounts : u16)]
pub struct Initialize<'info> {
    #[account(init,payer = admin, space = (8+32+8+8+8+8+32*300) as usize)]
    pub nft_account: ProgramAccount<'info, NftAccountData>,
    //#[account(signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct NftAccountData {
    pub admin: Pubkey,
    pub buyers: Vec<Pubkey>,
    pub nft_config_data: NftConfigData,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct NftConfigData {
    pub uuid: String,
    pub price: u64,
    pub items_available: u64,
    pub go_live_date: Option<i64>,
}

#[error]
pub enum ErrorCode {
    #[msg("Not enough SOL to pay for this minting")]
    NotEnoughSOL,
    #[msg("Could not convert from one datatype to other")]
    ConversionFailure,
}

#[derive(Accounts)]
pub struct UpdateNftConfig<'info> {
    #[account(mut , has_one = admin)]
    pub nft_account: ProgramAccount<'info, NftAccountData>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}
