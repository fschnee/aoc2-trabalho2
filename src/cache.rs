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
    pub conflict_misses: usize,
    pub capacity_misses: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReplacementPolicy {
    Lru,
    Fifo,
    Random,
}

pub trait Conjunto {
    fn get_elem_with_tag(&self, tag: usize) -> Option<&Data>;
    fn uninitialized_slots(&self) -> usize;
    fn insert_tag(&mut self, tag: usize, repl: ReplacementPolicy, rng: &mut rand::rngs::StdRng);
    fn get_highest_replaceability_mut(&mut self) -> Option<&mut Data>;
    fn register_hit(&mut self, tag: usize, repl: ReplacementPolicy);
}

impl Conjunto for Vec<Data> {
    fn get_elem_with_tag(&self, tag: usize) -> Option<&Data> {
        self.iter()
            .find(|&elem| elem.is_initialized && elem.tag == tag)
    }

    fn uninitialized_slots(&self) -> usize {
        self.iter()
            .fold(0, |acc, elem| acc + (!elem.is_initialized as usize))
    }

    fn insert_tag(&mut self, tag: usize, repl: ReplacementPolicy, rng: &mut rand::rngs::StdRng) {
        match repl {
            ReplacementPolicy::Lru | ReplacementPolicy::Fifo => {
                if let Some(elem) = self.get_highest_replaceability_mut() {
                    elem.tag = tag;
                    elem.replaceability = 0;
                } else {
                    self[0].tag = tag;
                    self[0].replaceability = 0;
                }

                self.iter_mut().for_each(|elem| elem.replaceability += 1);
            }
            ReplacementPolicy::Random => {
                if let Some(elem) = self.iter_mut().find(|elem| !elem.is_initialized) {
                    elem.is_initialized = true;
                    elem.tag = tag;
                } else {
                    use rand::seq::SliceRandom;

                    self.choose_mut(rng)
                        .expect("Tentou escolher número aleatório de um conjunto de 0 vias")
                        .tag = tag;
                }
            }
        }
    }

    fn get_highest_replaceability_mut(&mut self) -> Option<&mut Data> {
        let mut highest: Option<&mut Data> = None;
        for data in self.iter_mut() {
            if data.is_initialized {
                if let Some(otherdata) = &highest {
                    if data.replaceability > otherdata.replaceability {
                        highest = Some(data);
                    }
                } else {
                    highest = Some(data);
                }
            }
        }

        highest
    }

    // Assume que existe um elemento com a tag.
    fn register_hit(&mut self, tag: usize, repl: ReplacementPolicy) {
        if repl != ReplacementPolicy::Lru {
            return;
        }

        let elem: &mut Data = self.iter_mut().find(|elem| elem.tag == tag).unwrap();
        if elem.replaceability != 1 {
            elem.replaceability = 0;
            self.iter_mut().for_each(|e| e.replaceability += 1);
        }
    }
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
}
