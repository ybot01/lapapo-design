use core::array::from_fn;
use ed25519_dalek::SecretKey;
use ethnum::U256;
use rand::random;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

fn get_random_node_id() -> SecretKey {random()}

fn trailing_zeroes<const N: usize>(id_1: [u8;N], id_2: [u8;N]) -> usize {
    let mut total = 0;
    let mut result;
    for i in (0..N).rev(){
        result = (id_1[i]^id_2[i]).trailing_zeros();
        total += result;
        if result != 8 {break}
    }
    usize::try_from(total).unwrap()
}

static MIDPOINT_U256: LazyLock<U256> = LazyLock::new(|| (U256::MAX / 2) + 1);

fn log2_abs_diff(id_1: SecretKey, id_2: SecretKey) -> usize {
    let id_1_u256 = U256::from_be_bytes(id_1);
    let id_2_u256 = U256::from_be_bytes(id_2);
    let mut abs_diff = if id_1_u256 >= id_2_u256 {id_1_u256 - id_2_u256} else {id_2_u256 - id_1_u256};
    if abs_diff > *MIDPOINT_U256 {abs_diff = (U256::MAX - abs_diff) + U256::new(1)}
    //this gives floor(log2)
    let mut log2 = 0;
    while abs_diff > 1{
        abs_diff /= 2;
        log2 += 1;
    }
    log2
}

fn main() {
    const NETWORK_SIZE: usize = 10_000;
    const RUNS: usize = 1000;

    let mut hops_counters = HashMap::<u64, u64>::new();

    for _ in 0..RUNS{
        let mut other_node_ids = HashSet::new();
        //-2 as separately generate start node id and target node id
        for _ in 0..(NETWORK_SIZE-2) {_ = other_node_ids.insert(get_random_node_id())}

        let target_node_id = get_random_node_id();
        other_node_ids.insert(target_node_id);

        let mut current_node_id = get_random_node_id(); //self node id
        other_node_ids.insert(current_node_id);

        let mut hop_counter = 0;
        while current_node_id != target_node_id{
            hop_counter += 1;
            //find which other nodes should store
            let mut trailing_zeroes_pows: [HashSet<SecretKey>; 256] = from_fn(|_| HashSet::new());
            let mut log2_abs_diff_pows: [HashSet<SecretKey>; 256] = from_fn(|_| HashSet::new());
            for &node_id in other_node_ids.iter().filter(|&&x| x != current_node_id) {
                trailing_zeroes_pows[trailing_zeroes(current_node_id, node_id)].insert(node_id);
                log2_abs_diff_pows[255 - log2_abs_diff(current_node_id, node_id)].insert(node_id);
            }

            //calculate N by counting down until accumulated total is large enough
            let mut trailing_zeroes_node_count = 0;
            let mut log2_abs_diff_node_count = 0;
            for i in 64..256{
                trailing_zeroes_node_count += trailing_zeroes_pows[i].len() as u64;
                log2_abs_diff_node_count += log2_abs_diff_pows[i].len() as u64;
            }
            for i in (0..64).rev(){
                trailing_zeroes_node_count += trailing_zeroes_pows[i].len() as u64;
                log2_abs_diff_node_count += log2_abs_diff_pows[i].len() as u64;
                if (trailing_zeroes_node_count >= 2_u64.pow(i as u32)) && (log2_abs_diff_node_count >= 2_u64.pow(i as u32)) {
                    let mut stored_node_ids = HashSet::<SecretKey>::new();
                    for i in i..256{
                        stored_node_ids.extend(trailing_zeroes_pows[i].iter());
                        stored_node_ids.extend(log2_abs_diff_pows[i].iter());
                    }
                    //hop to closest one
                    current_node_id = stored_node_ids.into_iter().min_by_key(|x| log2_abs_diff(*x, target_node_id)).unwrap();
                    break;
                }
            }
        }

        hops_counters.entry(hop_counter).and_modify(|x| *x += 1).or_insert(1);
    }

    println!("Hop counters: {:#?}", hops_counters);
}
