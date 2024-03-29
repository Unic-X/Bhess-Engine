use nom::{self, multi::separated_list1, bytes::complete::tag, IResult};
//Functional Way of FEN parsing

pub fn parse_rank()->IResult<&str,Vec<u64>>{
    
}


pub fn parse_fen(fen: &str)->IResult<&str,Vec<u64>>{
    let (fen,bitboards) = separated_list1(tag("/"), parse_rank)(fen)?;
    Ok((fen,bitboards))
}


#[cfg(test)]
mod fen_tests{
    use super::*;

    #[test]
    fn fen_parse_check() {

    }


    #[test]
    fn fen_parse_castle_w() {
            
    }


    #[test]
    fn fen_parse_castle_b() {
            
    }


}


