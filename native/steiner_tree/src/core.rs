use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::error::Error;
use crate::state::State;
use crate::subsets::subsets;

pub(crate) enum Ret {
    Ok(usize, Vec<(usize, usize)>),
    Error(Error),
    /// Aborting the computation. state is mutated. The caller should save state for later invocations.
    Yielding,
}

pub(crate) fn compute(state: &mut State) -> Ret {
    let n = state.n;
    let m = state.edges.len();
    let k = state.terms.len();
    let terms = &state.terms;
    if k >= 20 {
        return Ret::Error(Error::TooLargeInput(n));
    }
    let mut count: i64 = n as i64;
    for _ in 0..k {
        count = count.saturating_mul(3);
    }
    if count >= 10_000_000 {
        return Ret::Error(Error::TooLargeInput(n));
    }
    let mut count: i64 = m as i64;
    for _ in 0..k {
        count = count.saturating_mul(2);
    }
    if count >= 10_000_000 {
        return Ret::Error(Error::TooLargeInput(n));
    }
    // Validation of arguments
    for &(x, y) in &state.edges {
        if x >= n || y >= n {
            return Ret::Error(Error::InvalidArg(
                state.n,
                state.edges.clone(),
                state.terms.clone(),
            ));
        }
    }
    for &t in &state.terms {
        if t >= n {
            return Ret::Error(Error::InvalidArg(
                state.n,
                state.edges.clone(),
                state.terms.clone(),
            ));
        }
    }
    // TODO: better phase handling (e.g. using enums)
    match state.phase {
        // Initialize: allocate memory
        0 => {
            state.dp = vec![vec![std::usize::MAX / 2; 1 << k]; n];
            state.phase = 1;
            return Ret::Yielding;
        }
        // Compute: perform O(3^k n + 2^k m log n)
        1 => {
            let dp = &mut state.dp;
            let mut g = vec![vec![]; n];
            for &(x, y) in &state.edges {
                g[x].push(y);
                g[y].push(x);
            }
            for i in 0..k {
                dp[terms[i]][1 << i] = 0;
            }
            // TODO: separate into chunks
            for s in 1..1 << k {
                // collect: dp[i][s] <- dp[i][t] + dp[i][s \ t]
                for i in 0..n {
                    for t in subsets(s) {
                        if t == 0 || t == s {
                            continue;
                        }
                        dp[i][s] = std::cmp::min(dp[i][s], dp[i][t] + dp[i][s - t]);
                    }
                }
                // Dijkstra: propagate info collected in the previous step to other nodes
                let mut que = BinaryHeap::new();
                for i in 0..n {
                    que.push((Reverse(dp[i][s]), i));
                }
                // TODO: constant-factor optimization
                while let Some((Reverse(dist), v)) = que.pop() {
                    for &w in &g[v] {
                        let new_dist = dist + 1;
                        if dp[w][s] <= new_dist {
                            continue;
                        }
                        dp[w][s] = new_dist;
                        que.push((Reverse(new_dist), w));
                    }
                }
            }
            let mut mi = std::usize::MAX;
            for i in 0..n {
                mi = std::cmp::min(mi, dp[i][(1 << k) - 1]);
            }
            // TODO: path reconstruction
            return Ret::Ok(mi, vec![]);
        }
        _ => panic!(),
    }
}
