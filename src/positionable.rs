use crate::math::Vec2;

pub trait Positionable {
    fn pos(&self) -> Vec2<u8>;

    fn set_pos(&mut self, pos: Vec2<u8>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_default_pos() {
        #[derive(Positionable)]
        struct Foo {
            pos: Vec2<u8>
        }

        let mut foo = Foo {
            pos: Vec2::from_values(4, 3)
        };

        assert_eq!(Vec2::from_values(4, 3), foo.pos());
        foo.set_pos(Vec2::from_values(3, 37));
        assert_eq!(Vec2::from_values(3, 37), foo.pos());
    }

    #[test]
    fn derive_custom_pos() {
        #[derive(Positionable)]
        #[PosField = "el_positione"]
        struct Foo {
            el_positione: Vec2<u8>
        }

        let mut foo = Foo {
            el_positione: Vec2::from_values(2, 2)
        };

        assert_eq!(Vec2::from_values(2, 2), foo.pos());
        foo.set_pos(Vec2::from_values(0, 75));
        assert_eq!(Vec2::from_values(0, 75), foo.pos());
    }
}
