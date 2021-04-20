#![cfg(feature = "test-bpf")]

use heap_program::*;
use solana_program_test::*;
use solana_program::{pubkey::Pubkey, system_instruction};
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};
use borsh::de::BorshDeserialize;

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "heap_program",
        id(),
        processor!(processor::Processor::process_instruction),
    )
}

pub async fn get_account(program_context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
    program_context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

pub async fn create_node_account( 
    program_context: &mut ProgramTestContext,
    heap: &Pubkey,
    account_to_create: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_node_account(
            &id(),
            &program_context.payer.pubkey(),
            heap,
            account_to_create,
        ).unwrap()],
        Some(&program_context.payer.pubkey()));
    transaction.sign(
        &[&program_context.payer],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await
}

pub async fn init_heap(
    program_context: &mut ProgramTestContext,
    authority: &Keypair,
) -> Result<Keypair, TransportError> {
    let rent = program_context.banks_client.get_rent().await.unwrap();
    let heap_min_rent = rent.minimum_balance(state::Heap::LEN);

    let heap_account = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::create_account(
            &program_context.payer.pubkey(),
            &heap_account.pubkey(),
            heap_min_rent,
            state::Heap::LEN as u64,
            &id(),
        ),
        instruction::init(
            &id(),
            &heap_account.pubkey(),
            &authority.pubkey(),
        ).unwrap()],
        Some(&program_context.payer.pubkey()));
    
    transaction.sign(
        &[&program_context.payer, &heap_account, authority],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(heap_account)
}

#[tokio::test]
async fn test_create_node_account() {
    let mut program_context = program_test().start_with_context().await;

    let heap_authority = Keypair::new();

    let heap_key: Keypair = init_heap(&mut program_context, &heap_authority).await.unwrap();

    let heap_account_data = get_account(&mut program_context, &heap_key.pubkey()).await;
    let heap = state::Heap::try_from_slice(&heap_account_data.data.as_slice()).unwrap();

    let (node_key, _) = Pubkey::find_program_address(&[&heap_key.pubkey().to_bytes()[..32], &heap.size.to_le_bytes()], &id());

    create_node_account(&mut program_context, &heap_key.pubkey(), &node_key).await.unwrap();
}
