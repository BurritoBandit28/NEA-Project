// This file contains all the tests for this project.


#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use crate::entities::dummy::DummyEntity;
    use crate::entity::Entity;
    use crate::game::Game;
    use crate::render::AssetData;
    use crate::utils::{mul_vec, normalise_vec};

    #[test]
    fn mul_vec_test() {
        // create a vector tuple
        let mut vector = (3.5, 6.0);
        // multiply by 4
        mul_vec(&mut vector, 4.0);
        assert_eq!(vector, (14.0, 24.0));

        // multiply by -1
        mul_vec(&mut vector, -1.0);
        assert_eq!(vector, (-14.0, -24.0));

        // multiply by 0
        mul_vec(&mut vector, 0.0);
        assert_eq!(vector, (0.0, 0.0));

    }

    #[test]
    fn normalise_vec_test() {
        // create vector tuple
        let vector = (3.0, 4.0); // Pythagorean Triple 3^2 + 4^2 = 5^2

        // get magnitude
        let mag = f32::sqrt((vector.0 * vector.0) + (vector.1 * vector.1));

        // equals 5
        assert_eq!(mag, 5.0);

        // normalise vector
        let normalised = normalise_vec(vector);

        // the magnitude should now be one, as it is a unit vector
        let mag_normalised = f32::sqrt((normalised.0 * normalised.0) + (normalised.1 * normalised.1));
        assert_eq!(mag_normalised, 1.0);

        // test for safely handling when magnitude is 0
        let vector = (0.0, 0.0);

        // when normalising, the x and y is divided by the magnitude
        // when the magnitude is 0, this isn't possible as x/0 is undefined
        let normalised = normalise_vec(vector);

        // instead of dividing, just return (0,0)
        assert_eq!(normalised, (0.0, 0.0));
    }

    #[test]
    fn world_space_screen_space_test() {
        // create a game instance
        let mut game = Game::initiate();
        // create an entity
        let mut dummy = DummyEntity::create(&mut game, AssetData::empty()); // represents entity

        // set its coords
        dummy.set_coords((-80f32, 0f32));

        // create player coordinates
        let player_coords = (0f32, 0f32);

        // get relative screen coordinates
        let mut out = dummy.screen(player_coords);

        assert_eq!(out, (80, 90));

        // again for off-screen coordinates
        dummy.set_coords((235f32, -103f32));

        out = dummy.screen(player_coords);
        assert_eq!(out, (395, -13))
    }

}