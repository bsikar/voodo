use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;

pub fn lenth_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = be_u16(input)?;
}