use std::collections::HashMap;
use std::ops::Add;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{eof, map};
use nom::IResult;
use nom::lib::std::collections::HashSet;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
struct HexTile { x: i32, y: i32, z: i32 }

impl Add for HexTile {
    type Output = HexTile;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl HexTile {
    fn neighbourhood(self) -> [Self; 6] {
        [
            self + Direction::NorthEast.offset(),
            self + Direction::NorthWest.offset(),
            self + Direction::West.offset(),
            self + Direction::SouthWest.offset(),
            self + Direction::SouthEast.offset(),
            self + Direction::East.offset()
        ]
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn offset(&self) -> HexTile {
        match self {
            Direction::NorthEast => HexTile { x: 1, y: 0, z: -1 },
            Direction::SouthEast => HexTile { x: 0, y: -1, z: 1 },
            Direction::SouthWest => HexTile { x: -1, y: 0, z: 1 },
            Direction::NorthWest => HexTile { x: 0, y: 1, z: -1 },
            Direction::West => HexTile { x: -1, y: 1, z: 0 },
            Direction::East => HexTile { x: 1, y: -1, z: 0 },
        }
    }
}

#[derive(Debug)]
struct Path {
    steps: Vec<Direction>
}

impl Path {
    fn walk(&self, init: HexTile) -> HexTile {
        self.steps.iter().fold(init, |h, d| h + d.offset())
    }
}

fn path(input: &str) -> IResult<&str, Path> {
    map(
        terminated(many1(direction), alt((tag("\n"), eof))),
        |steps| Path { steps },
    )(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let to_direction = |direction: Direction| { move |_: &str| direction };

    alt((
        map(tag("ne"), to_direction(Direction::NorthEast)),
        map(tag("se"), to_direction(Direction::SouthEast)),
        map(tag("nw"), to_direction(Direction::NorthWest)),
        map(tag("sw"), to_direction(Direction::SouthWest)),
        map(tag("e"), to_direction(Direction::East)),
        map(tag("w"), to_direction(Direction::West)),
    ))(input)
}

fn count_active(map: &HashMap<HexTile, bool>) -> usize {
    map.values().filter(|v| **v).count()
}

pub fn solve() {
    let mut rest = INPUT;
    let mut black = HashMap::<HexTile, bool>::new();
    while let Ok((rest2, path)) = path(rest) {
        rest = rest2;
        let walked = path.walk(Default::default());
        let flag = black.entry(walked).or_default();
        *flag = !*flag;
    }

    println!("Initially active: {}", count_active(&black));

    for day in 0..100 {
        let to_consider = black.iter()
            .filter_map(|(k, v)| if *v { Some(k) } else { None })
            .flat_map(|t|{
                let [n1, n2, n3, n4, n5, n6] = t.neighbourhood();
                vec![*t, n1, n2, n3, n4, n5, n6].into_iter()
            })
            .collect::<HashSet<_>>();

        black = to_consider.iter()
            .map(|tile|{
                let black_neighbours = tile.neighbourhood().iter()
                    .filter(|neighbour|{
                        *black.get(neighbour).unwrap_or(&false)
                    })
                    .count();
                let is_black = *black.get(tile).unwrap_or(&false);
                let becomes_black = if is_black {
                    black_neighbours == 1 || black_neighbours == 2
                } else {
                    black_neighbours == 2
                };

                (*tile, becomes_black)
            })
            .collect::<HashMap<_,_>>();
    }
    println!("Day 100: {}", count_active(&black))
}

const INPUT: &str = "swswswswseswswseseswswnwsesw
nwewsewneswenwesenesenw
neeeenwseseewwnenenesweeeeene
seseewnwseseseneswseeseseseesesesese
swseswswswswswswswswweneswswsw
seswsesesenwsesenesenwseseseseseseseesew
nwneenwneswnwneneneseneswnwnewnwsenene
wswswwwswswsewneswnwnwsenwswswewesw
eeseeneeneeeneew
swnewnwneneeneneseneneneneneneenenenene
seewswnwseneswnenenwww
eneneswnenenwnwwneneneseneneneswneswne
nwewswesewseseeewneseseseseeese
enwsenesweswwsesewsesesesesewnewsene
wseeseseseseseseseseswsesenesesesesesese
nenewnenenesenwsenewneneneswneesw
eseeneseneesesesesweseseeseewsewe
nenenenenenewneneneneneseeneswnenenenenee
nwsenwnwnwnwnweneeswnwenenwwnwnwwnw
neewnewneeeneneseeneseneeneneneeww
neswswswneswseswwwnwwwwswseswsewnene
wnwnwenwwewswwnwnwwneswwsenwese
wnwsewewwwnenwwwwnwnwnwswnwnwnwnw
wewwwwwwswwnwsewwnwnw
wsenenewsewwnese
wnenenwswseesweene
nwnwnwnenenwnwnwnwnwnwnwnweswnwswnenwnwnenw
swseeewnenweenwenenwswsenesw
wswswswswnwsewswneswswseswneswneswsesww
seswneseswswswseswwwseswswneseswswswnesw
sesweseswnwswsweseswswwnenwnesenwsew
newwseneseeseeseseseseseesesesewse
swswewwswsweswswswwwwswwnwswsewnesw
seneeeeeseeseeseseeeesesesw
swsesenwnwseswwseswswseseseseseswseseswne
wswwwneesewnesenwwswwnwwswnw
eneeeneeeeeneneneneneneeesewwnene
swswswsweswnwswswswswswswswswswneswswsw
sweswwswswwswswswswswswswswswneneswwsw
wswwswswenenwewnesweesenesesenenwe
sweneneswnwnweswneneeneeneswneeee
swwsweswswnwswswswswsw
nenewnwsenenenenenenenesewnenenenenene
nwenenenenwnwneneenenwnwswswnenewnenw
seswwsweswseneswswnwswwnenenenwswsewswse
seneswsenwnwnwnenwnwnew
esweseneeewseweeseenesenwnwswe
wswswsenewenwswwwwwnewswswesewwsw
seesewenweeeweseswwseseeneese
wwwswwnewswwewwsenwneswwwwsesw
wseseseseneneseswswseseseseswe
enenenenenewswneneneeneneneenweew
ewseneseenwneseneswsewnww
ewwwwswwwswwswwwwswwswwwnw
neewseswesesesewsewwnwseseseeswe
swwswwswswwwswswswwswswnwwswswsewe
wneesenesweenwsenwsweneeeeseew
wseneseenenwwwneesweewnenenenese
seenwwneneenesenewswnenesewnesenesew
nwnenwsenwnwenenwnenww
nwnwneeswnwnwewnwwswswnwnewnwsenwnenw
swswwwwneswswsweswswneswswsewswswwsw
eneneeeeewneee
seswswswesweswwswswswswsenwsw
wswswseswwwnesweswsweswswwswwwsww
eeesweeneeewnwneeseeesweneeee
nenenenenenenwneswnenenenwnenenenenesenene
nwwnwwwwwwwwwwew
eseseseswswsewswsesenwswswseswsweseswse
nwwswnenenenwneneneneenesenenenenenesene
seseseseswneseswswnwseneseneweeneswse
nenewnwnwnwwnwwnwwwnwsenwnwsenwnwnw
wwwswnwnwwnwnwewnw
neswsenesewsesesesesesesesesesewsesese
esesesweseeseeeseseeeenwesewese
wnewewnwwnwswnenwswwswwnw
wnenesenwnenenenwsenenwswnesenenenenene
eneseeeeneeneenwneneneeweswenee
seswnwwswneswseswseswseseswswswesesesw
nwswseseeswswewswswsesenwwswwswesesw
wewsenwnwwnwnwnwnwwenwwwwnwew
enwwnwnwnwnwnwnwnwnwwnwswnwnwnwnenwsenw
nwsewewwnwnwswnwnwswwnwsenwnwneee
wwewwswsesenwwwswewwneeeneww
wnewsewsenwwwnwwwnwnwwwwwwnewsw
wnewnwnweewnwsenwnwnewswswnenwswenwnw
wnwwswswwwnwwwwwnewnweww
wwswseswwwswwswnwwwnenewwenww
wswwswwenwwswwswwswswwswswswesw
ewswwwwenwwsenewnwwww
sewnwneswseswseswwswsewswneswnwswseswnee
neenwswewsenwwswnwswswwswneewswwe
wwwnwwwwnewewwwnwwwswwww
esesenwseseeeswseeeseenweseesesese
esesesewseseseneseeseeswseesesesesese
seseseseseseseneseswseseseswsesesesewse
sesesewseeseseneeseseseeeewnesesese
swswswswswswswnwswswswwnwswweeswswsw
seseseswwseseseseseseseenesewseeenew
neeswnwswneewnenenenenenenwneswne
nenwnwenwnwnwnwnwnwnwnwnwnwnwswnwnwnwnw
sweeeeseeeeenweeeseenweseeee
nenwseneneeenenesweneesenenenwneenene
wwneswswwwwwswwswnewwseswwwwsw
swsenenwseswnwseseseneswnesesesesenewsenww
nweswwseseseseswnwnwsesesesenesesesesesw
eneseseswwsweseswswsesewseeswswswnw
nwnwnwwnwwnewwnenwnwsewnwseesewnw
seseeeeeesewnweeeeesweesese
eeeswneneneneswnenenenenenenwneeene
nwseseseseseseseseseeseseswseseseseesenw
swewwwnwswsweeswnwwwnwwwseww
nenenewewseswneneeneneswneenenene
sewnewwwwwwwwnwwwwnwwwsweww
nesenewneneneneneneneneswenwweeenesw
swswwwneswnesewseswswwswswnwwswsww
nenwnwnenwsenwnwnenwnwnwnwnwwnwnwneewswne
eenweeeeswseeenwseeseseseeesee
seewenenenenwswseneeneenwwneewnee
seseseeeeeseneseseseesesesewseese
wwwenwseswswswswswswewswswswwnew
swwwsewnwesewswneewwesenwnwwne
neseesenwneswenweeesew
nenwesewwneswseseeswseeseeeseese
wwwnwnwwswnewwnwww
eseseseeenwseeeeeeseeeseenewee
nwsenenewnwnwnwnenwnene
swswswswswseswswwweswseswseneseswswsw
nwnenwnenesenwnwnwsenenenwsenwwenwww
neswwwwwwswswnewswswsww
nenwsenenewwnwnwnenwnenwseseenwnenewne
wnwwwwsewnwnwnwnwewwwswnenwww
nwnwwnwnwnwsesenwnwnwnese
wwwnwwwewwwnwweswnwwswnwwnwnwnw
nwnenwnwnwneenwneswnwnwnenwneeswnenenwnw
sesewswswswneneswnwwsewswswnwswwnee
nwneneneneenenesweneeneneneeswnenee
neeswenenwweesenwseneswee
nenenwneswnenwnenenesw
nwwwnwswsewwnwwewewsenwnwnwwnw
nwnenwenenenenenwwnenenwnenwnwswneenene
wwnwwwwwwnwnwwwwwwewwwwe
eeneeeeeeeeweseeesweswnwnene
enenenenenenenenenwwnewnenenenenenwe
esweeswsenenenwswswswwwseswneswswsww
eeeesweneeneeweeeneeee
nwseseswswswwwnwenewnenwsw
sweeeswnwseneeeeeesweeneseweee
swswswswneweswswswswswsw
swwsewnwsweneswweswnwswwseswswswww
nwnenenenenenenenwnwnwsenenenwnesewnwnenw
swseeeseweseseseseseesesesenwsewnee
neneneswnenenwnenenenwnwsenewenwswseswsw
sweseseswnwswenwsesweseeeseesenenese
seewenwsesenweeeseeneseeseseeswee
eneeeseseeeeeseeswweeseeeswenw
wwswswswneswswswwwswwswswswswsw
eeneenenwneeeneneeeeneeswenene
swwswwwnwswwseewswswwswswwswswswwnw
nenenwnenenewnwnenwnenwnesenenenene
neeswnweswneneewneeneeeeeee
swneneswseswswseswswwswswswswswnwneswsw
enwenwnweeseeweswswnwsweeenweswe
newswnenesewwwsweswwswswwswswswsww
nwwwwwnewwwwwsewwwwwwewse
eneeenenenwneneeeneseswnenenenenene
wswwneswnwwsewswseww
nwnwnwnwsenwsenwnwenwwnwwnwewnwnwwnw
wseneneenewnweenewwnenenenwsenwne
eswneneneneenenenenwnenwnenwnenwnwwnenene
swseneseseswsweswseswsewseseseseseswsese
swswswwswwwswswswsewswwswswswsweswnw
swneewenenwneewnweseswnwwnw
wswwwwwewwwwwwwww
nenenenenenenenwswnenewswnwneneneneese
sewsesewsesesenwseneneseseeesesewe
swswswswewswswswswswswswnwswswswwwsw
wnwsenenesewnenenesewnwnenenenenenenenene
newwwsewsenwnwnwwwnwnw
neswswwswneswswswneswswseswswswwswww
nwsewwwswweswswwwwswnewswswwwwsw
seseseswseseswneswswseseseseswsesesesenw
senwwwseswswseseswwswnesewnwnenene
eneeeeeswneneeneesweeneenenene
eeeeeeeeeneeweeeeesee
nesenwneneenenenwnenwswnenenenenwnwnene
neenwnenenwwnwswnwnenwwswenwnwnwswnw
neeneneeneeneeeneeneneenwewenesw
nwseswswnweenewwnenwnwnwswneeswwnw
swnewwwnwnwseewweswwnwwsww
nwsesweweewwnwwwsenewnewwwnwwsw
seeswsenwnwwswseswswsenwesweenwsese
nwwseeeeeeeeeeeeeeeeeene
swneswswswseswswwseswneswswswseseswswsww
esesesesesesenwseseswsesenesewwsesesw
eneeneneneneeneewneeneneneneene
wwnwnwnwwnwnwsewnwnwenwwnwww
seeseseesesesesesesesenwnwseseswse
nwwnwewseswnwwswwnwnwwnwsenenwsese
nwneswnwnenwnwnwnwwnwnwwnwnwnwnwnwseenw
wsenwnewswwwneseweswnwwwsenenwnew
swesesewsewswswsweswswswenwswseswswsw
eweeseneeeeseeeseeeeee
swwwsweweewneswswnwsenwswswwswswsw
sweswnewnwneenwswweseeeneseseseew
nenenesenenenwneneseneneswnenenwnenenenwnw
swswswsweswwswswswswswseswswswnwswswnwsw
eeweeeseneeneeneeeneswwsee
ewwwwwnenwwwwwwwsewwwwnww
nenwswnenenenenenenenwnwnenweseneswnewnw
wswwnwwswwswnwwwwwwwswesesw
sweeneneswsewneeeneneneneneenenene
enenwnwsenwwnwswnwnenenweswne
eenwswwnwenwenwesweswnweswswsesww
swwseseswnwsesweseswseseswswswnwnesesese
seeeeeeeeeeswnee
newsesesenesesweswsesesesesewne
enwnenwnwnwwnwwswwnwsenenwnwnwnwnwsw
nwswseneeseswwnwswnwsweenwneswnene
neswswswseswswswwewsww
seseseseseseswseseseseseswnenwseseseesesese
seswseeneswnwswsewswseswseneswswswseswnw
neneesweeneenenwneseesweneeneneenw
swsenwswsenwseneesw
wnwwwswwswwwswswswswsewwneswswsw
wswwwswwwnwwwwwnesewswwwwe
swseswseneswsenesesewswseswseswwsenwe
newnweneneneneneneneseeneneneneneewe
eseseseseneeseswsenwsesesesesewenwew
wseeenwseseneseewswseenwseseseseese
wswwnewewseswwswwswnewwwneswsww
wsenenenwesenewewnwewseeeseswswsese
nenenenenesenenenesenenenenenwenwnenenene
eweseneneeewewne
seseswsenwesesesenwweeeseseeeesese
swenwwneseseweseneweswswseseswnwswswse
seeewnwnenwwswnwnwnwenewnwwswsene
seneswswneseeswnesesesenesesewnewseesese
swnenwnweneeswnwnwnwnw
nenenenwneenesenene
swsweswsenwseseswnwseneseseseswseseseswse
swneswswswswwswwswswnwewswsweswnwsw
nwenenewnwnwseeeseseswewnenenwswswnew
eeneseweseseseseewee
nwseseseswswwseswneseseseseseswse
nwwswswswwswwwwswnwswewwsewswe
eeeseewseeeeeeeeeeneeee
eweeeeseneeneneeneeneneeewew
nwsenwnenenenwnwnenenenwnwnwnwnenenewne
swswswswseswswseswswswswswswswswswnwnwswsw
swswswseswseseswseswswseswsenwnwswsesesesw
swnenenenenwneneneneneneneswneneneneneenene
neneneeeenenesenewneeneeneenenee
eesesesesesesesenwwnwseeesenwswswse
senwswnenwseneeswwseswe
eeeweeewseneneneeeeneeneee
eeeeneeenesweewenew
nwswwwenwsenwsenenwswwweswsewwnwne
swswswseswneseseneswnwseseswsesesweswswswsw
eneenwneneeeneseneneneeeesenwnee
swwnwwnwsenwswwswseenesewseenewwe
eeseseweeeewsenesesesewee
nwsenwnwnwnwnwnwnwwwwnwwwnwnwnwwenw
neneswneenenenesenenenenenenwnenwnwnesew
eseseeeseswswweesweeeneesenwne
swswswseswwswsenwneneseseseeswseswnwsw
nwnwnwnwwnwwnwnwwswnwnwnwnwnwnwenwnwe
wnenwnwnesenwnwswneneenenwwneenwnwnene
seewseseseseeeenweeeeneseseswsee
swsenwsenwnwseseseswesweseswnw
swseswswswsweswswseswswswswseswsenewswsw
seswseseseswwesesesesesesesesesenwswnese
wswnewwseswswwswwwswswneswswwseswsw
nwwsesweeseseeseenesesenweeeesesese
neseeeseewseeseseesewseeese
nwnwnenwnenenenwnenwnenenenwswnwsenenenw
eseeseeseeswseneseseseeeseew
swswswswsesweswsesesenwswseswsesesesesw
wnenwneseseesenwwnenwswswneneweee
newnenenenwnwenwenwswneneseneneswnene
nwwnenwneneneswnwneenenwnenenwnese
eweewwwwwnewwwewwwwnwwswse
ewsweenenwnwswswnwnwnwswwnwnw
esenwnwnenwnwwnwnwwwwewseswwwnwsene
wseeeeeeseseseenwseeeseeseswee
sesesesesenwnwsewswseswseeeseseswnewne
neneneeneswnenenenenewnenwnenenesenenene
wseneseweneeeeew
sesewseseeseswsenewsesewsesenesese
neeneneswseneesweeseeneneneenwnwe
swswseswswswswwneswswswswswswswswswsw
wwwwsewswwswswwswneswwwwswww
seseseseswneseseseseseeseseswsesewsesesew
nwnwnwswnwnwnwnwnesenwnwnwsweenwnwnwswnwnw
seesewnwswneweeenweeseeeseseneswe
seesesesesesewseneewesesesesesesese
nwneeeewewneneeenenesenwswswswe
seseewneneeneneneneneneswwnwneewse
sesweeswwnweseeeenweeeenwee
wwswwswwwnewsewwwnewswweww
swswswnwswnenweseseswswswnenwswswseswswsw
eseswneeeeeneseeenenweeswesesesw
seseseseswswseswseswswswsenwswesesesesw
sweeenenenenwswnwsweeneneswnweeesee
wnenwnwnenwnwwswnenwnwseee
newweneeeeeseneeneswneweneenee
wswswsweseeswneseswswswenewnewwse
swswswweeneswenwswnw
swswsenwswseswswsesenwsesesesesesesesee
nwnwsenwnwnwnwnwwnwnwnwnwnenwnwsenwnw
seswenwnenwsewnwwnenwsewwnwnwesene
enwseweenweseseseeswswseneswesene
newnesenewneneseneneneneswnewneneneenee
swwwswwnwewneswwwwswswwseswww
sesenwweneneeswesenwwewnenw
seneseeseswseseneseswsesesesesesesesee
neneeeeneseneweneneneseseewewwe
eseeseseseseseeswswseseneseneseesee
eseneswnewswneswwsenenenwnwsenwwsww
nwnwsenenenwnwnwnwnenwnwnwswnenwnwnenenenw
nenwenwnwnwswnenenenwnenenwwnenwesenenenw
swswswseswswswswswswneneseswswswseswwnwnw
eeeewneeeeeneeneswnwesesw
eneneneeesenenesenwnwneeeeseeenew
seswseswsweswswswswseswswnwseswswse
nenwswwswswswswwswnewswwswswwwswwe
nwwnwswseewwwwnwwnewswweenwnwnw
wsenwnwswnwnwwnwsenwenwwnwnwnwnwwwnenw
swsweswswseswswswnwswswswswswnwseeswneswsw
swneswswwswwswswneswswwseswseswswneswswsw
wwwnwwwnwwseewewwnwwswweswne
wwnwnewsenwnwwwwwnwnwnwnwnwnwnwnw
swnwnwnwneneneneeneenwswnwnenwnenenese
nenwsenenenenenewnenwnenenenenene
swswswwwwswswene
neneeneeneneswneswnwnesewneenwnwnenesw
wswwwwswwswneswswwseeswswwwww
nenenwswsenwnwsesenwnenenenwnwnwsene
nwenwseeeenesweswseweseseseenwnesee
esenwsesesewseseneseseseseswseseseswsese
eeeweenwnewseeeeseswsenenwesesese
weeeeneewseeneeeeesweeswne
seseswsenesesesesesewsesesesesesesesese
neneenwneweseseeeeweeeeeneeee
eeesweeenwneeswnenweeeeeeenee
ewwnwwnewweswnwwsewewwnweww
eseneeeweeeeeeeeenesweenene
swswswswswwwwwswwneswneswswswswswww
sweeeeeneeeeeeeeeeneswsee
swseswseswneswswswseswswswswswswswswwsw
swnwsenwnenwnwwnwnwnwnwwsenenwenenwnenenw
nwseseswseseseeswnwswseswseswseswsenwse
swswenwnwswwseswnwsewseeneswseneswswnw
newseseesewnwsweneseeseseewseseew
wwwwnwwwwwwwwwewewwwww
nwswnenwwnwwnwnweesewnwnenwnenwnwnwnw
swneeseewsesewseewseseneneseseeeee
senweswnwswesesweswsewseeswswnwse
neeneeeeeeneswneneeweeseeneee
nwwnwwnwwswwwnesenwnwwwwwnwwnwse
nenwnewnwnenenwsenenenwnwnwnwewnwnwne
wnenwnwwnwwwewswswnwwwwnwnwwwww
neneseneneswnenenwwneneneeeneeneeenew
swswwswswswneseswswswswswnwswnwesweswsw
eweweeeeeeneseeewee
nwwswwwsenwnewswwseswnewswnweww
swswsesewsweseneeswswswsweswnwnwnwswwsw
swseswswswswseseseseswswseseswnwnesesesw
nwsenesweswnwnwsesweeswewsesesenesese
nwseseswseseneswsesenwseseseswsesesesenwe
seseeseseseseseseeseseesew
nenwnenenewnenenwnenwneneeneswenenwnenw
wnewwswswseswnenweswswswswswnwswswswswe
neenwswwnenenenwnwswenwenwnene
nwnwnwenwnwnwnwnwnenwswnwnwswnwnwnwnwne
seewwsenwwwwweswswwnwwwwswww
nwnwwwnwnwsesenwneenwsenenwswwnwsenwne
nwnewnwenwwneswwnewwswswwnwwnwswnwnw
seeeeeeeswseseseseeseeenwsesee
wnesenwwwwwnewwwsewwswwwwww
nwnwnenwnenwnwnwnwnwnwwnwnwnesenwnwne
nenewnwnenwswsenwenwnwnwnwneewnenwne
neeenwneneeneswweeeeneeeneeeee
nwswenenwnwnwswnenenwnwnwnwnenwnenwnwne
neeswswswswswswswwswnwswwswswswswesw
nwnwnwnwnwnwswnwenwnwswnwnenwnwnwnwnwnwnwnw
newwnwwsewwwwsewwewwwswsenwsw
nwseswneswwswswwswwwswswwwwseswwsw
wwswswwwwwwsewswwnewwwwww
seseseesesesesewsesewsene
wswswswewswswwewwwswswnwwswnwww
swswseswswswnwswswswswwswswswswswswsw
nwsesesweewnwneswwnenwnwnwsenenwswsenw
swswseseseswswseseseseseswswsesesene
nwesewwnwneseeseneswsesesweneseenwswsw
eeseeesenwnweesweeeseeeesesw
neeswwnenwnenenenwneenenenenwneswsesw
nwwwesenwwnwnwwwnwwnw
nwwswwwwenwwnesesesewenewswsww
swswwwwswsewwswneswswswwwsewneswswsw
eneeneneswnwnenewseeneseneesenenewene
nwwswwnwnwwnwnwnwewwnewnwnwnwwnwe
nwsweswnewwwwsweswswwesw
nwneenenenenenenewswsewenenewsenenene
seswseeseseseneewsesweseseneseesesese
swswswsweeneeewewnewwnwswnewsw
wneswswwsewswneseswswswswwsw
eneneeneeeneseneneswneenwneneenene
enwnenwswsweneeswenwwseeee
nwnwnwnwnenenwenwnwswnenwnwnwsenwnwnwnenwnw
sesewsenwseseesewnwsenesesesesesesese
nesenwneneswenenenenewnesesenwewsenew
seseswnewswesenwneswwewwneeseeswse
wseeseenweeeeeeeewswneeee
nwsesesweswsewseseswsesesesewswseneese
nwnwwwwswwwnesewneewsw
nwwnwsenenwneneneswnenenwnwnwswnwnw
swnwnesewwwnwwwwnwwneenwswwwwww
seseseswwneswswsenewneswswwswseswswse
eseseweenenewseneseseseseswseesew
seswswswnwseseswswseseswswswswsenwsenwnwe
swswswwweswwswswwswswwwswswswsw
wnwwnwwwnwwseenwesenwwnwswnwnenw
nwnwnwnwenwnwnenwnwnwwnenwneenwnenwswnenw
nwwsesesenwweesese
nwswnwnwnenewsewse
eswswneswwwwwwwwwnewwwwswnew
wwwswsewwwwwnww
swswswwwwwewnwwsewswwnwewswnee
nenwewneswnenwswnwswewneneneseswenwene
wnewwwewswsewwwwswnwwwwwwwsw
seeseseesenwewseseseseseseseesesee
wwnwwwnwwwwsewnwwwswwnwnwwe
ewsenenenwswswswswnweseewnwswsesenwsw
wwewwwwwsewswwwnwswwnwwwww
neswsweneseneswwwenesewswswewwsw
neneenenwnwnwswneswneswneneneneswneene
seswnwsesweswsewswnwnwneswnwswnwseswnesw
neswnwnwnenwnwnenenenesenwneneenenenwswne
esenwsesweseneswesewswenweswnesewne
enwnwnwnwnwnwswnwnwnwnwnwswnwneneenene
eneeneswnwneeweneseeesweneneenene
seseweeneswnwseseenwseseseseseeese
eswenwneeenwneseeneeneee
seseswseseseseneeseseswwswwswseseesese
nwseswswswswseseseseneswseseseseseseswswse
nenwswnwswnwnwnwnenwnwnwnenwenwnwnwnwsene
seswwesesweswswswsenwnewwsweswsww
eeeeseseeseeeeesenweesweese
eesenwesweeeeneenweeseeeeee
esesesesweseeesesenesewseseeesee
neneeeneenewnenenenenenesewse
nenwsenwnwwnwenwnenwnwswnwenwnwnwnwnw
wwseneneswnenwneswnwsesenwweseenewnw
swwswnwswwsewswswswswwswsewswswnesw
seeeeneesweweeneeeeneneneeenee
swswswnwseswswswswswnwswswswneswseswesw
wnenwwnesewwswwwsewwwnw
nwsenenwnwwnesweseswnesenwnwnenewsew
ewnwswswwwswwswwnwswesweewww
seeseswsesenwsesenwnweswswswswsewswsene
nwnwnwswnenwnenwnwnenenwnesenwesenwwnew
nwnwwnwnwnwnwwenwsenwnwwwnwnwwnwnwnw
wwwnwwwwseewwwwewewnewww
neswwwswwwswswsenewswswwwwwswwsw
swsewseeseeswseswwswswsesw
swswwswesewswnwswswswwsww
swseswswseswswsweseseseseswnwseswswswsw
wsesesesesesesesesesesesesesee
wnwwswseseneneneenesenenenenenenwee
ewsweswsenwseswwwnweenwnewnwnesew
nwsewneswwwwswneeswwsw
nwnwnwnwnenwnwenwnwneeswnwnwnwnwswnwnwnwnw
ewneneswesenesenwnwnewseww
nwnenwnwnwnenwnwsenwnwnwnenenw
eeeeeeenweseenweeeenwseseesw
wswwswewswwswwswwsw
nweesewseweseenwseseee
nwnenwnwnenenenwnwswnenwsenwnenwnwnwwnenw
nwnenwnwnwnwsenwnwnwsewnwnwnwnwnwnwnwnw
nenwnwnwnwswnenwnwnwnwenwnwnwnwnwnenenw
sweeeeeneneweeeeeeneeeenw
neswsewnenwneswnenenenenwnwenenwnwnenwne
nwwnwswnwswnwnwnwnwnwnwnwnwnenwnwnwnenw
wsweenwenwewneeweeswswwewee
sweswnwwswswswnweswswneswseswswswsesw
swneswswwwswwswewewwswwwwwnese
nwswswswswswswswswswseswswnewswswswsw
nenwnewweewnwnwswnwwsewwwswsenw
seseeewewseseseseseseesesesesesesese
swseswseswswseseswnwseneseeswnweswsesw
wenwnwnwnwnwnwsenwnwnwwwwnwwnwnwswne
nenenenenwnenwnewnenwswnwneneneneneenenw
nwnenenenesenwsenewnewnenwnesenenwnenenw
neneneneeneesweeeneneneneeswenwne
neneenwneeneneeswneeeeenenenenene
swseswseswneswswseseswsesesewnesewneseswsw
swneseewneseswnwneenwswseseseeswese
nwswnwnwnwwnwnwnwnwnwwnwnwewnwnwnwse
nwswnwwnwwnwwwewwnwwwewseenwnw
eeewneneneneeeswseneeweneenew
nwswswswswswswswswswswswwswneseeneswswsw
nwwwnwnwwnwswswnwneewwnwnwnwnwnwnwnw
swswseseeswseswseswswsesew
seesesesesesesesesewseswseseseseene
eesweseeneseswnweseweenwseesesee
enwenwswnenwnwnenwwnenenenewnwswesenwnw
enwweeseenweseeeeswesweeeneee
nwnwseswnewenwnewwsesenwnewnwnwnenwnw
swswswseswweswswswswseeseswneseswsenwne
nenwnwnenwnwnenenesenewneenenenwnewne
swnwwnwswewnwwnenwsewewwwnwnwwww
swneswswwwswnesww
eeeswwnweseweseneeeswsesesesesee
swswwseeseswswswswswsenesenwswswsesenesw
swnewwesenwneswenwenwe
wnwnewwwsewwwwswseswwwswwwsww
swwnwenwnwwnwnwsewwnenwnwwnwwww
nwnwnwnwnenenewnwnenwnwsenenwnwnwnenwne
nwnwnwnwswnwnwnwnwswnwwwenwnwnwsenenwnw
neneneswneseswneeneswnwnenewenwenwswnwne
sweeneseseeseseeneswnweeseseseeee
enesesewwenwwwnewneswsewsewnwnw
swseseseseseseeseseseenwenwswswenwnw
swseseseeneseseswswseseseseswsewsesesese
nweseneswseesenwseswseseswse
swewswseseswwwseneswnesesene
sesesesweseseeneeseeseseseswenwsese
nenwwnenwnwwswwswswwnwnewnwwwnwnww
swswswswwsweswswewswswwswswnwswsww
swwnwswseswsweseswswnwnenwnwsweswee
nesewnwnwnwenwnwwnwwswnwnwnwnwnwnwnwnw
newnenenenenwnenenenenenenwnenenesenene
wwswewwwwswswwwswwnwwwewsw
seeeseeseeeseeseeseeesenwsesesw
neneswseeneneenenwnwnenwnenesenenenewne
nwwneneseswnenenesw
nwnwnwswnwnwnwnenenenwnwnwnwnwnwnwsenwsw
swnwwwnwewwwnwnwwnewwwwnwwnw";