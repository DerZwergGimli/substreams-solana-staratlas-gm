
pub const PROCESS_EXCHANGE_ACCOUNTS_15: [&str; 14] = [
    "OrderTaker",
    "OrderTakerDepositTokenAccount",
    "OrderTakerReceiveTokenAccount",
    "CurrencyMint",
    "AssetMint",
    "OrderInitializer",
    "InitializerDepositTokenAccount",
    "InitializerReceiveTokenAccount",
    "OrderVaultAccount",
    "OrderVaultAuthority",
    "OrderAccount",
    "SaVault",
    "RegisteredCurrency",
    "OpenOrdersCounter"
];


pub const PROCESS_EXCHANGE_ACCOUNTS_19: [&str; 14] = [
    "OrderInitializer",
    "MarketVarsAccount",
    "DepositMint",
    "ReceiveMint",
    "OrderVaultAccount",
    "OrderVaultAuthority",
    "InitializerDepositTokenAccount",
    "InitializerReceiveTokenAccount",
    "OrderAccount",
    "RegisteredCurrency",
    "OpenOrdersCounter",
    "SystemProgram",
    "Rent",
    "TokenProgram",
];

pub const PROCESS_EXCHANGE_ACCOUNTS_32: [&str; 19] = [
    "OrderTaker",
    "OrderTakerDepositTokenAccount",
    "OrderTakerReceiveTokenAccount",
    "CurrencyMint",
    "AssetMint",
    "OrderInitializer",
    "InitializerDepositTokenAccount",
    "InitializerReceiveTokenAccount",
    "OrderVaultAccount",
    "OrderVaultAuthority",
    "OrderAccount",
    "SaVault",
    "RegisteredCurrency",
    "OpenOrdersCounter",
    "TokenProgram",
    "AtlasStaking",
    "RegisteredStake",
    "StakingAccount",
    "FeeReduction",
];

pub const PROCESS_INITIALIZE_ACCOUNTS: [&str; 14] = [
    "orderInitializer",
    "marketVarsAccount",
    "depositMint",
    "receiveMint",
    "orderVaultAccount",
    "orderVaultAuthority",
    "initializerDepositTokenAccount",
    "initializerReceiveTokenAccount",
    "orderAccount",
    "registeredCurrency",
    "openOrdersCounter",
    "systemProgram",
    "rent",
    "tokenProgram",
];


pub const PROCESS_CANCEL_ACCOUNTS: [&str; 10] = [
    "signer",
    "orderInitializer",
    "marketVarsAccount",
    "depositMint",
    "initializerDepositTokenAccount",
    "orderVaultAccount",
    "orderVaultAuthority",
    "orderAccount",
    "openOrdersCounter",
    "tokenProgram"
];