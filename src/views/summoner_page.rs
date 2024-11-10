use crate::app::{MetaStore, MetaStoreStoreFields};
use crate::backend::server_fns::get_summoner::get_summoner;
use crate::backend::server_fns::update_summoner::UpdateSummoner;
use crate::consts::{PlatformRoute, ProfileIcon};
use crate::views::summoner_page::summoner_nav::SummonerNav;
use leptos::context::provide_context;
use leptos::either::Either;
use leptos::prelude::{expect_context, OnAttribute, Set};
use leptos::prelude::{signal, ElementChild};
use leptos::prelude::{ActionForm, ClassAttribute, Get, Read, ServerAction, Suspend, Transition};
use leptos::server::Resource;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use crate::summoner_url;

pub mod summoner_search_page;
pub mod summoner_matches_page;
pub mod summoner_champions_page;
pub mod summoner_encounters_page;
pub mod summoner_live_page;
pub mod match_details;
mod summoner_nav;

#[component]
pub fn SummonerPage() -> impl IntoView {
    let update_summoner_action = ServerAction::<UpdateSummoner>::new();
    let meta_store = expect_context::<reactive_stores::Store<MetaStore>>();
    let params = use_params_map();

    let platform_type = move || {
        params.read().get("platform_type").clone().unwrap_or_default()
    };
    let summoner_slug = move || {
        params.read().get("summoner_slug").clone().unwrap_or_default()
    };


    // Update the summoner signal when resource changes
    let summoner_resource = Resource::new_blocking(
        move || (update_summoner_action.version().get(), platform_type(), summoner_slug()),
        |(_, pt, ss)| async move {
            //log!("Client::Fetching summoner: {}", ss);
            get_summoner(pt, ss).await
        },
    );


    let summoner_view = move || {
        Suspend::new(async move {
            match summoner_resource.await {
                Ok(summoner) => {
                    Either::Left({
                        let (summoner_signal, _) = signal(summoner.clone());
                        provide_context(summoner_signal);
                        provide_context(update_summoner_action.version());
                        meta_store.image().set(ProfileIcon::get_static_url(summoner.profile_icon_id));
                        view! {
                            <div class="flex justify-center">
                                <div class="flex justify-between w-[768px] mb-2">
                                    <div class="flex  mt-2 space-x-2">
                                        <img
                                            alt="Profile Icon"
                                            src=ProfileIcon::get_static_url(summoner.profile_icon_id)
                                            class="w-16 h-16"
                                        />
                                        <div class="flex flex-col items-start">
                                            <div>
                                                {summoner_signal().game_name}#{summoner_signal().tag_line}
                                            </div>
                                            <div>lvl. {summoner_signal().summoner_level}</div>
                                        </div>
                                        <ActionForm action=update_summoner_action>
                                            <input
                                                type="hidden"
                                                name="id"
                                                value=move || summoner_signal().id
                                            />
                                            <input
                                                type="hidden"
                                                name="puuid"
                                                value=move || summoner_signal().puuid.clone()
                                            />
                                            <input
                                                type="hidden"
                                                name="platform_type"
                                                value=move || summoner_signal().platform.as_region_str()
                                            />
                                            <button class="my-button" type="submit">
                                                Update
                                            </button>
                                        </ActionForm>
                                    </div>
                                </div>

                            </div>
                            <SummonerNav />
                        }
                    })
                }
                Err(_) => { Either::Right(()) }
            }
        })
    };

    view! {
        <Transition fallback=move || {
            view! { <div>"Loading summoner ..."</div> }
        }>{summoner_view}</Transition>
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Summoner {
    pub id: i32,
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub platform: PlatformRoute,
    pub updated_at: String,
    pub summoner_level: i64,
    pub profile_icon_id: i32,
}


impl Summoner {
    pub fn to_route_path(&self) -> String {
        summoner_url(self.platform.as_region_str(), &self.game_name, &self.tag_line)
    }


    /// Generates a slug from the game name and tag line.


    /// Returns the URL of the summoner's profile icon.
    pub fn profile_icon_url(&self) -> String {
        format!(
            "https://raw.communitydragon.org/latest/game/assets/ux/summonericons/profileicon{}.png",
            self.profile_icon_id
        )
    }
}
