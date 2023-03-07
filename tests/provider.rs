use ethrs::provider::DefaultBlockParam;
use ethrs::provider::Provider;
use ethrs::types::U256;

use lazy_static::lazy_static;
use std::error::Error;

lazy_static! {
    static ref PROVIDER: Provider = Provider::new("https://rpc.sepolia.org");
}

#[test]
fn test_block_number() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER.block_number().unwrap() > 2893700);
    Ok(())
}

#[test]
fn test_gas_price() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER.gas_price().unwrap() >= 8);
    Ok(())
}

#[test]
fn test_get_balance() -> Result<(), Box<dyn Error>> {
    assert!(
        PROVIDER
            .get_balance("0x0000000000000000000000000000000000000000", None, None)
            .unwrap()
            > 0
    );
    PROVIDER
        .get_balance(
            "0x0000000000000000000000000000000000000000",
            Some(DefaultBlockParam::EARLIEST),
            None,
        )
        .unwrap();
    assert!(
        PROVIDER
            .get_balance(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::LATEST),
                None,
            )
            .unwrap()
            > 0
    );
    assert!(
        PROVIDER
            .get_balance(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::PENDING),
                None,
            )
            .unwrap()
            > 0
    );
    assert!(
        PROVIDER
            .get_balance(
                "0x0000000000000000000000000000000000000000",
                None,
                Some(PROVIDER.block_number().unwrap() - 1),
            )
            .unwrap()
            > 0
    );
    Ok(())
}

#[test]
fn test_get_storage_at() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        PROVIDER
            .get_storage_at(
                "0x0000000000000000000000000000000000000000",
                "0x0",
                None,
                None
            )
            .unwrap(),
        "0x0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        PROVIDER
            .get_storage_at(
                "0x95ab1853c803c740e7b095776b217f0e8cbd2e16",
                "0x0",
                None,
                None
            )
            .unwrap(),
        "0x0000000000000000000000da9e8e71bb750a996af33ebb8abb18cd9eb9dc7500"
    );
    Ok(())
}

#[test]
fn test_get_transaction_count() -> Result<(), Box<dyn Error>> {
    assert!(
        PROVIDER
            .get_transaction_count("0x0000000000000000000000000000000000000000", None, None)
            .unwrap()
            == 0
    );
    assert!(
        PROVIDER
            .get_transaction_count(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::EARLIEST),
                None
            )
            .unwrap()
            == 0
    );
    assert!(
        PROVIDER
            .get_transaction_count(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::LATEST),
                None
            )
            .unwrap()
            == 0
    );
    assert!(
        PROVIDER
            .get_transaction_count(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::PENDING),
                None
            )
            .unwrap()
            == 0
    );
    assert!(
        PROVIDER
            .get_transaction_count(
                "0x0000000000000000000000000000000000000000",
                None,
                Some(PROVIDER.block_number().unwrap() - 1)
            )
            .unwrap()
            == 0
    );
    Ok(())
}

#[test]
fn test_get_block_transaction_count_by_hash() -> Result<(), Box<dyn Error>> {
    assert!(
        PROVIDER.get_block_transaction_count_by_hash(
            "0x6c4925c897c45d377d8fb3ef59df7e0cf97604fc85b909bb806818368fdc6b07"
        )? == Some(5)
    );
    assert!(PROVIDER
        .get_block_transaction_count_by_hash(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        )?
        .is_none());
    assert!(
        PROVIDER.get_block_transaction_count_by_hash(
            "0x68a52ca2491ab61f32d046021654b65859db15bd763a4e09f8ca0e923de707cd"
        )? == Some(0)
    );
    Ok(())
}

#[test]
fn test_get_block_by_hash() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER
        .get_block_by_hash("0x14c2bae040612f036c032f7f0eccf9b3389cd8c30d810df69abdf772f7acf6d8")?
        .is_some());
    assert!(PROVIDER
        .get_block_by_hash("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")?
        .is_none());
    assert!(PROVIDER
        .get_block_by_hash_with_tx(
            "0x33ddfd6eebe80ec8fe2fecfd8fbd7fa7abd5ceb8f53ec11dff1e90312c2828b5"
        )?
        .is_some());
    assert!(PROVIDER
        .get_block_by_hash_with_tx(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        )?
        .is_none());
    Ok(())
}

#[test]
fn test_get_block_by_number() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER.get_block_by_number(None, None)?.is_some());
    assert!(PROVIDER
        .get_block_by_number(Some(DefaultBlockParam::EARLIEST), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number(Some(DefaultBlockParam::LATEST), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number(Some(DefaultBlockParam::PENDING), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number(None, Some(PROVIDER.block_number().unwrap()))?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number(None, Some(9999999999))?
        .is_none());
    assert!(PROVIDER.get_block_by_number_with_tx(None, None)?.is_some());
    assert!(PROVIDER
        .get_block_by_number_with_tx(Some(DefaultBlockParam::EARLIEST), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number_with_tx(Some(DefaultBlockParam::LATEST), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number_with_tx(Some(DefaultBlockParam::PENDING), None)?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number_with_tx(None, Some(PROVIDER.block_number().unwrap()))?
        .is_some());
    assert!(PROVIDER
        .get_block_by_number_with_tx(None, Some(9999999999))?
        .is_none());
    Ok(())
}

#[test]
fn test_get_code() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        PROVIDER
            .get_code("0x0000000000000000000000000000000000000000", None, None)
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(
        PROVIDER
            .get_code(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::EARLIEST),
                None,
            )
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(
        PROVIDER
            .get_code(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::LATEST),
                None,
            )
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(
        PROVIDER
            .get_code(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::PENDING),
                None,
            )
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(
        PROVIDER
            .get_code(
                "0x0000000000000000000000000000000000000000",
                Some(DefaultBlockParam::FINALIZED),
                None,
            )
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(
        PROVIDER
            .get_code(
                "0x0000000000000000000000000000000000000000",
                None,
                Some(PROVIDER.block_number().unwrap() - 1),
            )
            .unwrap(),
        "0x".to_owned()
    );
    assert_eq!(PROVIDER.get_code("0x790830c1eaab862fd35dbce2e7ea1aebce32fce3", None, None).unwrap(), "0x6060604052600436106100ae5763ffffffff7c010000000000000000000000000000000000000000000000000000000060003504166306fdde0381146100b8578063095ea7b31461014257806318160ddd1461017857806323b872dd1461019d5780632e1a7d4d146101c5578063313ce567146101db57806370a082311461020457806395d89b4114610223578063a9059cbb14610236578063d0e30db0146100ae578063dd62ed3e14610258575b6100b661027d565b005b34156100c357600080fd5b6100cb6102d3565b60405160208082528190810183818151815260200191508051906020019080838360005b838110156101075780820151838201526020016100ef565b50505050905090810190601f1680156101345780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b341561014d57600080fd5b610164600160a060020a0360043516602435610371565b604051901515815260200160405180910390f35b341561018357600080fd5b61018b6103dd565b60405190815260200160405180910390f35b34156101a857600080fd5b610164600160a060020a03600435811690602435166044356103eb565b34156101d057600080fd5b6100b6600435610531565b34156101e657600080fd5b6101ee6105df565b60405160ff909116815260200160405180910390f35b341561020f57600080fd5b61018b600160a060020a03600435166105e8565b341561022e57600080fd5b6100cb6105fa565b341561024157600080fd5b610164600160a060020a0360043516602435610665565b341561026357600080fd5b61018b600160a060020a0360043581169060243516610679565b600160a060020a033316600081815260036020526040908190208054349081019091557fe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c915190815260200160405180910390a2565b60008054600181600116156101000203166002900480601f0160208091040260200160405190810160405280929190818152602001828054600181600116156101000203166002900480156103695780601f1061033e57610100808354040283529160200191610369565b820191906000526020600020905b81548152906001019060200180831161034c57829003601f168201915b505050505081565b600160a060020a03338116600081815260046020908152604080832094871680845294909152808220859055909291907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259085905190815260200160405180910390a350600192915050565b600160a060020a0330163190565b600160a060020a0383166000908152600360205260408120548290101561041157600080fd5b33600160a060020a031684600160a060020a03161415801561045b5750600160a060020a038085166000908152600460209081526040808320339094168352929052205460001914155b156104c257600160a060020a03808516600090815260046020908152604080832033909416835292905220548290101561049457600080fd5b600160a060020a03808516600090815260046020908152604080832033909416835292905220805483900390555b600160a060020a038085166000818152600360205260408082208054879003905592861680825290839020805486019055917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9085905190815260200160405180910390a35060019392505050565b600160a060020a0333166000908152600360205260409020548190101561055757600080fd5b600160a060020a033316600081815260036020526040908190208054849003905582156108fc0290839051600060405180830381858888f19350505050151561059f57600080fd5b33600160a060020a03167f7fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b658260405190815260200160405180910390a250565b60025460ff1681565b60036020526000908152604090205481565b60018054600181600116156101000203166002900480601f0160208091040260200160405190810160405280929190818152602001828054600181600116156101000203166002900480156103695780601f1061033e57610100808354040283529160200191610369565b60006106723384846103eb565b9392505050565b6004602090815260009283526040808420909152908252902054815600a165627a7a72305820976c9c45a8c1e47424c3304cee5b065aefb0c6539e9fb6b31dc3eee2abf17f650029");
    Ok(())
}

#[test]
fn test_get_transaction_by_hash() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER
        .get_transaction_by_hash(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        )?
        .is_none());
    assert!(PROVIDER
        .get_transaction_by_hash(
            "0xefdd363eae1829b4e57bd7e19975adfe471b8639b4ffa1b5ce511b7960525b79"
        )?
        .is_some());
    Ok(())
}

#[test]
fn test_get_transaction_by_block_hash_and_index() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER
        .get_transaction_by_block_hash_and_index(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            U256::from(1)
        )?
        .is_none());
    assert!(PROVIDER
        .get_transaction_by_block_hash_and_index(
            "0x4938120f0baffd265200d757b6da74e1d80e0a82ff0ed3d7eb3277613ce6f4a4",
            U256::from(1)
        )?
        .is_some());
    assert!(PROVIDER
        .get_transaction_by_block_number_and_index(U256::from("0x7FFFFFFFFFFFFFFF"), U256::from(1))?
        .is_none());
    Ok(())
}

#[test]
fn test_get_transaction_by_block_number_and_index() -> Result<(), Box<dyn Error>> {
    PROVIDER
        .get_transaction_by_block_number_and_index(
            U256::from(PROVIDER.block_number().unwrap()),
            U256::from(1),
        )
        .unwrap(); // some blocks may have no transactions
    assert!(PROVIDER
        .get_transaction_by_block_number_and_index(U256::from(2893700), U256::from(1))?
        .is_some());
    assert!(PROVIDER
        .get_transaction_by_block_number_and_index(U256::from("0x7FFFFFFFFFFFFFFF"), U256::from(1))?
        .is_none());
    Ok(())
}

#[test]
fn test_get_transaction_receipt() -> Result<(), Box<dyn Error>> {
    assert!(PROVIDER
        .get_transaction_receipt(
            "0x10e8caafb752c4b611c51dfa784168eebbf1b2819523ea6e8cdf7452552ef6c3"
        )?
        .is_some());
    assert!(PROVIDER
        .get_transaction_receipt(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        )?
        .is_none());
    Ok(())
}
