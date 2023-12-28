use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct GeoHash(String, pub Option<Vec<((f64, f64), (f64, f64))>>);

impl std::ops::Deref for GeoHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GeoHash {
    const DICT: &'static str = "0123456789bcdefghjkmnpqrstuvwxyz";
    const LON_MAX: i16 = 180;
    const LON_MIN: i16 = -180;

    const LAT_MAX: i16 = 90;
    const LAT_MIN: i16 = -90;
}

#[derive(Debug, Clone)]
pub enum Precision {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGTH,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl From<usize> for Precision {
    fn from(value: usize) -> Self {
        use Precision::*;

        match value {
            1 => ONE,
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGTH,
            _ => panic!("New value encountered"),
        }
    }
}

impl From<Precision> for usize {
    fn from(value: Precision) -> Self {
        value as usize
    }
}

impl GeoHash {
    pub fn new(lat: f64, lon: f64, pecision: Precision) -> GeoHash {
        let precision = Into::<usize>::into(pecision) + 1;
        let mut geo_hash = String::with_capacity(precision);
        let mut even_bit = true;
        let (mut lon_mx, mut lon_min, mut lat_mx, mut lat_min) = (
            GeoHash::LON_MAX as f64,
            GeoHash::LON_MIN as f64,
            GeoHash::LAT_MAX as f64,
            GeoHash::LAT_MIN as f64,
        );
        let mut lines = Vec::new();
        for _ in 0..precision {
            // 5 bit for base 32.
            let mut idx: usize = 0;
            for _ in 0..5 {
                match even_bit {
                    true => {
                        let lon_mid = (lon_mx + lon_min) / 2.;
                        if lon >= lon_mid {
                            idx = idx * 2 + 1;
                            lon_min = lon_mid;
                        } else {
                            idx = idx * 2;
                            lon_mx = lon_mid;
                        }
                    }
                    false => {
                        let lat_mid = (lat_mx + lat_min) / 2.;
                        if lat >= lat_mid {
                            idx = idx * 2 + 1;
                            lat_min = lat_mid;
                        } else {
                            idx = idx * 2;
                            lat_mx = lat_mid;
                        }
                    }
                }
                lines.push(((lon_min, lon_mx), (lat_min, lat_mx)));

                even_bit = !even_bit;
            }
            geo_hash.push(GeoHash::DICT.chars().nth(idx).unwrap());
        }

        GeoHash(geo_hash, Some(lines))
    }

    #[allow(dead_code)]
    fn adjecent_in_direction(&self, direction: Direction) -> GeoHash {
        use Direction::*;
        let border = HashMap::from([
            (North, ["prxz", "bcfguvyz"]),
            (South, ["028b", "0145hjnp"]),
            (East, ["bcfguvyz", "prxz"]),
            (West, ["0145hjnp", "028b"]),
        ]);

        let neighbor = HashMap::from([
            (
                North,
                [
                    "p0r21436x8zb9dcf5h7kjnmqesgutwvy",
                    "bc01fg45238967deuvhjyznpkmstqrwx",
                ],
            ),
            (
                South,
                [
                    "14365h7k9dcfesgujnmqp0r2twvyx8zb",
                    "238967debc01fg45kmstqrwxuvhjyznp",
                ],
            ),
            (
                East,
                [
                    "bc01fg45238967deuvhjyznpkmstqrwx",
                    "p0r21436x8zb9dcf5h7kjnmqesgutwvy",
                ],
            ),
            (
                West,
                [
                    "238967debc01fg45kmstqrwxuvhjyznp",
                    "14365h7k9dcfesgujnmqp0r2twvyx8zb",
                ],
            ),
        ]);

        let last_char = String::from(self.0.chars().last().unwrap());
        let mut parent = self.0[0..self.0.len() - 1].to_string();
        let pos = self.0.len() % 2;

        let border_value = border.get(&direction).unwrap()[pos];
        if border_value.contains(&last_char.as_str()) && !parent.is_empty() {
            parent = GeoHash(parent, None)
                .adjecent_in_direction(direction.clone())
                .0;
        }

        parent.push_str(
            &GeoHash::DICT
                .chars()
                .nth(
                    neighbor.get(&direction).unwrap()[pos]
                        .chars()
                        .position(|ch| ch == last_char.chars().next().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
        );

        GeoHash(parent, None)
    }

    #[allow(dead_code)]
    pub fn adjacent_in_all_direction(&self) -> HashMap<Direction, GeoHash> {
        use Direction::*;
        HashMap::from([
            (North, self.adjecent_in_direction(North)),
            (
                NorthEast,
                self.adjecent_in_direction(North)
                    .adjecent_in_direction(East),
            ),
            (East, self.adjecent_in_direction(East)),
            (
                SouthEast,
                self.adjecent_in_direction(South)
                    .adjecent_in_direction(East),
            ),
            (South, self.adjecent_in_direction(South)),
            (
                SouthWest,
                self.adjecent_in_direction(South)
                    .adjecent_in_direction(West),
            ),
            (West, self.adjecent_in_direction(West)),
            (
                NorthWest,
                self.adjecent_in_direction(North)
                    .adjecent_in_direction(West),
            ),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_precision_converter() {
        assert_eq!(0, Into::<usize>::into(Precision::ONE));
        assert_eq!(1_usize, Into::<usize>::into(Precision::TWO));
        assert_eq!(7_usize, Into::<usize>::into(Precision::EIGTH));
    }

    #[test]
    fn test_geo_hash() {
        assert_eq!("u120fxw", *GeoHash::new(52.205, 0.119, Precision::SEVEN));
    }

    #[test]
    fn test_all_neighbors() {
        use Direction::*;
        let geo_hash = GeoHash(String::from("gbsuv"), None);
        let expected = HashMap::from([
            (North, GeoHash(String::from("gbsvj"), None)),
            (South, GeoHash(String::from("gbsut"), None)),
            (East, GeoHash(String::from("gbsuy"), None)),
            (West, GeoHash(String::from("gbsuu"), None)),
            (NorthEast, GeoHash(String::from("gbsvn"), None)),
            (NorthWest, GeoHash(String::from("gbsvh"), None)),
            (SouthEast, GeoHash(String::from("gbsuw"), None)),
            (SouthWest, GeoHash(String::from("gbsus"), None)),
        ]);
        let actual = geo_hash.adjacent_in_all_direction();
        assert_eq!(expected, actual);
        // assert_eq!(expected.len(), actual.len());
        // for (key, value) in expected.iter() {
        //     assert_eq!(actual.get(key).unwrap(), value);
        // }
    }

    #[test]
    fn test_in_north() {
        let geo_hash = GeoHash(String::from("gbsuv"), None);
        assert_eq!("gbsvj", *geo_hash.adjecent_in_direction(Direction::North));
    }
}
