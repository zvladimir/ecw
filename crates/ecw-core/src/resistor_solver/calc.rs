//! The search for the closest resistor, or pair of resistors, to a target value.

use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex, MutexGuard};

use uom::si::electrical_resistance::ohm;
use uom::si::f64::ElectricalResistance;

use crate::domain::eseries::{ESeries, nearest_in};
use crate::domain::parse::parse_field;
use crate::domain::quantity::Quantity;
use crate::domain::unit::Measured;
use uom::num_traits::Zero;

/// The closest finds for a target value. `eseries_*` use the whole E-series, `pool_*` only the
/// resistors the user listed (`None` if the pool is empty). `*_single` is one resistor; the
/// pairs are two resistors, `(a, b)`, in series (`a + b`) or in parallel (`a ‖ b`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Report {
    pub eseries_single: ElectricalResistance,

    pub eseries_series: Option<(ElectricalResistance, ElectricalResistance)>,

    pub eseries_parallel: Option<(ElectricalResistance, ElectricalResistance)>,

    pub pool_single: Option<ElectricalResistance>,

    pub pool_series: Option<(ElectricalResistance, ElectricalResistance)>,

    pub pool_parallel: Option<(ElectricalResistance, ElectricalResistance)>,
}

type Ohms = ElectricalResistance;

/// Finds the closest single resistor and the closest pairs for `target`. The answers are
/// cached: the same request comes back many times while the user edits other fields.
pub fn solve(target: Ohms, eseries: ESeries, pool: &[Ohms]) -> Report {
    let key = CacheKey {
        target_micro_ohm: to_micro_ohm(target),
        eseries: eseries as u8,
        pool_hash: fnv1a_bits(pool),
    };

    if let Some(hit) = cache().get(&key) {
        return hit;
    }

    let report = solve_uncached(target, eseries, pool);
    cache().put(key, report);
    report
}

const CACHE_CAPACITY: usize = 128;

static CACHE: LazyLock<Mutex<BoundedCache>> =
    LazyLock::new(|| Mutex::new(BoundedCache::new(CACHE_CAPACITY)));

fn cache() -> MutexGuard<'static, BoundedCache> {
    CACHE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    target_micro_ohm: u64,

    eseries: u8,

    pool_hash: u64,
}

struct BoundedCache {
    map: HashMap<CacheKey, Report>,
    order: VecDeque<CacheKey>,
    capacity: usize,
}

impl BoundedCache {
    fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&self, key: &CacheKey) -> Option<Report> {
        self.map.get(key).copied()
    }

    fn put(&mut self, key: CacheKey, value: Report) {
        if let Some(existing) = self.map.get_mut(&key) {
            *existing = value;
            return;
        }

        if self.map.len() >= self.capacity
            && let Some(oldest) = self.order.pop_front()
        {
            self.map.remove(&oldest);
        }

        self.map.insert(key, value);
        self.order.push_back(key);
    }
}

fn to_micro_ohm(r: ElectricalResistance) -> u64 {
    let micro = r.to_base() * 1e6;

    if !micro.is_finite() || micro < 0.0 {
        return 0;
    }
    micro.round() as u64
}

fn fnv1a_bits(values: &[Ohms]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = FNV_OFFSET;
    for v in values {
        for byte in v.value.to_bits().to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }
    hash
}

fn solve_uncached(target: Ohms, eseries: ESeries, pool: &[Ohms]) -> Report {
    // The standard values are a table of plain numbers in ohms; here they become resistances.
    let series: Vec<Ohms> = eseries
        .values()
        .iter()
        .map(|&v| Ohms::new::<ohm>(v))
        .collect();
    Report {
        eseries_single: nearest_in(&series, target),
        eseries_series: find_best_series(&series, target),
        eseries_parallel: find_best_parallel(&series, target),
        pool_single: (!pool.is_empty()).then(|| nearest_in(pool, target)),
        pool_series: find_best_series(pool, target),
        pool_parallel: find_best_parallel(pool, target),
    }
}

/// The pair `a + b` closest to `target`; any two values of `sorted` (also the same one twice).
fn find_best_series(sorted: &[Ohms], target: Ohms) -> Option<(Ohms, Ohms)> {
    let mut best_error = Ohms::new::<ohm>(f64::INFINITY);
    let mut result = None;

    for &a in sorted {
        let b = nearest_in(sorted, target - a);
        let error = (a + b - target).abs();
        if error < best_error {
            best_error = error;
            result = Some((a, b));
        }
    }
    result
}

/// The pair `a ‖ b` closest to `target`. The parallel value grows with `b`, so the best `b`
/// is one of the two neighbours of the ideal one (the largest value when `a` alone is below
/// the target).
fn find_best_parallel(sorted: &[Ohms], target: Ohms) -> Option<(Ohms, Ohms)> {
    let last = *sorted.last()?;
    let mut best_error = Ohms::new::<ohm>(f64::INFINITY);
    let mut result = None;

    for &a in sorted {
        let candidates: [Ohms; 2] = if a > target {
            let b_ideal: Ohms = 1.0 / (1.0 / target - 1.0 / a);
            let hi_idx = sorted
                .partition_point(|&v| v < b_ideal)
                .min(sorted.len() - 1);
            [sorted[hi_idx.saturating_sub(1)], sorted[hi_idx]]
        } else {
            [last, last]
        };
        for b in candidates {
            let error = (parallel(a, b) - target).abs();
            if error < best_error {
                best_error = error;
                result = Some((a, b));
            }
        }
    }
    result
}

/// The resistors of a pool text (`10k, 2k; 100`), ascending, without repeats.
pub fn parse_pool(text: &str) -> Vec<Ohms> {
    let mut values: Vec<Ohms> = text
        .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .filter_map(|token| parse_field(token, Quantity::Resistance).ok().flatten())
        .filter(|m| m.is_exact())
        .map(|m| m.nominal)
        .filter(|v| v.is_finite() && *v > 0.0)
        .map(Ohms::new::<ohm>)
        .collect();

    values.sort_by(|a, b| a.value.total_cmp(&b.value));
    values.dedup_by(|a, b| a.value.to_bits() == b.value.to_bits());
    values
}

/// `a ‖ b`.
pub(crate) fn parallel(a: Ohms, b: Ohms) -> Ohms {
    let sum = a + b;
    if sum == Ohms::zero() {
        Ohms::new::<ohm>(f64::NAN)
    } else {
        a * b / sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(x: f64) -> Ohms {
        Ohms::new::<ohm>(x)
    }

    #[test]
    fn small_pools_still_give_pairs() {
        let pool = [r(1e3), r(10e3)];
        assert_eq!(find_best_series(&pool, r(4.3e3)), Some((r(1e3), r(1e3))));
        assert_eq!(
            find_best_parallel(&pool, r(4.3e3)),
            Some((r(10e3), r(10e3)))
        );
        assert_eq!(
            find_best_series(&[r(4.7e3)], r(4.3e3)),
            Some((r(4.7e3), r(4.7e3)))
        );
        assert_eq!(find_best_series(&[], r(4.3e3)), None);
    }

    #[test]
    fn pairs_match_a_brute_force_search() {
        let series: Vec<Ohms> = ESeries::E24.values().iter().map(|&v| r(v)).collect();
        for target in [18.4, 137.0, 4321.0, 76_543.0].map(r) {
            let all: Vec<Ohms> = series
                .iter()
                .copied()
                .filter(|v| *v > target / 50.0 && *v < target * 50.0)
                .collect();
            let mut best_series = f64::INFINITY;
            let mut best_parallel = f64::INFINITY;
            for &a in &all {
                for &b in &all {
                    best_series = best_series.min((a + b - target).abs().get::<ohm>());
                    best_parallel = best_parallel.min((parallel(a, b) - target).abs().get::<ohm>());
                }
            }
            let (a, b) = find_best_series(&series, target).unwrap();
            assert!(
                (a + b - target).abs().get::<ohm>() <= best_series * (1.0 + 1e-9),
                "series {target:?}"
            );
            let (a, b) = find_best_parallel(&series, target).unwrap();
            assert!(
                (parallel(a, b) - target).abs().get::<ohm>() <= best_parallel * (1.0 + 1e-9),
                "parallel {target:?}"
            );
        }
    }
}
