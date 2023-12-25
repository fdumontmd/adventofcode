use std::collections::HashMap;

pub mod custom_error;

pub mod part1;
pub mod part2;

fn solve_at(
    pat_idx: usize,
    bl_idx: usize,
    pattern: &[u8],
    block_lens: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // are we done?
    if bl_idx >= block_lens.len() {
        let count = if pat_idx >= pattern.len() || pattern[pat_idx..].iter().all(|b| *b != b'#') {
            1
        } else {
            0
        };
        memo.insert((pat_idx, bl_idx), count);
        return count;
    }

    if pat_idx >= pattern.len() || bl_idx >= block_lens.len() {
        memo.insert((pat_idx, bl_idx), 0);
        return 0;
    }

    // if there's a b'#' within pattern[pat_idx..pat_idx + block_lens[bl_idx]], must consume the
    // first block
    // simplify to: if first byte is b'#', must consume bloc
    let mut count = 0;
    if fit_block(&pattern[pat_idx..], block_lens[bl_idx]) {
        let next_block = (pat_idx + block_lens[bl_idx] + 1, bl_idx + 1);
        if !memo.contains_key(&next_block) {
            let next_block_count = solve_at(next_block.0, next_block.1, pattern, block_lens, memo);
            memo.insert(next_block, next_block_count);
        }
        count += memo[&next_block];
    }

    if pattern[pat_idx] != b'#' {
        count += solve_at(pat_idx + 1, bl_idx, pattern, block_lens, memo);
    }

    memo.insert((pat_idx, bl_idx), count);
    count
}

fn fit_block(pattern: &[u8], block_len: usize) -> bool {
    if pattern.len() >= block_len {
        for c in pattern[0..block_len].iter() {
            if *c == b'.' {
                return false;
            }
        }

        if pattern.len() > block_len && pattern[block_len] == b'#' {
            return false;
        }

        return true;
    }

    false
}
