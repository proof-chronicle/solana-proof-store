#![allow(unexpected_cfgs)]
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    system_instruction,
    system_program,
    sysvar::{rent::Rent, Sysvar},
    declare_id,
};
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("B973pRh2NL9DkdvcT8VhSJywqqueknd8JXMkVVn18T3j");

// Data structure to store on-chain
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProofData {
    pub url: String,
    pub content_hash: String,
    pub content_length: u64,
}

impl ProofData {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

// Simplified instructions - only StoreProof
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProofInstruction {
    StoreProof {
        url: String,
        content_hash: String,
        content_length: u64,
    },
}

impl ProofInstruction {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        // Serialize the instruction data directly
        borsh::to_vec(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProofInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        ProofInstruction::StoreProof { url, content_hash, content_length } => {
            store_proof(program_id, accounts, url, content_hash, content_length)
        }
    }
}

fn store_proof(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    url: String,
    content_hash: String,
    content_length: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Account that will pay for the transaction
    let payer = next_account_info(account_info_iter)?;
    
    // Account to store the proof data
    let proof_account = next_account_info(account_info_iter)?;
    
    // System program
    let system_program = next_account_info(account_info_iter)?;

    // Verify the system program
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Create the proof data
    let proof_data = ProofData {
        url: url.clone(),
        content_hash: content_hash.clone(),
        content_length,
    };

    // Calculate the space needed for the account
    let data_len = proof_data.try_to_vec().unwrap().len();
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(data_len);

    // Create the account if it doesn't exist
    if proof_account.lamports() == 0 {
        invoke(
            &system_instruction::create_account(
                payer.key,
                proof_account.key,
                lamports,
                data_len as u64,
                program_id,
            ),
            &[payer.clone(), proof_account.clone(), system_program.clone()],
        )?;
    }

    // Verify the account is owned by our program
    if proof_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Serialize and store the data
    proof_data.serialize(&mut &mut proof_account.data.borrow_mut()[..])?;
    msg!("Data stored successfully");

    msg!("Proof stored successfully!");
    msg!("URL: {}", url);
    msg!("Content Hash: {}", content_hash);
    msg!("Content Length: {}", content_length);

    Ok(())
}