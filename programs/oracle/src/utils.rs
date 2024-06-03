use anchor_lang::prelude::*;

pub fn check_limit(value: i32, min: i32, max: i32) -> bool {
    if min == 0 && max == 0 {
        return true;
    }

    if value >= min && value <= max {
        return true;
    }

    msg!("Value {} is out of range (min: {}, max: {})", value, min, max);
    false 
}

// Definition of custom error codes for the Oracle program
#[error_code]
pub enum OracleErrors {
    #[msg("Access denied! You do not have permission to perform this action.")]
    AccessDenied,
    
    #[msg("Subscription required! Subscribe to this feed at https://fact.finance.")]
    Subscribe,
}
