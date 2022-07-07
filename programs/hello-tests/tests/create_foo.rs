// #[cfg(feature="test-bpf")]
mod test {
    use {
        anchor_client::{
            anchor_lang::{AnchorSerialize, Discriminator},
            solana_sdk::{
                account::Account,
                commitment_config::CommitmentConfig,
                native_token::LAMPORTS_PER_SOL,
                pubkey::Pubkey,
                signature::{Keypair, Signer},
                system_instruction::transfer,
                system_program,
                transaction::Transaction,
            },
            Client, Cluster,
        },
        solana_program_test::{tokio, ProgramTest},
        std::rc::Rc,
    };

    #[tokio::test]
    async fn create_treasury_wait_1_hour() {
        let authority = Keypair::new();
        let (foo_pubkey, _expected_bump) = Pubkey::find_program_address(
            &[authority.pubkey().to_bytes().as_ref()],
            &hello_tests::id(),
        );

        let foo_account = {
            let mut foo_data = Vec::new();
            foo_data.extend_from_slice(&hello_tests::Foo::discriminator());
            foo_data.extend_from_slice(
                &hello_tests::Foo {
                    authority: authority.pubkey(),
                    ..hello_tests::Foo::default()
                }
                .try_to_vec()
                .unwrap(),
            );

            Account {
                // lamports: 1,
                data: foo_data,
                owner: hello_tests::id(),
                ..Account::default()
            }
        };

        let mut pt = ProgramTest::new("hello_tests", hello_tests::id(), None);
        pt.add_account(foo_pubkey, foo_account);
        pt.set_compute_max_units(4157);
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;

        let client = Client::new_with_options(
            Cluster::Debug,
            Rc::new(Keypair::new()),
            CommitmentConfig::processed(),
        );
        let program = client.program(hello_tests::id());
        let create_ix = program
            .request()
            .accounts(hello_tests::accounts::Initialize {
                foo: foo_pubkey,
                system_program: system_program::ID,
                authority: authority.pubkey(),
            })
            .args(hello_tests::instruction::Initialize {})
            .instructions()
            .unwrap()
            .pop()
            .unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[
                transfer(&payer.pubkey(), &authority.pubkey(), 100 * LAMPORTS_PER_SOL),
                create_ix,
            ],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );
        banks_client.process_transaction(transaction).await.unwrap();
    }
}
