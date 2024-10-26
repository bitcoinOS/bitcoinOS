use candid::Principal;

use crate::{
    domain::{BoxRecord, BoxStatus},
    repositories,
};

pub fn serve(user_id: Principal, box_status: BoxStatus) -> Option<Vec<BoxRecord>> {
    repositories::box_record::get_user_boxes_by_status(user_id, box_status)
}
