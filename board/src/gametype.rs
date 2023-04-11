// From https://boardgamegeek.com/thread/300883/letter-distribution
// boggleDice_Classic = ['AACIOT', 'ABILTY', 'ABJMO1', 'ACDEMP', 'ACELRS', 'ADENVZ', 'AHMORS', 'BIFORX', 'DENOSW', 'DKNOTU', 'EEFHIY', 'EGKLUY', 'EGINTV', 'EHINPS', 'ELPSTU', 'GILRUW']
// boggleDice_New = ['AAEEGN', 'ABBJOO', 'ACHOPS', 'AFFKPS', 'AOOTTW', 'CIMOTU', 'DEILRX', 'DELRVY', 'DISTTY', 'EEGHNW', 'EEINSU', 'EHRTVW', 'EIOSST', 'ELRTTY', 'HIMNU1', 'HLNNRZ']
// boggleDice_Big_Original = ['AAAFRS', 'AAEEEE', 'AAFIRS', 'ADENNN', 'AEEEEM', 'AEEGMU', 'AEGMNN', 'AFIRSY', 'BJK1XZ', 'CCENST', 'CEIILT', 'CEIPST', 'DDHNOT', 'DHHLOR', 'DHHLOR', 'DHLNOR', 'EIIITT', 'CEILPT', 'EMOTTT', 'ENSSSU', 'FIPRSY', 'GORRVW', 'IPRRRY', 'NOOTUW', 'OOOTTU']
// boggleDice_Big_Challenge = ['AAAFRS', 'AAEEEE', 'AAFIRS', 'ADENNN', 'AEEEEM', 'AEEGMU', 'AEGMNN', 'AFIRSY', 'BJK1XZ', 'CCENST', 'CEIILT', 'CEIPST', 'DDHNOT', 'DHHLOR', 'IKLM1U', 'DHLNOR', 'EIIITT', 'CEILPT', 'EMOTTT', 'ENSSSU', 'FIPRSY', 'GORRVW', 'IPRRRY', 'NOOTUW', 'OOOTTU']
// boggleDice_Big_Deluxe = ['AAAFRS', 'AAEEEE', 'AAFIRS', 'ADENNN', 'AEEEEM', 'AEEGMU', 'AEGMNN', 'AFIRSY', 'BJK1XZ', 'CCNSTW', 'CEIILT', 'CEIPST', 'DDLNOR', 'DHHLOR', 'DHHNOT', 'DHLNOR', 'EIIITT', 'CEILPT', 'EMOTTT', 'ENSSSU', 'FIPRSY', 'GORRVW', 'HIPRRY', 'NOOTUW', 'OOOTTU']
// boggleDice_Big_2012 = ['AAAFRS', 'AAEEEE', 'AAFIRS', 'ADENNN', 'AEEEEM', 'AEEGMU', 'AEGMNN', 'AFIRSY', 'BBJKXZ', 'CCENST', 'EIILST', 'CEIPST', 'DDHNOT', 'DHHLOR', 'DHHNOW', 'DHLNOR', 'EIIITT', 'EILPST', 'EMOTTT', 'ENSSSU', '123456', 'GORRVW', 'IPRSYY', 'NOOTUW', 'OOOTTU']
// boggleDice_Super_Big = ['AAAFRS', 'AAEEEE', 'AAEEOO', 'AAFIRS', 'ABDEIO', 'ADENNN', 'AEEEEM', 'AEEGMU', 'AEGMNN', 'AEILMN', 'AEINOU', 'AFIRSY', '123456', 'BBJKXZ', 'CCENST', 'CDDLNN', 'CEIITT', 'CEIPST', 'CFGNUY', 'DDHNOT', 'DHHLOR', 'DHHNOW', 'DHLNOR', 'EHILRS', 'EIILST', 'EILPST', 'EIO000', 'EMTTTO', 'ENSSSU', 'GORRVW', 'HIRSTV', 'HOPRST', 'IPRSYY', 'JK1WXZ', 'NOOTUW', 'OOOTTU']
// #0 = Blank, 1 = Qu, 2 = In, 3 = Th, 4 = Er, 5 = He, 6 = An

use crate::dice::Dice;

/// Game type enumeration
pub enum GameType {
    /// Original 4x4 English Boggle
    Classic,
    /// 4x4 English Boggle with easier dice
    New,
    /// Original 5x5 English Boggle
    BigOriginal,
    /// 5x5 English Boggle with challenge dice
    BigChallenge,
    /// 5x5 English Deluxe Boggle
    BigDeluxe,
    /// 2012 5x5 English Boggle
    Big2012,
    /// 6x6 English Boggle
    SuperBig,
}

impl GameType {
    /// Returns the dice used in a particular game variant
    pub fn dice(&self) -> Vec<Dice> {
        let faces = match self {
            GameType::Classic => vec![
                "AACIOT", "ABILTY", "ABJMO1", "ACDEMP", "ACELRS", "ADENVZ", "AHMORS", "BIFORX",
                "DENOSW", "DKNOTU", "EEFHIY", "EGKLUY", "EGINTV", "EHINPS", "ELPSTU", "GILRUW",
            ],
            GameType::New => vec![
                "AAEEGN", "ABBJOO", "ACHOPS", "AFFKPS", "AOOTTW", "CIMOTU", "DEILRX", "DELRVY",
                "DISTTY", "EEGHNW", "EEINSU", "EHRTVW", "EIOSST", "ELRTTY", "HIMNU1", "HLNNRZ",
            ],
            GameType::BigOriginal => vec![
                "AAAFRS", "AAEEEE", "AAFIRS", "ADENNN", "AEEEEM", "AEEGMU", "AEGMNN", "AFIRSY",
                "BJK1XZ", "CCENST", "CEIILT", "CEIPST", "DDHNOT", "DHHLOR", "DHHLOR", "DHLNOR",
                "EIIITT", "CEILPT", "EMOTTT", "ENSSSU", "FIPRSY", "GORRVW", "IPRRRY", "NOOTUW",
                "OOOTTU",
            ],
            GameType::BigChallenge => vec![
                "AAAFRS", "AAEEEE", "AAFIRS", "ADENNN", "AEEEEM", "AEEGMU", "AEGMNN", "AFIRSY",
                "BJK1XZ", "CCENST", "CEIILT", "CEIPST", "DDHNOT", "DHHLOR", "IKLM1U", "DHLNOR",
                "EIIITT", "CEILPT", "EMOTTT", "ENSSSU", "FIPRSY", "GORRVW", "IPRRRY", "NOOTUW",
                "OOOTTU",
            ],
            GameType::BigDeluxe => vec![
                "AAAFRS", "AAEEEE", "AAFIRS", "ADENNN", "AEEEEM", "AEEGMU", "AEGMNN", "AFIRSY",
                "BJK1XZ", "CCNSTW", "CEIILT", "CEIPST", "DDLNOR", "DHHLOR", "DHHNOT", "DHLNOR",
                "EIIITT", "CEILPT", "EMOTTT", "ENSSSU", "FIPRSY", "GORRVW", "HIPRRY", "NOOTUW",
                "OOOTTU",
            ],
            GameType::Big2012 => vec![
                "AAAFRS", "AAEEEE", "AAFIRS", "ADENNN", "AEEEEM", "AEEGMU", "AEGMNN", "AFIRSY",
                "BBJKXZ", "CCENST", "EIILST", "CEIPST", "DDHNOT", "DHHLOR", "DHHNOW", "DHLNOR",
                "EIIITT", "EILPST", "EMOTTT", "ENSSSU", "123456", "GORRVW", "IPRSYY", "NOOTUW",
                "OOOTTU",
            ],
            GameType::SuperBig => vec![
                "AAAFRS", "AAEEEE", "AAEEOO", "AAFIRS", "ABDEIO", "ADENNN", "AEEEEM", "AEEGMU",
                "AEGMNN", "AEILMN", "AEINOU", "AFIRSY", "123456", "BBJKXZ", "CCENST", "CDDLNN",
                "CEIITT", "CEIPST", "CFGNUY", "DDHNOT", "DHHLOR", "DHHNOW", "DHLNOR", "EHILRS",
                "EIILST", "EILPST", "EIO000", "EMTTTO", "ENSSSU", "GORRVW", "HIRSTV", "HOPRST",
                "IPRSYY", "JK1WXZ", "NOOTUW", "OOOTTU",
            ],
        };

        faces.iter().map(|f| Dice::from_string(f)).collect()
    }

    /// Returns the dimensions of the Boggle board for a variant
    pub fn layout(&self) -> (usize, usize) {
        match self {
            GameType::Classic | GameType::New => (4, 4),
            GameType::BigOriginal
            | GameType::BigChallenge
            | GameType::BigDeluxe
            | GameType::Big2012 => (5, 5),
            GameType::SuperBig => (6, 6),
        }
    }
}
