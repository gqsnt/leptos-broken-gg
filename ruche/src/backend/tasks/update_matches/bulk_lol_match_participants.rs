use crate::backend::ssr::AppResult;
use crate::backend::tasks::update_matches::TempParticipant;
use itertools::Itertools;

pub async fn bulk_insert_lol_match_participants(
    db: &sqlx::PgPool,
    participants: &[TempParticipant],
) -> AppResult<()> {
    let (
        champion_ids,
        summoner_ids,
        match_ids,
        summoner_spell1_ids,
        summoner_spell2_ids,
        team_ids,
        won_flags,
        champ_levels,
        kill_participations,
        kdas,
    ): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = participants
        .iter()
        .map(|p| {
            (
                p.champion_id,
                p.summoner_id,
                p.lol_match_id,
                p.summoner_spell1_id,
                p.summoner_spell2_id,
                p.team_id,
                p.won,
                p.champ_level,
                p.kill_participation,
                p.kda,
            )
        })
        .multiunzip();

    let (
        killss,
        deathss,
        assistss,
        damage_dealt_to_championss,
        damage_takens,
        gold_earneds,
        wards_placeds,
        css,
        css_per_minute,
        double_kills,
    ): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = participants
        .iter()
        .map(|p| {
            (
                p.kills,
                p.deaths,
                p.assists,
                p.damage_dealt_to_champions,
                p.damage_taken,
                p.gold_earned,
                p.wards_placed,
                p.cs,
                p.cs_per_minute,
                p.double_kills,
            )
        })
        .multiunzip();

    let (
        triple_kills,
        quadra_kills,
        penta_kills,
        perk_defense_ids,
        perk_flex_ids,
        perk_offense_ids,
        perk_primary_style_ids,
        perk_sub_style_ids,
        perk_primary_selection_ids,
        perk_primary_selection1_ids,
    ): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = participants
        .iter()
        .map(|p| {
            (
                p.triple_kills,
                p.quadra_kills,
                p.penta_kills,
                p.perk_defense_id,
                p.perk_flex_id,
                p.perk_offense_id,
                p.perk_primary_style_id,
                p.perk_sub_style_id,
                p.perk_primary_selection_id,
                p.perk_primary_selection1_id,
            )
        })
        .multiunzip();

    let (
        perk_primary_selection2_ids,
        perk_primary_selection3_ids,
        perk_sub_selection1_ids,
        perk_sub_selection2_ids,
        item0_ids,
        item1_ids,
        item2_ids,
        item3_ids,
        item4_ids,
        item5_ids,
        item6_ids,
    ): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = participants
        .iter()
        .map(|p| {
            (
                p.perk_primary_selection2_id,
                p.perk_primary_selection3_id,
                p.perk_sub_selection1_id,
                p.perk_sub_selection2_id,
                p.item0_id,
                p.item1_id,
                p.item2_id,
                p.item3_id,
                p.item4_id,
                p.item5_id,
                p.item6_id,
            )
        })
        .multiunzip();

    let sql = r#"
            INSERT INTO lol_match_participants (
                champion_id,
                summoner_id,
                lol_match_id,
                summoner_spell1_id,
                summoner_spell2_id,
                team_id,
                won,
                champ_level,
                kill_participation,
                kda,
                kills,
                deaths,
                assists,
                damage_dealt_to_champions,
                damage_taken,
                gold_earned,
                wards_placed,
                cs,
                cs_per_minute,
                double_kills,
                triple_kills,
                quadra_kills,
                penta_kills,
                perk_defense_id,
                perk_flex_id,
                perk_offense_id,
                perk_primary_style_id,
                perk_sub_style_id,
                perk_primary_selection_id,
                perk_primary_selection1_id,
                perk_primary_selection2_id,
                perk_primary_selection3_id,
                perk_sub_selection1_id,
                perk_sub_selection2_id,
                item0_id,
                item1_id,
                item2_id,
                item3_id,
                item4_id,
                item5_id,
                item6_id
            )
            SELECT * FROM UNNEST (
                $1::INT[],
                $2::INT[],
                $3::INT[],
                $4::INT[],
                $5::INT[],
                $6::INT[],
                $7::BOOL[],
                $8::INT[],
                $9::FLOAT8[],
                $10::FLOAT8[],
                $11::INT[],
                $12::INT[],
                $13::INT[],
                $14::INT[],
                $15::INT[],
                $16::INT[],
                $17::INT[],
                $18::INT[],
                $19::FLOAT8[],
                $20::INT[],
                $21::INT[],
                $22::INT[],
                $23::INT[],
                $24::INT[],
                $25::INT[],
                $26::INT[],
                $27::INT[],
                $28::INT[],
                $29::INT[],
                $30::INT[],
                $31::INT[],
                $32::INT[],
                $33::INT[],
                $34::INT[],
                $35::INT[],
                $36::INT[],
                $37::INT[],
                $38::INT[],
                $39::INT[],
                $40::INT[],
                $41::INT[]
            );
        "#;

    sqlx::query(sql)
        .bind(&champion_ids)
        .bind(&summoner_ids)
        .bind(&match_ids)
        .bind(&summoner_spell1_ids)
        .bind(&summoner_spell2_ids)
        .bind(&team_ids)
        .bind(&won_flags)
        .bind(&champ_levels)
        .bind(&kill_participations)
        .bind(&kdas)
        .bind(&killss)
        .bind(&deathss)
        .bind(&assistss)
        .bind(&damage_dealt_to_championss)
        .bind(&damage_takens)
        .bind(&gold_earneds)
        .bind(&wards_placeds)
        .bind(&css)
        .bind(&css_per_minute)
        .bind(&double_kills)
        .bind(&triple_kills)
        .bind(&quadra_kills)
        .bind(&penta_kills)
        .bind(&perk_defense_ids)
        .bind(&perk_flex_ids)
        .bind(&perk_offense_ids)
        .bind(&perk_primary_style_ids)
        .bind(&perk_sub_style_ids)
        .bind(&perk_primary_selection_ids)
        .bind(&perk_primary_selection1_ids)
        .bind(&perk_primary_selection2_ids)
        .bind(&perk_primary_selection3_ids)
        .bind(&perk_sub_selection1_ids)
        .bind(&perk_sub_selection2_ids)
        .bind(&item0_ids)
        .bind(&item1_ids)
        .bind(&item2_ids)
        .bind(&item3_ids)
        .bind(&item4_ids)
        .bind(&item5_ids)
        .bind(&item6_ids)
        .execute(db)
        .await?;

    Ok(())
}
