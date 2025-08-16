use crate::lexer::Token;

pub enum DotCommand {
    // ADDRSS(?),
    ASCII(String),
    BLOCK(usize),
    // BURN,
    BYTE(u8),
    END,
    // EQUATE,
    WORD(u16),
}

impl DotCommand {
    // pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
    //     let [dotcommand_token, other_tokens @ ..] = tokens else {
    //         return Err(Box::from("missing dot command token"));
    //     };
    //
    //     match &dotcommand_token.value.to_uppercase()[..] {
    //         ".ADDRSS" => {
    //             let x = other_tokens
    //             Self::ASCII()
    //         }
    //         ".ASCII" => ,
    //         ".BLOCK" => ,
    //         ".BURN" => ,
    //         ".BYTE" => ,
    //         ".END" => ,
    //         ".EQUATE" => ,
    //         ".WORD" => ,
    //     }
    // }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            // Self::ADDRSS => ?, // TODO need the lookup hash table to get this value
            Self::ASCII(s) => s.as_bytes().to_vec(), // FIXME this is clearly wrong
            Self::BLOCK(size) => vec![0; *size],
            // Self::BURN => ?, // WTF IS THIS LOL
            Self::BYTE(b) => vec![*b],
            Self::END => vec![],
            // Self::EQUATE ,
            Self::WORD(w) => w.to_ne_bytes().to_vec(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            // Self::ADDRSS(_) => 2,
            Self::ASCII(s) => s.len(), // FIXME this is clearly wrong
            Self::BLOCK(size) => *size,
            // Self::BURN => ?, // WTF IS THIS LOL
            Self::BYTE(_) => 1,
            Self::END => 1,
            // Self::EQUATE => 0,
            Self::WORD(_) => 2,
        }
    }
}
