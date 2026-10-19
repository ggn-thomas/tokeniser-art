use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
    },
    mpl_token_metadata::instructions::{ CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs },
    spl_token::instruction as token_instruction,
    spl_associated_token_account::instruction as associated_token_account_instruction,
};

pub fn initialize_mint(
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let associated_token_account = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    let edition_account = next_account_info(accounts_iter)?;

    // créer le compte associé
    //
    msg!("Creating token associated account...");
    invoke(
        &associated_token_account_instruction::create_associated_token_account(
            payer.key,
            payer.key,
            mint_account.key,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            associated_token_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )?;

    // Miné le token vers un compte associé
    //
    msg!("Minning token to associated token account...");
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            mint_authority.key,
            &[mint_authority.key],
            1,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            associated_token_account.clone(),
            token_program.clone(),
        ],
    )?;

    // Créé le compte édition
    //
    msg!("Creating edition account...");
    invoke(
        &CreateMasterEditionV3 {
            edition: *edition_account.key,
            mint: *mint_account.key,
            update_authority: *mint_authority.key,
            mint_authority: *mint_authority.key,
            payer: *payer.key,
            metadata: *metadata_account.key,
            token_program: *token_program.key,
            system_program: *system_program.key,
            rent: None,
        }
        .instruction(CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }),
        &[
            token_metadata_program.clone(),
            edition_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            mint_authority.clone(),
            payer.clone(),
            metadata_account.clone(),
            token_program.clone(),
            system_program.clone(),
            rent.clone(),
        ],
    )?;

    Ok(())
}