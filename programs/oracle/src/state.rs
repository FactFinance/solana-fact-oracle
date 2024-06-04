use anchor_lang::prelude::*;

/// Oracle controler account
#[account]
pub struct Settings {
    /// Public key of the owner who controls this data feed.
    pub owner: Pubkey,  
    /// Subscription price
    pub price: u64,  
}

/// Represents a data feed account, storing the current value, timestamp, license type, and owner.
/// This struct defines the core data associated with each feed.
#[account]
pub struct DataFeed {    
    /// The current value of the data feed.
    pub value: i32,
    /// The timestamp of the latest update to the data feed.
    pub timestamp: u32,
    /// License type: 0 for free access, 1 for subscription required.
    pub license: u8,
      
}

/// Represents a feed auditor account, storing the auditor's public key and the permissible value range.
/// Auditors are authorized to set the allowed min and max values for data feeds they audit.
#[account]
pub struct FeedAuditor {    
    /// Public key of the auditor.
    pub auditor: Pubkey,
    /// Minimum allowed value for the data feed.
    pub min: i32,
    /// Maximum allowed value for the data feed.
    pub max: i32,
}

/// Represents a list of subscribers allowed to access data feeds.
/// This struct is used to manage the subscribers who are permitted to query the oracle for data.
#[account]
pub struct Subscribers {    
    /// Vector of public keys representing the subscribers.
    pub subscribers: Vec<Pubkey>,
}

/// Represents a score of a duel
pub struct DuelFeed {    
    pub player1: String,    
    pub score1: i32,    
    pub player2: String,    
    pub score2: i32,    
    pub winner: i8,    
    pub timestamp: u32,    
}