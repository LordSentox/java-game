use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

pub trait Specialist: SpecialistInfo {
    fn can_act(&self, act_points: u8) -> bool { act_points != 0 }

    fn moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            SpecialistInfo::moves(&self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn special_moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> { Vec::new() }

    fn on_move(&mut self, act_points: &mut u8) {}

    fn can_move_other(&self, act_points: u8) -> bool { false }

    fn on_move_other(&mut self, act_points: &mut u8) {}

    fn drains(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            SpecialistInfo::drains(&self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn on_drain(&mut self, act_points: &mut u8) {}

    fn can_transfer_card<T: Positionable>(&self, other: &T, act_points: u8) -> bool {
        if act_points != 0 {
            SpecialistInfo::can_transfer_cards(&self, &other)
        }
        else {
            Vec::new()
        }
    }
}

pub trait SpecialistInfo: Positionable {
    fn moves(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        self.pos()
            .neighbours(map.limit_rect())
            .iter()
            .filter(|pos| map.is_standable(pos))
            .collect()
    }

    fn special_moves(&self, map: &FullMap) -> Vec<Vec2<u8>> { Vec::new() }

    fn can_move_others() -> bool { false }

    fn drains(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        let mut positions = self.pos().neighbours(map.limit_rect());
        positions.push(self.pos());
        positions
            .iter()
            .filter(|pos| map.is_standable(pos))
            .collect()
    }

    fn can_transfer_cards<T: Positionable>(&self, other: &T) -> bool { self.pos() == other.pos() }
}
