// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Erc20Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
    #[prost(string, tag="5")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub whitelist_pools: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Liquidity {
    #[prost(string, tag="1")]
    pub pool_address: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="4")]
    pub created_at_block_number: u64,
    #[prost(message, optional, tag="5")]
    pub token0: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag="6")]
    pub token1: ::core::option::Option<Erc20Token>,
    /// Integer
    #[prost(string, tag="7")]
    pub fee_tier: ::prost::alloc::string::String,
    /// internals
    #[prost(int32, tag="30")]
    pub tick_spacing: i32,
    #[prost(uint64, tag="31")]
    pub log_ordinal: u64,
    #[prost(string, tag="32")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(bool, tag="33")]
    pub ignore_pool: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub pool_sqrt_prices: ::prost::alloc::vec::Vec<events::PoolSqrtPrice>,
    #[prost(message, repeated, tag="2")]
    pub pool_liquidities: ::prost::alloc::vec::Vec<events::PoolLiquidity>,
    #[prost(message, repeated, tag="7")]
    pub fee_growth_global_updates: ::prost::alloc::vec::Vec<events::FeeGrowthGlobal>,
    #[prost(message, repeated, tag="10")]
    pub fee_growth_inside_updates: ::prost::alloc::vec::Vec<events::FeeGrowthInside>,
    #[prost(message, repeated, tag="11")]
    pub fee_growth_outside_updates: ::prost::alloc::vec::Vec<events::FeeGrowthOutside>,
    #[prost(message, repeated, tag="3")]
    pub pool_events: ::prost::alloc::vec::Vec<events::PoolEvent>,
    #[prost(message, repeated, tag="4")]
    pub transactions: ::prost::alloc::vec::Vec<events::Transaction>,
    #[prost(message, repeated, tag="5")]
    pub positions: ::prost::alloc::vec::Vec<events::Position>,
    #[prost(message, repeated, tag="6")]
    pub flashes: ::prost::alloc::vec::Vec<events::Flash>,
    #[prost(message, repeated, tag="8")]
    pub ticks_created: ::prost::alloc::vec::Vec<events::TickCreated>,
    #[prost(message, repeated, tag="9")]
    pub ticks_updated: ::prost::alloc::vec::Vec<events::TickUpdated>,
}
/// Nested message and enum types in `Events`.
pub mod events {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeeGrowthGlobal {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub ordinal: u64,
        #[prost(int32, tag="3")]
        pub token_idx: i32,
        /// Integer
        #[prost(string, tag="4")]
        pub new_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeeGrowthInside {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(int32, tag="2")]
        pub tick_idx: i32,
        #[prost(uint64, tag="3")]
        pub ordinal: u64,
        /// Integer
        #[prost(string, tag="4")]
        pub new_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeeGrowthOutside {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(int32, tag="2")]
        pub tick_lower: i32,
        #[prost(int32, tag="3")]
        pub tick_upper: i32,
        #[prost(uint64, tag="4")]
        pub ordinal: u64,
        /// Integer
        #[prost(string, tag="5")]
        pub new_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TickCreated {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="2")]
        pub idx: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub log_ordinal: u64,
        #[prost(uint64, tag="4")]
        pub created_at_timestamp: u64,
        #[prost(uint64, tag="5")]
        pub created_at_block_number: u64,
        /// Decimal
        #[prost(string, tag="6")]
        pub price0: ::prost::alloc::string::String,
        /// Decimal
        #[prost(string, tag="7")]
        pub price1: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="8")]
        pub amount: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TickUpdated {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="2")]
        pub idx: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub log_ordinal: u64,
        /// Integer
        #[prost(string, tag="4")]
        pub fee_growth_outside_0x_128: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="5")]
        pub fee_growth_outside_1x_128: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PoolSqrtPrice {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub ordinal: u64,
        /// Integer
        #[prost(string, tag="3")]
        pub sqrt_price: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="4")]
        pub tick: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PoolEvent {
        #[prost(uint64, tag="100")]
        pub log_ordinal: u64,
        #[prost(uint64, tag="101")]
        pub log_index: u64,
        #[prost(string, tag="102")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(string, tag="103")]
        pub token0: ::prost::alloc::string::String,
        #[prost(string, tag="104")]
        pub token1: ::prost::alloc::string::String,
        #[prost(string, tag="105")]
        pub fee: ::prost::alloc::string::String,
        #[prost(string, tag="106")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(uint64, tag="107")]
        pub timestamp: u64,
        #[prost(uint64, tag="108")]
        pub created_at_block_number: u64,
        #[prost(oneof="pool_event::Type", tags="1, 2, 3")]
        pub r#type: ::core::option::Option<pool_event::Type>,
    }
    /// Nested message and enum types in `PoolEvent`.
    pub mod pool_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Swap {
            #[prost(string, tag="1")]
            pub sender: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub recipient: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub origin: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="4")]
            pub amount_0: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="5")]
            pub amount_1: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="6")]
            pub sqrt_price: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="7")]
            pub liquidity: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="8")]
            pub tick: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Burn {
            #[prost(string, tag="1")]
            pub owner: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub origin: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="3")]
            pub amount: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="4")]
            pub amount_0: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="5")]
            pub amount_1: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="6")]
            pub tick_lower: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="7")]
            pub tick_upper: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Mint {
            #[prost(string, tag="1")]
            pub owner: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub sender: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub origin: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="4")]
            pub amount_0: ::prost::alloc::string::String,
            /// Decimal
            #[prost(string, tag="5")]
            pub amount_1: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="6")]
            pub tick_lower: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="7")]
            pub tick_upper: ::prost::alloc::string::String,
            /// Integer
            #[prost(string, tag="8")]
            pub amount: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            #[prost(message, tag="1")]
            Swap(Swap),
            #[prost(message, tag="2")]
            Burn(Burn),
            #[prost(message, tag="3")]
            Mint(Mint),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PoolLiquidity {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="2")]
        pub liquidity: ::prost::alloc::string::String,
        /// internals
        #[prost(uint64, tag="30")]
        pub log_ordinal: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Flash {
        #[prost(string, tag="1")]
        pub pool_address: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="2")]
        pub fee_growth_global_0x_128: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="3")]
        pub fee_growth_global_1x_128: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub log_ordinal: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transaction {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub block_number: u64,
        #[prost(uint64, tag="3")]
        pub timestamp: u64,
        #[prost(uint64, tag="4")]
        pub gas_used: u64,
        /// Integer
        #[prost(string, tag="5")]
        pub gas_price: ::prost::alloc::string::String,
        #[prost(uint64, tag="6")]
        pub log_ordinal: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(string, tag="1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub pool: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub token0: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub token1: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub tick_lower: ::prost::alloc::string::String,
        #[prost(string, tag="7")]
        pub tick_upper: ::prost::alloc::string::String,
        #[prost(string, tag="8")]
        pub transaction: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="9")]
        pub fee_growth_inside_0_last_x_128: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="10")]
        pub fee_growth_inside_1_last_x_128: ::prost::alloc::string::String,
        /// Integer
        #[prost(string, tag="11")]
        pub liquidity: ::prost::alloc::string::String,
        /// Decimal
        #[prost(string, tag="12")]
        pub amount0: ::prost::alloc::string::String,
        /// Decimal
        #[prost(string, tag="13")]
        pub amount1: ::prost::alloc::string::String,
        #[prost(enumeration="position::PositionType", tag="14")]
        pub position_type: i32,
        #[prost(uint64, tag="15")]
        pub log_ordinal: u64,
        #[prost(uint64, tag="16")]
        pub timestamp: u64,
        #[prost(uint64, tag="17")]
        pub block_number: u64,
    }
    /// Nested message and enum types in `Position`.
    pub mod position {
        /// internals
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum PositionType {
            Unset = 0,
            IncreaseLiquidity = 1,
            Collect = 2,
            DecreaseLiquidity = 3,
            Transfer = 4,
        }
        impl PositionType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PositionType::Unset => "UNSET",
                    PositionType::IncreaseLiquidity => "INCREASE_LIQUIDITY",
                    PositionType::Collect => "COLLECT",
                    PositionType::DecreaseLiquidity => "DECREASE_LIQUIDITY",
                    PositionType::Transfer => "TRANSFER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNSET" => Some(Self::Unset),
                    "INCREASE_LIQUIDITY" => Some(Self::IncreaseLiquidity),
                    "COLLECT" => Some(Self::Collect),
                    "DECREASE_LIQUIDITY" => Some(Self::DecreaseLiquidity),
                    "TRANSFER" => Some(Self::Transfer),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotPositions {
    #[prost(message, repeated, tag="1")]
    pub snapshot_positions: ::prost::alloc::vec::Vec<SnapshotPosition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotPosition {
    #[prost(string, tag="1")]
    pub pool: ::prost::alloc::string::String,
    /// the token_id of the position
    #[prost(string, tag="2")]
    pub position: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub timestamp: u64,
    /// Decimal
    #[prost(string, tag="7")]
    pub liquidity: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="8")]
    pub deposited_token0: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="9")]
    pub deposited_token1: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="10")]
    pub withdrawn_token0: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="11")]
    pub withdrawn_token1: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="12")]
    pub collected_fees_token0: ::prost::alloc::string::String,
    /// Decimal
    #[prost(string, tag="13")]
    pub collected_fees_token1: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub transaction: ::prost::alloc::string::String,
    /// Integer
    #[prost(string, tag="15")]
    pub fee_growth_inside_0_last_x_128: ::prost::alloc::string::String,
    /// Integer
    #[prost(string, tag="16")]
    pub fee_growth_inside_1_last_x_128: ::prost::alloc::string::String,
    /// internal
    #[prost(uint64, tag="17")]
    pub log_ordinal: u64,
}
// @@protoc_insertion_point(module)
