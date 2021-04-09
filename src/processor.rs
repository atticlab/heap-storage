//! Program state processor

use crate::{error::ProgramTemplateError, instruction::HeapInstruction, state::{Heap, Node, HEAP_VERSION, EMPTY_NODE_DATA}};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    program_error::ProgramError,
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
        let heap_account_info = next_account_info(account_info_iter)?;
        let authority_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let mut heap = Heap::try_from_slice(&heap_account_info.data.borrow())?;
        if heap.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !rent.is_exempt(heap_account_info.lamports(), heap_account_info.data_len()) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !authority_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        heap.version = HEAP_VERSION;
        heap.authority = *authority_account_info.key;
        heap.size = 0;

        heap.serialize(&mut *heap_account_info.data.borrow_mut()).map_err(|e| e.into())
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
