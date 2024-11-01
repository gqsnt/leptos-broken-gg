pub mod lol_match;
pub mod lol_match_participant;
pub mod summoner;


#[derive(sqlx::FromRow)]
struct Id {
    id: i32,
}

pub const DATE_FORMAT: &str = "%d/%m/%Y %H:%M";