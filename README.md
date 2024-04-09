![Fact Finance](https://fact.finance/_next/static/media/Logo.446023b4.svg)

# Fact Finance Oracle =

> Oracle is the main Rust program that provides a set of functions to create, update and access an Oracle  on the Solana blockchain.

## Main functions

### set_license()
    // Sets the license of the data feed
    // 0 for OpenOracle
    // 1 for Subscription required
    @param ctx: Context<SetLicense>, 
    @param license: u8

### add_subscription()
    // Adds a subscription to the data feed
    
### revoke_subscription()
    // Revokes a subscription from the data feed

### set_value()
    // Set/update the value of the data feed
    @param ctx: Context<SetValue>, 
    @param value: i32, 
    @param timestamp: u32, 
    @param symbol: String) 
    @return //none

### get_datafeed()
    // Gets data from the data feed
    // returns value, timestamp , confidendIndex
    // i32, u32, u8

## DataFeed account
The string format to create the address is:
  - wallet.publicKey.toBuffer()
  - Buffer.from("_")
  - Buffer.from(feedid.toString())]
  - oracle.programId

## Running Oracle Address
  - DevNet https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=devnet
  - TestNet https://explorer.solana.com/address/9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1?cluster=devnet
    
## Installation

```bash
  git clone https://github.com/FactFinance/Oracle.git
```

Then, install the dependencies:

```bash
  cd oracle
  npm install
```


Run:

```bash
    anchor run test
```

## Be in touch
  - https://fact.finance
  - https://twitter.com/TheFactOracle

## License

MIT
