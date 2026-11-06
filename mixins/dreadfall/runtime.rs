// This module is injected into game.rs by horizon-mixins. It targets the
// Vasysik 0.7.7 source tree, not an independently compiled plugin ABI.
use super::Game;
use super::native_mobs::NativeSpawn;
use horizon_client::text::Text;

const INTERVAL: i64 = 600; // 30 seconds at 20 TPS
const MAX_NEARBY: usize = 6;
const OFFSETS: [(i32, i32); 8] = [
    (8, 0), (6, 6), (0, 8), (-6, 6), (-8, 0), (-6, -6), (0, -8), (6, -6),
];

fn is_valuable_ore(block: &str) -> bool {
    matches!(block,
        "minecraft:iron_ore" | "minecraft:deepslate_iron_ore" |
        "minecraft:gold_ore" | "minecraft:deepslate_gold_ore" |
        "minecraft:diamond_ore" | "minecraft:deepslate_diamond_ore" |
        "minecraft:emerald_ore" | "minecraft:deepslate_emerald_ore")
}

fn floor_is_safe(block: &str) -> bool {
    // A conservative allowlist: do not spawn on lava, air, fences or half blocks.
    matches!(block,
        "minecraft:stone" | "minecraft:deepslate" | "minecraft:dirt" |
        "minecraft:grass_block" | "minecraft:coarse_dirt" | "minecraft:rooted_dirt" |
        "minecraft:podzol" | "minecraft:gravel" | "minecraft:sand" |
        "minecraft:red_sand" | "minecraft:cobblestone" | "minecraft:moss_block")
}

fn night(day_time: i64) -> bool {
    (13000..23000).contains(&day_time.rem_euclid(24000))
}

fn warn(game: &mut Game<'_>, uuid: u128, message: &str) {
    game.native_action_bar(uuid, Text::literal(message));
}

pub(super) fn level_tick(game: &mut Game<'_>, dimension: &str) {
    if dimension != "minecraft:overworld" || !night(game.native_day_time()) {
        return;
    }
    let tick = game.world.game_time;
    if tick <= 0 || tick % INTERVAL != 0 {
        return;
    }
    let players = game.native_server_players();
    for player in players {
        if player.dimension != dimension || player.game_mode != 0 || player.health <= 0.0 {
            continue;
        }
        let [x, y, z] = player.position;
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            continue;
        }
        let origin = [x.floor() as i32, y.floor() as i32, z.floor() as i32];
        // Block light, rather than sky light, makes torches and campfires a refuge.
        if game.native_light(origin).0 >= 8 {
            continue;
        }
        let nearby = game.native_mobs(dimension).into_iter().filter(|mob| {
            matches!(mob.kind.as_str(), "minecraft:zombie" | "minecraft:skeleton" | "minecraft:spider")
                && (mob.pos[0] - x).powi(2) + (mob.pos[1] - y).powi(2)
                    + (mob.pos[2] - z).powi(2) < 32.0 * 32.0
        }).count();
        if nearby >= MAX_NEARBY {
            continue;
        }
        let wave = (tick / INTERVAL) as usize;
        let (kind, name) = match wave % 3 {
            0 => ("minecraft:zombie", "Dreadbound Revenant"),
            1 => ("minecraft:skeleton", "Dreadbound Archer"),
            _ => ("minecraft:spider", "Dreadbound Stalker"),
        };
        let options = NativeSpawn { name: Some(name.to_owned()), ..Default::default() };
        // Only spawn on a known full floor with two air blocks and a loaded
        // chunk. This deliberately skips inaccessible or unfamiliar terrain.
        'search: for offset in 0..OFFSETS.len() {
            let (dx, dz) = OFFSETS[(wave + offset) % OFFSETS.len()];
            let sx = origin[0].saturating_add(dx);
            let sz = origin[2].saturating_add(dz);
            if !game.world.loaded(sx, sz) { continue; }
            for dy in [0, -1, 1, -2, 2, -3, 3] {
                let sy = origin[1].saturating_add(dy);
                let floor = [sx, sy.saturating_sub(1), sz];
                if floor_is_safe(&game.native_block(floor))
                    && game.native_block([sx, sy, sz]) == "minecraft:air"
                    && game.native_block([sx, sy.saturating_add(1), sz]) == "minecraft:air"
                    && game.native_light([sx, sy, sz]).0 < 8
                {
                    if game.native_spawn_mob(dimension, kind, [sx as f64 + 0.5, sy as f64, sz as f64 + 0.5], &options).is_some() {
                        warn(game, player.uuid, "Dreadfall: the darkness has sent a hunter.");
                        break 'search;
                    }
                }
            }
        }
    }
}

pub(super) fn destroy_block(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled || !is_valuable_ore(&game.native_block(pos)) { return; }
    let Some(player) = game.native_acting_player() else { return; };
    if player.game_mode != 0 { return; }
    let at = [player.position[0].floor() as i32, player.position[1].floor() as i32, player.position[2].floor() as i32];
    let (block_light, sky_light) = game.native_light(at);
    // Underground the player needs a real light source; daylight also permits mining.
    let daytime = player.dimension == "minecraft:overworld" && !night(game.native_day_time());
    if block_light < 5 && !(daytime && sky_light >= 8) {
        *canceled = true;
        warn(game, player.uuid, "Dreadfall: light the darkness before mining precious ore.");
    }
}

pub(super) fn right_click(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled || game.native_block(pos) != "minecraft:soul_campfire" { return; }
    let Some(player) = game.native_acting_player() else { return; };
    if player.game_mode != 0 { return; }
    let Some((held, _, plain)) = game.native_held() else { return; };
    if held != "minecraft:bone" || !plain { return; }
    // Claim only this exact interaction, leaving ordinary campfire use untouched.
    *canceled = true;
    if game.native_count_items("minecraft:bone") < 3 {
        warn(game, player.uuid, "Dreadfall: the soul fire demands three bones.");
    } else if game.native_room_for("minecraft:golden_carrot") < 1 {
        warn(game, player.uuid, "Dreadfall: make room for the soul fire's offering.");
    } else {
        let taken = game.native_take_items("minecraft:bone", 3);
        if taken != 3 {
            let _ = game.native_give_items("minecraft:bone", taken);
            warn(game, player.uuid, "Dreadfall: the offering failed.");
            return;
        }
        let given = game.native_give_items("minecraft:golden_carrot", 1);
        if given == 1 {
            warn(game, player.uuid, "Dreadfall: three bones become a golden carrot.");
        } else {
            // Best-effort rollback if the engine unexpectedly refused the item.
            let _ = game.native_give_items("minecraft:bone", 3);
            warn(game, player.uuid, "Dreadfall: the offering failed.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn night_wraps() {
        assert!(!night(12999)); assert!(night(13000)); assert!(night(22999));
        assert!(!night(23000)); assert!(night(-10000));
    }
    #[test] fn ore_and_floor_are_explicit() {
        assert!(is_valuable_ore("minecraft:deepslate_diamond_ore"));
        assert!(!is_valuable_ore("minecraft:coal_ore"));
        assert!(floor_is_safe("minecraft:stone"));
        assert!(!floor_is_safe("minecraft:lava"));
    }
}
