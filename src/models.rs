#[derive(Queryable)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub freq_days: i32,
    pub last_seen: Option<String>,
}
