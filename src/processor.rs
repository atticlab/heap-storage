//! Program state processor

use crate::{error::ProgramTemplateError, instruction::HeapInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Init new Heap
    pub fn process_init_heap(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Add Node to Heap
    pub fn process_add_node(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Remove Node from the Heap
    pub fn process_remove_node(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Swap two nodes
    pub fn process_swap(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction =
            HeapInstruction::try_from_slice(input).or(Err(ProgramTemplateError::ExampleError))?;
        match instruction {
            HeapInstruction::InitHeap => {
                msg!("Instruction: InitHeap");
                Self::process_init_heap(program_id, accounts)
            }
            HeapInstruction::AddNode(data) => {
                msg!("Instruction: AddNode");
                Self::process_add_node(program_id, accounts, data)
            }
            HeapInstruction::RemoveNode => {
                msg!("Instruction: RemoveNode");
                Self::process_remove_node(program_id, accounts)
            }
            HeapInstruction::Swap => {
                msg!("Instruction: Swap");
                Self::process_swap(program_id, accounts)
            }
        }
    }
}
