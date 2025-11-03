use crate::address::{AddrMode, Address, AddressTable};
use crate::lexer::Token;
use crate::register::Register;
use crate::types::Pep8Byte;

#[derive(Debug)]
pub enum Instruction {
    STOP,
    RETTR,
    MOVSPA,
    MOVFLGA,
    BR(Address),
    BRLE(Address),
    BRLT(Address),
    BREQ(Address),
    BRNE(Address),
    BRGE(Address),
    BRGT(Address),
    BRV(Address),
    BRC(Address),
    CALL(Address),
    NOTr(Register),
    NEGr(Register),
    ASLr(Register),
    ASRr(Register),
    ROLr(Register),
    RORr(Register),
    NOPn(Pep8Byte),
    NOP(Address),
    DECI(Address),
    DECO(Address),
    STRO(Address),
    CHARI(Address),
    CHARO(Address),
    RETn(Pep8Byte),
    ADDSP(Address),
    SUBSP(Address),
    ADDr(Register, Address),
    SUBr(Register, Address),
    ANDr(Register, Address),
    ORr(Register, Address),
    CPr(Register, Address),
    LDr(Register, Address),
    LDBYTEr(Register, Address),
    STr(Register, Address),
    STBYTEr(Register, Address),
}

impl Instruction {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        let [Token::Identifier(instruction), other_tokens @ ..] = tokens else {
            panic!("missing instruction token");
        };

        match &instruction.to_uppercase()[..] {
            "STOP" => Ok(Self::STOP),
            "RETTR" => Ok(Self::RETTR),
            "MOVSPA" => Ok(Self::MOVSPA),
            "MOVFLGA" => Ok(Self::MOVFLGA),
            "BR" => Ok(Self::BR(Address::from_tokens_short(other_tokens)?)),
            "BRLE" => Ok(Self::BRLE(Address::from_tokens_short(other_tokens)?)),
            "BRLT" => Ok(Self::BRLT(Address::from_tokens_short(other_tokens)?)),
            "BREQ" => Ok(Self::BREQ(Address::from_tokens_short(other_tokens)?)),
            "BRNE" => Ok(Self::BRNE(Address::from_tokens_short(other_tokens)?)),
            "BRGE" => Ok(Self::BRGE(Address::from_tokens_short(other_tokens)?)),
            "BRGT" => Ok(Self::BRGT(Address::from_tokens_short(other_tokens)?)),
            "BRV" => Ok(Self::BRV(Address::from_tokens_short(other_tokens)?)),
            "BRC" => Ok(Self::BRC(Address::from_tokens_short(other_tokens)?)),
            "CALL" => Ok(Self::CALL(Address::from_tokens_short(other_tokens)?)),
            "NOTA" => Ok(Self::NOTr(Register::Accumulator)),
            "NOTX" => Ok(Self::NOTr(Register::IndexRegister)),
            "NEGA" => Ok(Self::NEGr(Register::Accumulator)),
            "NEGX" => Ok(Self::NEGr(Register::IndexRegister)),
            "ASLA" => Ok(Self::ASLr(Register::Accumulator)),
            "ASLX" => Ok(Self::ASLr(Register::IndexRegister)),
            "ASRA" => Ok(Self::ASRr(Register::Accumulator)),
            "ASRX" => Ok(Self::ASRr(Register::IndexRegister)),
            "ROLA" => Ok(Self::ROLr(Register::Accumulator)),
            "ROLX" => Ok(Self::ROLr(Register::IndexRegister)),
            "RORA" => Ok(Self::RORr(Register::Accumulator)),
            "RORX" => Ok(Self::RORr(Register::IndexRegister)),
            "NOP0" => Ok(Self::NOPn(Pep8Byte::new(0))),
            "NOP1" => Ok(Self::NOPn(Pep8Byte::new(1))),
            "NOP2" => Ok(Self::NOPn(Pep8Byte::new(2))),
            "NOP3" => Ok(Self::NOPn(Pep8Byte::new(3))),
            "NOP" => Ok(Self::NOP(Address::from_tokens_long(
                other_tokens,
                &[AddrMode::Immediate],
            )?)),
            "DECI" => Ok(Self::DECI(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "DECO" => Ok(Self::DECO(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Immediate,
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "STRO" => Ok(Self::STRO(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelativeDeferred,
                ],
            )?)),
            "CHARI" => Ok(Self::CHARI(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "CHARO" => Ok(Self::CHARO(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Immediate,
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "RET0" => Ok(Self::RETn(Pep8Byte::new(0))),
            "RET1" => Ok(Self::RETn(Pep8Byte::new(1))),
            "RET2" => Ok(Self::RETn(Pep8Byte::new(2))),
            "RET3" => Ok(Self::RETn(Pep8Byte::new(3))),
            "RET4" => Ok(Self::RETn(Pep8Byte::new(4))),
            "RET5" => Ok(Self::RETn(Pep8Byte::new(5))),
            "RET6" => Ok(Self::RETn(Pep8Byte::new(6))),
            "RET7" => Ok(Self::RETn(Pep8Byte::new(7))),
            "ADDSP" => Ok(Self::ADDSP(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Immediate,
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "SUBSP" => Ok(Self::SUBSP(Address::from_tokens_long(
                other_tokens,
                &[
                    AddrMode::Immediate,
                    AddrMode::Direct,
                    AddrMode::Indirect,
                    AddrMode::StackRelative,
                    AddrMode::StackRelativeDeferred,
                    AddrMode::Indexed,
                    AddrMode::StackIndexed,
                    AddrMode::StackIndexedDeferred,
                ],
            )?)),
            "ADDA" => Ok(Self::ADDr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "ADDX" => Ok(Self::ADDr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "SUBA" => Ok(Self::SUBr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "SUBX" => Ok(Self::SUBr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "ANDA" => Ok(Self::ANDr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "ANDX" => Ok(Self::ANDr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "ORA" => Ok(Self::ORr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "ORX" => Ok(Self::ORr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "CPA" => Ok(Self::CPr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "CPX" => Ok(Self::CPr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "LDA" => Ok(Self::LDr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "LDX" => Ok(Self::LDr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "LDBYTEA" => Ok(Self::LDBYTEr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "LDBYTEX" => Ok(Self::LDBYTEr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Immediate,
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "STA" => Ok(Self::STr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "STX" => Ok(Self::STr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "STBYTEA" => Ok(Self::STBYTEr(
                Register::Accumulator,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            "STBYTEX" => Ok(Self::STBYTEr(
                Register::IndexRegister,
                Address::from_tokens_long(
                    other_tokens,
                    &[
                        AddrMode::Direct,
                        AddrMode::Indirect,
                        AddrMode::StackRelative,
                        AddrMode::StackRelativeDeferred,
                        AddrMode::Indexed,
                        AddrMode::StackIndexed,
                        AddrMode::StackIndexedDeferred,
                    ],
                )?,
            )),
            _ => Err(Box::from("invalid instruction token")),
        }
    }

    pub fn get_specifier(&self) -> u8 {
        match self {
            Self::STOP => 0b00000000,        // 00000000
            Self::RETTR => 0b00000001,       // 00000001
            Self::MOVSPA => 0b00000010,      // 00000010
            Self::MOVFLGA => 0b00000011,     // 00000011
            Self::BR(..) => 0b00000100,      // 0000010a
            Self::BRLE(..) => 0b00000110,    // 0000011a
            Self::BRLT(..) => 0b00001000,    // 0000100a
            Self::BREQ(..) => 0b00001010,    // 0000101a
            Self::BRNE(..) => 0b00001100,    // 0000110a
            Self::BRGE(..) => 0b00001110,    // 0000111a
            Self::BRGT(..) => 0b00010000,    // 0001000a
            Self::BRV(..) => 0b00010010,     // 0001001a
            Self::BRC(..) => 0b00010100,     // 0001010a
            Self::CALL(..) => 0b00010110,    // 0001011a
            Self::NOTr(..) => 0b00011000,    // 0001100r
            Self::NEGr(..) => 0b00011010,    // 0001101r
            Self::ASLr(..) => 0b00011100,    // 0001110r
            Self::ASRr(..) => 0b00011110,    // 0001111r
            Self::ROLr(..) => 0b00100000,    // 0010000r
            Self::RORr(..) => 0b00100010,    // 0010001r
            Self::NOPn(..) => 0b00100100,    // 001001nn
            Self::NOP(..) => 0b00101000,     // 00101aaa
            Self::DECI(..) => 0b00110000,    // 00110aaa
            Self::DECO(..) => 0b00111000,    // 00111aaa
            Self::STRO(..) => 0b01000000,    // 01000aaa
            Self::CHARI(..) => 0b01001000,   // 01001aaa
            Self::CHARO(..) => 0b01010000,   // 01010aaa
            Self::RETn(..) => 0b01011000,    // 01011nnn
            Self::ADDSP(..) => 0b01100000,   // 01100aaa
            Self::SUBSP(..) => 0b01101000,   // 01101aaa
            Self::ADDr(..) => 0b01110000,    // 0111raaa
            Self::SUBr(..) => 0b10000000,    // 1000raaa
            Self::ANDr(..) => 0b10010000,    // 1001raaa
            Self::ORr(..) => 0b10100000,     // 1010raaa
            Self::CPr(..) => 0b10110000,     // 1011raaa
            Self::LDr(..) => 0b11000000,     // 1100raaa
            Self::LDBYTEr(..) => 0b11010000, // 1101raaa
            Self::STr(..) => 0b11100000,     // 1110raaa
            Self::STBYTEr(..) => 0b11110000, // 1111raaa
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Self::STOP
            | Self::RETTR
            | Self::MOVSPA
            | Self::MOVFLGA
            | Self::NOTr(..)
            | Self::NEGr(..)
            | Self::ASLr(..)
            | Self::ASRr(..)
            | Self::ROLr(..)
            | Self::RORr(..)
            | Self::NOPn(..)
            | Self::RETn(..) => 1,
            Self::BR(..)
            | Self::BRLE(..)
            | Self::BRLT(..)
            | Self::BREQ(..)
            | Self::BRNE(..)
            | Self::BRGE(..)
            | Self::BRGT(..)
            | Self::BRV(..)
            | Self::BRC(..)
            | Self::CALL(..)
            | Self::NOP(..)
            | Self::DECI(..)
            | Self::DECO(..)
            | Self::STRO(..)
            | Self::CHARI(..)
            | Self::CHARO(..)
            | Self::ADDSP(..)
            | Self::SUBSP(..)
            | Self::ADDr(..)
            | Self::SUBr(..)
            | Self::ANDr(..)
            | Self::ORr(..)
            | Self::CPr(..)
            | Self::LDr(..)
            | Self::LDBYTEr(..)
            | Self::STr(..)
            | Self::STBYTEr(..) => 3,
        }
    }

    pub fn as_bytes(
        &self,
        address_table: &AddressTable,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = vec![];

        match self {
            Self::STOP | Self::RETTR | Self::MOVSPA | Self::MOVFLGA => {
                bytes.push(self.get_specifier())
            }
            Self::BR(address)
            | Self::BRLE(address)
            | Self::BRLT(address)
            | Self::BREQ(address)
            | Self::BRNE(address)
            | Self::BRGE(address)
            | Self::BRGT(address)
            | Self::BRV(address)
            | Self::BRC(address)
            | Self::CALL(address) => {
                let address_location = address_table
                    .resolve(address)
                    .ok_or::<Box<dyn std::error::Error>>(Box::from(format!(
                        "cannot resolve address {:?}",
                        address
                    )))?;

                bytes.push(self.get_specifier() + address.mode.as_byte_short()?);
                bytes.extend_from_slice(&address_location.as_bytes());
            }
            Self::NOTr(register)
            | Self::NEGr(register)
            | Self::ASLr(register)
            | Self::ASRr(register)
            | Self::ROLr(register)
            | Self::RORr(register) => bytes.push(0b00100010 + register.as_byte()), // 0010001r
            Self::NOPn(n) => {
                if *n > Pep8Byte::new(0b11) {
                    panic!("count for NOP is too large: {}", n.as_byte());
                } else {
                    bytes.push((Pep8Byte::new(0b00100100) + n).as_byte());
                }
            }
            Self::NOP(address)
            | Self::DECI(address)
            | Self::DECO(address)
            | Self::STRO(address)
            | Self::CHARI(address)
            | Self::CHARO(address)
            | Self::ADDSP(address)
            | Self::SUBSP(address) => {
                let address_location = address_table
                    .resolve(address)
                    .ok_or::<Box<dyn std::error::Error>>(Box::from(format!(
                        "cannot resolve address {:?}",
                        address
                    )))?;

                bytes.push(self.get_specifier() + address.mode.as_byte_long());
                bytes.extend_from_slice(&address_location.as_bytes());
            }
            Self::RETn(n) => {
                if *n > Pep8Byte::new(0b111) {
                    panic!("count for RET is too large: {}", n.as_byte());
                } else {
                    bytes.push((Pep8Byte::new(0b01011000) + n).as_byte());
                }
            }
            Self::ADDr(register, address)
            | Self::SUBr(register, address)
            | Self::ANDr(register, address)
            | Self::ORr(register, address)
            | Self::CPr(register, address)
            | Self::LDr(register, address)
            | Self::LDBYTEr(register, address)
            | Self::STr(register, address)
            | Self::STBYTEr(register, address) => {
                let address_location = address_table
                    .resolve(address)
                    .ok_or::<Box<dyn std::error::Error>>(Box::from(format!(
                        "cannot resolve address {:?}",
                        address
                    )))?;

                bytes.push(
                    self.get_specifier() + (register.as_byte() << 3) + address.mode.as_byte_long(),
                );
                bytes.extend_from_slice(&address_location.as_bytes());
            }
        }

        Ok(bytes)
    }
}
