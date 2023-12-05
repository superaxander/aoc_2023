use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/5.txt")?;

    let mut seeds = Vec::new();
    let mut current_map = Vec::new();
    let mut maps = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if seeds.is_empty() {
            let (_, remaining) = line.split_once(':').unwrap();
            seeds.extend(
                remaining
                    .split(' ')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap()),
            );
        } else if line.is_empty() || !line.chars().next().unwrap().is_ascii_digit() {
            if !current_map.is_empty() {
                maps.push(current_map.clone());
                current_map.clear();
            }
        } else {
            let mut split = line.split(' ');
            let dst_range_start = split.next().unwrap().parse::<usize>()?;
            let src_range_start = split.next().unwrap().parse::<usize>()?;
            let src_range_len = split.next().unwrap().parse::<usize>()?;
            current_map.push((dst_range_start, src_range_start, src_range_len));
        }
    }
    if !current_map.is_empty() {
        maps.push(current_map);
    }

    let solution_a = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |current, map| {
                for (dst_range_start, src_range_start, src_range_len) in map {
                    if *src_range_start <= current && current < src_range_start + src_range_len {
                        return dst_range_start + (current - src_range_start);
                    }
                }
                current
            })
        })
        .min()
        .unwrap();

    let solution_b = seeds
        .into_iter()
        .array_chunks()
        .map(|[start, len]| find_min(start, len, &maps))
        .min()
        .unwrap();

    Ok((solution_a, solution_b))
}

fn find_min(src_start: usize, src_len: usize, maps: &[Vec<(usize, usize, usize)>]) -> usize {
    if maps.is_empty() {
        return src_start;
    }
    let mut mapped_ranges = Vec::new();
    let mut unmapped_ranges = vec![(src_start, src_len)];
    for (dst_start, src_start, src_len) in &maps[0] {
        let mut new_unmapped: Vec<(usize, usize)> = Vec::new();
        unmapped_ranges.retain(|(our_start, our_len)| {
            if *our_start < *src_start + *src_len && *src_start < *our_start + *our_len {
                // We overlap so split
                let mut current = *our_start;
                let mut length = *our_len;
                while length > 0 {
                    if current >= *src_start {
                        if current < *src_start + *src_len {
                            let offset = current - *src_start;
                            let len = length.min(*src_len - offset);
                            mapped_ranges.push((*dst_start + offset, len));
                            current += len;
                            length -= len;
                        } else {
                            new_unmapped.push((current, length));
                            current += length;
                            length = 0;
                        }
                    } else {
                        let len = src_start - current;
                        new_unmapped.push((current, len));
                        current += len;
                        length -= len;
                    }
                }
                false
            } else {
                true
            }
        });

        unmapped_ranges.extend(new_unmapped);
    }

    mapped_ranges
        .into_iter()
        .chain(unmapped_ranges)
        .map(|(start, len)| find_min(start, len, &maps[1..]))
        .min()
        .unwrap()
}
