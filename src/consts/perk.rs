use std::fmt::Formatter;

#[repr(u16)]
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Perk {
    UNKNOWN = 0,
    StatsHealScaling = 5001,
    StatsArmor = 5002,
    StatsMagicResist = 5003,
    StatsAttackSpeed = 5005,
    StatsAbilityHaste = 5007,
    StatsAdaptiveForce = 5008,
    StatsMovementSpeed = 5010,
    StatsHealth = 5011,
    StatsResistScaling = 5012,
    StatsTenacitySlowResist = 5013,
    Domination = 8100,
    Electrocute = 8112,
    DarkHarvest = 8128,
    HailOfBlades = 9923,
    CheapShot = 8126,
    TasteOfBlood = 8139,
    SuddenImpact = 8143,
    ZombieWard = 8136,
    GhostPoro = 8120,
    EyeballCollection = 8138,
    RavenousHunter = 8135,
    IngeniousHunter = 8134,
    RelentlessHunter = 8105,
    UltimateHunter = 8106,
    Inspiration = 8300,
    GlacialAugment = 8351,
    UnsealedSpellbook = 8360,
    FirstStrike = 8369,
    HextechFlashtraption = 8306,
    MagicalFootwear = 8304,
    CashBack = 8321,
    PerfectTiming = 8313,
    TimeWarpTonic = 8352,
    BiscuitDelivery = 8345,
    CosmicInsight = 8347,
    ApproachVelocity = 8410,
    JackOfAllTrades = 8316,
    Precision = 8000,
    PressTheAttack = 8005,
    LethalTempo = 8008,
    FleetFootwork = 8021,
    Conqueror = 8010,
    AbsorbLife = 9101,
    Triumph = 9111,
    PresenceOfMind = 8009,
    LegendAlacrity = 9104,
    LegendHaste = 9105,
    LegendBloodline = 9103,
    CoupDeGrace = 8014,
    CutDown = 8017,
    LastStand = 8299,
    Resolve = 8400,
    GraspOfTheUndying = 8437,
    Aftershock = 8439,
    Guardian = 8465,
    Demolish = 8446,
    FontOfLife = 8463,
    ShieldBash = 8401,
    Conditioning = 8429,
    SecondWind = 8444,
    BonePlating = 8473,
    Overgrowth = 8451,
    Revitalize = 8453,
    Unflinching = 8242,
    Sorcery = 8200,
    SummonAery = 8214,
    ArcaneComet = 8229,
    PhaseRush = 8230,
    NullifyingOrb = 8224,
    ManaflowBand = 8226,
    NimbusCloack = 8275,
    Transcendence = 8210,
    Celerity = 8234,
    AbsoluteFocus = 8233,
    Scorch = 8237,
    Waterwalking = 8232,
    GatheringStorm = 8236,
}

impl From<u16> for Perk {
    fn from(value: u16) -> Self {
        match value {
            5001 => Perk::StatsHealScaling,
            5002 => Perk::StatsArmor,
            5003 => Perk::StatsMagicResist,
            5005 => Perk::StatsAttackSpeed,
            5007 => Perk::StatsAbilityHaste,
            5008 => Perk::StatsAdaptiveForce,
            5010 => Perk::StatsMovementSpeed,
            5011 => Perk::StatsHealth,
            5012 => Perk::StatsResistScaling,
            5013 => Perk::StatsTenacitySlowResist,
            8100 => Perk::Domination,
            8112 => Perk::Electrocute,
            8128 => Perk::DarkHarvest,
            9923 => Perk::HailOfBlades,
            8126 => Perk::CheapShot,
            8139 => Perk::TasteOfBlood,
            8143 => Perk::SuddenImpact,
            8136 => Perk::ZombieWard,
            8120 => Perk::GhostPoro,
            8138 => Perk::EyeballCollection,
            8135 => Perk::RavenousHunter,
            8134 => Perk::IngeniousHunter,
            8105 => Perk::RelentlessHunter,
            8106 => Perk::UltimateHunter,
            8300 => Perk::Inspiration,
            8351 => Perk::GlacialAugment,
            8360 => Perk::UnsealedSpellbook,
            8369 => Perk::FirstStrike,
            8306 => Perk::HextechFlashtraption,
            8304 => Perk::MagicalFootwear,
            8321 => Perk::CashBack,
            8313 => Perk::PerfectTiming,
            8352 => Perk::TimeWarpTonic,
            8345 => Perk::BiscuitDelivery,
            8347 => Perk::CosmicInsight,
            8410 => Perk::ApproachVelocity,
            8316 => Perk::JackOfAllTrades,
            8000 => Perk::Precision,
            _ => Perk::UNKNOWN,
        }
    }
}

impl std::fmt::Display for Perk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Perk::UNKNOWN => "UNKNOWN",
            Perk::StatsHealScaling => "StatsHealScaling",
            Perk::StatsArmor => "StatsArmor ",
            Perk::StatsMagicResist => "StatsMagicResist",
            Perk::StatsAttackSpeed => "StatsAttackSpeed ",
            Perk::StatsAbilityHaste => "StatsAbilityHaste",
            Perk::StatsAdaptiveForce => "StatsAdaptiveForce ",
            Perk::StatsMovementSpeed => "StatsMovementSpeed",
            Perk::StatsHealth => "StatsHealth ",
            Perk::StatsResistScaling => "StatsResistScaling",
            Perk::StatsTenacitySlowResist => "StatsTenacitySlowResist ",
            Perk::Domination => "Domination",
            Perk::Electrocute => "Electrocute ",
            Perk::DarkHarvest => "DarkHarvest",
            Perk::HailOfBlades => "HailOfBlades ",
            Perk::CheapShot => "CheapShot",
            Perk::TasteOfBlood => "TasteOfBlood ",
            Perk::SuddenImpact => "SuddenImpact",
            Perk::ZombieWard => "ZombieWard ",
            Perk::GhostPoro => "GhostPoro",
            Perk::EyeballCollection => "EyeballCollection ",
            Perk::RavenousHunter => "RavenousHunter",
            Perk::IngeniousHunter => "IngeniousHunter ",
            Perk::RelentlessHunter => "RelentlessHunter",
            Perk::UltimateHunter => "UltimateHunter ",
            Perk::Inspiration => "Inspiration",
            Perk::GlacialAugment => "GlacialAugment ",
            Perk::UnsealedSpellbook => "UnsealedSpellbook",
            Perk::FirstStrike => "FirstStrike ",
            Perk::HextechFlashtraption => "HextechFlashtraption",
            Perk::MagicalFootwear => "MagicalFootwear ",
            Perk::CashBack => "CashBack",
            Perk::PerfectTiming => "PerfectTiming ",
            Perk::TimeWarpTonic => "TimeWarpTonic",
            Perk::BiscuitDelivery => "BiscuitDelivery ",
            Perk::CosmicInsight => "CosmicInsight",
            Perk::ApproachVelocity => "ApproachVelocity ",
            Perk::JackOfAllTrades => "JackOfAllTrades",
            Perk::Precision => "Precision ",
            Perk::PressTheAttack => "PressTheAttack",
            Perk::LethalTempo => "LethalTempo ",
            Perk::FleetFootwork => "FleetFootwork",
            Perk::Conqueror => "Conqueror ",
            Perk::AbsorbLife => "AbsorbLife",
            Perk::Triumph => "Triumph ",
            Perk::PresenceOfMind => "PresenceOfMind",
            Perk::LegendAlacrity => "LegendAlacrity ",
            Perk::LegendHaste => "LegendHaste",
            Perk::LegendBloodline => "LegendBloodline ",
            Perk::CoupDeGrace => "CoupDeGrace",
            Perk::CutDown => "CutDown ",
            Perk::LastStand => "LastStand",
            Perk::Resolve => "Resolve ",
            Perk::GraspOfTheUndying => "GraspOfTheUndying",
            Perk::Aftershock => "Aftershock ",
            Perk::Guardian => "Guardian",
            Perk::Demolish => "Demolish ",
            Perk::FontOfLife => "FontOfLife",
            Perk::ShieldBash => "ShieldBash ",
            Perk::Conditioning => "Conditioning",
            Perk::SecondWind => "SecondWind ",
            Perk::BonePlating => "BonePlating",
            Perk::Overgrowth => "Overgrowth ",
            Perk::Revitalize => "Revitalize",
            Perk::Unflinching => "Unflinching ",
            Perk::Sorcery => "Sorcery",
            Perk::SummonAery => "SummonAery ",
            Perk::ArcaneComet => "ArcaneComet",
            Perk::PhaseRush => "PhaseRush ",
            Perk::NullifyingOrb => "NullifyingOrb",
            Perk::ManaflowBand => "ManaflowBand ",
            Perk::NimbusCloack => "NimbusCloack",
            Perk::Transcendence => "Transcendence ",
            Perk::Celerity => "Celerity",
            Perk::AbsoluteFocus => "AbsoluteFocus ",
            Perk::Scorch => "Scorch",
            Perk::Waterwalking => "Waterwalking ",
            Perk::GatheringStorm => "GatheringStorm",
        };
        write!(f, "{}", str)
    }
}
