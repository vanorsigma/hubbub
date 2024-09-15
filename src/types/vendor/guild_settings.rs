// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildSettings {
    #[prost(map = "fixed64, message", tag = "1")]
    pub channels: ::std::collections::HashMap<u64, guild_settings::ChannelSettings>,
    #[prost(uint32, tag = "2")]
    pub hub_progress: u32,
    #[prost(uint32, tag = "3")]
    pub guild_onboarding_progress: u32,
    #[prost(message, optional, tag = "4")]
    pub guild_recents_dismissed_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "5")]
    pub dismissed_guild_content: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub join_sound: ::core::option::Option<guild_settings::CustomCallSound>,
    #[prost(message, optional, tag = "7")]
    pub mobile_redesign_channel_list_settings: ::core::option::Option<
        guild_settings::ChannelListSettings,
    >,
    #[prost(bool, tag = "8")]
    pub disable_raid_alert_push: bool,
    #[prost(bool, tag = "9")]
    pub disable_raid_alert_nag: bool,
}
/// Nested message and enum types in `GuildSettings`.
pub mod guild_settings {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelIconEmoji {
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        #[prost(message, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "3")]
        pub color: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelSettings {
        #[prost(bool, tag = "1")]
        pub collapsed_in_inbox: bool,
        #[prost(message, optional, tag = "2")]
        pub icon_emoji: ::core::option::Option<ChannelIconEmoji>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomCallSound {
        #[prost(fixed64, tag = "1")]
        pub sound_id: u64,
        #[prost(fixed64, tag = "2")]
        pub guild_id: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelListSettings {
        #[prost(message, optional, tag = "1")]
        pub layout: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "2")]
        pub message_previews: ::core::option::Option<::prost::alloc::string::String>,
    }
}
