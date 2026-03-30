use crate::actor::components::TeamId;

#[derive(Debug, PartialEq)]
pub enum TeamStanding {
    Friendly,
    Neutral,
    Hostile,
}

pub fn get_standing(from: &TeamId, toward: &TeamId) -> TeamStanding {
    match (from, toward) {
        (TeamId::Player, TeamId::Player) => TeamStanding::Friendly,
        (TeamId::Player, TeamId::Zombie) => TeamStanding::Hostile,
        (TeamId::Zombie, TeamId::Player) => TeamStanding::Hostile,
        (TeamId::Zombie, TeamId::Zombie) => TeamStanding::Neutral,
    }
}
