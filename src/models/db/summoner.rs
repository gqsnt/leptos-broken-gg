use crate::consts::PlatformRoute;
use crate::error_template::{AppError, AppResult};
use crate::models::db::{Id, DATE_FORMAT};
use crate::models::entities::summoner::Summoner;
use crate::models::update::summoner_matches::TempSummoner;
use chrono::NaiveDateTime;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::Row;
use std::collections::HashMap;
use std::str::FromStr;


#[derive(sqlx::FromRow)]
pub struct IdPuuidUpdatedAt {
    id: i32,
    puuid: String,
    #[sqlx(default)]
    updated_at: Option<NaiveDateTime>,
}


#[derive(sqlx::FromRow)]
pub struct SummonerDb {
    pub id: i32,
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub platform: String,
    pub updated_at: NaiveDateTime,
    pub summoner_level: i64,
    pub profile_icon_id: i32,
}


impl Summoner {
    pub async fn find_by_exact_details(
        db: &sqlx::PgPool,
        platform_route: &PlatformRoute,
        game_name: &str,
        tag_line: &str,
    ) -> AppResult<Summoner> {
        sqlx::query_as::<_, SummonerDb>(
            "SELECT * FROM summoners WHERE game_name = $1 AND tag_line = $2 AND platform = $3 LIMIT 1"
        ).bind(game_name)
            .bind(tag_line)
            .bind(platform_route.as_region_str())
            .fetch_one(db)
            .await
            .map(|x| Self {
                id: x.id,
                game_name: x.game_name,
                tag_line: x.tag_line,
                puuid: x.puuid,
                platform: PlatformRoute::from_region_str(x.platform.as_str()).unwrap(),
                updated_at: x.updated_at.format(DATE_FORMAT).to_string(),
                summoner_level: x.summoner_level as i64,
                profile_icon_id: x.profile_icon_id,
            })
            .map_err(AppError::from)
    }


    pub async fn find_by_details(
        db: &sqlx::PgPool,
        platform_route: &PlatformRoute,
        game_name: &str,
        tag_line: &str,
    ) -> AppResult<Summoner> {
        sqlx::query_as::<_, SummonerDb>(
            "SELECT * FROM summoners WHERE LOWER(game_name) = LOWER($1) AND LOWER(tag_line) = LOWER($2) AND platform = $3"
        )
            .bind(game_name)
            .bind(tag_line)
            .bind(platform_route.as_region_str())

            .fetch_one(db)
            .await
            .map(|x| Self {
                id: x.id,
                game_name: x.game_name,
                tag_line: x.tag_line,
                puuid: x.puuid,
                platform: PlatformRoute::from_region_str(x.platform.as_str()).unwrap(),
                updated_at: x.updated_at.format(DATE_FORMAT).to_string(),
                summoner_level: x.summoner_level as i64,
                profile_icon_id: x.profile_icon_id,
            })
            .map_err(AppError::from)
    }

    pub async fn fetch_existing_summoners(
        db: &sqlx::PgPool,
        puuids: &[String],
    ) -> AppResult<HashMap<String, (i32, i32)>> {
        Ok(sqlx::query_as::<_, IdPuuidUpdatedAt>("
            SELECT id, puuid, updated_at
            FROM summoners
            WHERE puuid = ANY($1)
        ")
            .bind(puuids)

            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| {
                (row.puuid, (row.id, row.updated_at.unwrap().and_utc().timestamp() as i32))
            })
            .collect::<HashMap<String, (i32, i32)>>())
    }


    pub async fn bulk_update(db: &sqlx::PgPool, summoners: &[TempSummoner]) -> AppResult<()> {
        let game_names = summoners.iter().map(|x| x.game_name.clone()).collect::<Vec<String>>();
        let tag_lines = summoners.iter().map(|x| x.tag_line.clone()).collect::<Vec<String>>();
        let puuids = summoners.iter().map(|x| x.puuid.clone()).collect::<Vec<String>>();
        let platforms = summoners.iter().map(|x| x.platform.to_string()).collect::<Vec<String>>();
        let summoner_levels = summoners.iter().map(|x| x.summoner_level).collect::<Vec<i64>>();
        let profile_icon_ids = summoners.iter().map(|x| x.profile_icon_id).collect::<Vec<i32>>();
        let updated_ats = summoners.iter().map(|x| x.updated_at).collect::<Vec<DateTime<Utc>>>();

        let sql = r"
        UPDATE summoners
        SET
            game_name = data.game_name,
            tag_line = data.tag_line,
            platform = data.platform,
            summoner_level = data.summoner_level,
            profile_icon_id = data.profile_icon_id,
            updated_at = data.updated_at
        FROM (
            SELECT
                unnest($1::VARCHAR(16)[]) AS game_name,
                unnest($2::VARCHAR(5)[]) AS tag_line,
                unnest($3::VARCHAR(78)[]) AS puuid,
                unnest($4::VARCHAR(4)[]) AS platform,
                unnest($5::INT[]) AS summoner_level,
                unnest($6::INT[]) AS profile_icon_id,
                unnest($7::TIMESTAMP[]) AS updated_at
        ) AS data
        WHERE summoners.puuid = data.puuid;
        ";

        sqlx::query(sql)
            .bind(game_names)
            .bind(tag_lines)
            .bind(puuids)
            .bind(platforms)
            .bind(summoner_levels)
            .bind(profile_icon_ids)
            .bind(updated_ats)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn bulk_insert(db: &sqlx::PgPool, summoners: &[TempSummoner]) -> AppResult<HashMap<String, i32>> {
        let game_names = summoners.iter().map(|x| x.game_name.clone()).collect::<Vec<String>>();
        let tag_lines = summoners.iter().map(|x| x.tag_line.clone()).collect::<Vec<String>>();
        let puuids = summoners.iter().map(|x| x.puuid.clone()).collect::<Vec<String>>();
        let platforms = summoners.iter().map(|x| x.platform.to_string()).collect::<Vec<String>>();
        let summoner_levels = summoners.iter().map(|x| x.summoner_level).collect::<Vec<i64>>();
        let profile_icon_ids = summoners.iter().map(|x| x.profile_icon_id).collect::<Vec<i32>>();
        let updated_ats = summoners.iter().map(|x| x.updated_at).collect::<Vec<DateTime<Utc>>>();
        let sql = r"
        INSERT INTO
            summoners
            (
                game_name,
                tag_line,
                puuid,
                platform,
                summoner_level,
                profile_icon_id,
                updated_at
            ) SELECT * FROM UNNEST (
                $1::VARCHAR(16)[],
                $2::VARCHAR(5)[],
                $3::VARCHAR(78)[],
                $4::VARCHAR(4)[],
                $5::INT[],
                $6::INT[],
                $7::TIMESTAMP[]
            )
            ON CONFLICT (puuid)
            DO UPDATE SET
                game_name = EXCLUDED.game_name,
                tag_line = EXCLUDED.tag_line,
                platform = EXCLUDED.platform,
                summoner_level = EXCLUDED.summoner_level,
                profile_icon_id = EXCLUDED.profile_icon_id,
                updated_at = EXCLUDED.updated_at
            WHERE summoners.updated_at < EXCLUDED.updated_at
            returning id;
        ";
        let rows = sqlx::query_as::<_, Id>(sql)
            .bind(game_names)
            .bind(tag_lines)
            .bind(puuids)
            .bind(platforms)
            .bind(summoner_levels)
            .bind(profile_icon_ids)
            .bind(updated_ats)
            .fetch_all(db)
            .await?;
        Ok(rows.into_iter().enumerate().map(|(index, r)| (summoners.get(index).unwrap().puuid.clone(), r.id)).collect::<HashMap<String, i32>>())
    }

    pub async fn find_by_id(db: &sqlx::PgPool, id: i32) -> AppResult<Summoner> {
        sqlx::query_as::<_, SummonerDb>("SELECT * FROM summoners WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await
            .map(|x| Self {
                id: x.id,
                game_name: x.game_name,
                tag_line: x.tag_line,
                puuid: x.puuid,
                platform: PlatformRoute::from_str(x.platform.as_str()).unwrap(),
                updated_at: x.updated_at.format(DATE_FORMAT).to_string(),
                summoner_level: x.summoner_level as i64,
                profile_icon_id: x.profile_icon_id,
            })
            .map_err(AppError::from)
    }


    pub async fn update_summoner_by_id(
        db: &sqlx::PgPool,
        id: i32,
        platform_route: PlatformRoute,
        account: riven::models::account_v1::Account,
        summoner: riven::models::summoner_v4::Summoner,
    ) -> AppResult<Summoner> {
        sqlx::query(
            "UPDATE summoners SET game_name = $1, tag_line = $2, puuid = $3, summoner_level = $4, profile_icon_id = $5, platform = $6 WHERE id = $7"
        )
            .bind(account.game_name.clone())
            .bind(account.tag_line.clone())
            .bind(summoner.puuid.clone())
            .bind(summoner.summoner_level as i32)
            .bind(summoner.profile_icon_id)
            .bind(platform_route.as_region_str())
            .bind(id)
            .execute(db)
            .await?;
        Ok(
            Summoner {
                id,
                game_name: account.game_name.unwrap_or_default(),
                tag_line: account.tag_line.unwrap_or_default(),
                puuid: summoner.puuid,
                platform: platform_route.into(),
                updated_at: Utc::now().format(DATE_FORMAT).to_string(),
                summoner_level: summoner.summoner_level as i64,
                profile_icon_id: summoner.profile_icon_id,
            }
        )
    }

    pub async fn insert_summoner(
        db: &sqlx::PgPool,
        platform_route: PlatformRoute,
        account: riven::models::account_v1::Account,
        summoner: riven::models::summoner_v4::Summoner,
    ) -> AppResult<Summoner> {
        let rec = sqlx::query_as::<_, Id>(
            "INSERT INTO summoners(game_name, tag_line, puuid, platform, summoner_level, profile_icon_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
        )
            .bind(account.game_name.clone())
            .bind(account.tag_line.clone())
            .bind(summoner.puuid.clone())
            .bind(platform_route.as_region_str())
            .bind(summoner.summoner_level as i32)
            .bind(summoner.profile_icon_id)
            .fetch_one(db)
            .await?;
        Ok(Summoner {
            id: rec.id,
            game_name: account.game_name.unwrap_or_default(),
            tag_line: account.tag_line.unwrap_or_default(),
            puuid: summoner.puuid,
            platform: platform_route.into(),
            updated_at: Utc::now().format(DATE_FORMAT).to_string(),
            summoner_level: summoner.summoner_level as i64,
            profile_icon_id: summoner.profile_icon_id,
        })
    }


    pub async fn find_summoner_ids_by_puuids(db: &sqlx::PgPool, platform_route: PlatformRoute, puuids: &[String]) -> AppResult<HashMap<String, i32>> {
        Ok(sqlx::query_as::<_, IdPuuidUpdatedAt>(
            "SELECT id, puuid FROM summoners WHERE puuid = ANY($1) and platform = $2"
        )
            .bind(puuids)
            .bind(platform_route.as_region_str())

            .fetch_all(db)
            .await?
            .into_iter()
            .map(|x| (x.puuid, x.id))
            .collect::<HashMap<String, i32>>())
    }

    pub async fn find_summoner_id_by_puuid(db: &sqlx::PgPool, platform_route: PlatformRoute, puuid: &str) -> AppResult<i32> {
        sqlx::query_as::<_, Id>("SELECT id FROM summoners WHERE puuid = $1 and platform = $2")
            .bind(puuid)
            .bind(platform_route.as_region_str())
            .fetch_one(db)
            .await
            .map(|x| x.id)
            .map_err(AppError::from)
    }

    pub async fn insert_or_update_account_and_summoner(
        db: &sqlx::PgPool,
        platform_route: PlatformRoute,
        account: riven::models::account_v1::Account,
        summoner: riven::models::summoner_v4::Summoner,
    ) -> AppResult<Summoner> {
        match Summoner::find_summoner_id_by_puuid(db, platform_route, &summoner.puuid).await {
            Ok(id) => {
                Summoner::update_summoner_by_id(db, id, platform_route, account, summoner).await
            }
            Err(_) => {
                Summoner::insert_summoner(db, platform_route, account, summoner).await
            }
        }
    }
}
