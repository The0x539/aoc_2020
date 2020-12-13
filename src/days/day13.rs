use crate::prelude::*;

pub enum Day13 {}

impl Challenge for Day13 {
    type Input = (u64, Vec<Option<u64>>);
    type Output1 = u64;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut lines = data.lines();
        let start = lines.next().unwrap()?.parse()?;
        let buses = lines
            .next()
            .unwrap()?
            .split(',')
            .map(|b| b.parse().ok())
            .collect();
        Ok((start, buses))
    }

    fn part1((start, buses): Self::Input) -> Self::Output1 {
        for t in start.. {
            for id in buses.iter().copied().filter_map(|x| x) {
                if t % id == 0 {
                    return id * (t - start);
                }
            }
        }
        unreachable!()
    }

    fn part2((_, buses): Self::Input) -> Self::Output2 {
        let mut buses = buses
            .into_iter()
            .enumerate()
            .filter_map(|(a, b)| Some((a as u64, b? as u64)));

        let mut step = buses.next().unwrap().1;
        let mut t = 0;

        while let Some((dt, id)) = buses.next() {
            while (t + dt) % id != 0 {
                t += step;
            }
            step *= id;
            // now all future values of t will satisfy this bus's condition
            // so now we can worry only about the remaining buses
        }

        t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day13 as Challenge>::Input {
        (
            939,
            vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19),
            ],
        )
    }

    #[test]
    fn test_day13_part1() {
        assert_eq!(Day13::part1(sample_input()), 295);
    }

    #[test]
    fn test_day13_part2() {
        assert_eq!(Day13::part2(sample_input()), 1068781_u64.into());
    }
}
