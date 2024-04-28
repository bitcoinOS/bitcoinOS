use crate::repositories;

pub(crate) fn serve() {
    repositories::counter::increment_one();
}
