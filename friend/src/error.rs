use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum SocialError {
    InvalidInstruction,
    UserNotFound,
}

impl From<SocialError> for ProgramError {
    fn from(e: SocialError) -> ProgramError {
        match e {
            SocialError::InvalidInstruction => ProgramError::InvalidInstructionData,
            SocialError::UserNotFound => ProgramError::InvalidAccountData,
        }
    }
}
