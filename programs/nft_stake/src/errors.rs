use anchor_lang::error_code;

#[error_code]

pub enum StakeError{
    #[msg("Max stake reached")]
    MaxStakeReached,
    #[msg("Freeze period did not pass!")]
    FreezePeriodNotPassed,
    #[msg("Timestamp error!")]
    TimestampErr
}