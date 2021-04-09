//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Heap version
pub const HEAP_VERSION: u8 = 1;
/// Empty Node's data
pub const EMPTY_NODE_DATA: [u8; 32] = [0; 32];

/// Heap
#[repr(C)]
#[derive(Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Heap {
    /// Heap's version
    pub version: u8,
    /// Authority with rights to modify Heap and Node accounts
    pub authority: Pubkey,
    /// Size of heap
    pub size: u128,
}

/// Node
#[repr(C)]
#[derive(Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Node {
    /// Version
    pub version: u8,
    /// Node's index in heap
    pub index: u128,
    /// Data
    pub data: [u8; 32],
}

impl Heap {
    /// Check if Heap is initialized
    pub fn is_initialized(&self) -> bool {
        self.version == HEAP_VERSION
    }
}

impl Node {
    /// Check if Node is initialized
    pub fn is_initialized(&self) -> bool {
        // use the same version as for Heap because Node is belongs to certain Heap
        self.version == HEAP_VERSION
    }
    /// Check if Node contains data
    pub fn is_data_empty(&self) -> bool {
        self.data == EMPTY_NODE_DATA
    }
}
