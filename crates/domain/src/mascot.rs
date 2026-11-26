use std::fmt;
use std::str::FromStr;

/// The room's animated symbol, picked by the host from a fixed catalog of original
/// characters (assets in `apps/web/static/mascots/{webp,gif}/<name>.*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mascot {
    Alien,
    Astronaut,
    Cybercat,
    Dragon,
    Gamer,
    Ghost,
    Knight,
    Ninja,
    Pirate,
    Robot,
    Slime,
    #[default]
    Wizard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unknown mascot")]
pub struct MascotError;

impl Mascot {
    pub const ALL: [Self; 12] = [
        Self::Alien,
        Self::Astronaut,
        Self::Cybercat,
        Self::Dragon,
        Self::Gamer,
        Self::Ghost,
        Self::Knight,
        Self::Ninja,
        Self::Pirate,
        Self::Robot,
        Self::Slime,
        Self::Wizard,
    ];

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alien => "alien",
            Self::Astronaut => "astronaut",
            Self::Cybercat => "cybercat",
            Self::Dragon => "dragon",
            Self::Gamer => "gamer",
            Self::Ghost => "ghost",
            Self::Knight => "knight",
            Self::Ninja => "ninja",
            Self::Pirate => "pirate",
            Self::Robot => "robot",
            Self::Slime => "slime",
            Self::Wizard => "wizard",
        }
    }
}

impl FromStr for Mascot {
    type Err = MascotError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|mascot| mascot.as_str() == raw)
            .ok_or(MascotError)
    }
}

impl fmt::Display for Mascot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_mascot_round_trips_by_name() {
        for mascot in Mascot::ALL {
            assert_eq!(mascot.as_str().parse::<Mascot>(), Ok(mascot));
        }
    }

    #[test]
    fn unknown_names_are_rejected() {
        assert_eq!("pikachu".parse::<Mascot>(), Err(MascotError));
        assert_eq!("Wizard".parse::<Mascot>(), Err(MascotError));
    }
}
