#[derive(Debug)]
pub struct Timer {
    pub name: Option<String>,
    pub description: Option<String>,
    pub schedule: Option<String>,
    pub executable: Option<String>,
    pub exec_if_missed: Option<bool>,
    pub single_use: Option<bool>,
    pub recurring: Option<bool>,
    pub on_calendar: Option<bool>,
    pub from_boot: Option<bool>,
    pub normal_service: Option<bool>,
    pub service: Option<String>,
    pub already_made_service: Option<bool>,
    pub enable_at_login: Option<bool>,
    pub start_after_create: Option<bool>,
}
