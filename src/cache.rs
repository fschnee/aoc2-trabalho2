#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReplacementPolicy {
    Lru,
    Fifo,
    Random,
}

trait Conjunto {
    fn has_tag(&self, tag: usize) -> bool;
    fn get_index_by_tag(&self, tag: usize) -> Option<usize>;
    fn uninitialized_slots(&self) -> usize;
    fn first_vacant_slot_index(&self) -> Option<usize>;
    fn insert_tag(&mut self, tag: usize, repl: ReplacementPolicy, rng: &mut rand::rngs::StdRng);
    fn get_highest_replaceability_index(&self) -> Option<usize>;
}

impl Conjunto for Vec<Data> {
    fn has_tag(&self, tag: usize) -> bool {
        self.iter()
            .any(|elem| elem.is_initialized && elem.tag == tag)
    }

    fn get_index_by_tag(&self, tag: usize) -> Option<usize> {
        self.iter()
            .enumerate()
            .find(|(_, elem)| elem.tag == tag)
            .map(|(index, _)| index.to_owned())
    }

    fn uninitialized_slots(&self) -> usize {
        self.iter()
            .fold(0, |acc, elem| acc + (!elem.is_initialized as usize))
    }

    fn first_vacant_slot_index(&self) -> Option<usize> {
        self.iter()
            .enumerate()
            .find(|(_, elem)| !elem.is_initialized)
            .map(|(index, _)| index.to_owned())
    }

    fn insert_tag(&mut self, tag: usize, repl: ReplacementPolicy, rng: &mut rand::rngs::StdRng) {
        if self.has_tag(tag) && repl != ReplacementPolicy::Lru {
            return;
        }

        match repl {
            ReplacementPolicy::Random => {
                if let Some(vacancy_index) = self.first_vacant_slot_index() {
                    self[vacancy_index].is_initialized = true;
                    self[vacancy_index].tag = tag;
                } else {
                    use rand::seq::SliceRandom;

                    self.choose_mut(rng)
                        .expect("Tentou escolher número aleatório de um conjunto de 0 vias")
                        .tag = tag;
                }
            }
            ReplacementPolicy::Fifo => {
                if let Some(vacancy_index) = self.first_vacant_slot_index() {
                    self[vacancy_index].is_initialized = true;
                    self[vacancy_index].tag = tag;
                    self[vacancy_index].replaceability = 0;
                } else if let Some(replaced_index) = self.get_highest_replaceability_index() {
                    self[replaced_index].tag = tag;
                    self[replaced_index].replaceability = 0;
                }

                self.iter_mut().for_each(|elem| elem.replaceability += 1);
            }
            ReplacementPolicy::Lru => {
                if self.has_tag(tag) {
                    let tagged_index = self.get_index_by_tag(tag).unwrap();
                    let tagged_replaceability = self[tagged_index].replaceability;
                    self.iter_mut().for_each(|elem| {
                        if elem.replaceability < tagged_replaceability {
                            elem.replaceability += 1;
                        }
                    });
                    self[tagged_index].replaceability = 1;
                } else if let Some(vacancy_index) = self.first_vacant_slot_index() {
                    self[vacancy_index].is_initialized = true;
                    self[vacancy_index].tag = tag;
                    self[vacancy_index].replaceability = 0;
                    self.iter_mut().for_each(|elem| elem.replaceability += 1);
                } else {
                    let replaced_index = self.get_highest_replaceability_index().unwrap();
                    let replaced_replaceability = self[replaced_index].replaceability;
                    self.iter_mut().for_each(|elem| {
                        if elem.replaceability < replaced_replaceability {
                            elem.replaceability += 1;
                        }
                    });
                    self[replaced_index].tag = tag;
                    self[replaced_index].replaceability = 1;
                }
            }
        }
    }

    fn get_highest_replaceability_index(&self) -> Option<usize> {
        self.iter()
            .enumerate()
            .scan(None, |state: &mut Option<usize>, (index, elem)| {
                if elem.is_initialized && !(state.is_some() && state.unwrap() > elem.replaceability)
                {
                    *state = Some(index);
                }

                *state
            })
            .last()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Data,
    Instruction,
    Both,
}

#[derive(Default, Debug)]
pub struct Performance {
    pub slots_occupied: usize,
    pub accesses: usize,
    pub hits: usize,
    pub misses: usize,
    pub compulsory_misses: usize,
    pub capacity_misses: usize,
    pub conflict_misses: usize,
}

#[derive(Debug)]
pub struct Info {
    pub nsets: usize,
    pub bsize: usize,
    pub repl: ReplacementPolicy,
    pub assoc: usize,
    pub size: usize, // in bytes
    pub total_slots: usize,
    pub rng: rand::rngs::StdRng,
}

#[derive(Debug)]
pub struct Data {
    pub tag: usize,
    pub is_initialized: bool,
    // Higher means more likely to be replaced
    pub replaceability: usize,
}

#[derive(Debug)]
pub struct Cache {
    pub kind: Kind,
    pub performance: Performance,
    pub info: Info,
    pub data: Vec<Vec<Data>>,
}

impl Cache {
    pub fn create(
        nsets: usize,
        bsize: usize,
        repl: ReplacementPolicy,
        assoc: usize,
        kind: Kind,
    ) -> Cache {
        use rand::RngCore;
        Cache::create_with_seed(
            nsets,
            bsize,
            repl,
            assoc,
            kind,
            rand::thread_rng().next_u64(),
        )
    }

    pub fn create_with_seed(
        nsets: usize,
        bsize: usize,
        repl: ReplacementPolicy,
        assoc: usize,
        kind: Kind,
        random_repl_seed: u64,
    ) -> Cache {
        Cache {
            kind,
            performance: Default::default(),
            info: Info {
                nsets,
                bsize,
                repl,
                assoc,
                size: bsize * nsets * assoc,
                total_slots: nsets * assoc,
                rng: rand::SeedableRng::seed_from_u64(random_repl_seed),
            },
            data: {
                let mut vec: Vec<Vec<Data>> = Vec::with_capacity(nsets);

                for _index_conjunto in 0..nsets {
                    let mut conjunto = Vec::with_capacity(assoc);
                    for _index_via in 0..assoc {
                        conjunto.push(Data {
                            tag: 0,
                            is_initialized: false,
                            replaceability: 0,
                        })
                    }
                    vec.push(conjunto);
                }

                vec
            },
        }
    }

    pub fn print_perf(&self, verbosity: u8) {
        if verbosity == 1 {
            println!(
                "{}, {}, {}, {}, {}, {}",
                self.performance.accesses,
                self.performance.hits as f64 / self.performance.accesses as f64,
                self.performance.misses as f64 / self.performance.accesses as f64,
                self.performance.compulsory_misses as f64 / self.performance.misses as f64,
                self.performance.capacity_misses as f64 / self.performance.misses as f64,
                self.performance.conflict_misses as f64 / self.performance.misses as f64
            );
        } else {
            println!("{:#?}", self.performance);
        }
    }

    pub fn access_with(&mut self, index: usize, tag: usize, _offset: usize) -> AccessResult {
        self.performance.accesses += 1;

        if self.data[index].has_tag(tag) {
            self.performance.hits += 1;

            self.data[index].insert_tag(tag, self.info.repl, &mut self.info.rng);

            AccessResult::Hit
        } else {
            self.performance.misses += 1;

            if self.data[index].uninitialized_slots() > 0 {
                self.performance.compulsory_misses += 1;
                // Ocupa o slot porque ele vai ser enchido
                self.performance.slots_occupied += 1;
                self.data[index].insert_tag(tag, self.info.repl, &mut self.info.rng);

                AccessResult::Miss(MissTypes::Compulsory)
            } else if self.performance.slots_occupied == self.info.total_slots {
                self.performance.capacity_misses += 1;

                self.data[index].insert_tag(tag, self.info.repl, &mut self.info.rng);

                AccessResult::Miss(MissTypes::Capacity)
            } else {
                self.performance.conflict_misses += 1;

                self.data[index].insert_tag(tag, self.info.repl, &mut self.info.rng);

                AccessResult::Miss(MissTypes::Conflict)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AccessResult {
    Hit,
    Miss(MissTypes),
}

#[derive(Debug, PartialEq)]
pub enum MissTypes {
    Compulsory,
    Capacity,
    Conflict,
    CapacityAndConflict,
}
