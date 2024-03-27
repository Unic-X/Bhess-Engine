use crate::board::*;
use nom::{self, multi::separated_list1, bytes::complete::tag, IResult};
//Functional Way of FEN parsing


fn parse_rank()->IResult<>{}


fn parse_fen()->IResult{
    separated_list1(tag(r"/"), f);
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


