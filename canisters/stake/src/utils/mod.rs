use candid::Principal;

pub fn get_token_hash(canister_id: &Principal, token_id: &u32) -> String {
    let mut result = [0u8; 18];
    result[0..4].copy_from_slice(b"\x0Atid");
    result[4..14].copy_from_slice(canister_id.as_slice());
    result[14..18].copy_from_slice(&(token_id).to_be_bytes());
    let token_identifier = Principal::try_from(&result.to_vec()).unwrap();
    token_identifier.to_text()
}
