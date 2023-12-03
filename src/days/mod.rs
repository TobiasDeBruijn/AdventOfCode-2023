mod d1;
pub use d1::*;

pub trait Day {
    fn run(input: Vec<u8>, part: u8) -> color_eyre::Result<()>;
}
