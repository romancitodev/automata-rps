use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mob {
    Rock,
    Paper,
    Scissor,
}

impl Mob {
    pub fn image_path(&self) -> &str {
        match self {
            Self::Rock => "sprites/rock.png",
            Self::Paper => "sprites/paper.png",
            Self::Scissor => "sprites/scissors.png",
        }
    }
}

/// Marker data
#[derive(Component)]
pub struct Rock;
/// Marker data
#[derive(Component)]
pub struct Paper;
/// Marker data
#[derive(Component)]
pub struct Scissors;

// Logic

// The entity will attack if:
// [X] 1. Is exists allies and is close to him (2 allies at minimum). Else the entity goes for the next close allie.
// [X] 2. In case of the allies died, the entity might go for the first enemy in his trackline.

// The entity always must try to escape from their opponents.
