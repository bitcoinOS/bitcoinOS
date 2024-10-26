use crate::repositories;

pub(super) fn serve() {
    repositories::sequencer::increment_one();
}
