use std::io;
use std::io::Write;
use std::fs::File;
use self::world::*;
use self::math::*;

mod world;
// mod renderer;
mod math;

pub fn render_world(game:&Game, region:&Rect) -> Vec<char> {
    let render = game.render(region);
    let width = region.width() + 2;
    let height = region.height() + 1;
    let mut retval = vec!['.'; (width * height) as usize]; //Vec::with_capacity((width * height) as usize);

    for line in 0..height {
        // println!("Line: {}", line);
        // println!("IDX: {}", (line * width) as usize);
        // println!("Len: {}", retval.len());
        // retval.insert((line * width) as usize, '\n');
        retval[((line + 1) * width - 1) as usize] = '\n';
    }

    for (pos, marker) in render {
        let pos = pos - region.min;

        // retval.insert((x * y) as usize, marker);
        retval[(pos.x + pos.y * width) as usize] = marker;
    }

    retval
}

fn print(buf:Vec<char>) -> Result<(), io::Error> {
    let mut f = try!(File::create("out.txt"));

    try!(f.write(&buf.into_iter().collect::<String>().into_bytes()));

    Ok(())
}

#[allow(dead_code)]
fn main() {
    let pos = Point::create(5, 5);
    let mut game = Game::new();

    game.insert(pos, Entity { marker: '@' });

    let rendered = render_world(&game, &Rect::create(Point::create(0, 0), Point::create(10, 10)));

    match print(rendered) { Ok(()) => {}, Err(e) => panic!(e) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::math::*;
    use super::world::*;

    #[test]
    fn render_world_test() {
        let pos = Point::create(2, 2);
        let mut game = Game::new();

        game.insert(pos, Entity { marker: '@' });

        let rendered = render_world(&game, &Rect::create(Point::create(0, 0), Point::create(2, 2)));

        assert_eq!(rendered, [
            '.', '.', '.', '\n',
            '.', '.', '.', '\n',
            '.', '.', '@', '\n'
        ]);
    }
}
