#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextThrow {
    pub left: u32,
    pub right: u32,
    pub certainty: u32
}

impl NextThrow {
    // The API only sends the translated sentence ("Go left 12 blocks, or right 15 blocks, for ~95%
    // certainty..."), so the numbers are read back from it in order.
    pub fn parse(message: &str) -> Option<Self> {
        let mut numbers = message
            .split(|character: char| !character.is_ascii_digit())
            .filter(|part| !part.is_empty())
            .map(str::parse);

        Some(Self {
            left: numbers.next()?.ok()?,
            right: numbers.next()?.ok()?,
            certainty: numbers.next()?.ok()?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_numbers_from_the_sentence() {
        let message =
            "Go left 12 blocks, or right 15 blocks, for ~95% certainty after next measurement.";

        assert_eq!(
            NextThrow::parse(message),
            Some(NextThrow {
                left: 12,
                right: 15,
                certainty: 95
            })
        );
    }

    #[test]
    fn rejects_a_sentence_without_the_numbers() {
        assert_eq!(
            NextThrow::parse("Could not determine optimal sideways distance."),
            None
        );
    }
}
