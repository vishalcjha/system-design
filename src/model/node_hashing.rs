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

    pub fn get_four_server_pos() -> Vec<u32> {
        vec![10, 15, 2, 6]
    }

    fn get_num_distribution(
        server_count: u32,
        from: u32,
        to: u32,
        result: &mut BTreeMap<u32, (Vec<u32>, Vec<u32>)>,
        is_first_round: bool,
    ) {
        //let mut hasher = DefaultHasher::new();
        // server distribution with 0, 1, 2, 3, 4, is not distributed.
        // Fixing value for better distrubution.
        let server_hash_map = NodeHashing::get_four_server_pos()[0..server_count as usize]
            .into_iter()
            .map(|num| *num)
            .collect::<Vec<u32>>();
        let mut server_ring = vec![0; 19];
        for server_num in 0..server_count {
            server_ring[server_hash_map[server_num as usize] as usize] = server_num + 1;
        }

        let forced_distribution = HashMap::<u32, u32>::from([
            (5, 11),
            (6, 3),
            (7, 15),
            (8, 13),
            (9, 4),
            (10, 8),
            (11, 16),
            (12, 17),
            (13, 10),
            (14, 18),
            (15, 12),
            (16, 5),
        ]);
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

    pub fn data_mapping(&self, from: u32, to: u32) -> BTreeMap<u32, (Vec<u32>, Vec<u32>)> {
        match self {
            NodeHashing::ModBased(NonSliced {
                original_server_count,
                server_count,
            }) => {
                let node_distribution =
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
                    return (from..=to)
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

                node_distribution
            }

            NodeHashing::UnEven(sliced) => {
                let mut result = BTreeMap::new();
                NodeHashing::get_num_distribution(
                    sliced.original_server_count,
                    from,
                    to,
                    &mut result,
                    true,
                );
                if sliced.server_count != sliced.original_server_count {
                    NodeHashing::get_num_distribution(
                        sliced.server_count,
                        from,
                        to,
                        &mut result,
                        false,
                    );
                }

                result
            }

            NodeHashing::ConsistenHashing(sliced) => {
                let mut result = BTreeMap::new();
                NodeHashing::get_num_distribution(
                    sliced.original_server_count,
                    from,
                    to,
                    &mut result,
                    true,
                );
                if sliced.server_count != sliced.original_server_count {
                    NodeHashing::get_num_distribution(
                        sliced.server_count,
                        from,
                        to,
                        &mut result,
                        false,
                    );
                }

                result
            }
        }
    }
}
