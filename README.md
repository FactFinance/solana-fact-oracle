# Fact Finance Oracle 

At Fact Finance, we're reshaping web3 infrastructure with tailored, reliable real-world data. By forging strategic partnerships with official data providers, including research institutes, sports leagues, and governmental data agencies, we empower businesses to confidently navigate regional nuances. Our commitment to education and sustainability ensures a lasting impact and growth, driving tangible progress in the digital landscape. Join us in revolutionizing web3’s future—one data point at a time.

[Read more about the project](https://respected-yard-256.notion.site/About-Fact-Finance-c2c2a72cdc914fd4b3094d71fe045437)


# Demo address
Testnet - https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=testnet
Devnet - https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=devnet

## Repository Structure 

This repository provides a comprehensive set of smart contracts for Fact Finance Oracle protocol. The repository is divided into several modules: controller, oracle, auditor, store, and helpers. Each module serves a specific purpose in the overall system.

1. Settings: Manages access control and roles and price.
2. Feed: To storage and retrieval price feeds
3. Auditor: Contains contracts for auditing and verifying data.
4. Subscription: Manages oracle customers.
5. Consumer: Provides sample contract to interact with the oracle.

## How to interact with FAct Oracle

The PayPerUseOracle contract allows users to access data feeds on a pay-per-use basis. It includes functions to check feed prices, request data, and verify signatures.

Example Usage:

```rust

#[program]
mod consumer {
    use super::*;

    // Function to pull data from the Oracle program
    pub fn pull_oracle(ctx: Context<PullOracle>) -> anchor_lang::Result<()> {   
        // Calling the CPI method to get data from the Oracle program     
        let result = oracle::cpi::get_datafeed(
            CpiContext::new(
                ctx.accounts.oracle_program.to_account_info(),                
                GetDataFeed {
                    datafeed: ctx.accounts.datafeed.to_account_info(),
                    subscribers: ctx.accounts.subscribers.to_account_info(),
                    signer: ctx.accounts.signer.to_account_info(),
                },
            ),             
        );

        // Unpacking the result tuple
        let (value, timestamp, confidence) = result?.get();

        // Logging the retrieved data
        msg!("consumer value {} and timestamp {} with confidence {} ", value, timestamp, confidence);

        Ok(())
    }
}
```


## Main functions

### set_license()
```rust
    // Sets the license of the data feed
    // 0 for OpenOracle
    // 1 for Subscription required
    @param ctx: Context<SetLicense>, 
    @param license: u8
```

### add_subscription()
    // Adds a subscription to the data feed
    
### revoke_subscription()
    // Revokes a subscription from the data feed

### set_value()
```rust
    // Set/update the value of the data feed
    @param ctx: Context<SetValue>, 
    @param value: i32, 
    @param timestamp: u32, 
    @param symbol: String
    @param confidence: u8,
    @return //none
  ```

### get_datafeed()
    // Get data from the data feed
    // returns value, timestamp , confidendIndex
    // i32, u32, u8

## DataFeed account
The string format to create the address is:
  - wallet.publicKey.toBuffer()
  - Buffer.from("_")
  - Buffer.from(feedid.toString())
  - oracle.programId

## Dappi Integration
You can use the three Oracles to request a feed verification returned from https://api.fact.finance.



## Running Oracle Address
  - DevNet https://explorer.solana.com/address/FmtZYWDGyXb7qNhpsXQPZ3n23PC84sBoJGFSLuWz8uGR?cluster=devnet
  - TestNet https://explorer.solana.com/address/FmtZYWDGyXb7qNhpsXQPZ3n23PC84sBoJGFSLuWz8uGR?cluster=testnet
    
## Installation

```bash
  git clone git@github.com:FactFinance/solana-fact-oracle.git
```

Then, install the dependencies:

```bash
  cd solana-fact-oracle
  npm install
```


Compile, deploy and Run:

```bash
    anchor build && anchor deploy && anchor run test
```


Automated Test 
  
    ✔ Initialize the oracle! (40ms)
    ✔ Set Value! (731ms)
    ✔ Set License! (809ms)
    ✔ Add subscription! (810ms)
    ✔ Pull the oracle! (404ms)
    ✔ Revoke subscription! (810ms)



## License

This project is licensed under the Apache License 2.0. 

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Contact

For any questions or support, please contact us at support@fact.finance

## Be in touch
  - https://fact.finance
  - https://twitter.com/TheFactOracle

