fn main() {
    println!("Hello, world! I'm useless");

    let cb = simple_contract_bytes();
    let ib = init_bytes(cb.len().try_into().unwrap());

    println!("Init code: {}", bytes_string(&ib));
    println!("Contract code: {}", bytes_string(&cb));

    let tx_data = [ib, cb].concat();
    println!("Tx data: {}", bytes_string(&tx_data));
}

fn init_bytes(size: u8) -> Vec<u8> {
    let push1 = 0x60;
    let offset = 0x0d;
    let zero = 0x00;
    let codecopy = 0x39;
    let return_ = 0xF3;
    let invalid = 0xFE;

    #[rustfmt::skip]
    let bytes = [
        push1, size,
        push1, offset,
        push1, zero,
        codecopy,
        push1, size,
        push1, zero,
        return_,
        invalid
    ];

    return bytes.to_vec();
}

fn simple_contract_bytes() -> Vec<u8> {
    // Log offset, size, topic
    let offset = 0x00;
    let size = 0x00;
    let topic = 0x42;
    let log1 = 0xA1;
    let push1 = 0x60;
    let sstore = 0x55;
    /*
     *
     * PUSH1 topic   [topic]
     * PUSH1 size    [size, topic]
     * PUSH1 offset  [offset, size, topic]
     * LOG1
     * SSTORE        [offset, topic]
     *
     */
    #[rustfmt::skip]
    let bytes = [
        push1, topic,
        push1, size,
        push1, offset,
        log1,
        push1, topic,
        push1, offset,
        sstore
    ];

    return bytes.to_vec();
}

fn bytes_string(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |a, b| {
        let mut orig = a.clone();
        let string = format!("{:0>2x}", b);
        orig.push_str(&string);
        return orig;
    })
}

#[cfg(test)]
mod simple_contract_creation {
    use core::panic;
    use std::str::FromStr;

    use revm::{
        Context, ExecuteCommitEvm, ExecuteEvm, MainBuilder, MainContext,
        context::result::{ExecutionResult, Output},
        database::{CacheDB, EmptyDB},
        handler::EvmTr,
        primitives::{Bytes, TxKind, U256},
    };

    use crate::{bytes_string, init_bytes, simple_contract_bytes};

    #[test]
    fn contract_behavior() {
        // Creation Code
        let cb = simple_contract_bytes();
        let ib = init_bytes(cb.len().try_into().unwrap());
        let tx_data = [ib, cb].concat();
        let tx_data_string = bytes_string(&tx_data);

        // EVM environment
        let ctx = Context::mainnet()
            .modify_tx_chained(|tx| {
                tx.kind = TxKind::Create;
                tx.data = Bytes::from_str(&tx_data_string).expect("illegal bytes");
            })
            .with_db(CacheDB::<EmptyDB>::default());

        let mut evm = ctx.build_mainnet();
        let ref_tx = evm.replay_commit().unwrap();

        // Create contract
        let ExecutionResult::Success { output: Output::Create(_, Some(address)), .. } = ref_tx
        else {
            panic!("Failed to create contract: {ref_tx:#?}");
        };
        println!("Created contract at {address}");

        // Simulate Tx with empty calldata
        evm.ctx().modify_tx(|tx| {
            tx.kind = TxKind::Call(address);
            tx.data = Default::default();
            tx.nonce += 1;
        });
        let result = evm.replay().expect("tx simulation failed");

        // Assert
        let Some(storage0) = result
            .state
            .get(&address)
            .expect("contract not found")
            .storage
            .get::<U256>(&Default::default())
        else {
            panic!("Failed to write storage in the init code: {result:#?}");
        };

        println!("storage U256(0) at {address}:  {storage0:#?}");
        assert_eq!(storage0.present_value(), 0x42.try_into().unwrap(), "{result:#?}");
    }
}
