//! Program state processor

use crate::{error::HeapProgramError, instruction::HeapInstruction, state::{Heap, Node, HEAP_VERSION, EMPTY_NODE_DATA}};
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
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let heap_account_info = next_account_info(account_info_iter)?;
        let node_account_info = next_account_info(account_info_iter)?;
        let authority_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;
        // check if Heap is initialized
        let mut heap = Heap::try_from_slice(&heap_account_info.data.borrow())?;
        if heap.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        // check if Node is NOT initialized
        let mut node = Node::try_from_slice(&node_account_info.data.borrow())?;
        if node.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        // check if Node is rent exempt
        if !rent.is_exempt(node_account_info.lamports(), node_account_info.data_len()) {
            return Err(ProgramError::AccountNotRentExempt);
        }
        // check if Node address was generated correct ( should be ProgramAcc(HeapAcc, size parameter from Heap) )
            // also it checks that it's the last element
        let generated_node_address = Pubkey::create_program_address(&[&heap_account_info.key.to_bytes()[..32], &heap.size.to_le_bytes()], program_id)?;
        if generated_node_address != *node_account_info.key {
            return Err(HeapProgramError::WrongNodeAccount.into());
        }
        // check if authority key is the same as in Heap account
        if *authority_account_info.key != heap.authority {
            return Err(HeapProgramError::WrongAuthority.into());
        }
        // check if authority signed transaction
        if !authority_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check if data(argument) NOT empty
        if data == EMPTY_NODE_DATA {
            return Err(HeapProgramError::InvalidNodesData.into());
        }

        // write version to Node
        node.version = HEAP_VERSION;
        // write index to Node ( size value from Heap )
        node.index = heap.size;
        // write data to Node
        node.data = data;

        heap.size += 1;  // TODO: maybe have to use "save math"

        node.serialize(&mut *node_account_info.data.borrow_mut())?;
        heap.serialize(&mut *heap_account_info.data.borrow_mut()).map_err(|e| e.into())
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
            HeapInstruction::try_from_slice(input)?;
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
