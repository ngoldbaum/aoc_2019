use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Map<'a> {
    tiles: HashMap<CoordOrName, MapTile<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CoordOrName {
    coord: Option<(i64, i64)>,
    name: Option<String>,
}

impl Map<'_> {
    fn from_map_str(map_str: &str) -> Map {
        let mut map: Map = Map {
            tiles: HashMap::new(),
        };

        map
    }
}

#[derive(Debug)]
struct MapTile<'a> {
    name: CoordOrName,
    neighbors: (
        Option<&'a MapTile<'a>>,
        Option<&'a MapTile<'a>>,
        Option<&'a MapTile<'a>>,
        Option<&'a MapTile<'a>>,
    ),
}

mod tests {
    #[test]
    fn test() {
        use indoc::indoc;

        use super::*;

        let map_str = indoc!(
            "         A           
                      A           
               #######.#########  
               #######.........#  
               #######.#######.#  
               #######.#######.#  
               #######.#######.#  
               #####  B    ###.#  
             BC...##  C    ###.#  
               ##.##       ###.#  
               ##...DE  F  ###.#  
               #####    G  ###.#  
               #########.#####.#  
             DE..#######...###.#  
               #.#########.###.#  
             FG..#########.....#  
               ###########.#####  
                          Z       
                          Z     "
        );

        let map: Map = Map::from_map_str(map_str);
    }
}
