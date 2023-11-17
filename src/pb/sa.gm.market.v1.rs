// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GalacticMarketplaceInstructions {
    #[prost(message, repeated, tag="1")]
    pub galactic_marketplace_instructions: ::prost::alloc::vec::Vec<GalacticMarketplaceInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GalacticMarketplaceInstruction {
    #[prost(enumeration="galactic_marketplace_instruction::Instruction", tag="1")]
    pub instruction: i32,
    #[prost(message, optional, tag="2")]
    pub meta_data: ::core::option::Option<galactic_marketplace_instruction::MetaData>,
    #[prost(message, repeated, tag="3")]
    pub accounts: ::prost::alloc::vec::Vec<galactic_marketplace_instruction::Account>,
    #[prost(message, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<galactic_marketplace_instruction::Arg>,
    #[prost(message, repeated, tag="5")]
    pub parsed: ::prost::alloc::vec::Vec<galactic_marketplace_instruction::Arg>,
    #[prost(message, repeated, tag="6")]
    pub inner_instructions: ::prost::alloc::vec::Vec<super::super::super::super::sol::token::program::v1::TokenProgram>,
}
/// Nested message and enum types in `GalacticMarketplaceInstruction`.
pub mod galactic_marketplace_instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Account {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub address: ::prost::alloc::string::String,
        #[prost(bool, optional, tag="3")]
        pub is_mut: ::core::option::Option<bool>,
        #[prost(bool, optional, tag="4")]
        pub is_signer: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arg {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaData {
        #[prost(string, tag="1")]
        pub signature: ::prost::alloc::string::String,
        #[prost(int64, tag="2")]
        pub timestamp: i64,
        #[prost(uint64, tag="3")]
        pub block: u64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Instruction {
        CreateAccountWithSeed = 0,
        InitMarketplace = 1,
        ProcessCancel = 2,
        ProcessExchange = 3,
        ProcessInitializeBuy = 4,
        ProcessInitializeSell = 5,
        DeregisterCurrency = 6,
        UpdateCurrencyRoyalty = 7,
        InitOpenOrdersCounter = 8,
        RegisterCurrency = 9,
        UpdateAtlasRate = 10,
        InitializeMarketplace = 11,
        InitializeOpenOrdersCounter = 12,
        UpdateCurrencyVault = 13,
        UnknownTransaction = 14,
    }
    impl Instruction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Instruction::CreateAccountWithSeed => "CREATE_ACCOUNT_WITH_SEED",
                Instruction::InitMarketplace => "INIT_MARKETPLACE",
                Instruction::ProcessCancel => "PROCESS_CANCEL",
                Instruction::ProcessExchange => "PROCESS_EXCHANGE",
                Instruction::ProcessInitializeBuy => "PROCESS_INITIALIZE_BUY",
                Instruction::ProcessInitializeSell => "PROCESS_INITIALIZE_SELL",
                Instruction::DeregisterCurrency => "DEREGISTER_CURRENCY",
                Instruction::UpdateCurrencyRoyalty => "UPDATE_CURRENCY_ROYALTY",
                Instruction::InitOpenOrdersCounter => "INIT_OPEN_ORDERS_COUNTER",
                Instruction::RegisterCurrency => "REGISTER_CURRENCY",
                Instruction::UpdateAtlasRate => "UPDATE_ATLAS_RATE",
                Instruction::InitializeMarketplace => "INITIALIZE_MARKETPLACE",
                Instruction::InitializeOpenOrdersCounter => "INITIALIZE_OPEN_ORDERS_COUNTER",
                Instruction::UpdateCurrencyVault => "UPDATE_CURRENCY_VAULT",
                Instruction::UnknownTransaction => "UNKNOWN_TRANSACTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATE_ACCOUNT_WITH_SEED" => Some(Self::CreateAccountWithSeed),
                "INIT_MARKETPLACE" => Some(Self::InitMarketplace),
                "PROCESS_CANCEL" => Some(Self::ProcessCancel),
                "PROCESS_EXCHANGE" => Some(Self::ProcessExchange),
                "PROCESS_INITIALIZE_BUY" => Some(Self::ProcessInitializeBuy),
                "PROCESS_INITIALIZE_SELL" => Some(Self::ProcessInitializeSell),
                "DEREGISTER_CURRENCY" => Some(Self::DeregisterCurrency),
                "UPDATE_CURRENCY_ROYALTY" => Some(Self::UpdateCurrencyRoyalty),
                "INIT_OPEN_ORDERS_COUNTER" => Some(Self::InitOpenOrdersCounter),
                "REGISTER_CURRENCY" => Some(Self::RegisterCurrency),
                "UPDATE_ATLAS_RATE" => Some(Self::UpdateAtlasRate),
                "INITIALIZE_MARKETPLACE" => Some(Self::InitializeMarketplace),
                "INITIALIZE_OPEN_ORDERS_COUNTER" => Some(Self::InitializeOpenOrdersCounter),
                "UPDATE_CURRENCY_VAULT" => Some(Self::UpdateCurrencyVault),
                "UNKNOWN_TRANSACTION" => Some(Self::UnknownTransaction),
                _ => None,
            }
        }
    }
}
// @@protoc_insertion_point(module)
