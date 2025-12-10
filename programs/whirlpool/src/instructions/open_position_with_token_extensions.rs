use crate::manager::tick_array_manager::collect_rent_for_ticks_in_position;
use crate::state::*;
use crate::util::build_position_token_metadata;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::spl_token_2022;
use anchor_spl::token_2022::Token2022;

use crate::constants::nft::whirlpool_nft_update_auth::ID as WP_NFT_UPDATE_AUTH;
use crate::util::{
    initialize_position_mint_2022, initialize_position_token_account_2022,
    mint_position_token_2022_and_remove_authority,
};

#[derive(Accounts)]
pub struct OpenPositionWithTokenExtensions<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    /// CHECK: safe, the account that will be the owner of the position can be arbitrary
    pub owner: UncheckedAccount<'info>,

    #[account(init,
      payer = funder,
      space = Position::LEN,
      seeds = [b"position".as_ref(), position_mint.key().as_ref()],
      bump,
    )]
    pub position: Box<Account<'info, Position>>,

    /// CHECK: initialized in the handler
    #[account(mut)]
    pub position_mint: Signer<'info>,

    /// CHECK: initialized in the handler
    #[account(mut)]
    pub position_token_account: UncheckedAccount<'info>,

    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(address = spl_token_2022::ID)]
    pub token_2022_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: checked via account constraints
    #[account(address = WP_NFT_UPDATE_AUTH)]
    pub metadata_update_auth: UncheckedAccount<'info>,
}
