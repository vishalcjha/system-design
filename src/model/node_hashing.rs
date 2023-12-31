use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sliced {
    original_server_count: u32,
    server_count: u32,
    slice_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonSliced {
    original_server_count: u32,
    server_count: u32,
}

impl NonSliced {
    pub fn new(server_count: u32) -> NonSliced {
        NonSliced {
            original_server_count: server_count,
            server_count,
        }
    }

    pub fn updated(&self, server_count: u32) -> NonSliced {
        NonSliced {
            original_server_count: self.original_server_count,
            server_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeHashing {
    ModBased(NonSliced),
    UnEven(Sliced),
    ConsistenHashing(Sliced),
}

impl Sliced {
    pub fn new(original_server_count: u32, slice_count: u32) -> Self {
        Sliced {
            original_server_count,
            server_count: original_server_count,
            slice_count,
        }
    }

    pub fn updated_slice(&self, server_count: u32) -> Self {
        Sliced {
            original_server_count: self.original_server_count,
            server_count,
            slice_count: self.slice_count,
        }
    }
}

impl NodeHashing {
    pub fn new_mod_based(non_sliced: NonSliced) -> NodeHashing {
        NodeHashing::ModBased(non_sliced)
    }

    pub fn uneven(sliced: Sliced) -> NodeHashing {
        NodeHashing::UnEven(sliced)
    }

    pub fn consistent(sliced: Sliced) -> NodeHashing {
        NodeHashing::ConsistenHashing(sliced)
    }

    pub fn get_four_server_pos(is_consitent: bool) -> Vec<Vec<u32>> {
        if is_consitent {
            vec![vec![9, 17], vec![15, 4], vec![2, 12], vec![5, 7]]
        } else {
            vec![vec![9], vec![15], vec![2], vec![5]]
        }
    }

    pub fn get_num_distribution() -> HashMap<u32, u32> {
        HashMap::<u32, u32>::from([
            (5, 7),
            (6, 16),
            (7, 18),
            (8, 3),
            (9, 14),
            (10, 4),
            (11, 5),
            (12, 12),
            (13, 11),
            (14, 9),
            (15, 15),
            (16, 1),
        ])
    }

    fn map_num_to_node(
        server_count: u32,
        from: u32,
        to: u32,
        result: &mut BTreeMap<u32, (Vec<u32>, Vec<u32>)>,
        is_first_round: bool,
        is_consistent: bool,
    ) {
        //let mut hasher = DefaultHasher::new();
        // server distribution with 0, 1, 2, 3, 4, is not distributed.
        // Fixing value for better distrubution.
        let server_hash_map = NodeHashing::get_four_server_pos(is_consistent)
            [0..server_count as usize]
            .into_iter()
            .map(|num| num.clone())
            .collect::<Vec<Vec<u32>>>();
        let mut server_ring = vec![0; 19];
        for server_num in 0..server_count {
            for pos in &server_hash_map[server_num as usize] {
                server_ring[*pos as usize] = server_num + 1;
            }
        }

        let forced_distribution = NodeHashing::get_num_distribution();
        for i in from..=to {
            // (i * 89).hash(&mut hasher);
            let num_hash = *forced_distribution.get(&i).unwrap();
            let mut done = false;
            let mut owner: Option<u32> = None;
            for j in (0..=num_hash).rev() {
                if server_ring[j as usize] != 0 {
                    owner = Some(server_ring[j as usize]);
                    done = true;
                    break;
                }
            }

            if !done {
                for j in (num_hash..19).rev() {
                    if server_ring[j as usize] != 0 {
                        owner = Some(server_ring[j as usize]);
                        break;
                    }
                }
            }

            result
                .entry(owner.unwrap() - 1)
                .and_modify(|current| {
                    if is_first_round {
                        current.0.push(i)
                    } else {
                        current.1.push(i)
                    }
                })
                .or_insert_with(|| {
                    if is_first_round {
                        (vec![i], Vec::new())
                    } else {
                        (Vec::new(), vec![i])
                    }
                });
        }
    }

    pub fn data_mapping(
        &self,
        from: u32,
        to: u32,
    ) -> (
        BTreeMap<u32, (Vec<u32>, Vec<u32>)>,
        Option<(HashMap<u32, u32>, HashMap<u32, Vec<u32>>)>,
    ) {
        match self {
            NodeHashing::ModBased(NonSliced {
                original_server_count,
                server_count,
            }) => {
                let mut node_distribution =
                    (from..=to)
                        .into_iter()
                        .fold(BTreeMap::new(), |mut map, current| {
                            map.entry(current % original_server_count)
                                .and_modify(|entry: &mut (Vec<u32>, Vec<u32>)| {
                                    entry.0.push(current)
                                })
                                .or_insert_with(|| (vec![current], Vec::new()));
                            map
                        });
                if original_server_count != server_count {
                    node_distribution =
                        (from..=to)
                            .into_iter()
                            .fold(node_distribution, |mut map, current| {
                                map.entry(current % server_count)
                                    .and_modify(|entry: &mut (Vec<u32>, Vec<u32>)| {
                                        entry.1.push(current)
                                    })
                                    .or_insert_with(|| (Vec::new(), vec![current]));
                                map
                            });
                }

                (node_distribution, None)
            }

            NodeHashing::UnEven(sliced) => {
                let mut result = BTreeMap::new();
                NodeHashing::map_num_to_node(
                    sliced.original_server_count,
                    from,
                    to,
                    &mut result,
                    true,
                    false,
                );
                if sliced.server_count != sliced.original_server_count {
                    NodeHashing::map_num_to_node(
                        sliced.server_count,
                        from,
                        to,
                        &mut result,
                        false,
                        false,
                    );
                }

                let server_map = NodeHashing::get_four_server_pos(false)
                    [0..sliced.server_count as usize]
                    .into_iter()
                    .enumerate()
                    .map(|(index, pos)| (index as u32, pos.clone()))
                    .collect();
                (
                    result,
                    Some((NodeHashing::get_num_distribution(), server_map)),
                )
            }

            NodeHashing::ConsistenHashing(sliced) => {
                let mut result = BTreeMap::new();
                NodeHashing::map_num_to_node(
                    sliced.original_server_count,
                    from,
                    to,
                    &mut result,
                    true,
                    true,
                );
                if sliced.server_count != sliced.original_server_count {
                    NodeHashing::map_num_to_node(
                        sliced.server_count,
                        from,
                        to,
                        &mut result,
                        false,
                        true,
                    );
                }

                let server_map = NodeHashing::get_four_server_pos(true)
                    [0..sliced.server_count as usize]
                    .into_iter()
                    .enumerate()
                    .map(|(index, pos)| (index as u32, pos.clone()))
                    .collect();
                (
                    result,
                    Some((NodeHashing::get_num_distribution(), server_map)),
                )
            }
        }
    }
}
