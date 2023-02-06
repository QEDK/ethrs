use ethrs::provider::DefaultBlockParam;
use ethrs::provider::Provider;
use std::error::Error;

#[test]
fn test_block_number() -> Result<(), Box<dyn Error>> {
    let provider: Provider = Provider::new("https://rpc.sepolia.org");
    provider.block_number().unwrap();
    Ok(())
}

#[test]
fn test_get_balance() -> Result<(), Box<dyn Error>> {
    let provider: Provider = Provider::new("https://rpc.sepolia.org");
    provider
        .get_balance("0x0000000000000000000000000000000000000000", None, None)
        .unwrap();
    provider
        .get_balance(
            "0x0000000000000000000000000000000000000000",
            Some(DefaultBlockParam::EARLIEST),
            None,
        )
        .unwrap();
    provider
        .get_balance(
            "0x0000000000000000000000000000000000000000",
            Some(DefaultBlockParam::LATEST),
            None,
        )
        .unwrap();
    provider
        .get_balance(
            "0x0000000000000000000000000000000000000000",
            Some(DefaultBlockParam::PENDING),
            None,
        )
        .unwrap();
    provider
        .get_balance(
            "0x0000000000000000000000000000000000000000",
            None,
            Some(provider.block_number().unwrap() - 1),
        )
        .unwrap();
    Ok(())
}
