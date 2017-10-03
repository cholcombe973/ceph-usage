// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct PoolUsage {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    num_bytes: ::std::option::Option<u64>,
    num_kb: ::std::option::Option<u64>,
    num_objects: ::std::option::Option<u64>,
    num_object_clones: ::std::option::Option<u64>,
    num_object_copies: ::std::option::Option<u64>,
    num_objects_missing_on_primary: ::std::option::Option<u64>,
    num_objects_unfound: ::std::option::Option<u64>,
    num_objects_degraded: ::std::option::Option<u64>,
    num_rd: ::std::option::Option<u64>,
    num_rd_kb: ::std::option::Option<u64>,
    num_wr: ::std::option::Option<u64>,
    num_wr_kb: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoolUsage {}

impl PoolUsage {
    pub fn new() -> PoolUsage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoolUsage {
        static mut instance: ::protobuf::lazy::Lazy<PoolUsage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoolUsage,
        };
        unsafe {
            instance.get(PoolUsage::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required uint64 num_bytes = 2;

    pub fn clear_num_bytes(&mut self) {
        self.num_bytes = ::std::option::Option::None;
    }

    pub fn has_num_bytes(&self) -> bool {
        self.num_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_bytes(&mut self, v: u64) {
        self.num_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_num_bytes(&self) -> u64 {
        self.num_bytes.unwrap_or(0)
    }

    fn get_num_bytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_bytes
    }

    fn mut_num_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_bytes
    }

    // required uint64 num_kb = 3;

    pub fn clear_num_kb(&mut self) {
        self.num_kb = ::std::option::Option::None;
    }

    pub fn has_num_kb(&self) -> bool {
        self.num_kb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_kb(&mut self, v: u64) {
        self.num_kb = ::std::option::Option::Some(v);
    }

    pub fn get_num_kb(&self) -> u64 {
        self.num_kb.unwrap_or(0)
    }

    fn get_num_kb_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_kb
    }

    fn mut_num_kb_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_kb
    }

    // required uint64 num_objects = 4;

    pub fn clear_num_objects(&mut self) {
        self.num_objects = ::std::option::Option::None;
    }

    pub fn has_num_objects(&self) -> bool {
        self.num_objects.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_objects(&mut self, v: u64) {
        self.num_objects = ::std::option::Option::Some(v);
    }

    pub fn get_num_objects(&self) -> u64 {
        self.num_objects.unwrap_or(0)
    }

    fn get_num_objects_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_objects
    }

    fn mut_num_objects_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_objects
    }

    // required uint64 num_object_clones = 5;

    pub fn clear_num_object_clones(&mut self) {
        self.num_object_clones = ::std::option::Option::None;
    }

    pub fn has_num_object_clones(&self) -> bool {
        self.num_object_clones.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_object_clones(&mut self, v: u64) {
        self.num_object_clones = ::std::option::Option::Some(v);
    }

    pub fn get_num_object_clones(&self) -> u64 {
        self.num_object_clones.unwrap_or(0)
    }

    fn get_num_object_clones_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_object_clones
    }

    fn mut_num_object_clones_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_object_clones
    }

    // required uint64 num_object_copies = 6;

    pub fn clear_num_object_copies(&mut self) {
        self.num_object_copies = ::std::option::Option::None;
    }

    pub fn has_num_object_copies(&self) -> bool {
        self.num_object_copies.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_object_copies(&mut self, v: u64) {
        self.num_object_copies = ::std::option::Option::Some(v);
    }

    pub fn get_num_object_copies(&self) -> u64 {
        self.num_object_copies.unwrap_or(0)
    }

    fn get_num_object_copies_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_object_copies
    }

    fn mut_num_object_copies_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_object_copies
    }

    // required uint64 num_objects_missing_on_primary = 7;

    pub fn clear_num_objects_missing_on_primary(&mut self) {
        self.num_objects_missing_on_primary = ::std::option::Option::None;
    }

    pub fn has_num_objects_missing_on_primary(&self) -> bool {
        self.num_objects_missing_on_primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_objects_missing_on_primary(&mut self, v: u64) {
        self.num_objects_missing_on_primary = ::std::option::Option::Some(v);
    }

    pub fn get_num_objects_missing_on_primary(&self) -> u64 {
        self.num_objects_missing_on_primary.unwrap_or(0)
    }

    fn get_num_objects_missing_on_primary_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_objects_missing_on_primary
    }

    fn mut_num_objects_missing_on_primary_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_objects_missing_on_primary
    }

    // required uint64 num_objects_unfound = 8;

    pub fn clear_num_objects_unfound(&mut self) {
        self.num_objects_unfound = ::std::option::Option::None;
    }

    pub fn has_num_objects_unfound(&self) -> bool {
        self.num_objects_unfound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_objects_unfound(&mut self, v: u64) {
        self.num_objects_unfound = ::std::option::Option::Some(v);
    }

    pub fn get_num_objects_unfound(&self) -> u64 {
        self.num_objects_unfound.unwrap_or(0)
    }

    fn get_num_objects_unfound_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_objects_unfound
    }

    fn mut_num_objects_unfound_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_objects_unfound
    }

    // required uint64 num_objects_degraded = 9;

    pub fn clear_num_objects_degraded(&mut self) {
        self.num_objects_degraded = ::std::option::Option::None;
    }

    pub fn has_num_objects_degraded(&self) -> bool {
        self.num_objects_degraded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_objects_degraded(&mut self, v: u64) {
        self.num_objects_degraded = ::std::option::Option::Some(v);
    }

    pub fn get_num_objects_degraded(&self) -> u64 {
        self.num_objects_degraded.unwrap_or(0)
    }

    fn get_num_objects_degraded_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_objects_degraded
    }

    fn mut_num_objects_degraded_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_objects_degraded
    }

    // required uint64 num_rd = 10;

    pub fn clear_num_rd(&mut self) {
        self.num_rd = ::std::option::Option::None;
    }

    pub fn has_num_rd(&self) -> bool {
        self.num_rd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_rd(&mut self, v: u64) {
        self.num_rd = ::std::option::Option::Some(v);
    }

    pub fn get_num_rd(&self) -> u64 {
        self.num_rd.unwrap_or(0)
    }

    fn get_num_rd_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_rd
    }

    fn mut_num_rd_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_rd
    }

    // required uint64 num_rd_kb = 11;

    pub fn clear_num_rd_kb(&mut self) {
        self.num_rd_kb = ::std::option::Option::None;
    }

    pub fn has_num_rd_kb(&self) -> bool {
        self.num_rd_kb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_rd_kb(&mut self, v: u64) {
        self.num_rd_kb = ::std::option::Option::Some(v);
    }

    pub fn get_num_rd_kb(&self) -> u64 {
        self.num_rd_kb.unwrap_or(0)
    }

    fn get_num_rd_kb_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_rd_kb
    }

    fn mut_num_rd_kb_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_rd_kb
    }

    // required uint64 num_wr = 12;

    pub fn clear_num_wr(&mut self) {
        self.num_wr = ::std::option::Option::None;
    }

    pub fn has_num_wr(&self) -> bool {
        self.num_wr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_wr(&mut self, v: u64) {
        self.num_wr = ::std::option::Option::Some(v);
    }

    pub fn get_num_wr(&self) -> u64 {
        self.num_wr.unwrap_or(0)
    }

    fn get_num_wr_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_wr
    }

    fn mut_num_wr_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_wr
    }

    // required uint64 num_wr_kb = 13;

    pub fn clear_num_wr_kb(&mut self) {
        self.num_wr_kb = ::std::option::Option::None;
    }

    pub fn has_num_wr_kb(&self) -> bool {
        self.num_wr_kb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_wr_kb(&mut self, v: u64) {
        self.num_wr_kb = ::std::option::Option::Some(v);
    }

    pub fn get_num_wr_kb(&self) -> u64 {
        self.num_wr_kb.unwrap_or(0)
    }

    fn get_num_wr_kb_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_wr_kb
    }

    fn mut_num_wr_kb_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_wr_kb
    }
}

impl ::protobuf::Message for PoolUsage {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.num_bytes.is_none() {
            return false;
        }
        if self.num_kb.is_none() {
            return false;
        }
        if self.num_objects.is_none() {
            return false;
        }
        if self.num_object_clones.is_none() {
            return false;
        }
        if self.num_object_copies.is_none() {
            return false;
        }
        if self.num_objects_missing_on_primary.is_none() {
            return false;
        }
        if self.num_objects_unfound.is_none() {
            return false;
        }
        if self.num_objects_degraded.is_none() {
            return false;
        }
        if self.num_rd.is_none() {
            return false;
        }
        if self.num_rd_kb.is_none() {
            return false;
        }
        if self.num_wr.is_none() {
            return false;
        }
        if self.num_wr_kb.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_bytes = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_kb = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_objects = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_object_clones = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_object_copies = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_objects_missing_on_primary = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_objects_unfound = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_objects_degraded = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_rd = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_rd_kb = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_wr = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_wr_kb = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.num_bytes {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_kb {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_objects {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_object_clones {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_object_copies {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_objects_missing_on_primary {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_objects_unfound {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_objects_degraded {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_rd {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_rd_kb {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_wr {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_wr_kb {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.num_bytes {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.num_kb {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.num_objects {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.num_object_clones {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.num_object_copies {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.num_objects_missing_on_primary {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.num_objects_unfound {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.num_objects_degraded {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.num_rd {
            os.write_uint64(10, v)?;
        }
        if let Some(v) = self.num_rd_kb {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.num_wr {
            os.write_uint64(12, v)?;
        }
        if let Some(v) = self.num_wr_kb {
            os.write_uint64(13, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PoolUsage {
    fn new() -> PoolUsage {
        PoolUsage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoolUsage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PoolUsage::get_name_for_reflect,
                    PoolUsage::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_bytes",
                    PoolUsage::get_num_bytes_for_reflect,
                    PoolUsage::mut_num_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_kb",
                    PoolUsage::get_num_kb_for_reflect,
                    PoolUsage::mut_num_kb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_objects",
                    PoolUsage::get_num_objects_for_reflect,
                    PoolUsage::mut_num_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_object_clones",
                    PoolUsage::get_num_object_clones_for_reflect,
                    PoolUsage::mut_num_object_clones_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_object_copies",
                    PoolUsage::get_num_object_copies_for_reflect,
                    PoolUsage::mut_num_object_copies_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_objects_missing_on_primary",
                    PoolUsage::get_num_objects_missing_on_primary_for_reflect,
                    PoolUsage::mut_num_objects_missing_on_primary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_objects_unfound",
                    PoolUsage::get_num_objects_unfound_for_reflect,
                    PoolUsage::mut_num_objects_unfound_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_objects_degraded",
                    PoolUsage::get_num_objects_degraded_for_reflect,
                    PoolUsage::mut_num_objects_degraded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_rd",
                    PoolUsage::get_num_rd_for_reflect,
                    PoolUsage::mut_num_rd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_rd_kb",
                    PoolUsage::get_num_rd_kb_for_reflect,
                    PoolUsage::mut_num_rd_kb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_wr",
                    PoolUsage::get_num_wr_for_reflect,
                    PoolUsage::mut_num_wr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_wr_kb",
                    PoolUsage::get_num_wr_kb_for_reflect,
                    PoolUsage::mut_num_wr_kb_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoolUsage>(
                    "PoolUsage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoolUsage {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_num_bytes();
        self.clear_num_kb();
        self.clear_num_objects();
        self.clear_num_object_clones();
        self.clear_num_object_copies();
        self.clear_num_objects_missing_on_primary();
        self.clear_num_objects_unfound();
        self.clear_num_objects_degraded();
        self.clear_num_rd();
        self.clear_num_rd_kb();
        self.clear_num_wr();
        self.clear_num_wr_kb();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoolUsage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoolUsage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterUsage {
    // message fields
    kb: ::std::option::Option<u64>,
    kb_used: ::std::option::Option<u64>,
    kb_avail: ::std::option::Option<u64>,
    num_objects: ::std::option::Option<u64>,
    pool_info: ::protobuf::RepeatedField<PoolUsage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterUsage {}

impl ClusterUsage {
    pub fn new() -> ClusterUsage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterUsage {
        static mut instance: ::protobuf::lazy::Lazy<ClusterUsage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterUsage,
        };
        unsafe {
            instance.get(ClusterUsage::new)
        }
    }

    // required uint64 kb = 1;

    pub fn clear_kb(&mut self) {
        self.kb = ::std::option::Option::None;
    }

    pub fn has_kb(&self) -> bool {
        self.kb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb(&mut self, v: u64) {
        self.kb = ::std::option::Option::Some(v);
    }

    pub fn get_kb(&self) -> u64 {
        self.kb.unwrap_or(0)
    }

    fn get_kb_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb
    }

    fn mut_kb_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb
    }

    // required uint64 kb_used = 2;

    pub fn clear_kb_used(&mut self) {
        self.kb_used = ::std::option::Option::None;
    }

    pub fn has_kb_used(&self) -> bool {
        self.kb_used.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_used(&mut self, v: u64) {
        self.kb_used = ::std::option::Option::Some(v);
    }

    pub fn get_kb_used(&self) -> u64 {
        self.kb_used.unwrap_or(0)
    }

    fn get_kb_used_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_used
    }

    fn mut_kb_used_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_used
    }

    // required uint64 kb_avail = 3;

    pub fn clear_kb_avail(&mut self) {
        self.kb_avail = ::std::option::Option::None;
    }

    pub fn has_kb_avail(&self) -> bool {
        self.kb_avail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_avail(&mut self, v: u64) {
        self.kb_avail = ::std::option::Option::Some(v);
    }

    pub fn get_kb_avail(&self) -> u64 {
        self.kb_avail.unwrap_or(0)
    }

    fn get_kb_avail_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_avail
    }

    fn mut_kb_avail_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_avail
    }

    // required uint64 num_objects = 4;

    pub fn clear_num_objects(&mut self) {
        self.num_objects = ::std::option::Option::None;
    }

    pub fn has_num_objects(&self) -> bool {
        self.num_objects.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_objects(&mut self, v: u64) {
        self.num_objects = ::std::option::Option::Some(v);
    }

    pub fn get_num_objects(&self) -> u64 {
        self.num_objects.unwrap_or(0)
    }

    fn get_num_objects_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.num_objects
    }

    fn mut_num_objects_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.num_objects
    }

    // repeated .ceph_usage.PoolUsage pool_info = 5;

    pub fn clear_pool_info(&mut self) {
        self.pool_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_pool_info(&mut self, v: ::protobuf::RepeatedField<PoolUsage>) {
        self.pool_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pool_info(&mut self) -> &mut ::protobuf::RepeatedField<PoolUsage> {
        &mut self.pool_info
    }

    // Take field
    pub fn take_pool_info(&mut self) -> ::protobuf::RepeatedField<PoolUsage> {
        ::std::mem::replace(&mut self.pool_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_pool_info(&self) -> &[PoolUsage] {
        &self.pool_info
    }

    fn get_pool_info_for_reflect(&self) -> &::protobuf::RepeatedField<PoolUsage> {
        &self.pool_info
    }

    fn mut_pool_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PoolUsage> {
        &mut self.pool_info
    }
}

impl ::protobuf::Message for ClusterUsage {
    fn is_initialized(&self) -> bool {
        if self.kb.is_none() {
            return false;
        }
        if self.kb_used.is_none() {
            return false;
        }
        if self.kb_avail.is_none() {
            return false;
        }
        if self.num_objects.is_none() {
            return false;
        }
        for v in &self.pool_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.kb = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.kb_used = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.kb_avail = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_objects = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pool_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.kb {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.kb_used {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.kb_avail {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_objects {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.pool_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kb {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.kb_used {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.kb_avail {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.num_objects {
            os.write_uint64(4, v)?;
        }
        for v in &self.pool_info {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClusterUsage {
    fn new() -> ClusterUsage {
        ClusterUsage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterUsage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb",
                    ClusterUsage::get_kb_for_reflect,
                    ClusterUsage::mut_kb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_used",
                    ClusterUsage::get_kb_used_for_reflect,
                    ClusterUsage::mut_kb_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_avail",
                    ClusterUsage::get_kb_avail_for_reflect,
                    ClusterUsage::mut_kb_avail_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_objects",
                    ClusterUsage::get_num_objects_for_reflect,
                    ClusterUsage::mut_num_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoolUsage>>(
                    "pool_info",
                    ClusterUsage::get_pool_info_for_reflect,
                    ClusterUsage::mut_pool_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterUsage>(
                    "ClusterUsage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterUsage {
    fn clear(&mut self) {
        self.clear_kb();
        self.clear_kb_used();
        self.clear_kb_avail();
        self.clear_num_objects();
        self.clear_pool_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterUsage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterUsage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10protos/api.proto\x12\nceph_usage\"\xb0\x02\n\tPoolUsage\x12\x0c\n\
    \x04name\x18\x01\x20\x02(\t\x12\x11\n\tnum_bytes\x18\x02\x20\x02(\x04\
    \x12\x0e\n\x06num_kb\x18\x03\x20\x02(\x04\x12\x13\n\x0bnum_objects\x18\
    \x04\x20\x02(\x04\x12\x19\n\x11num_object_clones\x18\x05\x20\x02(\x04\
    \x12\x19\n\x11num_object_copies\x18\x06\x20\x02(\x04\x12&\n\x1enum_objec\
    ts_missing_on_primary\x18\x07\x20\x02(\x04\x12\x1b\n\x13num_objects_unfo\
    und\x18\x08\x20\x02(\x04\x12\x1c\n\x14num_objects_degraded\x18\t\x20\x02\
    (\x04\x12\x0e\n\x06num_rd\x18\n\x20\x02(\x04\x12\x11\n\tnum_rd_kb\x18\
    \x0b\x20\x02(\x04\x12\x0e\n\x06num_wr\x18\x0c\x20\x02(\x04\x12\x11\n\tnu\
    m_wr_kb\x18\r\x20\x02(\x04\"|\n\x0cClusterUsage\x12\n\n\x02kb\x18\x01\
    \x20\x02(\x04\x12\x0f\n\x07kb_used\x18\x02\x20\x02(\x04\x12\x10\n\x08kb_\
    avail\x18\x03\x20\x02(\x04\x12\x13\n\x0bnum_objects\x18\x04\x20\x02(\x04\
    \x12(\n\tpool_info\x18\x05\x20\x03(\x0b2\x15.ceph_usage.PoolUsageB\x02H\
    \x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
