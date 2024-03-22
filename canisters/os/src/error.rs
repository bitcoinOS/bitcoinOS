use candid::CandidType;

#[derive(Debug, CandidType)]
pub enum Error {
    AlreadyExists,
    CreateCanisterFailed { msg: String },
    WriteError { msg: String },
    Unknown,
}
