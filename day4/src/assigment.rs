use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Assignments {
    first_assigment: RangeInclusive<u32>,
    second_assigment: RangeInclusive<u32>,
}

struct MyRange(RangeInclusive<u32>);

impl FromStr for MyRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        let start: u32 = split.next().ok_or(())?.parse().expect("Cannot parse start");
        let end: u32 = split.next().ok_or(())?.parse().expect("Cannot parse end");
        Ok(Self(start..=end))
    }
}

impl FromStr for Assignments {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let first_assigment = split.next().ok_or(())?;
        let second_assigment = split.next().ok_or(())?;

        let first_assigment = first_assigment.parse::<MyRange>()?.0;
        let second_assigment = second_assigment.parse::<MyRange>()?.0;

        Ok(Self {
            first_assigment,
            second_assigment,
        })
    }
}

impl Assignments {
    pub fn does_one_contain_another(&self) -> bool {
        range_contain_another(&self.first_assigment, &self.second_assigment)
            || range_contain_another(&self.second_assigment, &self.first_assigment)
    }

    pub fn intersects(&self) -> bool {
        self.first_assigment.contains(self.second_assigment.start())
            || self.second_assigment.contains(self.first_assigment.start())
    }
}

fn range_contain_another<Idx>(range: &RangeInclusive<Idx>, another: &RangeInclusive<Idx>) -> bool
where
    Idx: PartialOrd<Idx>,
{
    range.contains(another.start()) && range.contains(another.end())
}
