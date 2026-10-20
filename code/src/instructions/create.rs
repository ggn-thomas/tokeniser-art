#[allow(deprecated)]
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_pack::Pack,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_token::{instruction as token_instruction, state::Mint},
    mpl_token_metadata::instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs },
    mpl_token_metadata::types::DataV2,
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreateNftArgs {
    pub nft_title: String,
    pub nft_symbol: String,
    pub nft_uri: String,
}

pub fn create(
    accounts: &[AccountInfo],
    args: CreateNftArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let _associated_token_account = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;
    let _associated_token_program = next_account_info(accounts_iter)?;
    let _edition_account = next_account_info(accounts_iter)?;

    // create mint account
    //
    msg!("Creating mint account...");
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            Rent::get()?.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ]
    )?;

    // initialize mint account
    //
    msg!("Initializing mint account...");
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            Some(mint_authority.key),
            0, // // 0 decimals, an nft cannot be split
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ]
    )?;

    // create NFT metadata account
    //
    msg!("Creating metadata account...");
    invoke(
        &CreateMetadataAccountV3 {
            metadata: *metadata_account.key,
            mint: *mint_account.key,
            mint_authority: *mint_authority.key,
            payer: *payer.key,
            update_authority: (*mint_authority.key, false),
            system_program: *system_program.key,
            rent: None,
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: args.nft_title,
                symbol: args.nft_symbol,
                uri: args.nft_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: false,
            collection_details: None,
        }),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            system_program.clone(),
            token_metadata_program.clone(),
        ],
    )?;

    Ok(())
}