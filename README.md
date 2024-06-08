![Fact Finance](https://fact.finance/_next/static/media/Logo.446023b4.svg)

# Fact Finance Oracle 

> Fact Oracle is the main Rust/Anchor program that provides a set of functions to create, update and access an Oracle on the Solana blockchain.

## Main functions

### set_license()
    // Sets the license of the data feed
    // 0 for OpenOracle
    // 1 for Subscription required
    @param ctx: Context<SetLicense>, 
    @param license: u8

### set_value()
    // Set/update the value of the data feed
    @param ctx: Context<SetValue>, 
    @param value: i32, 
    @param timestamp: u32, 
    @param symbol: String
    @return //none

### subscribe()
    // Users can subscribe the oracle paying with BONK
    @param address // consumer program 

### get_datafeed()
    // Get data from the data feed
    // returns value, timestamp , confidendIndex
    // i32, u32, u8    
    
### add_subscription()
    // Admin access to adds subscription 
    
### revoke_subscription()
    // Admin access to revoke a subscription 

## State structure
    DataFeed 
    FeedAudtior
    Subscribers
    Settings

## Running Oracle Address
  - DevNet https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=devnet
  - TestNet https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=devnet
    
## Installation

```bash
  git clone https://github.com/FactFinance/SolanaOracle.git
```

Then, install the dependencies:

```bash
  cd solanaoracle
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


## Be in touch
  - https://fact.finance
  - https://twitter.com/TheFactOracle

## License

MIT
