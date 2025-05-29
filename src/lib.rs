#![allow(unexpected_cfgs)]
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
    program::invoke,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Data structure to store on-chain
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProofData {
    pub url: String,
    pub hash: String,
    pub created_at: String,
    pub is_initialized: bool,
}

impl ProofData {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }
}

// Instructions that can be sent to the program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProofInstruction {
    /// Store proof data
    /// Accounts expected:
    /// 0. `[signer]` The account paying for the transaction
    /// 1. `[writable]` The proof data account to create/write to
    /// 2. `[]` System program
    StoreProof {
        url: String,
        hash: String,
        created_at: String,
    },
    
    /// Get proof data (for verification)
    /// Accounts expected:
    /// 0. `[]` The proof data account to read from
    GetProof,
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
        ProofInstruction::StoreProof { url, hash, created_at } => {
            msg!("Instruction: StoreProof");
            store_proof(program_id, accounts, url, hash, created_at)
        }
        ProofInstruction::GetProof => {
            msg!("Instruction: GetProof");
            get_proof(accounts)
        }
    }
}

fn store_proof(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    url: String,
    hash: String,
    created_at: String,
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
        hash: hash.clone(),
        created_at: created_at.clone(),
        is_initialized: true,
    };

    // Calculate the space needed for the account
    let data_len = proof_data.try_to_vec().unwrap().len();
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(data_len);

    // Create the account if it doesn't exist
    if proof_account.lamports() == 0 {
        msg!("Creating new proof account");
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

    msg!("Proof stored successfully!");
    msg!("URL: {}", url);
    msg!("Hash: {}", hash);
    msg!("Created at: {}", created_at);

    Ok(())
}

fn get_proof(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let proof_account = next_account_info(account_info_iter)?;

    // Deserialize the proof data
    let proof_data = ProofData::try_from_slice(&proof_account.data.borrow())?;

    if !proof_data.is_initialized {
        msg!("Account not initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    msg!("Proof data retrieved:");
    msg!("URL: {}", proof_data.url);
    msg!("Hash: {}", proof_data.hash);
    msg!("Created at: {}", proof_data.created_at);

    Ok(())
}