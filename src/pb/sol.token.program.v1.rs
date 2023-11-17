// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProgram {
    #[prost(oneof="token_program::Program", tags="1")]
    pub program: ::core::option::Option<token_program::Program>,
}
/// Nested message and enum types in `TokenProgram`.
pub mod token_program {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Program {
        #[prost(message, tag="1")]
        TokenTransferChecked(super::TokenTransferChecked),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenTransferChecked {
    #[prost(string, tag="4")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub destination: ::prost::alloc::string::String,
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub token_amount: ::core::option::Option<token_transfer_checked::TokenAmount>,
}
/// Nested message and enum types in `TokenTransferChecked`.
pub mod token_transfer_checked {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TokenAmount {
        #[prost(uint64, tag="1")]
        pub amount: u64,
        #[prost(uint32, tag="2")]
        pub decimals: u32,
        #[prost(float, tag="3")]
        pub ui_amount: f32,
        #[prost(string, tag="4")]
        pub ui_amount_string: ::prost::alloc::string::String,
    }
}
// @@protoc_insertion_point(module)
