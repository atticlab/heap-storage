//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum HeapInstruction {
    /// InitHeap
    ///
    ///   0. `[w]` Heap account
    ///   1. `[s]` Authority
    ///   2. `[]` Rent account
    InitHeap,

    /// AddNode
    ///
    ///   0. `[w]` Heap account
    ///   1. `[w]` Node account, should not be initialized
    ///   2. `[s]` Authority
    ///   3. `[]` Rent account
    AddNode([u8; 32]),

    /// RemoveNode
    ///
    ///   0. `[w]` Heap account
    ///   1. `[w]` Node account to remove
    ///   2. `[w]` Leaf node to write to root node
    ///   3. `[s]` Authority
    RemoveNode,

    /// Swap
    ///   0. `[]` Heap account
    ///   1. `[w]` Parent node
    ///   2. `[w]` Child node
    ///   3. `[s]` Authority
    Swap,
}

/// Create `InitHeap` instruction
pub fn init(
    program_id: &Pubkey,
    heap_account: &Pubkey,
    authority_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = HeapInstruction::InitHeap;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*heap_account, false),
        AccountMeta::new_readonly(*authority_account, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `AddNode` instruction
pub fn add_node(
    program_id: &Pubkey,
    heap_account: &Pubkey,
    node_account: &Pubkey,
    authority_account: &Pubkey,
    node_data: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = HeapInstruction::AddNode(node_data);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*heap_account, false),
        AccountMeta::new(*node_account, false),
        AccountMeta::new_readonly(*authority_account, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `RemoveNode` instruction
pub fn remove_node(
    program_id: &Pubkey,
    heap_account: &Pubkey,
    node_account: &Pubkey,
    leaf_account: &Pubkey,
    authority_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = HeapInstruction::RemoveNode;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*heap_account, false),
        AccountMeta::new(*node_account, false),
        AccountMeta::new(*leaf_account, false),
        AccountMeta::new_readonly(*authority_account, true),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `Swap` instruction
pub fn swap(
    program_id: &Pubkey,
    heap_account: &Pubkey,
    parent_node_account: &Pubkey,
    child_node_account: &Pubkey,
    authority_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = HeapInstruction::Swap;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*heap_account, false),
        AccountMeta::new(*parent_node_account, false),
        AccountMeta::new(*child_node_account, false),
        AccountMeta::new_readonly(*authority_account, true),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
