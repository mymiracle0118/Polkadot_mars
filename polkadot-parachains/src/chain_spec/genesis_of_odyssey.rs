use super::*;
use odyssey_runtime;
use odyssey_runtime::{AccountId, AuraId, Signature};
use odyssey_runtime::constants;
use odyssey_runtime::Balance as OdysseyBalance;

const ODYSSEY_ED: OdysseyBalance =
    odyssey_runtime::constants::currency::EXISTENTIAL_DEPOSIT;



pub fn odyssey_session_keys(keys: AuraId) -> odyssey_runtime::opaque::SessionKeys {
    odyssey_runtime::opaque::SessionKeys { aura: keys }
}

pub(crate) fn odyssey_genesis(
    candidates: Vec<(AccountId, NimbusId, OdysseyBalance)>,
    nominations: Vec<(AccountId, AccountId, OdysseyBalance)>,
    root_key: AccountId,
    invulnerables: Vec<(AccountId, AuraId)>,
    // initial_authorities: Vec<AuraId>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> odyssey_runtime::GenesisConfig {
    const TOTAL_ISSUANCE: OdysseyBalance = constants::currency::AMAS_UNITS * 1_000_000_000; // one billion
    let endowment: OdysseyBalance = TOTAL_ISSUANCE / endowed_accounts.len() as u128;

    odyssey_runtime::GenesisConfig {
        system: odyssey_runtime::SystemConfig {
            code: odyssey_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: odyssey_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, endowment))
                .collect(),
        },
        sudo: odyssey_runtime::SudoConfig { key: root_key.clone() },
        parachain_info: odyssey_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: odyssey_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: ODYSSEY_ED,
            ..Default::default()
        },
        session: odyssey_runtime::SessionConfig {
            keys: invulnerables
                .iter()
                .cloned()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                  // account id
                        acc.clone(),                  // validator id
                        odyssey_session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        democracy: odyssey_runtime::DemocracyConfig::default(),
        council: odyssey_runtime::CouncilConfig::default(),
        technical_committee: odyssey_runtime::TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((endowed_accounts.len() + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
        treasury: Default::default(),
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        vesting: odyssey_runtime::VestingConfig { vesting: vec![] },
        // parachain_staking: ParachainStakingConfig {
        // 	candidates: candidates
        // 		.iter()
        // 		.cloned()
        // 		.map(|(account, _, bond)| (account, bond))
        // 		.collect(),
        // 	nominations,
        // 	inflation_config: ares_inflation_config(),
        // },
        ocw_module: odyssey_runtime::OCWModuleConfig {
            _phantom: Default::default(),
            request_base: Vec::new(),
            price_pool_depth: 3u32,
            price_allowable_offset: 10u8,
            price_requests: vec![
                // price_key, request_uri, parse_version, fraction_num, request interval
                ("btc-usdt".as_bytes().to_vec(), "btc".as_bytes().to_vec(), 2u32, 4, 2),
                ("eth-usdt".as_bytes().to_vec(), "eth".as_bytes().to_vec(), 2u32, 4, 2),
                ("dot-usdt".as_bytes().to_vec(), "dot".as_bytes().to_vec(), 2u32, 4, 2),
                ("link-usdt".as_bytes().to_vec(), "link".as_bytes().to_vec(), 2u32, 4, 2),

                ("ada-usdt".as_bytes().to_vec(), "ada".as_bytes().to_vec(), 2u32, 4, 4),
                ("xrp-usdt".as_bytes().to_vec(), "xrp".as_bytes().to_vec(), 2u32, 4, 4),
                ("sol-usdt".as_bytes().to_vec(), "sol".as_bytes().to_vec(), 2u32, 4, 4),
                ("uni-usdt".as_bytes().to_vec(), "uni".as_bytes().to_vec(), 2u32, 4, 4),
                ("bnb-usdt".as_bytes().to_vec(), "bnb".as_bytes().to_vec(), 2u32, 4, 4),
                ("1inch-usdt".as_bytes().to_vec(), "1inch".as_bytes().to_vec(), 2u32, 4, 4),
                ("atom-usdt".as_bytes().to_vec(), "atom".as_bytes().to_vec(), 2u32, 4, 4),
                ("trx-usdt".as_bytes().to_vec(), "trx".as_bytes().to_vec(), 2u32, 4, 4),
                ("aave-usdt".as_bytes().to_vec(), "aave".as_bytes().to_vec(), 2u32, 4, 4),
                ("snx-usdt".as_bytes().to_vec(), "snx".as_bytes().to_vec(), 2u32, 4, 4),

                ("avax-usdt".as_bytes().to_vec(), "avax".as_bytes().to_vec(), 2u32, 4, 5),
                ("ltc-usdt".as_bytes().to_vec(), "ltc".as_bytes().to_vec(), 2u32, 4, 5),
                ("bch-usdt".as_bytes().to_vec(), "bch".as_bytes().to_vec(), 2u32, 4, 5),
                ("fil-usdt".as_bytes().to_vec(), "fil".as_bytes().to_vec(), 2u32, 4, 5),
                ("etc-usdt".as_bytes().to_vec(), "etc".as_bytes().to_vec(), 2u32, 4, 5),
                ("eos-usdt".as_bytes().to_vec(), "eos".as_bytes().to_vec(), 2u32, 4, 5),
                ("dash-usdt".as_bytes().to_vec(), "dash".as_bytes().to_vec(), 2u32, 4, 5),
                ("comp-usdt".as_bytes().to_vec(), "comp".as_bytes().to_vec(), 2u32, 4, 5),
                ("matic-usdt".as_bytes().to_vec(), "matic".as_bytes().to_vec(), 2u32, 4, 5),

                ("doge-usdt".as_bytes().to_vec(), "doge".as_bytes().to_vec(), 2u32, 4, 8),
                ("luna-usdt".as_bytes().to_vec(), "luna".as_bytes().to_vec(), 2u32, 4, 8),
                ("ftt-usdt".as_bytes().to_vec(), "ftt".as_bytes().to_vec(), 2u32, 4, 8),
                ("xlm-usdt".as_bytes().to_vec(), "xlm".as_bytes().to_vec(), 2u32, 4, 8),
                ("vet-usdt".as_bytes().to_vec(), "vet".as_bytes().to_vec(), 2u32, 4, 8),
                ("icp-usdt".as_bytes().to_vec(), "icp".as_bytes().to_vec(), 2u32, 4, 8),
                ("theta-usdt".as_bytes().to_vec(), "theta".as_bytes().to_vec(), 2u32, 4, 8),
                ("algo-usdt".as_bytes().to_vec(), "algo".as_bytes().to_vec(), 2u32, 4, 8),
                ("xmr-usdt".as_bytes().to_vec(), "xmr".as_bytes().to_vec(), 2u32, 4, 8),
                ("xtz-usdt".as_bytes().to_vec(), "xtz".as_bytes().to_vec(), 2u32, 4, 8),
                ("egld-usdt".as_bytes().to_vec(), "egld".as_bytes().to_vec(), 2u32, 4, 8),
                ("axs-usdt".as_bytes().to_vec(), "axs".as_bytes().to_vec(), 2u32, 4, 8),
                ("iota-usdt".as_bytes().to_vec(), "iota".as_bytes().to_vec(), 2u32, 4, 8),
                ("ftm-usdt".as_bytes().to_vec(), "ftm".as_bytes().to_vec(), 2u32, 4, 8),
                ("ksm-usdt".as_bytes().to_vec(), "ksm".as_bytes().to_vec(), 2u32, 4, 4),
                ("hbar-usdt".as_bytes().to_vec(), "hbar".as_bytes().to_vec(), 2u32, 4, 8),
                ("neo-usdt".as_bytes().to_vec(), "neo".as_bytes().to_vec(), 2u32, 4, 8),
                ("waves-usdt".as_bytes().to_vec(), "waves".as_bytes().to_vec(), 2u32, 4, 8),
                ("mkr-usdt".as_bytes().to_vec(), "mkr".as_bytes().to_vec(), 2u32, 4, 8),
                ("near-usdt".as_bytes().to_vec(), "near".as_bytes().to_vec(), 2u32, 4, 8),
                ("btt-usdt".as_bytes().to_vec(), "btt".as_bytes().to_vec(), 2u32, 4, 8),
                ("chz-usdt".as_bytes().to_vec(), "chz".as_bytes().to_vec(), 2u32, 4, 8),
                ("stx-usdt".as_bytes().to_vec(), "stx".as_bytes().to_vec(), 2u32, 4, 8),
                ("dcr-usdt".as_bytes().to_vec(), "dcr".as_bytes().to_vec(), 2u32, 4, 8),
                ("xem-usdt".as_bytes().to_vec(), "xem".as_bytes().to_vec(), 2u32, 4, 8),
                ("omg-usdt".as_bytes().to_vec(), "omg".as_bytes().to_vec(), 2u32, 4, 8),
                ("zec-usdt".as_bytes().to_vec(), "zec".as_bytes().to_vec(), 2u32, 4, 8),
                ("sushi-usdt".as_bytes().to_vec(), "sushi".as_bytes().to_vec(), 2u32, 4, 8),
                ("enj-usdt".as_bytes().to_vec(), "enj".as_bytes().to_vec(), 2u32, 4, 8),
                ("mana-usdt".as_bytes().to_vec(), "mana".as_bytes().to_vec(), 2u32, 4, 8),
                ("yfi-usdt".as_bytes().to_vec(), "yfi".as_bytes().to_vec(), 2u32, 4, 8),
                ("iost-usdt".as_bytes().to_vec(), "iost".as_bytes().to_vec(), 2u32, 4, 8),
                ("qtum-usdt".as_bytes().to_vec(), "qtum".as_bytes().to_vec(), 2u32, 4, 8),
                ("bat-usdt".as_bytes().to_vec(), "bat".as_bytes().to_vec(), 2u32, 4, 8),
                ("zil-usdt".as_bytes().to_vec(), "zil".as_bytes().to_vec(), 2u32, 4, 8),
                ("icx-usdt".as_bytes().to_vec(), "icx".as_bytes().to_vec(), 2u32, 4, 8),
                ("grt-usdt".as_bytes().to_vec(), "grt".as_bytes().to_vec(), 2u32, 4, 8),
                ("celo-usdt".as_bytes().to_vec(), "celo".as_bytes().to_vec(), 2u32, 4, 8),
                ("zen-usdt".as_bytes().to_vec(), "zen".as_bytes().to_vec(), 2u32, 4, 8),
                ("ren-usdt".as_bytes().to_vec(), "ren".as_bytes().to_vec(), 2u32, 4, 8),
                ("sc-usdt".as_bytes().to_vec(), "sc".as_bytes().to_vec(), 2u32, 4, 8),
                ("zrx-usdt".as_bytes().to_vec(), "zrx".as_bytes().to_vec(), 2u32, 4, 8),
                ("ont-usdt".as_bytes().to_vec(), "ont".as_bytes().to_vec(), 2u32, 4, 8),
                ("nano-usdt".as_bytes().to_vec(), "nano".as_bytes().to_vec(), 2u32, 4, 8),
                ("crv-usdt".as_bytes().to_vec(), "crv".as_bytes().to_vec(), 2u32, 4, 8),
                ("bnt-usdt".as_bytes().to_vec(), "bnt".as_bytes().to_vec(), 2u32, 4, 8),
                ("fet-usdt".as_bytes().to_vec(), "fet".as_bytes().to_vec(), 2u32, 4, 8),
                ("uma-usdt".as_bytes().to_vec(), "uma".as_bytes().to_vec(), 2u32, 4, 8),
                ("iotx-usdt".as_bytes().to_vec(), "iotx".as_bytes().to_vec(), 2u32, 4, 8),
                ("lrc-usdt".as_bytes().to_vec(), "lrc".as_bytes().to_vec(), 2u32, 4, 8),
                ("sand-usdt".as_bytes().to_vec(), "sand".as_bytes().to_vec(), 2u32, 4, 8),
                ("srm-usdt".as_bytes().to_vec(), "srm".as_bytes().to_vec(), 2u32, 4, 8),
                ("kava-usdt".as_bytes().to_vec(), "kava".as_bytes().to_vec(), 2u32, 4, 8),
                ("knc-usdt".as_bytes().to_vec(), "knc".as_bytes().to_vec(), 2u32, 4, 8),
            ],
        },
    }
}