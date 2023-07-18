use chess_wasm::chess::*;
use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x} - ", byte))?;
        }
        Ok(())
    }
}

pub trait Engine {
    fn perft(&mut self, fen: &str, moves: &[String], depth: usize) -> io::Result<Perft>;
}

#[derive(Debug)]
pub struct Perft {
    total_count: u128,
    child_count: BTreeMap<String, u128>,
}

impl Perft {
    pub fn new(total_count: u128, child_count: BTreeMap<String, u128>) -> Perft {
        Perft {
            total_count,
            child_count,
        }
    }

    pub fn total_count(&self) -> u128 {
        self.total_count
    }

    pub fn child_count(&self) -> &BTreeMap<String, u128> {
        &self.child_count
    }
}

pub struct Stockfish {
    child: Child,
    inp: BufReader<ChildStdout>,
    out: ChildStdin,
}

impl Stockfish {
    pub fn new() -> io::Result<Stockfish> {
        let mut child = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut inp = BufReader::new(child.stdout.take().expect("stdout not captured"));
        // consume/skip header
        let mut buf = String::new();
        inp.read_line(&mut buf)?;

        let out = child.stdin.take().expect("stdin not captured");

        Ok(Stockfish { child, inp, out })
    }
}

impl Engine for Stockfish {
    fn perft(&mut self, fen: &str, moves: &[String], depth: usize) -> io::Result<Perft> {
        // send command to stockfish
        write!(self.out, "position fen {}", fen)?;
        if !moves.is_empty() {
            write!(self.out, " moves {}", moves.join(" "))?;
        }
        write!(self.out, "\ngo perft {}\n", depth)?;

        let mut buf = String::new();

        // parse child counts
        let mut child_count = BTreeMap::new();
        loop {
            buf.clear();
            self.inp.read_line(&mut buf)?;
            if buf.trim().is_empty() {
                break;
            }
            let mut parts = buf.trim().split(": ");
            let move_ = parts
                .next()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "unexpected end of line")
                })?
                .to_string();
            let count = parts
                .next()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "unexpected end of line")
                })?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            child_count.insert(move_, count);
        }

        // parse total count
        buf.clear();
        self.inp.read_line(&mut buf)?;
        let mut parts = buf.trim().split(": ");
        let total_count = parts
            .nth(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "unexpected end of line"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // throw away empty line
        buf.clear();
        self.inp.read_line(&mut buf)?;

        Ok(Perft {
            child_count,
            total_count,
        })
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn main() {
    let mut chess = Chess::new();
    let default = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    chess.load_fen(default.clone());
    // chess.load_fen(
    //     "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
    // );

    // println!("{:?}", chess.perft(4, true));

    let mut s = Stockfish::new().unwrap();

    let result = s.perft(default.as_str(), &[], 3);

    if let Ok(r) = result {
        println!("{:?}", r.child_count);
    }

    // let mut sum = 0;
    // for (_, val) in chess.moves {
    //     sum += val;
    // }

    // println!("SUMMMMM {}", sum);
}
