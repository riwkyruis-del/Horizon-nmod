fn dreadfall_destroy_block(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    crate::game::dreadfall::destroy_block(game, pos, canceled);
}
