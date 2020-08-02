#[derive(Debug, PartialEq, Eq)]
pub struct RGB(pub u8, pub u8, pub u8);

pub trait SimilarityCalculator<T> {
    fn similarity(&self, other: &T) -> f64;
}

impl SimilarityCalculator<RGB> for RGB {
    fn similarity(&self, other: &RGB) -> f64 {
        let red_diff = (self.0 as i16 - other.0 as i16).abs();
        let green_diff = (self.1 as i16 - other.1 as i16).abs();
        let blue_diff = (self.2 as i16 - other.2 as i16).abs();

        let total_diff = red_diff + green_diff + blue_diff;

        if total_diff == 0 {
            return 1.0;
        }

        return 1.0 - (total_diff as f64 / (255 * 3) as f64);
    }
}

#[inline]
pub fn parse_hash(value: &[u8]) -> Result<RGB, ()> {
    match value.len() {
        // 8 => Ok(rgba(
        //     from_hex(value[0])? * 16 + from_hex(value[1])?,
        //     from_hex(value[2])? * 16 + from_hex(value[3])?,
        //     from_hex(value[4])? * 16 + from_hex(value[5])?,
        //     from_hex(value[6])? * 16 + from_hex(value[7])?,
        // )),
        6 => Ok(RGB(
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
        )),
        // 4 => Ok(rgba(
        //     from_hex(value[0])? * 17,
        //     from_hex(value[1])? * 17,
        //     from_hex(value[2])? * 17,
        //     from_hex(value[3])? * 17,
        // )),
        3 => Ok(RGB(
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
        )),
        _ => Err(()),
    }
}

#[inline]
fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_test() {
        assert_eq!(from_hex("#00FF99"), RGB(0, 255, 153));
        assert_eq!(from_hex("#000000"), RGB(0, 0, 0));
        assert_eq!(from_hex("#FFFFFF"), RGB(255, 255, 255));
    }
}
