use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::collections::HashSet;

// Data will be organized in (col, row) format
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (input, (walls, person)) = parse(Span::new(input))
        .map_err(|e| miette!("Parsing failed: {}", e))?;

    let rows = input.location_line() as i32;
    let cols = (input.get_column() - 1) as i32; // Subtract EoF character offset

    let guard = Person {
        position: person.0,
        direction: convert(person.1),
    };
    let mut tmp_guard = guard;

    let mut new_walls: HashSet<IVec2> =
        HashSet::from([tmp_guard.position]);

    while (1..=rows).contains(&tmp_guard.position[0])
        && (1..=cols).contains(&tmp_guard.position[1])
    {
        let next_position: IVec2 = tmp_guard.position
            + tmp_guard.direction.get_move();

        if walls.contains(&next_position) {
            tmp_guard.direction =
                tmp_guard.direction.turn_right();
        } else {
            tmp_guard.position = next_position;
            new_walls.insert(tmp_guard.position);
        }
    }

    new_walls.remove(&guard.position); // Remove the guard's starting position
    new_walls.remove(&tmp_guard.position); // Remove the guard's out-of-bounds position

    let loops = new_walls
        .into_iter()
        .filter(|new_wall| {
            let mut tmp_guard = guard;
            let mut positions: HashSet<Person> =
                HashSet::from([tmp_guard]);

            if new_wall.eq(&tmp_guard.position) {
                return false;
            };

            while (1..=rows)
                .contains(&tmp_guard.position[0])
                && (1..=cols)
                    .contains(&tmp_guard.position[1])
            {
                let next_position: IVec2 = tmp_guard
                    .position
                    + tmp_guard.direction.get_move();

                if walls.contains(&next_position)
                    || new_wall.eq(&next_position)
                {
                    tmp_guard.direction =
                        tmp_guard.direction.turn_right();
                } else {
                    tmp_guard.position = next_position;
                    if positions.contains(&tmp_guard) {
                        return true;
                    }
                    positions.insert(tmp_guard);
                }
            }
            false
        })
        .count();

    Ok(loops.to_string()) // Subtract the step out
                          // of bounds
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse(
    input: Span,
) -> IResult<Span, (Vec<IVec2>, (IVec2, char))> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;

    let guard = items
        .iter()
        .flatten()
        .find(|(_position, value)| {
            matches!(value, &'^' | &'>' | &'v' | &'<')
        })
        .cloned()
        .expect("There should be a player");

    let walls: Vec<IVec2> = items
        .into_iter()
        .flatten()
        .filter(|(_position, value)| value.eq(&'#'))
        .map(|(position, _value)| position)
        .collect();

    Ok((input, (walls, guard)))
}

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let row = input.location_line();
    let col = input.get_column();
    let (input, token) = one_of(".#^>v<")(input)?;

    Ok((
        input,
        (
            IVec2::new(col as i32, row as i32),
            token,
        ),
    ))
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Person {
    position: IVec2,
    direction: Direction,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn get_move(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::Y,
            Direction::West => IVec2::NEG_X,
        }
    }
}

fn convert(orientation: char) -> Direction {
    match orientation {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("Invalid orientation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
// 1832 too high
