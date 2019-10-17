//! Items that can be placed on a map, either full or black and white should
//! implement the [Positionable](crate::positionable::Positionable) trait.

use crate::map::FieldPos;

/// The item can be placed on a map.
pub trait Positionable {
    fn pos(&self) -> FieldPos;

    fn set_pos(&mut self, pos: FieldPos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_default_pos() {
        #[derive(Positionable)]
        struct Foo {
            pos: FieldPos
        }

        let mut foo = Foo {
            pos: FieldPos::from_values(4, 3)
        };

        assert_eq!(FieldPos::from_values(4, 3), foo.pos());
        foo.set_pos(FieldPos::from_values(3, 37));
        assert_eq!(FieldPos::from_values(3, 37), foo.pos());
    }

    #[test]
    fn derive_custom_pos() {
        #[derive(Positionable)]
        #[PosField = "el_positione"]
        struct Foo {
            el_positione: FieldPos
        }

        let mut foo = Foo {
            el_positione: FieldPos::from_values(2, 2)
        };

        assert_eq!(FieldPos::from_values(2, 2), foo.pos());
        foo.set_pos(FieldPos::from_values(0, 75));
        assert_eq!(FieldPos::from_values(0, 75), foo.pos());
    }
}
