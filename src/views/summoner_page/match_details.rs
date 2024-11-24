use std::fmt::Formatter;
use crate::backend::server_fns::get_match_details::get_match_details;
use crate::views::summoner_page::match_details::match_details_build::MatchDetailsBuild;
use crate::views::summoner_page::match_details::match_details_overview::MatchDetailsOverview;
use crate::views::summoner_page::match_details::match_details_team::MatchDetailsTeam;
use crate::views::summoner_page::Summoner;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::server_fn::rkyv::{Deserialize, Serialize, Archive};
use leptos::{component, view, IntoView};
use crate::consts::platform_route::PlatformRoute;
use crate::utils::{string_to_fixed_array, GameName, ProPlayerSlug, RiotMatchId, TagLine};

pub mod match_details_overview;
pub mod match_details_team;
pub mod match_details_build;

#[component]
pub fn MatchDetails(match_id: i32, riot_match_id: RiotMatchId, platform: PlatformRoute, summoner: ReadSignal<Summoner>) -> impl IntoView {
    let match_details = Resource::new_rkyv(
        move || (match_id, riot_match_id, platform.clone(), summoner().id),
        |(match_id, riot_match_id, platform, summoner_id)| async move {
            get_match_details(match_id, Some(summoner_id), platform,riot_match_id ).await
        },
    );
    let (match_detail_tab, set_match_detail_tab) = signal("overview".to_string());
    let match_detail_view = Suspend::new(async move {
        match match_details.await {
            Ok(match_details) => Either::Left({
                let (match_details_signal, _) = signal(match_details);
                view! {
                    <Show when=move || match_detail_tab() == "overview">
                        <MatchDetailsOverview match_details=match_details_signal summoner />
                    </Show>
                    <Show when=move || match_detail_tab() == "team">
                        <MatchDetailsTeam
                            _match_details=match_details_signal
                            _summoner_id=summoner().id
                        />
                    </Show>
                    <Show when=move || match_detail_tab() == "build">
                        <MatchDetailsBuild
                            match_details=match_details_signal
                            summoner_id=summoner().id
                        />
                    </Show>
                }
            }),
            Err(_) => Either::Right(())
        }
    });

    view! {
        <div class="mt-2 w-full">
            <div class="flex space-x-2 mb-2">
                <button
                    on:click=move |_| set_match_detail_tab("overview".to_string())
                    class=move || {
                        if match_detail_tab() == "overview" { "active-tab" } else { "default-tab" }
                    }
                >
                    Overview
                </button>
                <button
                    on:click=move |_| set_match_detail_tab("team".to_string())
                    class=move || {
                        if match_detail_tab() == "team" { "active-tab" } else { "default-tab" }
                    }
                >
                    Team
                </button>
                <button
                    on:click=move |_| set_match_detail_tab("build".to_string())
                    class=move || {
                        if match_detail_tab() == "build" { "active-tab" } else { "default-tab" }
                    }
                >
                    Build
                </button>
            </div>
            <div>
                <Transition fallback=move || {
                    view! { <div class="text-center">Loading Match Details</div> }
                }>{match_detail_view}</Transition>
            </div>
        </div>
    }
}


#[derive(Clone, Serialize, Deserialize, Archive)]
pub struct LolMatchParticipantDetails {
    pub id: i32,
    pub lol_match_id: i32,
    pub summoner_id: i32,
    pub game_name: GameName,
    pub tag_line: TagLine,
    pub platform: PlatformRoute,
    pub summoner_pro_player_slug: Option<ProPlayerSlug>,
    pub summoner_icon_id: u16,
    pub summoner_level: u16,
    pub encounter_count: u16,
    pub champion_id: u16,
    pub team_id: u16,
    pub won: bool,
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub champ_level: u16,
    pub kda: f32,
    pub kill_participation: f32,
    pub damage_dealt_to_champions: u32,
    pub damage_taken: u32,
    pub gold_earned: u32,
    pub wards_placed: u16,
    pub cs: u16,
    pub summoner_spell1_id: u16,
    pub summoner_spell2_id: u16,
    pub perk_defense_id: u16,
    pub perk_flex_id: u16,
    pub perk_offense_id: u16,
    pub perk_primary_style_id: u16,
    pub perk_sub_style_id: u16,
    pub perk_primary_selection_id: u16,
    pub perk_primary_selection1_id: u16,
    pub perk_primary_selection2_id: u16,
    pub perk_primary_selection3_id: u16,
    pub perk_sub_selection1_id: u16,
    pub perk_sub_selection2_id: u16,
    pub item0_id: u32,
    pub item1_id: u32,
    pub item2_id: u32,
    pub item3_id: u32,
    pub item4_id: u32,
    pub item5_id: u32,
    pub item6_id: u32,
    pub items_event_timeline: Vec<(u16, Vec<ItemEvent>)>,
    pub skills_timeline: Vec<Skill>,
}


#[derive(Clone, Serialize, Deserialize, Archive)]
pub struct LolMatchTimeline {
    pub id: i32,
    pub lol_match_id: i32,
    pub summoner_id: i32,
    pub items_event_timeline: Vec<(u16, Vec<ItemEvent>)>,
    pub skills_timeline: Vec<Skill>,
}


#[cfg_attr(feature = "ssr", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Deserialize, Serialize, Archive)]
pub struct ItemEvent {
    pub item_id: u32,
    pub event_type: ItemEventType,
}


#[repr(u8)]
#[cfg_attr(feature = "ssr", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Deserialize, Serialize, Archive, PartialEq)]
pub enum ItemEventType {
    Purchased,
    Sold,
}


#[repr(u8)]
#[cfg_attr(feature = "ssr", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Deserialize, Serialize, Archive, PartialEq, Copy)]
pub enum Skill {
    Q=1,
    W=2,
    E=3,
    R=4,
}

impl std::fmt::Display for Skill{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Skill::Q => write!(f, "Q"),
            Skill::W => write!(f, "W"),
            Skill::E => write!(f, "E"),
            Skill::R => write!(f, "R"),
        }
    }
}

impl From<u8> for Skill {
    fn from(value: u8) -> Self {
        match value {
            1 => Skill::Q,
            2 => Skill::W,
            3 => Skill::E,
            4 => Skill::R,
            _ => panic!("Invalid Skill value: {}", value),
        }
    }
}



