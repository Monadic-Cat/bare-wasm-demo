use ::core::sync::atomic::{AtomicU64, Ordering};

// Use a WASM specialized allocator in the hopes of reducing code size.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
extern "C" fn count() -> u64 {
    // Daily reminder that I don't know how to choose atomic orderings.
    COUNTER.fetch_add(1, Ordering::SeqCst);
    COUNTER.fetch_add(1, Ordering::SeqCst);
    COUNTER.fetch_add(1, Ordering::SeqCst);
    COUNTER.load(Ordering::SeqCst)
}

/// A prime sieve to demonstrate doing allocation on WASM,
/// and hopefully strip out a bunch of cruft.
// This Clippy lint is wrong in our situation, as Vec<T> cannot
// cross the limited WASM ABI.
#[allow(clippy::box_vec)]
#[no_mangle]
pub extern "C" fn primes(bound: u64) -> Box<Vec<u64>> {
    let mut primes = vec![3, 7];
    'outer: for n in *primes.last().unwrap() .. bound {
        for check in primes.iter().filter(|check| check.pow(2) < n) {
            if n.rem_euclid(*check) == 0 {
                continue 'outer;
            }
        }
        primes.push(n);
    }

    Box::new(primes)
}
