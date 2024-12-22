use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::path::Path;

use anyhow::{Context, Result};

const INPUT_FILE: &str = "input.txt";

/// * K = a given integer
/// * V = set of integers that __CANNOT__ precede K
type RuleMap = HashMap<u8, HashSet<u8>>;
type PagesList = Vec<Vec<u8>>;

fn main() {
    if let Err(e) = runner() {
        eprintln!("ERROR: {}", e);
    }
}

fn runner() -> Result<()> {
    println!("===== DAY 05 =====");

    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    // let input_reader = BufReader::new(File::open(input_path)?);
    let raw_input = std::fs::read_to_string(input_path)?;
    let (rules, pages) = parse_input(&raw_input)?;

    let ans_one = part_one(&rules, &pages);
    println!("Part one:\t{}", ans_one);

    let (rules, mut pages) = parse_input_two(&raw_input)?;

    let ans_two = part_two(&rules, &mut pages);
    println!("Part two:\t{}", ans_two);

    Ok(())
}

fn parse_input(raw_input: &str) -> Result<(RuleMap, PagesList)> {
    let (rules, pages) = raw_input
        .split_once("\n\n")
        .context("Input not in two sections")?;

    let parsed_rules = rules.lines().try_fold(
        HashMap::<u8, HashSet<u8>>::new(),
        |mut acc, rule| -> Result<_> {
            let (predicate, item) = rule.split_once('|').context("Rule missing pipe divider")?;
            let predicate = predicate.parse()?;
            let item = item.parse()?;
            acc.entry(predicate).or_default().insert(item);
            Ok(acc)
        },
    )?;

    let parsed_pages = pages
        .lines()
        .map(|page_list| {
            Ok(page_list
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((parsed_rules, parsed_pages))
}

/// 1. iterate through pages
/// 1. iterate through each page
/// 1. for each item in page:
///    1. get the list of its value from the rules map
///    1. get the subset of the page that comes before the current item
///    1. if the subset contains any items from the list, the page is OUT OF ORDER
///    1. Otherwise, it is IN ORDER: we find the middle value and add it to the cumulative sum
fn part_one<O, I>(rules: &RuleMap, pages: O) -> usize
where
    O: IntoIterator<Item = I>,
    I: AsRef<[u8]>,
{
    pages
        .into_iter()
        .filter_map(|page| {
            let page = page.as_ref();
            let mut good = true;

            for (idx, item) in page.iter().enumerate() {
                let Some(banned_items) = rules.get(item) else {
                    continue;
                };
                for el in &page[..idx] {
                    if banned_items.contains(el) {
                        good = false;
                        break;
                    }
                }
            }

            good.then_some(page[page.len() / 2] as usize)
        })
        .sum()
}

fn parse_input_two(raw_input: &str) -> Result<(HashSet<(u8, u8)>, PagesList)> {
    let (rules, pages) = raw_input
        .split_once("\n\n")
        .context("Input not in two sections")?;

    let parsed_rules: HashSet<(u8, u8)> = rules
        .lines()
        .map(|line| -> Result<_, ParseIntError> { Ok((line[0..2].parse()?, line[3..].parse()?)) })
        .collect::<Result<_, _>>()?;

    let parsed_pages = pages
        .lines()
        .map(|page_list| {
            Ok(page_list
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((parsed_rules, parsed_pages))
}

fn part_two<O, I>(rules: &HashSet<(u8, u8)>, pages: O) -> usize
where
    O: IntoIterator<Item = I>,
    I: AsMut<[u8]>,
{
    let cmp = |a: u8, b: u8| {
        if rules.contains(&(a, b)) {
            Ordering::Less
        } else if rules.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    pages
        .into_iter()
        .filter_map(|mut page| {
            let page = page.as_mut();
            (!page.is_sorted_by(|a, b| Ordering::Greater != cmp(*a, *b))).then(|| {
                page.sort_by(|a, b| cmp(*a, *b));
                page[page.len() / 2] as usize
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        let expected = 143;

        let input = parse_input(INPUT).unwrap();

        let actual = part_one(&input.0, &input.1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 123;

        let mut input = parse_input_two(INPUT).unwrap();

        let actual = part_two(&input.0, &mut input.1);
        assert_eq!(expected, actual);
    }
}
