#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Data,
    Instruction,
    Both,
}

#[derive(Default, Debug)]
pub struct Performance {
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

#[derive(Debug)]
pub struct Info {
    pub nsets: usize,
    pub bsize: usize,
    pub repl: ReplacementPolicy,
    pub assoc: usize,
    pub size: usize, // in bytes
}

#[derive(Debug)]
pub struct Data {
    pub tag: usize,
    pub is_initialized: bool,
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
        Cache {
            kind,
            performance: Default::default(),
            info: Info {
                nsets,
                bsize,
                repl,
                assoc,
                size: bsize * nsets * assoc,
            },
            data: {
                let mut vec: Vec<Vec<Data>> = Vec::with_capacity(nsets);

                for _index_conjunto in 0..nsets {
                    let mut conjunto = Vec::with_capacity(assoc);
                    for _index_via in 0..assoc {
                        conjunto.push(Data {
                            tag: 0,
                            is_initialized: false,
                        })
                    }
                    vec.push(conjunto);
                }

                vec
            },
        }
    }
}
