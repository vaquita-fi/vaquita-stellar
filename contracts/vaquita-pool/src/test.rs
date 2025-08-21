#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Logs}, Env, Vec};

#[test]
fn test() {
    let env = Env::from_ledger_snapshot_file("snapshot.json");
    let admin = Address::from_str(&env, "GDFF477UQUWOFIHSOEXPPOD5GPUSEW6OLGZB5FCPS6AFRAOPTMINA55X");
    let token = Address::from_str(&env, "CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU");
    let pool_address = Address::from_str(&env, "CDDG7DLOWSHRYQ2HWGZEZ4UTR7LPTKFFHN3QUCSZEXOWOPARMONX6T65");
    let lock_periods = Vec::from_array(&env, [604800]);
    let contract_id = env.register(VaquitaPool, ());
    let client = VaquitaPoolClient::new(&env, &contract_id);
    client.initialize(&admin, &token, &pool_address, &lock_periods);

    let caller = Address::from_str(&env, "GDFF477UQUWOFIHSOEXPPOD5GPUSEW6OLGZB5FCPS6AFRAOPTMINA55X");
    let deposit_id = String::from_str(&env, "a");
    let amount = 10000000;
    let period = 604800;

    client.mock_all_auths().deposit(&caller, &deposit_id, &amount, &period);
}
