/// match, matches! ε if let
/// match
/// 1. ππ



// ε‘«η©Ί
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // ε¨θΏιεΉι South ζ North
            println!("South or North");
        },
        _ => println!("West"),
    };
}

