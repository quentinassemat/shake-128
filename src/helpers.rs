// Inspiré de https://keccak.team/keccak_specs_summary.html#rotationOffsets

use hex::encode;
use std::fmt;

const NR: usize = 24; // number of rounds
const R: usize = 168; // rate / 8 parce qu'on travaille avec des bytes
const R2: usize = 21; // rate / 64 parce qu'on travaille avec des u64

// pre computation :
const RC: [u64; NR] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

const ROT: [[u8; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

// début du programme

pub struct State {
    pub state: [[u64; 5]; 5],
}

impl State {
    pub fn new() -> State {
        State {
            state: [[0u64; 5]; 5],
        }
    }

    pub fn round(&mut self, index_round: usize) {
        let mut b = [[0u64; 5]; 5];
        let mut c = [0u64; 5];
        let mut d = [0u64; 5];

        // if index_round == 0 {
        //     println!("{}", self);
        // }

        // theta
        for x in 0..5 {
            c[x] = self.state[x][0]
                ^ self.state[x][1]
                ^ self.state[x][2]
                ^ self.state[x][3]
                ^ self.state[x][4];
        }

        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ (c[(x + 1) % 5].rotate_left(1));
        }
        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] ^= d[x];
            }
        }

        // rho pi
        for x in 0..5 {
            for y in 0..5 {
                b[y][(2 * x + 3 * y) % 5] = self.state[x][y].rotate_left(ROT[x][y].into());
                // on pourrait peut être encore optimiser ici en précalculant les indices comme nous l'avons déjà fait.
            }
        }

        // chi
        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = b[x][y] ^ (!(b[(x + 1) % 5][y]) & b[(x + 2) % 5][y]);
            }
        }

        // iota
        self.state[0][0] ^= RC[index_round];
    }

    pub fn kecccakf(&mut self) {
        for index_round in 0..NR {
            self.round(index_round);
        }
    }
}

pub fn shake128(input: &mut Vec<u8>, output_len: u64) -> Vec<u8> {
    // Padding
    input.push(0x1F);
    while input.len() % 168 != 0 {
        input.push(0);
    }
    let len = input.len();
    input[len - 1] ^= 0x80;
    let mut p: Vec<u64> = Vec::with_capacity(len / 8); // input mais rangé par u64 pour les opérations à venir
    for i in 0..(len / 8) {
        let mut temp = [0u8; 8];
        for j in 0..8 {
            temp[j] = input[8 * i + j];
        }
        p.push(u64::from_le_bytes(temp));
    }

    let n = len / R;

    // Initialisation
    let mut s = State::new();

    // Absorbing phase
    for i in 0..n {
        for x in 0..5 {
            for y in 0..5 {
                if (x + 5 * y) < R2 {
                    s.state[x][y] ^= p.get(i * R2 + x + (5 * y)).expect("erreur index");
                }
            }
        }
        s.kecccakf();
    }

    // Squeezing phase
    let mut z: Vec<u64> = Vec::new();
    while z.len() as u64 * 8 < output_len {
        for y in 0..5 {
            for x in 0..5 {
                if (x + 5 * y) < R2 {
                    z.push(s.state[x][y]);
                }
            }
        }
        s.kecccakf();
    }
    let mut output: Vec<u8> = Vec::with_capacity(output_len as usize);
    for i in 0..(output_len / 8) {
        let temp = z[i as usize].to_le_bytes();
        for j in 0..8 {
            output.push(temp[j]);
        }
    }
    let temp = z[(output_len / 8) as usize].to_le_bytes();
    for j in 0..(output_len % 8) {
        output.push(temp[j as usize]);
    }
    output
}

// Principalement pour afficher l'état pour débugguer
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        let mut count = 0;
        for y in 0..5 {
            for x in 0..5 {
                display.push_str(&encode(self.state[x][y].to_le_bytes()));
                // print!("{}", encode(self.state[x][y].to_le_bytes()));
                count += 1;
                if count % 2 == 0 {
                    // println!("");
                    display.push('\n');
                } else {
                    // print!(" ");
                    display.push(' ');
                }
            }
        }
        display.push('\n');
        write!(f, "{}", display)
    }
}