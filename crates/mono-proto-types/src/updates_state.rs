#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpdatesStateDto {
    pub pts: i32,
    pub qts: i32,
    pub date: i32,
    pub seq: i32,
}
