use std::collections::{HashMap, LinkedList};
use math::{Point, Rect};

#[derive(Debug)]
pub struct Entity {
    pub marker:char,
}

#[derive(Debug)]
pub struct Game {
    entities:LinkedList<Entity>,
    map:HashMap<Point, Entity>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities: LinkedList::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pos:Point, entity:Entity) {
        self.map.insert(pos, entity);
    }

    pub fn render(&self, region:&Rect) -> HashMap<Point, char> {
        let mut output = HashMap::<Point, char>::new();

        // TODO(schoon) - Switch to a quadtree and optimize.
        for (point, entity) in &self.map {
            if region.contains(point) {
                output.insert(Point::from(point), entity.marker);
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use math::{Point, Rect};

    #[test]
    fn game_render_test() {
        let pos = Point::new();
        let mut game = Game::new();

        game.map.insert(pos, Entity { marker: '@' });

        let render = game.render(&Rect { min: pos, max: pos });

        match render.get(&pos) {
            Some(entity) => assert_eq!(*entity, '@'),
            None => panic!(),
        }
    }
}
