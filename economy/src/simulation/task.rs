use super::item_count_list::ItemCountList;

#[derive(Clone)]
pub struct Task {
    pub inputs: ItemCountList,
    pub outputs: ItemCountList,
    pub work_points: u64,
}
