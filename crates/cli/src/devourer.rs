enum Nanite {
    Searcher,
}

enum Signal {
    ToNest(u8),
    ToResources(u8),
}

enum CellContent {
    Empty(Option<Signal>),
    Nanite {
        nanite: Nanite,
        signal: Option<Signal>,
    },
    Resurces(u8),
}
