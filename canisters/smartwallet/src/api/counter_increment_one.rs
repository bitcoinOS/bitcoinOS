use crate::repositories;

pub(super) fn serve() {
    repositories::counter::increment_one();
}
