const INPUT: &str = include_str!("input.txt");

fn is_report_safe(report: &[isize]) -> bool {
    let analysis: Vec<isize> = report.windows(2).map(|w| w[0] - w[1]).collect();

    analysis.iter().all(|d| d.abs() >= 1 && d.abs() <= 3)
        && (analysis.iter().all(|d| d.is_positive()) || analysis.iter().all(|d| d.is_negative()))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|r| is_report_safe(r))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|r| is_report_almost_safe(r))
        .count()
}

fn is_report_safe_with_safety(report: &[isize]) -> bool {
    for p in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(p);
        if is_report_safe(&report) {
            return true;
        }
    }
    false
}

fn is_report_almost_safe(report: &[isize]) -> bool {
    is_report_safe(report) || {
        let analysis: Vec<isize> = report.windows(2).map(|w| w[0] - w[1]).collect();

        // do we have just one problem?
        if analysis
            .iter()
            .filter(|d| d.abs() < 1 || d.abs() > 3)
            .count()
            > 1
        {
            false
        } else {
            let positive_count = analysis.iter().filter(|d| d.is_positive()).count();
            let negative_count = analysis.len() - positive_count;

            if positive_count <= 2 || negative_count <= 2 {
                is_report_safe_with_safety(report)
            } else {
                false
            }
        }
    }
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test_case(TEST_INPUT, 2; "test input")]
    #[test_case(INPUT, 421; "input")]
    fn test_part_1(input: &str, safe_reports: usize) {
        assert_eq!(safe_reports, part1(input));
    }

    #[test_case(TEST_INPUT, 4; "test input")]
    #[test_case(INPUT, 476; "input")]
    fn test_part_2(input: &str, safe_reports: usize) {
        assert_eq!(safe_reports, part2(input));
    }
}
