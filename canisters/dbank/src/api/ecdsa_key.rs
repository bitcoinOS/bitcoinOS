use crate::repositories;

pub(crate) fn serve() -> String {
    repositories::metadata::get_metadata().ecdsa_key_id.name
}
