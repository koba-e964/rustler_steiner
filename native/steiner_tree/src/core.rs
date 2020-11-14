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

const INFINITY: usize = std::usize::MAX / 2;

pub(crate) fn compute(state: &mut State) -> Ret {
    let n = state.n;
    let k = state.terms.len();
    let terms = &state.terms;
    if k >= 30 {
        return Ret::Error(Error::TooLargeInput(n));
    }
    let mut count: i64 = n as i64;
    for _ in 0..k {
        count = count.saturating_mul(2);
    }
    if count >= 40_000_000 {
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
            state.dp = vec![vec![INFINITY; 1 << k]; n];
            state.pre = vec![vec![(3, 0); 1 << k]; n];
            state.phase = 1;
            return Ret::Yielding;
        }
        // Compute: perform O(3^k n + 2^k m log n)
        1 => {
            let dp = &mut state.dp;
            let pre = &mut state.pre;
            for i in 0..k {
                dp[terms[i]][1 << i] = 0;
                pre[terms[i]][1 << i] = (0, 0);
            }
            state.phase = 2;
            state.loop_index = 1;
            return Ret::Yielding;
        }
        2 => {
            let dp = &mut state.dp;
            let pre = &mut state.pre;
            let mut g = vec![vec![]; n];
            for &(x, y) in &state.edges {
                g[x].push((y, 1));
                g[y].push((x, 1));
            }
            // separated into chunks
            for s in state.loop_index..1 << k {
                // collect: dp[i][s] <- dp[i][t] + dp[i][s \ t]
                for i in 0..n {
                    for t in subsets(s) {
                        if t == 0 || t == s {
                            continue;
                        }
                        if dp[i][s] > dp[i][t] + dp[i][s - t] {
                            dp[i][s] = dp[i][t] + dp[i][s - t];
                            pre[i][s] = (1, t);
                        }
                    }
                }
                // Dijkstra: propagate info collected in the previous step to other nodes
                let mut que = BinaryHeap::new();
                for i in 0..n {
                    que.push((Reverse(dp[i][s]), i));
                }
                // TODO: constant-factor optimization
                while let Some((Reverse(dist), v)) = que.pop() {
                    for &(w, c) in &g[v] {
                        let new_dist = dist + c;
                        if dp[w][s] <= new_dist {
                            continue;
                        }
                        dp[w][s] = new_dist;
                        pre[w][s] = (2, v);
                        que.push((Reverse(new_dist), w));
                    }
                }
                state.loop_index += 1;
                return Ret::Yielding;
            }
            let mut mi = (INFINITY, 0);
            for i in 0..n {
                mi = std::cmp::min(mi, (dp[i][(1 << k) - 1], i));
            }
            if mi.0 >= INFINITY {
                // Terminals are not connected; return an error.
                return Ret::Error(Error::TerminalNotConnected);
            }
            // solution reconstruction
            let mut cur = vec![(mi.1, (1 << k) - 1)];
            let mut used_edges = vec![];
            while let Some((cur_v, cur_set)) = cur.pop() {
                let (kind, index) = pre[cur_v][cur_set];
                assert!(kind <= 2);
                match kind {
                    0 => {}
                    1 => {
                        cur.push((cur_v, index));
                        cur.push((cur_v, cur_set - index));
                    }
                    2 => {
                        used_edges.push((index, cur_v));
                        cur.push((index, cur_set));
                    }
                    _ => unreachable!(),
                }
            }
            return Ret::Ok(mi.0, used_edges);
        }
        _ => panic!(),
    }
}
