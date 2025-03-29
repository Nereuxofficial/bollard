// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerRecord {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "3")]
    pub platforms: ::prost::alloc::vec::Vec<super::super::super::super::pb::Platform>,
    #[prost(message, repeated, tag = "4")]
    pub gc_policy: ::prost::alloc::vec::Vec<GcPolicy>,
    #[prost(message, optional, tag = "5")]
    pub buildkit_version: ::core::option::Option<BuildkitVersion>,
    #[prost(message, repeated, tag = "6")]
    pub cdi_devices: ::prost::alloc::vec::Vec<CdiDevice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcPolicy {
    #[prost(bool, tag = "1")]
    pub all: bool,
    #[prost(int64, tag = "2")]
    pub keep_duration: i64,
    #[prost(string, repeated, tag = "4")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// reservedSpace was renamed from freeBytes
    #[prost(int64, tag = "3")]
    pub reserved_space: i64,
    #[prost(int64, tag = "5")]
    pub max_used_space: i64,
    #[prost(int64, tag = "6")]
    pub min_free_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildkitVersion {
    #[prost(string, tag = "1")]
    pub package: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub revision: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdiDevice {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub auto_allow: bool,
    #[prost(map = "string, string", tag = "3")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(bool, tag = "4")]
    pub on_demand: bool,
}
