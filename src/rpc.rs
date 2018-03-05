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
pub struct CreateWalletRequest {
    // message fields
    pub password: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateWalletRequest {}

impl CreateWalletRequest {
    pub fn new() -> CreateWalletRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateWalletRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateWalletRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateWalletRequest,
        };
        unsafe {
            instance.get(CreateWalletRequest::new)
        }
    }

    // bytes password = 1;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.password, ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }
}

impl ::protobuf::Message for CreateWalletRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.password)?;
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
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.password.is_empty() {
            os.write_bytes(1, &self.password)?;
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

impl ::protobuf::MessageStatic for CreateWalletRequest {
    fn new() -> CreateWalletRequest {
        CreateWalletRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateWalletRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    CreateWalletRequest::get_password_for_reflect,
                    CreateWalletRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateWalletRequest>(
                    "CreateWalletRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateWalletRequest {
    fn clear(&mut self) {
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateWalletRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateWalletRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateWalletResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateWalletResponse {}

impl CreateWalletResponse {
    pub fn new() -> CreateWalletResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateWalletResponse {
        static mut instance: ::protobuf::lazy::Lazy<CreateWalletResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateWalletResponse,
        };
        unsafe {
            instance.get(CreateWalletResponse::new)
        }
    }
}

impl ::protobuf::Message for CreateWalletResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for CreateWalletResponse {
    fn new() -> CreateWalletResponse {
        CreateWalletResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateWalletResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CreateWalletResponse>(
                    "CreateWalletResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateWalletResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateWalletResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateWalletResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockWalletRequest {
    // message fields
    pub password: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnlockWalletRequest {}

impl UnlockWalletRequest {
    pub fn new() -> UnlockWalletRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlockWalletRequest {
        static mut instance: ::protobuf::lazy::Lazy<UnlockWalletRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockWalletRequest,
        };
        unsafe {
            instance.get(UnlockWalletRequest::new)
        }
    }

    // bytes password = 1;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.password, ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }
}

impl ::protobuf::Message for UnlockWalletRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.password)?;
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
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.password.is_empty() {
            os.write_bytes(1, &self.password)?;
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

impl ::protobuf::MessageStatic for UnlockWalletRequest {
    fn new() -> UnlockWalletRequest {
        UnlockWalletRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlockWalletRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    UnlockWalletRequest::get_password_for_reflect,
                    UnlockWalletRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlockWalletRequest>(
                    "UnlockWalletRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlockWalletRequest {
    fn clear(&mut self) {
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockWalletRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockWalletRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockWalletResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnlockWalletResponse {}

impl UnlockWalletResponse {
    pub fn new() -> UnlockWalletResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlockWalletResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnlockWalletResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockWalletResponse,
        };
        unsafe {
            instance.get(UnlockWalletResponse::new)
        }
    }
}

impl ::protobuf::Message for UnlockWalletResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for UnlockWalletResponse {
    fn new() -> UnlockWalletResponse {
        UnlockWalletResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlockWalletResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UnlockWalletResponse>(
                    "UnlockWalletResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlockWalletResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockWalletResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockWalletResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Transaction {
    // message fields
    pub tx_hash: ::std::string::String,
    pub amount: i64,
    pub num_confirmations: i32,
    pub block_hash: ::std::string::String,
    pub block_height: i32,
    pub time_stamp: i64,
    pub total_fees: i64,
    pub dest_addresses: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // string tx_hash = 1;

    pub fn clear_tx_hash(&mut self) {
        self.tx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_hash(&mut self, v: ::std::string::String) {
        self.tx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_hash(&mut self) -> &mut ::std::string::String {
        &mut self.tx_hash
    }

    // Take field
    pub fn take_tx_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx_hash, ::std::string::String::new())
    }

    pub fn get_tx_hash(&self) -> &str {
        &self.tx_hash
    }

    fn get_tx_hash_for_reflect(&self) -> &::std::string::String {
        &self.tx_hash
    }

    fn mut_tx_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx_hash
    }

    // int64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i64 {
        &mut self.amount
    }

    // int32 num_confirmations = 3;

    pub fn clear_num_confirmations(&mut self) {
        self.num_confirmations = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_confirmations(&mut self, v: i32) {
        self.num_confirmations = v;
    }

    pub fn get_num_confirmations(&self) -> i32 {
        self.num_confirmations
    }

    fn get_num_confirmations_for_reflect(&self) -> &i32 {
        &self.num_confirmations
    }

    fn mut_num_confirmations_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_confirmations
    }

    // string block_hash = 4;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::string::String) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::string::String {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.block_hash, ::std::string::String::new())
    }

    pub fn get_block_hash(&self) -> &str {
        &self.block_hash
    }

    fn get_block_hash_for_reflect(&self) -> &::std::string::String {
        &self.block_hash
    }

    fn mut_block_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.block_hash
    }

    // int32 block_height = 5;

    pub fn clear_block_height(&mut self) {
        self.block_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_height(&mut self, v: i32) {
        self.block_height = v;
    }

    pub fn get_block_height(&self) -> i32 {
        self.block_height
    }

    fn get_block_height_for_reflect(&self) -> &i32 {
        &self.block_height
    }

    fn mut_block_height_for_reflect(&mut self) -> &mut i32 {
        &mut self.block_height
    }

    // int64 time_stamp = 6;

    pub fn clear_time_stamp(&mut self) {
        self.time_stamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_time_stamp(&mut self, v: i64) {
        self.time_stamp = v;
    }

    pub fn get_time_stamp(&self) -> i64 {
        self.time_stamp
    }

    fn get_time_stamp_for_reflect(&self) -> &i64 {
        &self.time_stamp
    }

    fn mut_time_stamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.time_stamp
    }

    // int64 total_fees = 7;

    pub fn clear_total_fees(&mut self) {
        self.total_fees = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_fees(&mut self, v: i64) {
        self.total_fees = v;
    }

    pub fn get_total_fees(&self) -> i64 {
        self.total_fees
    }

    fn get_total_fees_for_reflect(&self) -> &i64 {
        &self.total_fees
    }

    fn mut_total_fees_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_fees
    }

    // repeated string dest_addresses = 8;

    pub fn clear_dest_addresses(&mut self) {
        self.dest_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_dest_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.dest_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dest_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.dest_addresses
    }

    // Take field
    pub fn take_dest_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.dest_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_dest_addresses(&self) -> &[::std::string::String] {
        &self.dest_addresses
    }

    fn get_dest_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.dest_addresses
    }

    fn mut_dest_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.dest_addresses
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx_hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amount = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_confirmations = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.block_hash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.block_height = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time_stamp = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_fees = tmp;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.dest_addresses)?;
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
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx_hash);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_confirmations != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_confirmations, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.block_hash);
        }
        if self.block_height != 0 {
            my_size += ::protobuf::rt::value_size(5, self.block_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.time_stamp != 0 {
            my_size += ::protobuf::rt::value_size(6, self.time_stamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_fees != 0 {
            my_size += ::protobuf::rt::value_size(7, self.total_fees, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.dest_addresses {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_hash.is_empty() {
            os.write_string(1, &self.tx_hash)?;
        }
        if self.amount != 0 {
            os.write_int64(2, self.amount)?;
        }
        if self.num_confirmations != 0 {
            os.write_int32(3, self.num_confirmations)?;
        }
        if !self.block_hash.is_empty() {
            os.write_string(4, &self.block_hash)?;
        }
        if self.block_height != 0 {
            os.write_int32(5, self.block_height)?;
        }
        if self.time_stamp != 0 {
            os.write_int64(6, self.time_stamp)?;
        }
        if self.total_fees != 0 {
            os.write_int64(7, self.total_fees)?;
        }
        for v in &self.dest_addresses {
            os.write_string(8, &v)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx_hash",
                    Transaction::get_tx_hash_for_reflect,
                    Transaction::mut_tx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amount",
                    Transaction::get_amount_for_reflect,
                    Transaction::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_confirmations",
                    Transaction::get_num_confirmations_for_reflect,
                    Transaction::mut_num_confirmations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "block_hash",
                    Transaction::get_block_hash_for_reflect,
                    Transaction::mut_block_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_height",
                    Transaction::get_block_height_for_reflect,
                    Transaction::mut_block_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time_stamp",
                    Transaction::get_time_stamp_for_reflect,
                    Transaction::mut_time_stamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_fees",
                    Transaction::get_total_fees_for_reflect,
                    Transaction::mut_total_fees_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dest_addresses",
                    Transaction::get_dest_addresses_for_reflect,
                    Transaction::mut_dest_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_tx_hash();
        self.clear_amount();
        self.clear_num_confirmations();
        self.clear_block_hash();
        self.clear_block_height();
        self.clear_time_stamp();
        self.clear_total_fees();
        self.clear_dest_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTransactionsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTransactionsRequest {}

impl GetTransactionsRequest {
    pub fn new() -> GetTransactionsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTransactionsRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetTransactionsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTransactionsRequest,
        };
        unsafe {
            instance.get(GetTransactionsRequest::new)
        }
    }
}

impl ::protobuf::Message for GetTransactionsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for GetTransactionsRequest {
    fn new() -> GetTransactionsRequest {
        GetTransactionsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTransactionsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetTransactionsRequest>(
                    "GetTransactionsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTransactionsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTransactionsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTransactionsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionDetails {
    // message fields
    pub transactions: ::protobuf::RepeatedField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionDetails {}

impl TransactionDetails {
    pub fn new() -> TransactionDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionDetails {
        static mut instance: ::protobuf::lazy::Lazy<TransactionDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionDetails,
        };
        unsafe {
            instance.get(TransactionDetails::new)
        }
    }

    // repeated .lnrpc.Transaction transactions = 1;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<Transaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for TransactionDetails {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.transactions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TransactionDetails {
    fn new() -> TransactionDetails {
        TransactionDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transactions",
                    TransactionDetails::get_transactions_for_reflect,
                    TransactionDetails::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionDetails>(
                    "TransactionDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionDetails {
    fn clear(&mut self) {
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendRequest {
    // message fields
    pub dest: ::std::vec::Vec<u8>,
    pub dest_string: ::std::string::String,
    pub amt: i64,
    pub payment_hash: ::std::vec::Vec<u8>,
    pub payment_hash_string: ::std::string::String,
    pub payment_request: ::std::string::String,
    pub final_cltv_delta: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendRequest {}

impl SendRequest {
    pub fn new() -> SendRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendRequest {
        static mut instance: ::protobuf::lazy::Lazy<SendRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendRequest,
        };
        unsafe {
            instance.get(SendRequest::new)
        }
    }

    // bytes dest = 1;

    pub fn clear_dest(&mut self) {
        self.dest.clear();
    }

    // Param is passed by value, moved
    pub fn set_dest(&mut self, v: ::std::vec::Vec<u8>) {
        self.dest = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dest(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dest
    }

    // Take field
    pub fn take_dest(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dest, ::std::vec::Vec::new())
    }

    pub fn get_dest(&self) -> &[u8] {
        &self.dest
    }

    fn get_dest_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dest
    }

    fn mut_dest_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dest
    }

    // string dest_string = 2;

    pub fn clear_dest_string(&mut self) {
        self.dest_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_dest_string(&mut self, v: ::std::string::String) {
        self.dest_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dest_string(&mut self) -> &mut ::std::string::String {
        &mut self.dest_string
    }

    // Take field
    pub fn take_dest_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dest_string, ::std::string::String::new())
    }

    pub fn get_dest_string(&self) -> &str {
        &self.dest_string
    }

    fn get_dest_string_for_reflect(&self) -> &::std::string::String {
        &self.dest_string
    }

    fn mut_dest_string_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.dest_string
    }

    // int64 amt = 3;

    pub fn clear_amt(&mut self) {
        self.amt = 0;
    }

    // Param is passed by value, moved
    pub fn set_amt(&mut self, v: i64) {
        self.amt = v;
    }

    pub fn get_amt(&self) -> i64 {
        self.amt
    }

    fn get_amt_for_reflect(&self) -> &i64 {
        &self.amt
    }

    fn mut_amt_for_reflect(&mut self) -> &mut i64 {
        &mut self.amt
    }

    // bytes payment_hash = 4;

    pub fn clear_payment_hash(&mut self) {
        self.payment_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.payment_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payment_hash
    }

    // Take field
    pub fn take_payment_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payment_hash, ::std::vec::Vec::new())
    }

    pub fn get_payment_hash(&self) -> &[u8] {
        &self.payment_hash
    }

    fn get_payment_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.payment_hash
    }

    fn mut_payment_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payment_hash
    }

    // string payment_hash_string = 5;

    pub fn clear_payment_hash_string(&mut self) {
        self.payment_hash_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_hash_string(&mut self, v: ::std::string::String) {
        self.payment_hash_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_hash_string(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash_string
    }

    // Take field
    pub fn take_payment_hash_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_hash_string, ::std::string::String::new())
    }

    pub fn get_payment_hash_string(&self) -> &str {
        &self.payment_hash_string
    }

    fn get_payment_hash_string_for_reflect(&self) -> &::std::string::String {
        &self.payment_hash_string
    }

    fn mut_payment_hash_string_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash_string
    }

    // string payment_request = 6;

    pub fn clear_payment_request(&mut self) {
        self.payment_request.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_request(&mut self, v: ::std::string::String) {
        self.payment_request = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_request(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }

    // Take field
    pub fn take_payment_request(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_request, ::std::string::String::new())
    }

    pub fn get_payment_request(&self) -> &str {
        &self.payment_request
    }

    fn get_payment_request_for_reflect(&self) -> &::std::string::String {
        &self.payment_request
    }

    fn mut_payment_request_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }

    // int32 final_cltv_delta = 7;

    pub fn clear_final_cltv_delta(&mut self) {
        self.final_cltv_delta = 0;
    }

    // Param is passed by value, moved
    pub fn set_final_cltv_delta(&mut self, v: i32) {
        self.final_cltv_delta = v;
    }

    pub fn get_final_cltv_delta(&self) -> i32 {
        self.final_cltv_delta
    }

    fn get_final_cltv_delta_for_reflect(&self) -> &i32 {
        &self.final_cltv_delta
    }

    fn mut_final_cltv_delta_for_reflect(&mut self) -> &mut i32 {
        &mut self.final_cltv_delta
    }
}

impl ::protobuf::Message for SendRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dest)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.dest_string)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amt = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payment_hash)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_hash_string)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_request)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.final_cltv_delta = tmp;
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
        if !self.dest.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.dest);
        }
        if !self.dest_string.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.dest_string);
        }
        if self.amt != 0 {
            my_size += ::protobuf::rt::value_size(3, self.amt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.payment_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.payment_hash);
        }
        if !self.payment_hash_string.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.payment_hash_string);
        }
        if !self.payment_request.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.payment_request);
        }
        if self.final_cltv_delta != 0 {
            my_size += ::protobuf::rt::value_size(7, self.final_cltv_delta, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.dest.is_empty() {
            os.write_bytes(1, &self.dest)?;
        }
        if !self.dest_string.is_empty() {
            os.write_string(2, &self.dest_string)?;
        }
        if self.amt != 0 {
            os.write_int64(3, self.amt)?;
        }
        if !self.payment_hash.is_empty() {
            os.write_bytes(4, &self.payment_hash)?;
        }
        if !self.payment_hash_string.is_empty() {
            os.write_string(5, &self.payment_hash_string)?;
        }
        if !self.payment_request.is_empty() {
            os.write_string(6, &self.payment_request)?;
        }
        if self.final_cltv_delta != 0 {
            os.write_int32(7, self.final_cltv_delta)?;
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

impl ::protobuf::MessageStatic for SendRequest {
    fn new() -> SendRequest {
        SendRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dest",
                    SendRequest::get_dest_for_reflect,
                    SendRequest::mut_dest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dest_string",
                    SendRequest::get_dest_string_for_reflect,
                    SendRequest::mut_dest_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amt",
                    SendRequest::get_amt_for_reflect,
                    SendRequest::mut_amt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payment_hash",
                    SendRequest::get_payment_hash_for_reflect,
                    SendRequest::mut_payment_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_hash_string",
                    SendRequest::get_payment_hash_string_for_reflect,
                    SendRequest::mut_payment_hash_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_request",
                    SendRequest::get_payment_request_for_reflect,
                    SendRequest::mut_payment_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "final_cltv_delta",
                    SendRequest::get_final_cltv_delta_for_reflect,
                    SendRequest::mut_final_cltv_delta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendRequest>(
                    "SendRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendRequest {
    fn clear(&mut self) {
        self.clear_dest();
        self.clear_dest_string();
        self.clear_amt();
        self.clear_payment_hash();
        self.clear_payment_hash_string();
        self.clear_payment_request();
        self.clear_final_cltv_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendResponse {
    // message fields
    pub payment_error: ::std::string::String,
    pub payment_preimage: ::std::vec::Vec<u8>,
    pub payment_route: ::protobuf::SingularPtrField<Route>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendResponse {}

impl SendResponse {
    pub fn new() -> SendResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendResponse {
        static mut instance: ::protobuf::lazy::Lazy<SendResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendResponse,
        };
        unsafe {
            instance.get(SendResponse::new)
        }
    }

    // string payment_error = 1;

    pub fn clear_payment_error(&mut self) {
        self.payment_error.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_error(&mut self, v: ::std::string::String) {
        self.payment_error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_error(&mut self) -> &mut ::std::string::String {
        &mut self.payment_error
    }

    // Take field
    pub fn take_payment_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_error, ::std::string::String::new())
    }

    pub fn get_payment_error(&self) -> &str {
        &self.payment_error
    }

    fn get_payment_error_for_reflect(&self) -> &::std::string::String {
        &self.payment_error
    }

    fn mut_payment_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_error
    }

    // bytes payment_preimage = 2;

    pub fn clear_payment_preimage(&mut self) {
        self.payment_preimage.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_preimage(&mut self, v: ::std::vec::Vec<u8>) {
        self.payment_preimage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_preimage(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payment_preimage
    }

    // Take field
    pub fn take_payment_preimage(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payment_preimage, ::std::vec::Vec::new())
    }

    pub fn get_payment_preimage(&self) -> &[u8] {
        &self.payment_preimage
    }

    fn get_payment_preimage_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.payment_preimage
    }

    fn mut_payment_preimage_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payment_preimage
    }

    // .lnrpc.Route payment_route = 3;

    pub fn clear_payment_route(&mut self) {
        self.payment_route.clear();
    }

    pub fn has_payment_route(&self) -> bool {
        self.payment_route.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payment_route(&mut self, v: Route) {
        self.payment_route = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_route(&mut self) -> &mut Route {
        if self.payment_route.is_none() {
            self.payment_route.set_default();
        }
        self.payment_route.as_mut().unwrap()
    }

    // Take field
    pub fn take_payment_route(&mut self) -> Route {
        self.payment_route.take().unwrap_or_else(|| Route::new())
    }

    pub fn get_payment_route(&self) -> &Route {
        self.payment_route.as_ref().unwrap_or_else(|| Route::default_instance())
    }

    fn get_payment_route_for_reflect(&self) -> &::protobuf::SingularPtrField<Route> {
        &self.payment_route
    }

    fn mut_payment_route_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Route> {
        &mut self.payment_route
    }
}

impl ::protobuf::Message for SendResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.payment_route {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payment_preimage)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payment_route)?;
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
        if !self.payment_error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.payment_error);
        }
        if !self.payment_preimage.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.payment_preimage);
        }
        if let Some(ref v) = self.payment_route.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.payment_error.is_empty() {
            os.write_string(1, &self.payment_error)?;
        }
        if !self.payment_preimage.is_empty() {
            os.write_bytes(2, &self.payment_preimage)?;
        }
        if let Some(ref v) = self.payment_route.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for SendResponse {
    fn new() -> SendResponse {
        SendResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_error",
                    SendResponse::get_payment_error_for_reflect,
                    SendResponse::mut_payment_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payment_preimage",
                    SendResponse::get_payment_preimage_for_reflect,
                    SendResponse::mut_payment_preimage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Route>>(
                    "payment_route",
                    SendResponse::get_payment_route_for_reflect,
                    SendResponse::mut_payment_route_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendResponse>(
                    "SendResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendResponse {
    fn clear(&mut self) {
        self.clear_payment_error();
        self.clear_payment_preimage();
        self.clear_payment_route();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelPoint {
    // message fields
    pub output_index: u32,
    // message oneof groups
    funding_txid: ::std::option::Option<ChannelPoint_oneof_funding_txid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelPoint {}

#[derive(Clone,PartialEq)]
pub enum ChannelPoint_oneof_funding_txid {
    funding_txid_bytes(::std::vec::Vec<u8>),
    funding_txid_str(::std::string::String),
}

impl ChannelPoint {
    pub fn new() -> ChannelPoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelPoint {
        static mut instance: ::protobuf::lazy::Lazy<ChannelPoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelPoint,
        };
        unsafe {
            instance.get(ChannelPoint::new)
        }
    }

    // bytes funding_txid_bytes = 1;

    pub fn clear_funding_txid_bytes(&mut self) {
        self.funding_txid = ::std::option::Option::None;
    }

    pub fn has_funding_txid_bytes(&self) -> bool {
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_funding_txid_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_funding_txid_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(_)) = self.funding_txid {
        } else {
            self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(::std::vec::Vec::new()));
        }
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_funding_txid_bytes(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_funding_txid_bytes() {
            match self.funding_txid.take() {
                ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_funding_txid_bytes(&self) -> &[u8] {
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(ref v)) => v,
            _ => &[],
        }
    }

    // string funding_txid_str = 2;

    pub fn clear_funding_txid_str(&mut self) {
        self.funding_txid = ::std::option::Option::None;
    }

    pub fn has_funding_txid_str(&self) -> bool {
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_funding_txid_str(&mut self, v: ::std::string::String) {
        self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(v))
    }

    // Mutable pointer to the field.
    pub fn mut_funding_txid_str(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(_)) = self.funding_txid {
        } else {
            self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(::std::string::String::new()));
        }
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_funding_txid_str(&mut self) -> ::std::string::String {
        if self.has_funding_txid_str() {
            match self.funding_txid.take() {
                ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_funding_txid_str(&self) -> &str {
        match self.funding_txid {
            ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(ref v)) => v,
            _ => "",
        }
    }

    // uint32 output_index = 3;

    pub fn clear_output_index(&mut self) {
        self.output_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_output_index(&mut self, v: u32) {
        self.output_index = v;
    }

    pub fn get_output_index(&self) -> u32 {
        self.output_index
    }

    fn get_output_index_for_reflect(&self) -> &u32 {
        &self.output_index
    }

    fn mut_output_index_for_reflect(&mut self) -> &mut u32 {
        &mut self.output_index
    }
}

impl ::protobuf::Message for ChannelPoint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_bytes(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.funding_txid = ::std::option::Option::Some(ChannelPoint_oneof_funding_txid::funding_txid_str(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.output_index = tmp;
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
        if self.output_index != 0 {
            my_size += ::protobuf::rt::value_size(3, self.output_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.funding_txid {
            match v {
                &ChannelPoint_oneof_funding_txid::funding_txid_bytes(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &ChannelPoint_oneof_funding_txid::funding_txid_str(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.output_index != 0 {
            os.write_uint32(3, self.output_index)?;
        }
        if let ::std::option::Option::Some(ref v) = self.funding_txid {
            match v {
                &ChannelPoint_oneof_funding_txid::funding_txid_bytes(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &ChannelPoint_oneof_funding_txid::funding_txid_str(ref v) => {
                    os.write_string(2, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for ChannelPoint {
    fn new() -> ChannelPoint {
        ChannelPoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelPoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "funding_txid_bytes",
                    ChannelPoint::has_funding_txid_bytes,
                    ChannelPoint::get_funding_txid_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "funding_txid_str",
                    ChannelPoint::has_funding_txid_str,
                    ChannelPoint::get_funding_txid_str,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "output_index",
                    ChannelPoint::get_output_index_for_reflect,
                    ChannelPoint::mut_output_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelPoint>(
                    "ChannelPoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelPoint {
    fn clear(&mut self) {
        self.clear_funding_txid_bytes();
        self.clear_funding_txid_str();
        self.clear_output_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelPoint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LightningAddress {
    // message fields
    pub pubkey: ::std::string::String,
    pub host: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LightningAddress {}

impl LightningAddress {
    pub fn new() -> LightningAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LightningAddress {
        static mut instance: ::protobuf::lazy::Lazy<LightningAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LightningAddress,
        };
        unsafe {
            instance.get(LightningAddress::new)
        }
    }

    // string pubkey = 1;

    pub fn clear_pubkey(&mut self) {
        self.pubkey.clear();
    }

    // Param is passed by value, moved
    pub fn set_pubkey(&mut self, v: ::std::string::String) {
        self.pubkey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubkey(&mut self) -> &mut ::std::string::String {
        &mut self.pubkey
    }

    // Take field
    pub fn take_pubkey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pubkey, ::std::string::String::new())
    }

    pub fn get_pubkey(&self) -> &str {
        &self.pubkey
    }

    fn get_pubkey_for_reflect(&self) -> &::std::string::String {
        &self.pubkey
    }

    fn mut_pubkey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pubkey
    }

    // string host = 2;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.host, ::std::string::String::new())
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    fn get_host_for_reflect(&self) -> &::std::string::String {
        &self.host
    }

    fn mut_host_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }
}

impl ::protobuf::Message for LightningAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pubkey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.host)?;
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
        if !self.pubkey.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pubkey);
        }
        if !self.host.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.host);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pubkey.is_empty() {
            os.write_string(1, &self.pubkey)?;
        }
        if !self.host.is_empty() {
            os.write_string(2, &self.host)?;
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

impl ::protobuf::MessageStatic for LightningAddress {
    fn new() -> LightningAddress {
        LightningAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<LightningAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pubkey",
                    LightningAddress::get_pubkey_for_reflect,
                    LightningAddress::mut_pubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host",
                    LightningAddress::get_host_for_reflect,
                    LightningAddress::mut_host_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LightningAddress>(
                    "LightningAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LightningAddress {
    fn clear(&mut self) {
        self.clear_pubkey();
        self.clear_host();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LightningAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LightningAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendManyRequest {
    // message fields
    pub AddrToAmount: ::std::collections::HashMap<::std::string::String, i64>,
    pub target_conf: i32,
    pub sat_per_byte: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendManyRequest {}

impl SendManyRequest {
    pub fn new() -> SendManyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendManyRequest {
        static mut instance: ::protobuf::lazy::Lazy<SendManyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendManyRequest,
        };
        unsafe {
            instance.get(SendManyRequest::new)
        }
    }

    // repeated .lnrpc.SendManyRequest.AddrToAmountEntry AddrToAmount = 1;

    pub fn clear_AddrToAmount(&mut self) {
        self.AddrToAmount.clear();
    }

    // Param is passed by value, moved
    pub fn set_AddrToAmount(&mut self, v: ::std::collections::HashMap<::std::string::String, i64>) {
        self.AddrToAmount = v;
    }

    // Mutable pointer to the field.
    pub fn mut_AddrToAmount(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, i64> {
        &mut self.AddrToAmount
    }

    // Take field
    pub fn take_AddrToAmount(&mut self) -> ::std::collections::HashMap<::std::string::String, i64> {
        ::std::mem::replace(&mut self.AddrToAmount, ::std::collections::HashMap::new())
    }

    pub fn get_AddrToAmount(&self) -> &::std::collections::HashMap<::std::string::String, i64> {
        &self.AddrToAmount
    }

    fn get_AddrToAmount_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, i64> {
        &self.AddrToAmount
    }

    fn mut_AddrToAmount_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, i64> {
        &mut self.AddrToAmount
    }

    // int32 target_conf = 3;

    pub fn clear_target_conf(&mut self) {
        self.target_conf = 0;
    }

    // Param is passed by value, moved
    pub fn set_target_conf(&mut self, v: i32) {
        self.target_conf = v;
    }

    pub fn get_target_conf(&self) -> i32 {
        self.target_conf
    }

    fn get_target_conf_for_reflect(&self) -> &i32 {
        &self.target_conf
    }

    fn mut_target_conf_for_reflect(&mut self) -> &mut i32 {
        &mut self.target_conf
    }

    // int64 sat_per_byte = 5;

    pub fn clear_sat_per_byte(&mut self) {
        self.sat_per_byte = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_per_byte(&mut self, v: i64) {
        self.sat_per_byte = v;
    }

    pub fn get_sat_per_byte(&self) -> i64 {
        self.sat_per_byte
    }

    fn get_sat_per_byte_for_reflect(&self) -> &i64 {
        &self.sat_per_byte
    }

    fn mut_sat_per_byte_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_per_byte
    }
}

impl ::protobuf::Message for SendManyRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt64>(wire_type, is, &mut self.AddrToAmount)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target_conf = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_per_byte = tmp;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt64>(1, &self.AddrToAmount);
        if self.target_conf != 0 {
            my_size += ::protobuf::rt::value_size(3, self.target_conf, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_per_byte != 0 {
            my_size += ::protobuf::rt::value_size(5, self.sat_per_byte, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt64>(1, &self.AddrToAmount, os)?;
        if self.target_conf != 0 {
            os.write_int32(3, self.target_conf)?;
        }
        if self.sat_per_byte != 0 {
            os.write_int64(5, self.sat_per_byte)?;
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

impl ::protobuf::MessageStatic for SendManyRequest {
    fn new() -> SendManyRequest {
        SendManyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendManyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt64>(
                    "AddrToAmount",
                    SendManyRequest::get_AddrToAmount_for_reflect,
                    SendManyRequest::mut_AddrToAmount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target_conf",
                    SendManyRequest::get_target_conf_for_reflect,
                    SendManyRequest::mut_target_conf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_per_byte",
                    SendManyRequest::get_sat_per_byte_for_reflect,
                    SendManyRequest::mut_sat_per_byte_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendManyRequest>(
                    "SendManyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendManyRequest {
    fn clear(&mut self) {
        self.clear_AddrToAmount();
        self.clear_target_conf();
        self.clear_sat_per_byte();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendManyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendManyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendManyResponse {
    // message fields
    pub txid: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendManyResponse {}

impl SendManyResponse {
    pub fn new() -> SendManyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendManyResponse {
        static mut instance: ::protobuf::lazy::Lazy<SendManyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendManyResponse,
        };
        unsafe {
            instance.get(SendManyResponse::new)
        }
    }

    // string txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: ::std::string::String) {
        self.txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txid(&mut self) -> &mut ::std::string::String {
        &mut self.txid
    }

    // Take field
    pub fn take_txid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.txid, ::std::string::String::new())
    }

    pub fn get_txid(&self) -> &str {
        &self.txid
    }

    fn get_txid_for_reflect(&self) -> &::std::string::String {
        &self.txid
    }

    fn mut_txid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.txid
    }
}

impl ::protobuf::Message for SendManyResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.txid)?;
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
        if !self.txid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.txid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.txid.is_empty() {
            os.write_string(1, &self.txid)?;
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

impl ::protobuf::MessageStatic for SendManyResponse {
    fn new() -> SendManyResponse {
        SendManyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendManyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "txid",
                    SendManyResponse::get_txid_for_reflect,
                    SendManyResponse::mut_txid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendManyResponse>(
                    "SendManyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendManyResponse {
    fn clear(&mut self) {
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendManyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendManyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendCoinsRequest {
    // message fields
    pub addr: ::std::string::String,
    pub amount: i64,
    pub target_conf: i32,
    pub sat_per_byte: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendCoinsRequest {}

impl SendCoinsRequest {
    pub fn new() -> SendCoinsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendCoinsRequest {
        static mut instance: ::protobuf::lazy::Lazy<SendCoinsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendCoinsRequest,
        };
        unsafe {
            instance.get(SendCoinsRequest::new)
        }
    }

    // string addr = 1;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: ::std::string::String) {
        self.addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.addr, ::std::string::String::new())
    }

    pub fn get_addr(&self) -> &str {
        &self.addr
    }

    fn get_addr_for_reflect(&self) -> &::std::string::String {
        &self.addr
    }

    fn mut_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }

    // int64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i64 {
        &mut self.amount
    }

    // int32 target_conf = 3;

    pub fn clear_target_conf(&mut self) {
        self.target_conf = 0;
    }

    // Param is passed by value, moved
    pub fn set_target_conf(&mut self, v: i32) {
        self.target_conf = v;
    }

    pub fn get_target_conf(&self) -> i32 {
        self.target_conf
    }

    fn get_target_conf_for_reflect(&self) -> &i32 {
        &self.target_conf
    }

    fn mut_target_conf_for_reflect(&mut self) -> &mut i32 {
        &mut self.target_conf
    }

    // int64 sat_per_byte = 5;

    pub fn clear_sat_per_byte(&mut self) {
        self.sat_per_byte = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_per_byte(&mut self, v: i64) {
        self.sat_per_byte = v;
    }

    pub fn get_sat_per_byte(&self) -> i64 {
        self.sat_per_byte
    }

    fn get_sat_per_byte_for_reflect(&self) -> &i64 {
        &self.sat_per_byte
    }

    fn mut_sat_per_byte_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_per_byte
    }
}

impl ::protobuf::Message for SendCoinsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.addr)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amount = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target_conf = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_per_byte = tmp;
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
        if !self.addr.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.addr);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.target_conf != 0 {
            my_size += ::protobuf::rt::value_size(3, self.target_conf, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_per_byte != 0 {
            my_size += ::protobuf::rt::value_size(5, self.sat_per_byte, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.addr.is_empty() {
            os.write_string(1, &self.addr)?;
        }
        if self.amount != 0 {
            os.write_int64(2, self.amount)?;
        }
        if self.target_conf != 0 {
            os.write_int32(3, self.target_conf)?;
        }
        if self.sat_per_byte != 0 {
            os.write_int64(5, self.sat_per_byte)?;
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

impl ::protobuf::MessageStatic for SendCoinsRequest {
    fn new() -> SendCoinsRequest {
        SendCoinsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendCoinsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addr",
                    SendCoinsRequest::get_addr_for_reflect,
                    SendCoinsRequest::mut_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amount",
                    SendCoinsRequest::get_amount_for_reflect,
                    SendCoinsRequest::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target_conf",
                    SendCoinsRequest::get_target_conf_for_reflect,
                    SendCoinsRequest::mut_target_conf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_per_byte",
                    SendCoinsRequest::get_sat_per_byte_for_reflect,
                    SendCoinsRequest::mut_sat_per_byte_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendCoinsRequest>(
                    "SendCoinsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendCoinsRequest {
    fn clear(&mut self) {
        self.clear_addr();
        self.clear_amount();
        self.clear_target_conf();
        self.clear_sat_per_byte();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendCoinsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendCoinsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SendCoinsResponse {
    // message fields
    pub txid: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendCoinsResponse {}

impl SendCoinsResponse {
    pub fn new() -> SendCoinsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendCoinsResponse {
        static mut instance: ::protobuf::lazy::Lazy<SendCoinsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendCoinsResponse,
        };
        unsafe {
            instance.get(SendCoinsResponse::new)
        }
    }

    // string txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: ::std::string::String) {
        self.txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txid(&mut self) -> &mut ::std::string::String {
        &mut self.txid
    }

    // Take field
    pub fn take_txid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.txid, ::std::string::String::new())
    }

    pub fn get_txid(&self) -> &str {
        &self.txid
    }

    fn get_txid_for_reflect(&self) -> &::std::string::String {
        &self.txid
    }

    fn mut_txid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.txid
    }
}

impl ::protobuf::Message for SendCoinsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.txid)?;
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
        if !self.txid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.txid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.txid.is_empty() {
            os.write_string(1, &self.txid)?;
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

impl ::protobuf::MessageStatic for SendCoinsResponse {
    fn new() -> SendCoinsResponse {
        SendCoinsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendCoinsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "txid",
                    SendCoinsResponse::get_txid_for_reflect,
                    SendCoinsResponse::mut_txid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendCoinsResponse>(
                    "SendCoinsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendCoinsResponse {
    fn clear(&mut self) {
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SendCoinsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SendCoinsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewAddressRequest {
    // message fields
    pub field_type: NewAddressRequest_AddressType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewAddressRequest {}

impl NewAddressRequest {
    pub fn new() -> NewAddressRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewAddressRequest {
        static mut instance: ::protobuf::lazy::Lazy<NewAddressRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewAddressRequest,
        };
        unsafe {
            instance.get(NewAddressRequest::new)
        }
    }

    // .lnrpc.NewAddressRequest.AddressType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: NewAddressRequest_AddressType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> NewAddressRequest_AddressType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &NewAddressRequest_AddressType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut NewAddressRequest_AddressType {
        &mut self.field_type
    }
}

impl ::protobuf::Message for NewAddressRequest {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
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
        if self.field_type != NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH {
            os.write_enum(1, self.field_type.value())?;
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

impl ::protobuf::MessageStatic for NewAddressRequest {
    fn new() -> NewAddressRequest {
        NewAddressRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewAddressRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NewAddressRequest_AddressType>>(
                    "type",
                    NewAddressRequest::get_field_type_for_reflect,
                    NewAddressRequest::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewAddressRequest>(
                    "NewAddressRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewAddressRequest {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewAddressRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewAddressRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NewAddressRequest_AddressType {
    WITNESS_PUBKEY_HASH = 0,
    NESTED_PUBKEY_HASH = 1,
    PUBKEY_HASH = 2,
}

impl ::protobuf::ProtobufEnum for NewAddressRequest_AddressType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NewAddressRequest_AddressType> {
        match value {
            0 => ::std::option::Option::Some(NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH),
            1 => ::std::option::Option::Some(NewAddressRequest_AddressType::NESTED_PUBKEY_HASH),
            2 => ::std::option::Option::Some(NewAddressRequest_AddressType::PUBKEY_HASH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NewAddressRequest_AddressType] = &[
            NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH,
            NewAddressRequest_AddressType::NESTED_PUBKEY_HASH,
            NewAddressRequest_AddressType::PUBKEY_HASH,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NewAddressRequest_AddressType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NewAddressRequest_AddressType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NewAddressRequest_AddressType {
}

impl ::std::default::Default for NewAddressRequest_AddressType {
    fn default() -> Self {
        NewAddressRequest_AddressType::WITNESS_PUBKEY_HASH
    }
}

impl ::protobuf::reflect::ProtobufValue for NewAddressRequest_AddressType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewWitnessAddressRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewWitnessAddressRequest {}

impl NewWitnessAddressRequest {
    pub fn new() -> NewWitnessAddressRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewWitnessAddressRequest {
        static mut instance: ::protobuf::lazy::Lazy<NewWitnessAddressRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewWitnessAddressRequest,
        };
        unsafe {
            instance.get(NewWitnessAddressRequest::new)
        }
    }
}

impl ::protobuf::Message for NewWitnessAddressRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for NewWitnessAddressRequest {
    fn new() -> NewWitnessAddressRequest {
        NewWitnessAddressRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewWitnessAddressRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<NewWitnessAddressRequest>(
                    "NewWitnessAddressRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewWitnessAddressRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewWitnessAddressRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewWitnessAddressRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewAddressResponse {
    // message fields
    pub address: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewAddressResponse {}

impl NewAddressResponse {
    pub fn new() -> NewAddressResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewAddressResponse {
        static mut instance: ::protobuf::lazy::Lazy<NewAddressResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewAddressResponse,
        };
        unsafe {
            instance.get(NewAddressResponse::new)
        }
    }

    // string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::string::String {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }
}

impl ::protobuf::Message for NewAddressResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
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

impl ::protobuf::MessageStatic for NewAddressResponse {
    fn new() -> NewAddressResponse {
        NewAddressResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewAddressResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    NewAddressResponse::get_address_for_reflect,
                    NewAddressResponse::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewAddressResponse>(
                    "NewAddressResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewAddressResponse {
    fn clear(&mut self) {
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewAddressResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewAddressResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignMessageRequest {
    // message fields
    pub msg: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignMessageRequest {}

impl SignMessageRequest {
    pub fn new() -> SignMessageRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignMessageRequest {
        static mut instance: ::protobuf::lazy::Lazy<SignMessageRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignMessageRequest,
        };
        unsafe {
            instance.get(SignMessageRequest::new)
        }
    }

    // bytes msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.msg, ::std::vec::Vec::new())
    }

    pub fn get_msg(&self) -> &[u8] {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.msg
    }
}

impl ::protobuf::Message for SignMessageRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.msg)?;
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
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.msg.is_empty() {
            os.write_bytes(1, &self.msg)?;
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

impl ::protobuf::MessageStatic for SignMessageRequest {
    fn new() -> SignMessageRequest {
        SignMessageRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignMessageRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg",
                    SignMessageRequest::get_msg_for_reflect,
                    SignMessageRequest::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignMessageRequest>(
                    "SignMessageRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignMessageRequest {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignMessageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignMessageRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignMessageResponse {
    // message fields
    pub signature: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignMessageResponse {}

impl SignMessageResponse {
    pub fn new() -> SignMessageResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignMessageResponse {
        static mut instance: ::protobuf::lazy::Lazy<SignMessageResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignMessageResponse,
        };
        unsafe {
            instance.get(SignMessageResponse::new)
        }
    }

    // string signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::string::String) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::string::String {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.signature, ::std::string::String::new())
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::string::String {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.signature
    }
}

impl ::protobuf::Message for SignMessageResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.signature)?;
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
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.signature.is_empty() {
            os.write_string(1, &self.signature)?;
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

impl ::protobuf::MessageStatic for SignMessageResponse {
    fn new() -> SignMessageResponse {
        SignMessageResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignMessageResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "signature",
                    SignMessageResponse::get_signature_for_reflect,
                    SignMessageResponse::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignMessageResponse>(
                    "SignMessageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignMessageResponse {
    fn clear(&mut self) {
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignMessageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignMessageResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyMessageRequest {
    // message fields
    pub msg: ::std::vec::Vec<u8>,
    pub signature: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyMessageRequest {}

impl VerifyMessageRequest {
    pub fn new() -> VerifyMessageRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyMessageRequest {
        static mut instance: ::protobuf::lazy::Lazy<VerifyMessageRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyMessageRequest,
        };
        unsafe {
            instance.get(VerifyMessageRequest::new)
        }
    }

    // bytes msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.msg, ::std::vec::Vec::new())
    }

    pub fn get_msg(&self) -> &[u8] {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.msg
    }

    // string signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::string::String) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::string::String {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.signature, ::std::string::String::new())
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::string::String {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.signature
    }
}

impl ::protobuf::Message for VerifyMessageRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.msg)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.signature)?;
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
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.msg);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.msg.is_empty() {
            os.write_bytes(1, &self.msg)?;
        }
        if !self.signature.is_empty() {
            os.write_string(2, &self.signature)?;
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

impl ::protobuf::MessageStatic for VerifyMessageRequest {
    fn new() -> VerifyMessageRequest {
        VerifyMessageRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyMessageRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg",
                    VerifyMessageRequest::get_msg_for_reflect,
                    VerifyMessageRequest::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "signature",
                    VerifyMessageRequest::get_signature_for_reflect,
                    VerifyMessageRequest::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyMessageRequest>(
                    "VerifyMessageRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyMessageRequest {
    fn clear(&mut self) {
        self.clear_msg();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyMessageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyMessageRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyMessageResponse {
    // message fields
    pub valid: bool,
    pub pubkey: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyMessageResponse {}

impl VerifyMessageResponse {
    pub fn new() -> VerifyMessageResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyMessageResponse {
        static mut instance: ::protobuf::lazy::Lazy<VerifyMessageResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyMessageResponse,
        };
        unsafe {
            instance.get(VerifyMessageResponse::new)
        }
    }

    // bool valid = 1;

    pub fn clear_valid(&mut self) {
        self.valid = false;
    }

    // Param is passed by value, moved
    pub fn set_valid(&mut self, v: bool) {
        self.valid = v;
    }

    pub fn get_valid(&self) -> bool {
        self.valid
    }

    fn get_valid_for_reflect(&self) -> &bool {
        &self.valid
    }

    fn mut_valid_for_reflect(&mut self) -> &mut bool {
        &mut self.valid
    }

    // string pubkey = 2;

    pub fn clear_pubkey(&mut self) {
        self.pubkey.clear();
    }

    // Param is passed by value, moved
    pub fn set_pubkey(&mut self, v: ::std::string::String) {
        self.pubkey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubkey(&mut self) -> &mut ::std::string::String {
        &mut self.pubkey
    }

    // Take field
    pub fn take_pubkey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pubkey, ::std::string::String::new())
    }

    pub fn get_pubkey(&self) -> &str {
        &self.pubkey
    }

    fn get_pubkey_for_reflect(&self) -> &::std::string::String {
        &self.pubkey
    }

    fn mut_pubkey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pubkey
    }
}

impl ::protobuf::Message for VerifyMessageResponse {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.valid = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pubkey)?;
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
        if self.valid != false {
            my_size += 2;
        }
        if !self.pubkey.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.pubkey);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.valid != false {
            os.write_bool(1, self.valid)?;
        }
        if !self.pubkey.is_empty() {
            os.write_string(2, &self.pubkey)?;
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

impl ::protobuf::MessageStatic for VerifyMessageResponse {
    fn new() -> VerifyMessageResponse {
        VerifyMessageResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyMessageResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "valid",
                    VerifyMessageResponse::get_valid_for_reflect,
                    VerifyMessageResponse::mut_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pubkey",
                    VerifyMessageResponse::get_pubkey_for_reflect,
                    VerifyMessageResponse::mut_pubkey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyMessageResponse>(
                    "VerifyMessageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyMessageResponse {
    fn clear(&mut self) {
        self.clear_valid();
        self.clear_pubkey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyMessageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyMessageResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectPeerRequest {
    // message fields
    pub addr: ::protobuf::SingularPtrField<LightningAddress>,
    pub perm: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectPeerRequest {}

impl ConnectPeerRequest {
    pub fn new() -> ConnectPeerRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectPeerRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConnectPeerRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectPeerRequest,
        };
        unsafe {
            instance.get(ConnectPeerRequest::new)
        }
    }

    // .lnrpc.LightningAddress addr = 1;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    pub fn has_addr(&self) -> bool {
        self.addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: LightningAddress) {
        self.addr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut LightningAddress {
        if self.addr.is_none() {
            self.addr.set_default();
        }
        self.addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_addr(&mut self) -> LightningAddress {
        self.addr.take().unwrap_or_else(|| LightningAddress::new())
    }

    pub fn get_addr(&self) -> &LightningAddress {
        self.addr.as_ref().unwrap_or_else(|| LightningAddress::default_instance())
    }

    fn get_addr_for_reflect(&self) -> &::protobuf::SingularPtrField<LightningAddress> {
        &self.addr
    }

    fn mut_addr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LightningAddress> {
        &mut self.addr
    }

    // bool perm = 2;

    pub fn clear_perm(&mut self) {
        self.perm = false;
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: bool) {
        self.perm = v;
    }

    pub fn get_perm(&self) -> bool {
        self.perm
    }

    fn get_perm_for_reflect(&self) -> &bool {
        &self.perm
    }

    fn mut_perm_for_reflect(&mut self) -> &mut bool {
        &mut self.perm
    }
}

impl ::protobuf::Message for ConnectPeerRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.addr {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.addr)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.perm = tmp;
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
        if let Some(ref v) = self.addr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.perm != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.addr.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.perm != false {
            os.write_bool(2, self.perm)?;
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

impl ::protobuf::MessageStatic for ConnectPeerRequest {
    fn new() -> ConnectPeerRequest {
        ConnectPeerRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectPeerRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LightningAddress>>(
                    "addr",
                    ConnectPeerRequest::get_addr_for_reflect,
                    ConnectPeerRequest::mut_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "perm",
                    ConnectPeerRequest::get_perm_for_reflect,
                    ConnectPeerRequest::mut_perm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConnectPeerRequest>(
                    "ConnectPeerRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectPeerRequest {
    fn clear(&mut self) {
        self.clear_addr();
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectPeerRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectPeerRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectPeerResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectPeerResponse {}

impl ConnectPeerResponse {
    pub fn new() -> ConnectPeerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectPeerResponse {
        static mut instance: ::protobuf::lazy::Lazy<ConnectPeerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectPeerResponse,
        };
        unsafe {
            instance.get(ConnectPeerResponse::new)
        }
    }
}

impl ::protobuf::Message for ConnectPeerResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ConnectPeerResponse {
    fn new() -> ConnectPeerResponse {
        ConnectPeerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectPeerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ConnectPeerResponse>(
                    "ConnectPeerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectPeerResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectPeerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectPeerResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisconnectPeerRequest {
    // message fields
    pub pub_key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisconnectPeerRequest {}

impl DisconnectPeerRequest {
    pub fn new() -> DisconnectPeerRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisconnectPeerRequest {
        static mut instance: ::protobuf::lazy::Lazy<DisconnectPeerRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisconnectPeerRequest,
        };
        unsafe {
            instance.get(DisconnectPeerRequest::new)
        }
    }

    // string pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }
}

impl ::protobuf::Message for DisconnectPeerRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pub_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_string(1, &self.pub_key)?;
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

impl ::protobuf::MessageStatic for DisconnectPeerRequest {
    fn new() -> DisconnectPeerRequest {
        DisconnectPeerRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisconnectPeerRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    DisconnectPeerRequest::get_pub_key_for_reflect,
                    DisconnectPeerRequest::mut_pub_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisconnectPeerRequest>(
                    "DisconnectPeerRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisconnectPeerRequest {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisconnectPeerRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisconnectPeerRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisconnectPeerResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisconnectPeerResponse {}

impl DisconnectPeerResponse {
    pub fn new() -> DisconnectPeerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisconnectPeerResponse {
        static mut instance: ::protobuf::lazy::Lazy<DisconnectPeerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisconnectPeerResponse,
        };
        unsafe {
            instance.get(DisconnectPeerResponse::new)
        }
    }
}

impl ::protobuf::Message for DisconnectPeerResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for DisconnectPeerResponse {
    fn new() -> DisconnectPeerResponse {
        DisconnectPeerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisconnectPeerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DisconnectPeerResponse>(
                    "DisconnectPeerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisconnectPeerResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisconnectPeerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisconnectPeerResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HTLC {
    // message fields
    pub incoming: bool,
    pub amount: i64,
    pub hash_lock: ::std::vec::Vec<u8>,
    pub expiration_height: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HTLC {}

impl HTLC {
    pub fn new() -> HTLC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HTLC {
        static mut instance: ::protobuf::lazy::Lazy<HTLC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HTLC,
        };
        unsafe {
            instance.get(HTLC::new)
        }
    }

    // bool incoming = 1;

    pub fn clear_incoming(&mut self) {
        self.incoming = false;
    }

    // Param is passed by value, moved
    pub fn set_incoming(&mut self, v: bool) {
        self.incoming = v;
    }

    pub fn get_incoming(&self) -> bool {
        self.incoming
    }

    fn get_incoming_for_reflect(&self) -> &bool {
        &self.incoming
    }

    fn mut_incoming_for_reflect(&mut self) -> &mut bool {
        &mut self.incoming
    }

    // int64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i64 {
        &mut self.amount
    }

    // bytes hash_lock = 3;

    pub fn clear_hash_lock(&mut self) {
        self.hash_lock.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash_lock = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_lock(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash_lock
    }

    // Take field
    pub fn take_hash_lock(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash_lock, ::std::vec::Vec::new())
    }

    pub fn get_hash_lock(&self) -> &[u8] {
        &self.hash_lock
    }

    fn get_hash_lock_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash_lock
    }

    fn mut_hash_lock_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash_lock
    }

    // uint32 expiration_height = 4;

    pub fn clear_expiration_height(&mut self) {
        self.expiration_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_expiration_height(&mut self, v: u32) {
        self.expiration_height = v;
    }

    pub fn get_expiration_height(&self) -> u32 {
        self.expiration_height
    }

    fn get_expiration_height_for_reflect(&self) -> &u32 {
        &self.expiration_height
    }

    fn mut_expiration_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.expiration_height
    }
}

impl ::protobuf::Message for HTLC {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.incoming = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amount = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash_lock)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.expiration_height = tmp;
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
        if self.incoming != false {
            my_size += 2;
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash_lock.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.hash_lock);
        }
        if self.expiration_height != 0 {
            my_size += ::protobuf::rt::value_size(4, self.expiration_height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.incoming != false {
            os.write_bool(1, self.incoming)?;
        }
        if self.amount != 0 {
            os.write_int64(2, self.amount)?;
        }
        if !self.hash_lock.is_empty() {
            os.write_bytes(3, &self.hash_lock)?;
        }
        if self.expiration_height != 0 {
            os.write_uint32(4, self.expiration_height)?;
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

impl ::protobuf::MessageStatic for HTLC {
    fn new() -> HTLC {
        HTLC::new()
    }

    fn descriptor_static(_: ::std::option::Option<HTLC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "incoming",
                    HTLC::get_incoming_for_reflect,
                    HTLC::mut_incoming_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amount",
                    HTLC::get_amount_for_reflect,
                    HTLC::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash_lock",
                    HTLC::get_hash_lock_for_reflect,
                    HTLC::mut_hash_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "expiration_height",
                    HTLC::get_expiration_height_for_reflect,
                    HTLC::mut_expiration_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HTLC>(
                    "HTLC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HTLC {
    fn clear(&mut self) {
        self.clear_incoming();
        self.clear_amount();
        self.clear_hash_lock();
        self.clear_expiration_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HTLC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTLC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActiveChannel {
    // message fields
    pub active: bool,
    pub remote_pubkey: ::std::string::String,
    pub channel_point: ::std::string::String,
    pub chan_id: u64,
    pub capacity: i64,
    pub local_balance: i64,
    pub remote_balance: i64,
    pub commit_fee: i64,
    pub commit_weight: i64,
    pub fee_per_kw: i64,
    pub unsettled_balance: i64,
    pub total_satoshis_sent: i64,
    pub total_satoshis_received: i64,
    pub num_updates: u64,
    pub pending_htlcs: ::protobuf::RepeatedField<HTLC>,
    pub csv_delay: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActiveChannel {}

impl ActiveChannel {
    pub fn new() -> ActiveChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActiveChannel {
        static mut instance: ::protobuf::lazy::Lazy<ActiveChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActiveChannel,
        };
        unsafe {
            instance.get(ActiveChannel::new)
        }
    }

    // bool active = 1;

    pub fn clear_active(&mut self) {
        self.active = false;
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: bool) {
        self.active = v;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    fn get_active_for_reflect(&self) -> &bool {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut bool {
        &mut self.active
    }

    // string remote_pubkey = 2;

    pub fn clear_remote_pubkey(&mut self) {
        self.remote_pubkey.clear();
    }

    // Param is passed by value, moved
    pub fn set_remote_pubkey(&mut self, v: ::std::string::String) {
        self.remote_pubkey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_pubkey(&mut self) -> &mut ::std::string::String {
        &mut self.remote_pubkey
    }

    // Take field
    pub fn take_remote_pubkey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remote_pubkey, ::std::string::String::new())
    }

    pub fn get_remote_pubkey(&self) -> &str {
        &self.remote_pubkey
    }

    fn get_remote_pubkey_for_reflect(&self) -> &::std::string::String {
        &self.remote_pubkey
    }

    fn mut_remote_pubkey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.remote_pubkey
    }

    // string channel_point = 3;

    pub fn clear_channel_point(&mut self) {
        self.channel_point.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_point(&mut self, v: ::std::string::String) {
        self.channel_point = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_point(&mut self) -> &mut ::std::string::String {
        &mut self.channel_point
    }

    // Take field
    pub fn take_channel_point(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.channel_point, ::std::string::String::new())
    }

    pub fn get_channel_point(&self) -> &str {
        &self.channel_point
    }

    fn get_channel_point_for_reflect(&self) -> &::std::string::String {
        &self.channel_point
    }

    fn mut_channel_point_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.channel_point
    }

    // uint64 chan_id = 4;

    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }

    fn get_chan_id_for_reflect(&self) -> &u64 {
        &self.chan_id
    }

    fn mut_chan_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.chan_id
    }

    // int64 capacity = 5;

    pub fn clear_capacity(&mut self) {
        self.capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: i64) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> i64 {
        self.capacity
    }

    fn get_capacity_for_reflect(&self) -> &i64 {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.capacity
    }

    // int64 local_balance = 6;

    pub fn clear_local_balance(&mut self) {
        self.local_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_local_balance(&mut self, v: i64) {
        self.local_balance = v;
    }

    pub fn get_local_balance(&self) -> i64 {
        self.local_balance
    }

    fn get_local_balance_for_reflect(&self) -> &i64 {
        &self.local_balance
    }

    fn mut_local_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.local_balance
    }

    // int64 remote_balance = 7;

    pub fn clear_remote_balance(&mut self) {
        self.remote_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_remote_balance(&mut self, v: i64) {
        self.remote_balance = v;
    }

    pub fn get_remote_balance(&self) -> i64 {
        self.remote_balance
    }

    fn get_remote_balance_for_reflect(&self) -> &i64 {
        &self.remote_balance
    }

    fn mut_remote_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.remote_balance
    }

    // int64 commit_fee = 8;

    pub fn clear_commit_fee(&mut self) {
        self.commit_fee = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_fee(&mut self, v: i64) {
        self.commit_fee = v;
    }

    pub fn get_commit_fee(&self) -> i64 {
        self.commit_fee
    }

    fn get_commit_fee_for_reflect(&self) -> &i64 {
        &self.commit_fee
    }

    fn mut_commit_fee_for_reflect(&mut self) -> &mut i64 {
        &mut self.commit_fee
    }

    // int64 commit_weight = 9;

    pub fn clear_commit_weight(&mut self) {
        self.commit_weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_weight(&mut self, v: i64) {
        self.commit_weight = v;
    }

    pub fn get_commit_weight(&self) -> i64 {
        self.commit_weight
    }

    fn get_commit_weight_for_reflect(&self) -> &i64 {
        &self.commit_weight
    }

    fn mut_commit_weight_for_reflect(&mut self) -> &mut i64 {
        &mut self.commit_weight
    }

    // int64 fee_per_kw = 10;

    pub fn clear_fee_per_kw(&mut self) {
        self.fee_per_kw = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_per_kw(&mut self, v: i64) {
        self.fee_per_kw = v;
    }

    pub fn get_fee_per_kw(&self) -> i64 {
        self.fee_per_kw
    }

    fn get_fee_per_kw_for_reflect(&self) -> &i64 {
        &self.fee_per_kw
    }

    fn mut_fee_per_kw_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee_per_kw
    }

    // int64 unsettled_balance = 11;

    pub fn clear_unsettled_balance(&mut self) {
        self.unsettled_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_unsettled_balance(&mut self, v: i64) {
        self.unsettled_balance = v;
    }

    pub fn get_unsettled_balance(&self) -> i64 {
        self.unsettled_balance
    }

    fn get_unsettled_balance_for_reflect(&self) -> &i64 {
        &self.unsettled_balance
    }

    fn mut_unsettled_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.unsettled_balance
    }

    // int64 total_satoshis_sent = 12;

    pub fn clear_total_satoshis_sent(&mut self) {
        self.total_satoshis_sent = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_satoshis_sent(&mut self, v: i64) {
        self.total_satoshis_sent = v;
    }

    pub fn get_total_satoshis_sent(&self) -> i64 {
        self.total_satoshis_sent
    }

    fn get_total_satoshis_sent_for_reflect(&self) -> &i64 {
        &self.total_satoshis_sent
    }

    fn mut_total_satoshis_sent_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_satoshis_sent
    }

    // int64 total_satoshis_received = 13;

    pub fn clear_total_satoshis_received(&mut self) {
        self.total_satoshis_received = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_satoshis_received(&mut self, v: i64) {
        self.total_satoshis_received = v;
    }

    pub fn get_total_satoshis_received(&self) -> i64 {
        self.total_satoshis_received
    }

    fn get_total_satoshis_received_for_reflect(&self) -> &i64 {
        &self.total_satoshis_received
    }

    fn mut_total_satoshis_received_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_satoshis_received
    }

    // uint64 num_updates = 14;

    pub fn clear_num_updates(&mut self) {
        self.num_updates = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_updates(&mut self, v: u64) {
        self.num_updates = v;
    }

    pub fn get_num_updates(&self) -> u64 {
        self.num_updates
    }

    fn get_num_updates_for_reflect(&self) -> &u64 {
        &self.num_updates
    }

    fn mut_num_updates_for_reflect(&mut self) -> &mut u64 {
        &mut self.num_updates
    }

    // repeated .lnrpc.HTLC pending_htlcs = 15;

    pub fn clear_pending_htlcs(&mut self) {
        self.pending_htlcs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_htlcs(&mut self, v: ::protobuf::RepeatedField<HTLC>) {
        self.pending_htlcs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_htlcs(&mut self) -> &mut ::protobuf::RepeatedField<HTLC> {
        &mut self.pending_htlcs
    }

    // Take field
    pub fn take_pending_htlcs(&mut self) -> ::protobuf::RepeatedField<HTLC> {
        ::std::mem::replace(&mut self.pending_htlcs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_htlcs(&self) -> &[HTLC] {
        &self.pending_htlcs
    }

    fn get_pending_htlcs_for_reflect(&self) -> &::protobuf::RepeatedField<HTLC> {
        &self.pending_htlcs
    }

    fn mut_pending_htlcs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HTLC> {
        &mut self.pending_htlcs
    }

    // uint32 csv_delay = 16;

    pub fn clear_csv_delay(&mut self) {
        self.csv_delay = 0;
    }

    // Param is passed by value, moved
    pub fn set_csv_delay(&mut self, v: u32) {
        self.csv_delay = v;
    }

    pub fn get_csv_delay(&self) -> u32 {
        self.csv_delay
    }

    fn get_csv_delay_for_reflect(&self) -> &u32 {
        &self.csv_delay
    }

    fn mut_csv_delay_for_reflect(&mut self) -> &mut u32 {
        &mut self.csv_delay
    }
}

impl ::protobuf::Message for ActiveChannel {
    fn is_initialized(&self) -> bool {
        for v in &self.pending_htlcs {
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
                    let tmp = is.read_bool()?;
                    self.active = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remote_pubkey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.channel_point)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.chan_id = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.capacity = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.local_balance = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.remote_balance = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_fee = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_weight = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee_per_kw = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.unsettled_balance = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_satoshis_sent = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_satoshis_received = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_updates = tmp;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_htlcs)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.csv_delay = tmp;
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
        if self.active != false {
            my_size += 2;
        }
        if !self.remote_pubkey.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.remote_pubkey);
        }
        if !self.channel_point.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.channel_point);
        }
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.capacity != 0 {
            my_size += ::protobuf::rt::value_size(5, self.capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.local_balance != 0 {
            my_size += ::protobuf::rt::value_size(6, self.local_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.remote_balance != 0 {
            my_size += ::protobuf::rt::value_size(7, self.remote_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_fee != 0 {
            my_size += ::protobuf::rt::value_size(8, self.commit_fee, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_weight != 0 {
            my_size += ::protobuf::rt::value_size(9, self.commit_weight, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_per_kw != 0 {
            my_size += ::protobuf::rt::value_size(10, self.fee_per_kw, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.unsettled_balance != 0 {
            my_size += ::protobuf::rt::value_size(11, self.unsettled_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_satoshis_sent != 0 {
            my_size += ::protobuf::rt::value_size(12, self.total_satoshis_sent, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_satoshis_received != 0 {
            my_size += ::protobuf::rt::value_size(13, self.total_satoshis_received, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_updates != 0 {
            my_size += ::protobuf::rt::value_size(14, self.num_updates, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.pending_htlcs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.csv_delay != 0 {
            my_size += ::protobuf::rt::value_size(16, self.csv_delay, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.active != false {
            os.write_bool(1, self.active)?;
        }
        if !self.remote_pubkey.is_empty() {
            os.write_string(2, &self.remote_pubkey)?;
        }
        if !self.channel_point.is_empty() {
            os.write_string(3, &self.channel_point)?;
        }
        if self.chan_id != 0 {
            os.write_uint64(4, self.chan_id)?;
        }
        if self.capacity != 0 {
            os.write_int64(5, self.capacity)?;
        }
        if self.local_balance != 0 {
            os.write_int64(6, self.local_balance)?;
        }
        if self.remote_balance != 0 {
            os.write_int64(7, self.remote_balance)?;
        }
        if self.commit_fee != 0 {
            os.write_int64(8, self.commit_fee)?;
        }
        if self.commit_weight != 0 {
            os.write_int64(9, self.commit_weight)?;
        }
        if self.fee_per_kw != 0 {
            os.write_int64(10, self.fee_per_kw)?;
        }
        if self.unsettled_balance != 0 {
            os.write_int64(11, self.unsettled_balance)?;
        }
        if self.total_satoshis_sent != 0 {
            os.write_int64(12, self.total_satoshis_sent)?;
        }
        if self.total_satoshis_received != 0 {
            os.write_int64(13, self.total_satoshis_received)?;
        }
        if self.num_updates != 0 {
            os.write_uint64(14, self.num_updates)?;
        }
        for v in &self.pending_htlcs {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.csv_delay != 0 {
            os.write_uint32(16, self.csv_delay)?;
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

impl ::protobuf::MessageStatic for ActiveChannel {
    fn new() -> ActiveChannel {
        ActiveChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActiveChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "active",
                    ActiveChannel::get_active_for_reflect,
                    ActiveChannel::mut_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "remote_pubkey",
                    ActiveChannel::get_remote_pubkey_for_reflect,
                    ActiveChannel::mut_remote_pubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_point",
                    ActiveChannel::get_channel_point_for_reflect,
                    ActiveChannel::mut_channel_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chan_id",
                    ActiveChannel::get_chan_id_for_reflect,
                    ActiveChannel::mut_chan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "capacity",
                    ActiveChannel::get_capacity_for_reflect,
                    ActiveChannel::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "local_balance",
                    ActiveChannel::get_local_balance_for_reflect,
                    ActiveChannel::mut_local_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "remote_balance",
                    ActiveChannel::get_remote_balance_for_reflect,
                    ActiveChannel::mut_remote_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_fee",
                    ActiveChannel::get_commit_fee_for_reflect,
                    ActiveChannel::mut_commit_fee_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_weight",
                    ActiveChannel::get_commit_weight_for_reflect,
                    ActiveChannel::mut_commit_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee_per_kw",
                    ActiveChannel::get_fee_per_kw_for_reflect,
                    ActiveChannel::mut_fee_per_kw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "unsettled_balance",
                    ActiveChannel::get_unsettled_balance_for_reflect,
                    ActiveChannel::mut_unsettled_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_satoshis_sent",
                    ActiveChannel::get_total_satoshis_sent_for_reflect,
                    ActiveChannel::mut_total_satoshis_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_satoshis_received",
                    ActiveChannel::get_total_satoshis_received_for_reflect,
                    ActiveChannel::mut_total_satoshis_received_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_updates",
                    ActiveChannel::get_num_updates_for_reflect,
                    ActiveChannel::mut_num_updates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTLC>>(
                    "pending_htlcs",
                    ActiveChannel::get_pending_htlcs_for_reflect,
                    ActiveChannel::mut_pending_htlcs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "csv_delay",
                    ActiveChannel::get_csv_delay_for_reflect,
                    ActiveChannel::mut_csv_delay_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActiveChannel>(
                    "ActiveChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActiveChannel {
    fn clear(&mut self) {
        self.clear_active();
        self.clear_remote_pubkey();
        self.clear_channel_point();
        self.clear_chan_id();
        self.clear_capacity();
        self.clear_local_balance();
        self.clear_remote_balance();
        self.clear_commit_fee();
        self.clear_commit_weight();
        self.clear_fee_per_kw();
        self.clear_unsettled_balance();
        self.clear_total_satoshis_sent();
        self.clear_total_satoshis_received();
        self.clear_num_updates();
        self.clear_pending_htlcs();
        self.clear_csv_delay();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActiveChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActiveChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListChannelsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListChannelsRequest {}

impl ListChannelsRequest {
    pub fn new() -> ListChannelsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListChannelsRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListChannelsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListChannelsRequest,
        };
        unsafe {
            instance.get(ListChannelsRequest::new)
        }
    }
}

impl ::protobuf::Message for ListChannelsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ListChannelsRequest {
    fn new() -> ListChannelsRequest {
        ListChannelsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListChannelsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListChannelsRequest>(
                    "ListChannelsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListChannelsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListChannelsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListChannelsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListChannelsResponse {
    // message fields
    pub channels: ::protobuf::RepeatedField<ActiveChannel>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListChannelsResponse {}

impl ListChannelsResponse {
    pub fn new() -> ListChannelsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListChannelsResponse {
        static mut instance: ::protobuf::lazy::Lazy<ListChannelsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListChannelsResponse,
        };
        unsafe {
            instance.get(ListChannelsResponse::new)
        }
    }

    // repeated .lnrpc.ActiveChannel channels = 11;

    pub fn clear_channels(&mut self) {
        self.channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_channels(&mut self, v: ::protobuf::RepeatedField<ActiveChannel>) {
        self.channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channels(&mut self) -> &mut ::protobuf::RepeatedField<ActiveChannel> {
        &mut self.channels
    }

    // Take field
    pub fn take_channels(&mut self) -> ::protobuf::RepeatedField<ActiveChannel> {
        ::std::mem::replace(&mut self.channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_channels(&self) -> &[ActiveChannel] {
        &self.channels
    }

    fn get_channels_for_reflect(&self) -> &::protobuf::RepeatedField<ActiveChannel> {
        &self.channels
    }

    fn mut_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ActiveChannel> {
        &mut self.channels
    }
}

impl ::protobuf::Message for ListChannelsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.channels {
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
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channels)?;
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
        for value in &self.channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.channels {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListChannelsResponse {
    fn new() -> ListChannelsResponse {
        ListChannelsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListChannelsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActiveChannel>>(
                    "channels",
                    ListChannelsResponse::get_channels_for_reflect,
                    ListChannelsResponse::mut_channels_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListChannelsResponse>(
                    "ListChannelsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListChannelsResponse {
    fn clear(&mut self) {
        self.clear_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListChannelsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListChannelsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Peer {
    // message fields
    pub pub_key: ::std::string::String,
    pub address: ::std::string::String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub sat_sent: i64,
    pub sat_recv: i64,
    pub inbound: bool,
    pub ping_time: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Peer {}

impl Peer {
    pub fn new() -> Peer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Peer {
        static mut instance: ::protobuf::lazy::Lazy<Peer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Peer,
        };
        unsafe {
            instance.get(Peer::new)
        }
    }

    // string pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // string address = 3;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::string::String {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // uint64 bytes_sent = 4;

    pub fn clear_bytes_sent(&mut self) {
        self.bytes_sent = 0;
    }

    // Param is passed by value, moved
    pub fn set_bytes_sent(&mut self, v: u64) {
        self.bytes_sent = v;
    }

    pub fn get_bytes_sent(&self) -> u64 {
        self.bytes_sent
    }

    fn get_bytes_sent_for_reflect(&self) -> &u64 {
        &self.bytes_sent
    }

    fn mut_bytes_sent_for_reflect(&mut self) -> &mut u64 {
        &mut self.bytes_sent
    }

    // uint64 bytes_recv = 5;

    pub fn clear_bytes_recv(&mut self) {
        self.bytes_recv = 0;
    }

    // Param is passed by value, moved
    pub fn set_bytes_recv(&mut self, v: u64) {
        self.bytes_recv = v;
    }

    pub fn get_bytes_recv(&self) -> u64 {
        self.bytes_recv
    }

    fn get_bytes_recv_for_reflect(&self) -> &u64 {
        &self.bytes_recv
    }

    fn mut_bytes_recv_for_reflect(&mut self) -> &mut u64 {
        &mut self.bytes_recv
    }

    // int64 sat_sent = 6;

    pub fn clear_sat_sent(&mut self) {
        self.sat_sent = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_sent(&mut self, v: i64) {
        self.sat_sent = v;
    }

    pub fn get_sat_sent(&self) -> i64 {
        self.sat_sent
    }

    fn get_sat_sent_for_reflect(&self) -> &i64 {
        &self.sat_sent
    }

    fn mut_sat_sent_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_sent
    }

    // int64 sat_recv = 7;

    pub fn clear_sat_recv(&mut self) {
        self.sat_recv = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_recv(&mut self, v: i64) {
        self.sat_recv = v;
    }

    pub fn get_sat_recv(&self) -> i64 {
        self.sat_recv
    }

    fn get_sat_recv_for_reflect(&self) -> &i64 {
        &self.sat_recv
    }

    fn mut_sat_recv_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_recv
    }

    // bool inbound = 8;

    pub fn clear_inbound(&mut self) {
        self.inbound = false;
    }

    // Param is passed by value, moved
    pub fn set_inbound(&mut self, v: bool) {
        self.inbound = v;
    }

    pub fn get_inbound(&self) -> bool {
        self.inbound
    }

    fn get_inbound_for_reflect(&self) -> &bool {
        &self.inbound
    }

    fn mut_inbound_for_reflect(&mut self) -> &mut bool {
        &mut self.inbound
    }

    // int64 ping_time = 9;

    pub fn clear_ping_time(&mut self) {
        self.ping_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_ping_time(&mut self, v: i64) {
        self.ping_time = v;
    }

    pub fn get_ping_time(&self) -> i64 {
        self.ping_time
    }

    fn get_ping_time_for_reflect(&self) -> &i64 {
        &self.ping_time
    }

    fn mut_ping_time_for_reflect(&mut self) -> &mut i64 {
        &mut self.ping_time
    }
}

impl ::protobuf::Message for Peer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_sent = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_recv = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_sent = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_recv = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.inbound = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ping_time = tmp;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pub_key);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.address);
        }
        if self.bytes_sent != 0 {
            my_size += ::protobuf::rt::value_size(4, self.bytes_sent, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bytes_recv != 0 {
            my_size += ::protobuf::rt::value_size(5, self.bytes_recv, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_sent != 0 {
            my_size += ::protobuf::rt::value_size(6, self.sat_sent, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_recv != 0 {
            my_size += ::protobuf::rt::value_size(7, self.sat_recv, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.inbound != false {
            my_size += 2;
        }
        if self.ping_time != 0 {
            my_size += ::protobuf::rt::value_size(9, self.ping_time, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_string(1, &self.pub_key)?;
        }
        if !self.address.is_empty() {
            os.write_string(3, &self.address)?;
        }
        if self.bytes_sent != 0 {
            os.write_uint64(4, self.bytes_sent)?;
        }
        if self.bytes_recv != 0 {
            os.write_uint64(5, self.bytes_recv)?;
        }
        if self.sat_sent != 0 {
            os.write_int64(6, self.sat_sent)?;
        }
        if self.sat_recv != 0 {
            os.write_int64(7, self.sat_recv)?;
        }
        if self.inbound != false {
            os.write_bool(8, self.inbound)?;
        }
        if self.ping_time != 0 {
            os.write_int64(9, self.ping_time)?;
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

impl ::protobuf::MessageStatic for Peer {
    fn new() -> Peer {
        Peer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Peer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    Peer::get_pub_key_for_reflect,
                    Peer::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    Peer::get_address_for_reflect,
                    Peer::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_sent",
                    Peer::get_bytes_sent_for_reflect,
                    Peer::mut_bytes_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_recv",
                    Peer::get_bytes_recv_for_reflect,
                    Peer::mut_bytes_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_sent",
                    Peer::get_sat_sent_for_reflect,
                    Peer::mut_sat_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_recv",
                    Peer::get_sat_recv_for_reflect,
                    Peer::mut_sat_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "inbound",
                    Peer::get_inbound_for_reflect,
                    Peer::mut_inbound_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ping_time",
                    Peer::get_ping_time_for_reflect,
                    Peer::mut_ping_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Peer>(
                    "Peer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Peer {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.clear_address();
        self.clear_bytes_sent();
        self.clear_bytes_recv();
        self.clear_sat_sent();
        self.clear_sat_recv();
        self.clear_inbound();
        self.clear_ping_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Peer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListPeersRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListPeersRequest {}

impl ListPeersRequest {
    pub fn new() -> ListPeersRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListPeersRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListPeersRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListPeersRequest,
        };
        unsafe {
            instance.get(ListPeersRequest::new)
        }
    }
}

impl ::protobuf::Message for ListPeersRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ListPeersRequest {
    fn new() -> ListPeersRequest {
        ListPeersRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListPeersRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListPeersRequest>(
                    "ListPeersRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListPeersRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListPeersRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListPeersRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListPeersResponse {
    // message fields
    pub peers: ::protobuf::RepeatedField<Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListPeersResponse {}

impl ListPeersResponse {
    pub fn new() -> ListPeersResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListPeersResponse {
        static mut instance: ::protobuf::lazy::Lazy<ListPeersResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListPeersResponse,
        };
        unsafe {
            instance.get(ListPeersResponse::new)
        }
    }

    // repeated .lnrpc.Peer peers = 1;

    pub fn clear_peers(&mut self) {
        self.peers.clear();
    }

    // Param is passed by value, moved
    pub fn set_peers(&mut self, v: ::protobuf::RepeatedField<Peer>) {
        self.peers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peers(&mut self) -> &mut ::protobuf::RepeatedField<Peer> {
        &mut self.peers
    }

    // Take field
    pub fn take_peers(&mut self) -> ::protobuf::RepeatedField<Peer> {
        ::std::mem::replace(&mut self.peers, ::protobuf::RepeatedField::new())
    }

    pub fn get_peers(&self) -> &[Peer] {
        &self.peers
    }

    fn get_peers_for_reflect(&self) -> &::protobuf::RepeatedField<Peer> {
        &self.peers
    }

    fn mut_peers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Peer> {
        &mut self.peers
    }
}

impl ::protobuf::Message for ListPeersResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.peers {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.peers)?;
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
        for value in &self.peers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.peers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListPeersResponse {
    fn new() -> ListPeersResponse {
        ListPeersResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListPeersResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Peer>>(
                    "peers",
                    ListPeersResponse::get_peers_for_reflect,
                    ListPeersResponse::mut_peers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListPeersResponse>(
                    "ListPeersResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListPeersResponse {
    fn clear(&mut self) {
        self.clear_peers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListPeersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListPeersResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetInfoRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInfoRequest {}

impl GetInfoRequest {
    pub fn new() -> GetInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInfoRequest,
        };
        unsafe {
            instance.get(GetInfoRequest::new)
        }
    }
}

impl ::protobuf::Message for GetInfoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for GetInfoRequest {
    fn new() -> GetInfoRequest {
        GetInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetInfoRequest>(
                    "GetInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInfoRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetInfoResponse {
    // message fields
    pub identity_pubkey: ::std::string::String,
    pub alias: ::std::string::String,
    pub num_pending_channels: u32,
    pub num_active_channels: u32,
    pub num_peers: u32,
    pub block_height: u32,
    pub block_hash: ::std::string::String,
    pub synced_to_chain: bool,
    pub testnet: bool,
    pub chains: ::protobuf::RepeatedField<::std::string::String>,
    pub uris: ::protobuf::RepeatedField<::std::string::String>,
    pub best_header_timestamp: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInfoResponse {}

impl GetInfoResponse {
    pub fn new() -> GetInfoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInfoResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetInfoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInfoResponse,
        };
        unsafe {
            instance.get(GetInfoResponse::new)
        }
    }

    // string identity_pubkey = 1;

    pub fn clear_identity_pubkey(&mut self) {
        self.identity_pubkey.clear();
    }

    // Param is passed by value, moved
    pub fn set_identity_pubkey(&mut self, v: ::std::string::String) {
        self.identity_pubkey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identity_pubkey(&mut self) -> &mut ::std::string::String {
        &mut self.identity_pubkey
    }

    // Take field
    pub fn take_identity_pubkey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.identity_pubkey, ::std::string::String::new())
    }

    pub fn get_identity_pubkey(&self) -> &str {
        &self.identity_pubkey
    }

    fn get_identity_pubkey_for_reflect(&self) -> &::std::string::String {
        &self.identity_pubkey
    }

    fn mut_identity_pubkey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.identity_pubkey
    }

    // string alias = 2;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // uint32 num_pending_channels = 3;

    pub fn clear_num_pending_channels(&mut self) {
        self.num_pending_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_pending_channels(&mut self, v: u32) {
        self.num_pending_channels = v;
    }

    pub fn get_num_pending_channels(&self) -> u32 {
        self.num_pending_channels
    }

    fn get_num_pending_channels_for_reflect(&self) -> &u32 {
        &self.num_pending_channels
    }

    fn mut_num_pending_channels_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_pending_channels
    }

    // uint32 num_active_channels = 4;

    pub fn clear_num_active_channels(&mut self) {
        self.num_active_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_active_channels(&mut self, v: u32) {
        self.num_active_channels = v;
    }

    pub fn get_num_active_channels(&self) -> u32 {
        self.num_active_channels
    }

    fn get_num_active_channels_for_reflect(&self) -> &u32 {
        &self.num_active_channels
    }

    fn mut_num_active_channels_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_active_channels
    }

    // uint32 num_peers = 5;

    pub fn clear_num_peers(&mut self) {
        self.num_peers = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_peers(&mut self, v: u32) {
        self.num_peers = v;
    }

    pub fn get_num_peers(&self) -> u32 {
        self.num_peers
    }

    fn get_num_peers_for_reflect(&self) -> &u32 {
        &self.num_peers
    }

    fn mut_num_peers_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_peers
    }

    // uint32 block_height = 6;

    pub fn clear_block_height(&mut self) {
        self.block_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_height(&mut self, v: u32) {
        self.block_height = v;
    }

    pub fn get_block_height(&self) -> u32 {
        self.block_height
    }

    fn get_block_height_for_reflect(&self) -> &u32 {
        &self.block_height
    }

    fn mut_block_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.block_height
    }

    // string block_hash = 8;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::string::String) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::string::String {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.block_hash, ::std::string::String::new())
    }

    pub fn get_block_hash(&self) -> &str {
        &self.block_hash
    }

    fn get_block_hash_for_reflect(&self) -> &::std::string::String {
        &self.block_hash
    }

    fn mut_block_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.block_hash
    }

    // bool synced_to_chain = 9;

    pub fn clear_synced_to_chain(&mut self) {
        self.synced_to_chain = false;
    }

    // Param is passed by value, moved
    pub fn set_synced_to_chain(&mut self, v: bool) {
        self.synced_to_chain = v;
    }

    pub fn get_synced_to_chain(&self) -> bool {
        self.synced_to_chain
    }

    fn get_synced_to_chain_for_reflect(&self) -> &bool {
        &self.synced_to_chain
    }

    fn mut_synced_to_chain_for_reflect(&mut self) -> &mut bool {
        &mut self.synced_to_chain
    }

    // bool testnet = 10;

    pub fn clear_testnet(&mut self) {
        self.testnet = false;
    }

    // Param is passed by value, moved
    pub fn set_testnet(&mut self, v: bool) {
        self.testnet = v;
    }

    pub fn get_testnet(&self) -> bool {
        self.testnet
    }

    fn get_testnet_for_reflect(&self) -> &bool {
        &self.testnet
    }

    fn mut_testnet_for_reflect(&mut self) -> &mut bool {
        &mut self.testnet
    }

    // repeated string chains = 11;

    pub fn clear_chains(&mut self) {
        self.chains.clear();
    }

    // Param is passed by value, moved
    pub fn set_chains(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.chains = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chains(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.chains
    }

    // Take field
    pub fn take_chains(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.chains, ::protobuf::RepeatedField::new())
    }

    pub fn get_chains(&self) -> &[::std::string::String] {
        &self.chains
    }

    fn get_chains_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.chains
    }

    fn mut_chains_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.chains
    }

    // repeated string uris = 12;

    pub fn clear_uris(&mut self) {
        self.uris.clear();
    }

    // Param is passed by value, moved
    pub fn set_uris(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.uris = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uris(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.uris
    }

    // Take field
    pub fn take_uris(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.uris, ::protobuf::RepeatedField::new())
    }

    pub fn get_uris(&self) -> &[::std::string::String] {
        &self.uris
    }

    fn get_uris_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.uris
    }

    fn mut_uris_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.uris
    }

    // int64 best_header_timestamp = 13;

    pub fn clear_best_header_timestamp(&mut self) {
        self.best_header_timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_best_header_timestamp(&mut self, v: i64) {
        self.best_header_timestamp = v;
    }

    pub fn get_best_header_timestamp(&self) -> i64 {
        self.best_header_timestamp
    }

    fn get_best_header_timestamp_for_reflect(&self) -> &i64 {
        &self.best_header_timestamp
    }

    fn mut_best_header_timestamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.best_header_timestamp
    }
}

impl ::protobuf::Message for GetInfoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.identity_pubkey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_pending_channels = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_active_channels = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_peers = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.block_height = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.block_hash)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.synced_to_chain = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.testnet = tmp;
                },
                11 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.chains)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.uris)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.best_header_timestamp = tmp;
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
        if !self.identity_pubkey.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.identity_pubkey);
        }
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.alias);
        }
        if self.num_pending_channels != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_pending_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_active_channels != 0 {
            my_size += ::protobuf::rt::value_size(4, self.num_active_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_peers != 0 {
            my_size += ::protobuf::rt::value_size(5, self.num_peers, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.block_height != 0 {
            my_size += ::protobuf::rt::value_size(6, self.block_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.block_hash);
        }
        if self.synced_to_chain != false {
            my_size += 2;
        }
        if self.testnet != false {
            my_size += 2;
        }
        for value in &self.chains {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        for value in &self.uris {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        if self.best_header_timestamp != 0 {
            my_size += ::protobuf::rt::value_size(13, self.best_header_timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.identity_pubkey.is_empty() {
            os.write_string(1, &self.identity_pubkey)?;
        }
        if !self.alias.is_empty() {
            os.write_string(2, &self.alias)?;
        }
        if self.num_pending_channels != 0 {
            os.write_uint32(3, self.num_pending_channels)?;
        }
        if self.num_active_channels != 0 {
            os.write_uint32(4, self.num_active_channels)?;
        }
        if self.num_peers != 0 {
            os.write_uint32(5, self.num_peers)?;
        }
        if self.block_height != 0 {
            os.write_uint32(6, self.block_height)?;
        }
        if !self.block_hash.is_empty() {
            os.write_string(8, &self.block_hash)?;
        }
        if self.synced_to_chain != false {
            os.write_bool(9, self.synced_to_chain)?;
        }
        if self.testnet != false {
            os.write_bool(10, self.testnet)?;
        }
        for v in &self.chains {
            os.write_string(11, &v)?;
        };
        for v in &self.uris {
            os.write_string(12, &v)?;
        };
        if self.best_header_timestamp != 0 {
            os.write_int64(13, self.best_header_timestamp)?;
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

impl ::protobuf::MessageStatic for GetInfoResponse {
    fn new() -> GetInfoResponse {
        GetInfoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInfoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identity_pubkey",
                    GetInfoResponse::get_identity_pubkey_for_reflect,
                    GetInfoResponse::mut_identity_pubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    GetInfoResponse::get_alias_for_reflect,
                    GetInfoResponse::mut_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_pending_channels",
                    GetInfoResponse::get_num_pending_channels_for_reflect,
                    GetInfoResponse::mut_num_pending_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_active_channels",
                    GetInfoResponse::get_num_active_channels_for_reflect,
                    GetInfoResponse::mut_num_active_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_peers",
                    GetInfoResponse::get_num_peers_for_reflect,
                    GetInfoResponse::mut_num_peers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "block_height",
                    GetInfoResponse::get_block_height_for_reflect,
                    GetInfoResponse::mut_block_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "block_hash",
                    GetInfoResponse::get_block_hash_for_reflect,
                    GetInfoResponse::mut_block_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "synced_to_chain",
                    GetInfoResponse::get_synced_to_chain_for_reflect,
                    GetInfoResponse::mut_synced_to_chain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "testnet",
                    GetInfoResponse::get_testnet_for_reflect,
                    GetInfoResponse::mut_testnet_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chains",
                    GetInfoResponse::get_chains_for_reflect,
                    GetInfoResponse::mut_chains_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uris",
                    GetInfoResponse::get_uris_for_reflect,
                    GetInfoResponse::mut_uris_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "best_header_timestamp",
                    GetInfoResponse::get_best_header_timestamp_for_reflect,
                    GetInfoResponse::mut_best_header_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetInfoResponse>(
                    "GetInfoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInfoResponse {
    fn clear(&mut self) {
        self.clear_identity_pubkey();
        self.clear_alias();
        self.clear_num_pending_channels();
        self.clear_num_active_channels();
        self.clear_num_peers();
        self.clear_block_height();
        self.clear_block_hash();
        self.clear_synced_to_chain();
        self.clear_testnet();
        self.clear_chains();
        self.clear_uris();
        self.clear_best_header_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetInfoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetInfoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfirmationUpdate {
    // message fields
    pub block_sha: ::std::vec::Vec<u8>,
    pub block_height: i32,
    pub num_confs_left: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfirmationUpdate {}

impl ConfirmationUpdate {
    pub fn new() -> ConfirmationUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfirmationUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ConfirmationUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfirmationUpdate,
        };
        unsafe {
            instance.get(ConfirmationUpdate::new)
        }
    }

    // bytes block_sha = 1;

    pub fn clear_block_sha(&mut self) {
        self.block_sha.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_sha(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_sha = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_sha(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_sha
    }

    // Take field
    pub fn take_block_sha(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_sha, ::std::vec::Vec::new())
    }

    pub fn get_block_sha(&self) -> &[u8] {
        &self.block_sha
    }

    fn get_block_sha_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block_sha
    }

    fn mut_block_sha_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_sha
    }

    // int32 block_height = 2;

    pub fn clear_block_height(&mut self) {
        self.block_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_height(&mut self, v: i32) {
        self.block_height = v;
    }

    pub fn get_block_height(&self) -> i32 {
        self.block_height
    }

    fn get_block_height_for_reflect(&self) -> &i32 {
        &self.block_height
    }

    fn mut_block_height_for_reflect(&mut self) -> &mut i32 {
        &mut self.block_height
    }

    // uint32 num_confs_left = 3;

    pub fn clear_num_confs_left(&mut self) {
        self.num_confs_left = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_confs_left(&mut self, v: u32) {
        self.num_confs_left = v;
    }

    pub fn get_num_confs_left(&self) -> u32 {
        self.num_confs_left
    }

    fn get_num_confs_left_for_reflect(&self) -> &u32 {
        &self.num_confs_left
    }

    fn mut_num_confs_left_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_confs_left
    }
}

impl ::protobuf::Message for ConfirmationUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_sha)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.block_height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_confs_left = tmp;
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
        if !self.block_sha.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block_sha);
        }
        if self.block_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.block_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_confs_left != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_confs_left, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block_sha.is_empty() {
            os.write_bytes(1, &self.block_sha)?;
        }
        if self.block_height != 0 {
            os.write_int32(2, self.block_height)?;
        }
        if self.num_confs_left != 0 {
            os.write_uint32(3, self.num_confs_left)?;
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

impl ::protobuf::MessageStatic for ConfirmationUpdate {
    fn new() -> ConfirmationUpdate {
        ConfirmationUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfirmationUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_sha",
                    ConfirmationUpdate::get_block_sha_for_reflect,
                    ConfirmationUpdate::mut_block_sha_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_height",
                    ConfirmationUpdate::get_block_height_for_reflect,
                    ConfirmationUpdate::mut_block_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_confs_left",
                    ConfirmationUpdate::get_num_confs_left_for_reflect,
                    ConfirmationUpdate::mut_num_confs_left_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfirmationUpdate>(
                    "ConfirmationUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfirmationUpdate {
    fn clear(&mut self) {
        self.clear_block_sha();
        self.clear_block_height();
        self.clear_num_confs_left();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfirmationUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfirmationUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelOpenUpdate {
    // message fields
    pub channel_point: ::protobuf::SingularPtrField<ChannelPoint>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelOpenUpdate {}

impl ChannelOpenUpdate {
    pub fn new() -> ChannelOpenUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelOpenUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ChannelOpenUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelOpenUpdate,
        };
        unsafe {
            instance.get(ChannelOpenUpdate::new)
        }
    }

    // .lnrpc.ChannelPoint channel_point = 1;

    pub fn clear_channel_point(&mut self) {
        self.channel_point.clear();
    }

    pub fn has_channel_point(&self) -> bool {
        self.channel_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_point(&mut self, v: ChannelPoint) {
        self.channel_point = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_point(&mut self) -> &mut ChannelPoint {
        if self.channel_point.is_none() {
            self.channel_point.set_default();
        }
        self.channel_point.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_point(&mut self) -> ChannelPoint {
        self.channel_point.take().unwrap_or_else(|| ChannelPoint::new())
    }

    pub fn get_channel_point(&self) -> &ChannelPoint {
        self.channel_point.as_ref().unwrap_or_else(|| ChannelPoint::default_instance())
    }

    fn get_channel_point_for_reflect(&self) -> &::protobuf::SingularPtrField<ChannelPoint> {
        &self.channel_point
    }

    fn mut_channel_point_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChannelPoint> {
        &mut self.channel_point
    }
}

impl ::protobuf::Message for ChannelOpenUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.channel_point {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel_point)?;
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
        if let Some(ref v) = self.channel_point.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.channel_point.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ChannelOpenUpdate {
    fn new() -> ChannelOpenUpdate {
        ChannelOpenUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelOpenUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelPoint>>(
                    "channel_point",
                    ChannelOpenUpdate::get_channel_point_for_reflect,
                    ChannelOpenUpdate::mut_channel_point_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelOpenUpdate>(
                    "ChannelOpenUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelOpenUpdate {
    fn clear(&mut self) {
        self.clear_channel_point();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelOpenUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelOpenUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelCloseUpdate {
    // message fields
    pub closing_txid: ::std::vec::Vec<u8>,
    pub success: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelCloseUpdate {}

impl ChannelCloseUpdate {
    pub fn new() -> ChannelCloseUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelCloseUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ChannelCloseUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelCloseUpdate,
        };
        unsafe {
            instance.get(ChannelCloseUpdate::new)
        }
    }

    // bytes closing_txid = 1;

    pub fn clear_closing_txid(&mut self) {
        self.closing_txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_closing_txid(&mut self, v: ::std::vec::Vec<u8>) {
        self.closing_txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_closing_txid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.closing_txid
    }

    // Take field
    pub fn take_closing_txid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.closing_txid, ::std::vec::Vec::new())
    }

    pub fn get_closing_txid(&self) -> &[u8] {
        &self.closing_txid
    }

    fn get_closing_txid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.closing_txid
    }

    fn mut_closing_txid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.closing_txid
    }

    // bool success = 2;

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    pub fn get_success(&self) -> bool {
        self.success
    }

    fn get_success_for_reflect(&self) -> &bool {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut bool {
        &mut self.success
    }
}

impl ::protobuf::Message for ChannelCloseUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.closing_txid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = tmp;
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
        if !self.closing_txid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.closing_txid);
        }
        if self.success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.closing_txid.is_empty() {
            os.write_bytes(1, &self.closing_txid)?;
        }
        if self.success != false {
            os.write_bool(2, self.success)?;
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

impl ::protobuf::MessageStatic for ChannelCloseUpdate {
    fn new() -> ChannelCloseUpdate {
        ChannelCloseUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelCloseUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "closing_txid",
                    ChannelCloseUpdate::get_closing_txid_for_reflect,
                    ChannelCloseUpdate::mut_closing_txid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    ChannelCloseUpdate::get_success_for_reflect,
                    ChannelCloseUpdate::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelCloseUpdate>(
                    "ChannelCloseUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelCloseUpdate {
    fn clear(&mut self) {
        self.clear_closing_txid();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelCloseUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelCloseUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseChannelRequest {
    // message fields
    pub channel_point: ::protobuf::SingularPtrField<ChannelPoint>,
    pub force: bool,
    pub target_conf: i32,
    pub sat_per_byte: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseChannelRequest {}

impl CloseChannelRequest {
    pub fn new() -> CloseChannelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseChannelRequest {
        static mut instance: ::protobuf::lazy::Lazy<CloseChannelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseChannelRequest,
        };
        unsafe {
            instance.get(CloseChannelRequest::new)
        }
    }

    // .lnrpc.ChannelPoint channel_point = 1;

    pub fn clear_channel_point(&mut self) {
        self.channel_point.clear();
    }

    pub fn has_channel_point(&self) -> bool {
        self.channel_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_point(&mut self, v: ChannelPoint) {
        self.channel_point = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_point(&mut self) -> &mut ChannelPoint {
        if self.channel_point.is_none() {
            self.channel_point.set_default();
        }
        self.channel_point.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_point(&mut self) -> ChannelPoint {
        self.channel_point.take().unwrap_or_else(|| ChannelPoint::new())
    }

    pub fn get_channel_point(&self) -> &ChannelPoint {
        self.channel_point.as_ref().unwrap_or_else(|| ChannelPoint::default_instance())
    }

    fn get_channel_point_for_reflect(&self) -> &::protobuf::SingularPtrField<ChannelPoint> {
        &self.channel_point
    }

    fn mut_channel_point_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChannelPoint> {
        &mut self.channel_point
    }

    // bool force = 2;

    pub fn clear_force(&mut self) {
        self.force = false;
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: bool) {
        self.force = v;
    }

    pub fn get_force(&self) -> bool {
        self.force
    }

    fn get_force_for_reflect(&self) -> &bool {
        &self.force
    }

    fn mut_force_for_reflect(&mut self) -> &mut bool {
        &mut self.force
    }

    // int32 target_conf = 3;

    pub fn clear_target_conf(&mut self) {
        self.target_conf = 0;
    }

    // Param is passed by value, moved
    pub fn set_target_conf(&mut self, v: i32) {
        self.target_conf = v;
    }

    pub fn get_target_conf(&self) -> i32 {
        self.target_conf
    }

    fn get_target_conf_for_reflect(&self) -> &i32 {
        &self.target_conf
    }

    fn mut_target_conf_for_reflect(&mut self) -> &mut i32 {
        &mut self.target_conf
    }

    // int64 sat_per_byte = 4;

    pub fn clear_sat_per_byte(&mut self) {
        self.sat_per_byte = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_per_byte(&mut self, v: i64) {
        self.sat_per_byte = v;
    }

    pub fn get_sat_per_byte(&self) -> i64 {
        self.sat_per_byte
    }

    fn get_sat_per_byte_for_reflect(&self) -> &i64 {
        &self.sat_per_byte
    }

    fn mut_sat_per_byte_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_per_byte
    }
}

impl ::protobuf::Message for CloseChannelRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.channel_point {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel_point)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.force = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target_conf = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_per_byte = tmp;
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
        if let Some(ref v) = self.channel_point.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.force != false {
            my_size += 2;
        }
        if self.target_conf != 0 {
            my_size += ::protobuf::rt::value_size(3, self.target_conf, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_per_byte != 0 {
            my_size += ::protobuf::rt::value_size(4, self.sat_per_byte, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.channel_point.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.force != false {
            os.write_bool(2, self.force)?;
        }
        if self.target_conf != 0 {
            os.write_int32(3, self.target_conf)?;
        }
        if self.sat_per_byte != 0 {
            os.write_int64(4, self.sat_per_byte)?;
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

impl ::protobuf::MessageStatic for CloseChannelRequest {
    fn new() -> CloseChannelRequest {
        CloseChannelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseChannelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelPoint>>(
                    "channel_point",
                    CloseChannelRequest::get_channel_point_for_reflect,
                    CloseChannelRequest::mut_channel_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "force",
                    CloseChannelRequest::get_force_for_reflect,
                    CloseChannelRequest::mut_force_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target_conf",
                    CloseChannelRequest::get_target_conf_for_reflect,
                    CloseChannelRequest::mut_target_conf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_per_byte",
                    CloseChannelRequest::get_sat_per_byte_for_reflect,
                    CloseChannelRequest::mut_sat_per_byte_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseChannelRequest>(
                    "CloseChannelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseChannelRequest {
    fn clear(&mut self) {
        self.clear_channel_point();
        self.clear_force();
        self.clear_target_conf();
        self.clear_sat_per_byte();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseChannelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseChannelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseStatusUpdate {
    // message oneof groups
    update: ::std::option::Option<CloseStatusUpdate_oneof_update>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseStatusUpdate {}

#[derive(Clone,PartialEq)]
pub enum CloseStatusUpdate_oneof_update {
    close_pending(PendingUpdate),
    confirmation(ConfirmationUpdate),
    chan_close(ChannelCloseUpdate),
}

impl CloseStatusUpdate {
    pub fn new() -> CloseStatusUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseStatusUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CloseStatusUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseStatusUpdate,
        };
        unsafe {
            instance.get(CloseStatusUpdate::new)
        }
    }

    // .lnrpc.PendingUpdate close_pending = 1;

    pub fn clear_close_pending(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_close_pending(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_close_pending(&mut self, v: PendingUpdate) {
        self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(v))
    }

    // Mutable pointer to the field.
    pub fn mut_close_pending(&mut self) -> &mut PendingUpdate {
        if let ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(PendingUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_close_pending(&mut self) -> PendingUpdate {
        if self.has_close_pending() {
            match self.update.take() {
                ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(v)) => v,
                _ => panic!(),
            }
        } else {
            PendingUpdate::new()
        }
    }

    pub fn get_close_pending(&self) -> &PendingUpdate {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(ref v)) => v,
            _ => PendingUpdate::default_instance(),
        }
    }

    // .lnrpc.ConfirmationUpdate confirmation = 2;

    pub fn clear_confirmation(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_confirmation(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_confirmation(&mut self, v: ConfirmationUpdate) {
        self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(v))
    }

    // Mutable pointer to the field.
    pub fn mut_confirmation(&mut self) -> &mut ConfirmationUpdate {
        if let ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(ConfirmationUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_confirmation(&mut self) -> ConfirmationUpdate {
        if self.has_confirmation() {
            match self.update.take() {
                ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(v)) => v,
                _ => panic!(),
            }
        } else {
            ConfirmationUpdate::new()
        }
    }

    pub fn get_confirmation(&self) -> &ConfirmationUpdate {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(ref v)) => v,
            _ => ConfirmationUpdate::default_instance(),
        }
    }

    // .lnrpc.ChannelCloseUpdate chan_close = 3;

    pub fn clear_chan_close(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_chan_close(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chan_close(&mut self, v: ChannelCloseUpdate) {
        self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chan_close(&mut self) -> &mut ChannelCloseUpdate {
        if let ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(ChannelCloseUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chan_close(&mut self) -> ChannelCloseUpdate {
        if self.has_chan_close() {
            match self.update.take() {
                ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(v)) => v,
                _ => panic!(),
            }
        } else {
            ChannelCloseUpdate::new()
        }
    }

    pub fn get_chan_close(&self) -> &ChannelCloseUpdate {
        match self.update {
            ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(ref v)) => v,
            _ => ChannelCloseUpdate::default_instance(),
        }
    }
}

impl ::protobuf::Message for CloseStatusUpdate {
    fn is_initialized(&self) -> bool {
        if let Some(CloseStatusUpdate_oneof_update::close_pending(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CloseStatusUpdate_oneof_update::confirmation(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CloseStatusUpdate_oneof_update::chan_close(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::close_pending(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::confirmation(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(CloseStatusUpdate_oneof_update::chan_close(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.update {
            match v {
                &CloseStatusUpdate_oneof_update::close_pending(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CloseStatusUpdate_oneof_update::confirmation(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CloseStatusUpdate_oneof_update::chan_close(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.update {
            match v {
                &CloseStatusUpdate_oneof_update::close_pending(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CloseStatusUpdate_oneof_update::confirmation(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CloseStatusUpdate_oneof_update::chan_close(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for CloseStatusUpdate {
    fn new() -> CloseStatusUpdate {
        CloseStatusUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseStatusUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PendingUpdate>(
                    "close_pending",
                    CloseStatusUpdate::has_close_pending,
                    CloseStatusUpdate::get_close_pending,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConfirmationUpdate>(
                    "confirmation",
                    CloseStatusUpdate::has_confirmation,
                    CloseStatusUpdate::get_confirmation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ChannelCloseUpdate>(
                    "chan_close",
                    CloseStatusUpdate::has_chan_close,
                    CloseStatusUpdate::get_chan_close,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseStatusUpdate>(
                    "CloseStatusUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseStatusUpdate {
    fn clear(&mut self) {
        self.clear_close_pending();
        self.clear_confirmation();
        self.clear_chan_close();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseStatusUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseStatusUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingUpdate {
    // message fields
    pub txid: ::std::vec::Vec<u8>,
    pub output_index: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingUpdate {}

impl PendingUpdate {
    pub fn new() -> PendingUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingUpdate {
        static mut instance: ::protobuf::lazy::Lazy<PendingUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingUpdate,
        };
        unsafe {
            instance.get(PendingUpdate::new)
        }
    }

    // bytes txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: ::std::vec::Vec<u8>) {
        self.txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.txid
    }

    // Take field
    pub fn take_txid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.txid, ::std::vec::Vec::new())
    }

    pub fn get_txid(&self) -> &[u8] {
        &self.txid
    }

    fn get_txid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.txid
    }

    fn mut_txid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.txid
    }

    // uint32 output_index = 2;

    pub fn clear_output_index(&mut self) {
        self.output_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_output_index(&mut self, v: u32) {
        self.output_index = v;
    }

    pub fn get_output_index(&self) -> u32 {
        self.output_index
    }

    fn get_output_index_for_reflect(&self) -> &u32 {
        &self.output_index
    }

    fn mut_output_index_for_reflect(&mut self) -> &mut u32 {
        &mut self.output_index
    }
}

impl ::protobuf::Message for PendingUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.txid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.output_index = tmp;
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
        if !self.txid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.txid);
        }
        if self.output_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.output_index, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.txid.is_empty() {
            os.write_bytes(1, &self.txid)?;
        }
        if self.output_index != 0 {
            os.write_uint32(2, self.output_index)?;
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

impl ::protobuf::MessageStatic for PendingUpdate {
    fn new() -> PendingUpdate {
        PendingUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "txid",
                    PendingUpdate::get_txid_for_reflect,
                    PendingUpdate::mut_txid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "output_index",
                    PendingUpdate::get_output_index_for_reflect,
                    PendingUpdate::mut_output_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingUpdate>(
                    "PendingUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingUpdate {
    fn clear(&mut self) {
        self.clear_txid();
        self.clear_output_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpenChannelRequest {
    // message fields
    pub node_pubkey: ::std::vec::Vec<u8>,
    pub node_pubkey_string: ::std::string::String,
    pub local_funding_amount: i64,
    pub push_sat: i64,
    pub target_conf: i32,
    pub sat_per_byte: i64,
    pub private: bool,
    pub min_htlc_msat: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpenChannelRequest {}

impl OpenChannelRequest {
    pub fn new() -> OpenChannelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpenChannelRequest {
        static mut instance: ::protobuf::lazy::Lazy<OpenChannelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpenChannelRequest,
        };
        unsafe {
            instance.get(OpenChannelRequest::new)
        }
    }

    // bytes node_pubkey = 2;

    pub fn clear_node_pubkey(&mut self) {
        self.node_pubkey.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_pubkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.node_pubkey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_pubkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.node_pubkey
    }

    // Take field
    pub fn take_node_pubkey(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.node_pubkey, ::std::vec::Vec::new())
    }

    pub fn get_node_pubkey(&self) -> &[u8] {
        &self.node_pubkey
    }

    fn get_node_pubkey_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.node_pubkey
    }

    fn mut_node_pubkey_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.node_pubkey
    }

    // string node_pubkey_string = 3;

    pub fn clear_node_pubkey_string(&mut self) {
        self.node_pubkey_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_pubkey_string(&mut self, v: ::std::string::String) {
        self.node_pubkey_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_pubkey_string(&mut self) -> &mut ::std::string::String {
        &mut self.node_pubkey_string
    }

    // Take field
    pub fn take_node_pubkey_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node_pubkey_string, ::std::string::String::new())
    }

    pub fn get_node_pubkey_string(&self) -> &str {
        &self.node_pubkey_string
    }

    fn get_node_pubkey_string_for_reflect(&self) -> &::std::string::String {
        &self.node_pubkey_string
    }

    fn mut_node_pubkey_string_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node_pubkey_string
    }

    // int64 local_funding_amount = 4;

    pub fn clear_local_funding_amount(&mut self) {
        self.local_funding_amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_local_funding_amount(&mut self, v: i64) {
        self.local_funding_amount = v;
    }

    pub fn get_local_funding_amount(&self) -> i64 {
        self.local_funding_amount
    }

    fn get_local_funding_amount_for_reflect(&self) -> &i64 {
        &self.local_funding_amount
    }

    fn mut_local_funding_amount_for_reflect(&mut self) -> &mut i64 {
        &mut self.local_funding_amount
    }

    // int64 push_sat = 5;

    pub fn clear_push_sat(&mut self) {
        self.push_sat = 0;
    }

    // Param is passed by value, moved
    pub fn set_push_sat(&mut self, v: i64) {
        self.push_sat = v;
    }

    pub fn get_push_sat(&self) -> i64 {
        self.push_sat
    }

    fn get_push_sat_for_reflect(&self) -> &i64 {
        &self.push_sat
    }

    fn mut_push_sat_for_reflect(&mut self) -> &mut i64 {
        &mut self.push_sat
    }

    // int32 target_conf = 6;

    pub fn clear_target_conf(&mut self) {
        self.target_conf = 0;
    }

    // Param is passed by value, moved
    pub fn set_target_conf(&mut self, v: i32) {
        self.target_conf = v;
    }

    pub fn get_target_conf(&self) -> i32 {
        self.target_conf
    }

    fn get_target_conf_for_reflect(&self) -> &i32 {
        &self.target_conf
    }

    fn mut_target_conf_for_reflect(&mut self) -> &mut i32 {
        &mut self.target_conf
    }

    // int64 sat_per_byte = 7;

    pub fn clear_sat_per_byte(&mut self) {
        self.sat_per_byte = 0;
    }

    // Param is passed by value, moved
    pub fn set_sat_per_byte(&mut self, v: i64) {
        self.sat_per_byte = v;
    }

    pub fn get_sat_per_byte(&self) -> i64 {
        self.sat_per_byte
    }

    fn get_sat_per_byte_for_reflect(&self) -> &i64 {
        &self.sat_per_byte
    }

    fn mut_sat_per_byte_for_reflect(&mut self) -> &mut i64 {
        &mut self.sat_per_byte
    }

    // bool private = 8;

    pub fn clear_private(&mut self) {
        self.private = false;
    }

    // Param is passed by value, moved
    pub fn set_private(&mut self, v: bool) {
        self.private = v;
    }

    pub fn get_private(&self) -> bool {
        self.private
    }

    fn get_private_for_reflect(&self) -> &bool {
        &self.private
    }

    fn mut_private_for_reflect(&mut self) -> &mut bool {
        &mut self.private
    }

    // int64 min_htlc_msat = 9;

    pub fn clear_min_htlc_msat(&mut self) {
        self.min_htlc_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_htlc_msat(&mut self, v: i64) {
        self.min_htlc_msat = v;
    }

    pub fn get_min_htlc_msat(&self) -> i64 {
        self.min_htlc_msat
    }

    fn get_min_htlc_msat_for_reflect(&self) -> &i64 {
        &self.min_htlc_msat
    }

    fn mut_min_htlc_msat_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_htlc_msat
    }
}

impl ::protobuf::Message for OpenChannelRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.node_pubkey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_pubkey_string)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.local_funding_amount = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.push_sat = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target_conf = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.sat_per_byte = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.private = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.min_htlc_msat = tmp;
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
        if !self.node_pubkey.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.node_pubkey);
        }
        if !self.node_pubkey_string.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.node_pubkey_string);
        }
        if self.local_funding_amount != 0 {
            my_size += ::protobuf::rt::value_size(4, self.local_funding_amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.push_sat != 0 {
            my_size += ::protobuf::rt::value_size(5, self.push_sat, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.target_conf != 0 {
            my_size += ::protobuf::rt::value_size(6, self.target_conf, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sat_per_byte != 0 {
            my_size += ::protobuf::rt::value_size(7, self.sat_per_byte, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.private != false {
            my_size += 2;
        }
        if self.min_htlc_msat != 0 {
            my_size += ::protobuf::rt::value_size(9, self.min_htlc_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.node_pubkey.is_empty() {
            os.write_bytes(2, &self.node_pubkey)?;
        }
        if !self.node_pubkey_string.is_empty() {
            os.write_string(3, &self.node_pubkey_string)?;
        }
        if self.local_funding_amount != 0 {
            os.write_int64(4, self.local_funding_amount)?;
        }
        if self.push_sat != 0 {
            os.write_int64(5, self.push_sat)?;
        }
        if self.target_conf != 0 {
            os.write_int32(6, self.target_conf)?;
        }
        if self.sat_per_byte != 0 {
            os.write_int64(7, self.sat_per_byte)?;
        }
        if self.private != false {
            os.write_bool(8, self.private)?;
        }
        if self.min_htlc_msat != 0 {
            os.write_int64(9, self.min_htlc_msat)?;
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

impl ::protobuf::MessageStatic for OpenChannelRequest {
    fn new() -> OpenChannelRequest {
        OpenChannelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpenChannelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "node_pubkey",
                    OpenChannelRequest::get_node_pubkey_for_reflect,
                    OpenChannelRequest::mut_node_pubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_pubkey_string",
                    OpenChannelRequest::get_node_pubkey_string_for_reflect,
                    OpenChannelRequest::mut_node_pubkey_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "local_funding_amount",
                    OpenChannelRequest::get_local_funding_amount_for_reflect,
                    OpenChannelRequest::mut_local_funding_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "push_sat",
                    OpenChannelRequest::get_push_sat_for_reflect,
                    OpenChannelRequest::mut_push_sat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target_conf",
                    OpenChannelRequest::get_target_conf_for_reflect,
                    OpenChannelRequest::mut_target_conf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sat_per_byte",
                    OpenChannelRequest::get_sat_per_byte_for_reflect,
                    OpenChannelRequest::mut_sat_per_byte_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "private",
                    OpenChannelRequest::get_private_for_reflect,
                    OpenChannelRequest::mut_private_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_htlc_msat",
                    OpenChannelRequest::get_min_htlc_msat_for_reflect,
                    OpenChannelRequest::mut_min_htlc_msat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpenChannelRequest>(
                    "OpenChannelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpenChannelRequest {
    fn clear(&mut self) {
        self.clear_node_pubkey();
        self.clear_node_pubkey_string();
        self.clear_local_funding_amount();
        self.clear_push_sat();
        self.clear_target_conf();
        self.clear_sat_per_byte();
        self.clear_private();
        self.clear_min_htlc_msat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpenChannelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpenChannelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpenStatusUpdate {
    // message oneof groups
    update: ::std::option::Option<OpenStatusUpdate_oneof_update>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpenStatusUpdate {}

#[derive(Clone,PartialEq)]
pub enum OpenStatusUpdate_oneof_update {
    chan_pending(PendingUpdate),
    confirmation(ConfirmationUpdate),
    chan_open(ChannelOpenUpdate),
}

impl OpenStatusUpdate {
    pub fn new() -> OpenStatusUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpenStatusUpdate {
        static mut instance: ::protobuf::lazy::Lazy<OpenStatusUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpenStatusUpdate,
        };
        unsafe {
            instance.get(OpenStatusUpdate::new)
        }
    }

    // .lnrpc.PendingUpdate chan_pending = 1;

    pub fn clear_chan_pending(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_chan_pending(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chan_pending(&mut self, v: PendingUpdate) {
        self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chan_pending(&mut self) -> &mut PendingUpdate {
        if let ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(PendingUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chan_pending(&mut self) -> PendingUpdate {
        if self.has_chan_pending() {
            match self.update.take() {
                ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(v)) => v,
                _ => panic!(),
            }
        } else {
            PendingUpdate::new()
        }
    }

    pub fn get_chan_pending(&self) -> &PendingUpdate {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(ref v)) => v,
            _ => PendingUpdate::default_instance(),
        }
    }

    // .lnrpc.ConfirmationUpdate confirmation = 2;

    pub fn clear_confirmation(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_confirmation(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_confirmation(&mut self, v: ConfirmationUpdate) {
        self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(v))
    }

    // Mutable pointer to the field.
    pub fn mut_confirmation(&mut self) -> &mut ConfirmationUpdate {
        if let ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(ConfirmationUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_confirmation(&mut self) -> ConfirmationUpdate {
        if self.has_confirmation() {
            match self.update.take() {
                ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(v)) => v,
                _ => panic!(),
            }
        } else {
            ConfirmationUpdate::new()
        }
    }

    pub fn get_confirmation(&self) -> &ConfirmationUpdate {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(ref v)) => v,
            _ => ConfirmationUpdate::default_instance(),
        }
    }

    // .lnrpc.ChannelOpenUpdate chan_open = 3;

    pub fn clear_chan_open(&mut self) {
        self.update = ::std::option::Option::None;
    }

    pub fn has_chan_open(&self) -> bool {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chan_open(&mut self, v: ChannelOpenUpdate) {
        self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chan_open(&mut self) -> &mut ChannelOpenUpdate {
        if let ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(_)) = self.update {
        } else {
            self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(ChannelOpenUpdate::new()));
        }
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chan_open(&mut self) -> ChannelOpenUpdate {
        if self.has_chan_open() {
            match self.update.take() {
                ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(v)) => v,
                _ => panic!(),
            }
        } else {
            ChannelOpenUpdate::new()
        }
    }

    pub fn get_chan_open(&self) -> &ChannelOpenUpdate {
        match self.update {
            ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(ref v)) => v,
            _ => ChannelOpenUpdate::default_instance(),
        }
    }
}

impl ::protobuf::Message for OpenStatusUpdate {
    fn is_initialized(&self) -> bool {
        if let Some(OpenStatusUpdate_oneof_update::chan_pending(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(OpenStatusUpdate_oneof_update::confirmation(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(OpenStatusUpdate_oneof_update::chan_open(ref v)) = self.update {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_pending(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::confirmation(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.update = ::std::option::Option::Some(OpenStatusUpdate_oneof_update::chan_open(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.update {
            match v {
                &OpenStatusUpdate_oneof_update::chan_pending(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &OpenStatusUpdate_oneof_update::confirmation(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &OpenStatusUpdate_oneof_update::chan_open(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.update {
            match v {
                &OpenStatusUpdate_oneof_update::chan_pending(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &OpenStatusUpdate_oneof_update::confirmation(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &OpenStatusUpdate_oneof_update::chan_open(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for OpenStatusUpdate {
    fn new() -> OpenStatusUpdate {
        OpenStatusUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpenStatusUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PendingUpdate>(
                    "chan_pending",
                    OpenStatusUpdate::has_chan_pending,
                    OpenStatusUpdate::get_chan_pending,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConfirmationUpdate>(
                    "confirmation",
                    OpenStatusUpdate::has_confirmation,
                    OpenStatusUpdate::get_confirmation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ChannelOpenUpdate>(
                    "chan_open",
                    OpenStatusUpdate::has_chan_open,
                    OpenStatusUpdate::get_chan_open,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpenStatusUpdate>(
                    "OpenStatusUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpenStatusUpdate {
    fn clear(&mut self) {
        self.clear_chan_pending();
        self.clear_confirmation();
        self.clear_chan_open();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpenStatusUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpenStatusUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingHTLC {
    // message fields
    pub incoming: bool,
    pub amount: i64,
    pub outpoint: ::std::string::String,
    pub maturity_height: u32,
    pub blocks_til_maturity: i32,
    pub stage: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingHTLC {}

impl PendingHTLC {
    pub fn new() -> PendingHTLC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingHTLC {
        static mut instance: ::protobuf::lazy::Lazy<PendingHTLC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingHTLC,
        };
        unsafe {
            instance.get(PendingHTLC::new)
        }
    }

    // bool incoming = 1;

    pub fn clear_incoming(&mut self) {
        self.incoming = false;
    }

    // Param is passed by value, moved
    pub fn set_incoming(&mut self, v: bool) {
        self.incoming = v;
    }

    pub fn get_incoming(&self) -> bool {
        self.incoming
    }

    fn get_incoming_for_reflect(&self) -> &bool {
        &self.incoming
    }

    fn mut_incoming_for_reflect(&mut self) -> &mut bool {
        &mut self.incoming
    }

    // int64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i64 {
        &mut self.amount
    }

    // string outpoint = 3;

    pub fn clear_outpoint(&mut self) {
        self.outpoint.clear();
    }

    // Param is passed by value, moved
    pub fn set_outpoint(&mut self, v: ::std::string::String) {
        self.outpoint = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outpoint(&mut self) -> &mut ::std::string::String {
        &mut self.outpoint
    }

    // Take field
    pub fn take_outpoint(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.outpoint, ::std::string::String::new())
    }

    pub fn get_outpoint(&self) -> &str {
        &self.outpoint
    }

    fn get_outpoint_for_reflect(&self) -> &::std::string::String {
        &self.outpoint
    }

    fn mut_outpoint_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.outpoint
    }

    // uint32 maturity_height = 4;

    pub fn clear_maturity_height(&mut self) {
        self.maturity_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_maturity_height(&mut self, v: u32) {
        self.maturity_height = v;
    }

    pub fn get_maturity_height(&self) -> u32 {
        self.maturity_height
    }

    fn get_maturity_height_for_reflect(&self) -> &u32 {
        &self.maturity_height
    }

    fn mut_maturity_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.maturity_height
    }

    // int32 blocks_til_maturity = 5;

    pub fn clear_blocks_til_maturity(&mut self) {
        self.blocks_til_maturity = 0;
    }

    // Param is passed by value, moved
    pub fn set_blocks_til_maturity(&mut self, v: i32) {
        self.blocks_til_maturity = v;
    }

    pub fn get_blocks_til_maturity(&self) -> i32 {
        self.blocks_til_maturity
    }

    fn get_blocks_til_maturity_for_reflect(&self) -> &i32 {
        &self.blocks_til_maturity
    }

    fn mut_blocks_til_maturity_for_reflect(&mut self) -> &mut i32 {
        &mut self.blocks_til_maturity
    }

    // uint32 stage = 6;

    pub fn clear_stage(&mut self) {
        self.stage = 0;
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: u32) {
        self.stage = v;
    }

    pub fn get_stage(&self) -> u32 {
        self.stage
    }

    fn get_stage_for_reflect(&self) -> &u32 {
        &self.stage
    }

    fn mut_stage_for_reflect(&mut self) -> &mut u32 {
        &mut self.stage
    }
}

impl ::protobuf::Message for PendingHTLC {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.incoming = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amount = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.outpoint)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maturity_height = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.blocks_til_maturity = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stage = tmp;
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
        if self.incoming != false {
            my_size += 2;
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.outpoint.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.outpoint);
        }
        if self.maturity_height != 0 {
            my_size += ::protobuf::rt::value_size(4, self.maturity_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.blocks_til_maturity != 0 {
            my_size += ::protobuf::rt::value_size(5, self.blocks_til_maturity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.stage != 0 {
            my_size += ::protobuf::rt::value_size(6, self.stage, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.incoming != false {
            os.write_bool(1, self.incoming)?;
        }
        if self.amount != 0 {
            os.write_int64(2, self.amount)?;
        }
        if !self.outpoint.is_empty() {
            os.write_string(3, &self.outpoint)?;
        }
        if self.maturity_height != 0 {
            os.write_uint32(4, self.maturity_height)?;
        }
        if self.blocks_til_maturity != 0 {
            os.write_int32(5, self.blocks_til_maturity)?;
        }
        if self.stage != 0 {
            os.write_uint32(6, self.stage)?;
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

impl ::protobuf::MessageStatic for PendingHTLC {
    fn new() -> PendingHTLC {
        PendingHTLC::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingHTLC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "incoming",
                    PendingHTLC::get_incoming_for_reflect,
                    PendingHTLC::mut_incoming_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amount",
                    PendingHTLC::get_amount_for_reflect,
                    PendingHTLC::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "outpoint",
                    PendingHTLC::get_outpoint_for_reflect,
                    PendingHTLC::mut_outpoint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maturity_height",
                    PendingHTLC::get_maturity_height_for_reflect,
                    PendingHTLC::mut_maturity_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "blocks_til_maturity",
                    PendingHTLC::get_blocks_til_maturity_for_reflect,
                    PendingHTLC::mut_blocks_til_maturity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stage",
                    PendingHTLC::get_stage_for_reflect,
                    PendingHTLC::mut_stage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingHTLC>(
                    "PendingHTLC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingHTLC {
    fn clear(&mut self) {
        self.clear_incoming();
        self.clear_amount();
        self.clear_outpoint();
        self.clear_maturity_height();
        self.clear_blocks_til_maturity();
        self.clear_stage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingHTLC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingHTLC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsRequest {}

impl PendingChannelsRequest {
    pub fn new() -> PendingChannelsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsRequest {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsRequest,
        };
        unsafe {
            instance.get(PendingChannelsRequest::new)
        }
    }
}

impl ::protobuf::Message for PendingChannelsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for PendingChannelsRequest {
    fn new() -> PendingChannelsRequest {
        PendingChannelsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsRequest>(
                    "PendingChannelsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsResponse {
    // message fields
    pub total_limbo_balance: i64,
    pub pending_open_channels: ::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel>,
    pub pending_closing_channels: ::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel>,
    pub pending_force_closing_channels: ::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsResponse {}

impl PendingChannelsResponse {
    pub fn new() -> PendingChannelsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsResponse {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsResponse,
        };
        unsafe {
            instance.get(PendingChannelsResponse::new)
        }
    }

    // int64 total_limbo_balance = 1;

    pub fn clear_total_limbo_balance(&mut self) {
        self.total_limbo_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_limbo_balance(&mut self, v: i64) {
        self.total_limbo_balance = v;
    }

    pub fn get_total_limbo_balance(&self) -> i64 {
        self.total_limbo_balance
    }

    fn get_total_limbo_balance_for_reflect(&self) -> &i64 {
        &self.total_limbo_balance
    }

    fn mut_total_limbo_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_limbo_balance
    }

    // repeated .lnrpc.PendingChannelsResponse.PendingOpenChannel pending_open_channels = 2;

    pub fn clear_pending_open_channels(&mut self) {
        self.pending_open_channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_open_channels(&mut self, v: ::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel>) {
        self.pending_open_channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_open_channels(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel> {
        &mut self.pending_open_channels
    }

    // Take field
    pub fn take_pending_open_channels(&mut self) -> ::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel> {
        ::std::mem::replace(&mut self.pending_open_channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_open_channels(&self) -> &[PendingChannelsResponse_PendingOpenChannel] {
        &self.pending_open_channels
    }

    fn get_pending_open_channels_for_reflect(&self) -> &::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel> {
        &self.pending_open_channels
    }

    fn mut_pending_open_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_PendingOpenChannel> {
        &mut self.pending_open_channels
    }

    // repeated .lnrpc.PendingChannelsResponse.ClosedChannel pending_closing_channels = 3;

    pub fn clear_pending_closing_channels(&mut self) {
        self.pending_closing_channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_closing_channels(&mut self, v: ::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel>) {
        self.pending_closing_channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_closing_channels(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel> {
        &mut self.pending_closing_channels
    }

    // Take field
    pub fn take_pending_closing_channels(&mut self) -> ::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel> {
        ::std::mem::replace(&mut self.pending_closing_channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_closing_channels(&self) -> &[PendingChannelsResponse_ClosedChannel] {
        &self.pending_closing_channels
    }

    fn get_pending_closing_channels_for_reflect(&self) -> &::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel> {
        &self.pending_closing_channels
    }

    fn mut_pending_closing_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_ClosedChannel> {
        &mut self.pending_closing_channels
    }

    // repeated .lnrpc.PendingChannelsResponse.ForceClosedChannel pending_force_closing_channels = 4;

    pub fn clear_pending_force_closing_channels(&mut self) {
        self.pending_force_closing_channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_force_closing_channels(&mut self, v: ::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel>) {
        self.pending_force_closing_channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_force_closing_channels(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel> {
        &mut self.pending_force_closing_channels
    }

    // Take field
    pub fn take_pending_force_closing_channels(&mut self) -> ::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel> {
        ::std::mem::replace(&mut self.pending_force_closing_channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_force_closing_channels(&self) -> &[PendingChannelsResponse_ForceClosedChannel] {
        &self.pending_force_closing_channels
    }

    fn get_pending_force_closing_channels_for_reflect(&self) -> &::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel> {
        &self.pending_force_closing_channels
    }

    fn mut_pending_force_closing_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PendingChannelsResponse_ForceClosedChannel> {
        &mut self.pending_force_closing_channels
    }
}

impl ::protobuf::Message for PendingChannelsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.pending_open_channels {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pending_closing_channels {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pending_force_closing_channels {
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
                    let tmp = is.read_int64()?;
                    self.total_limbo_balance = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_open_channels)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_closing_channels)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_force_closing_channels)?;
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
        if self.total_limbo_balance != 0 {
            my_size += ::protobuf::rt::value_size(1, self.total_limbo_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.pending_open_channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pending_closing_channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pending_force_closing_channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.total_limbo_balance != 0 {
            os.write_int64(1, self.total_limbo_balance)?;
        }
        for v in &self.pending_open_channels {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.pending_closing_channels {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.pending_force_closing_channels {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PendingChannelsResponse {
    fn new() -> PendingChannelsResponse {
        PendingChannelsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_limbo_balance",
                    PendingChannelsResponse::get_total_limbo_balance_for_reflect,
                    PendingChannelsResponse::mut_total_limbo_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_PendingOpenChannel>>(
                    "pending_open_channels",
                    PendingChannelsResponse::get_pending_open_channels_for_reflect,
                    PendingChannelsResponse::mut_pending_open_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_ClosedChannel>>(
                    "pending_closing_channels",
                    PendingChannelsResponse::get_pending_closing_channels_for_reflect,
                    PendingChannelsResponse::mut_pending_closing_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_ForceClosedChannel>>(
                    "pending_force_closing_channels",
                    PendingChannelsResponse::get_pending_force_closing_channels_for_reflect,
                    PendingChannelsResponse::mut_pending_force_closing_channels_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsResponse>(
                    "PendingChannelsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsResponse {
    fn clear(&mut self) {
        self.clear_total_limbo_balance();
        self.clear_pending_open_channels();
        self.clear_pending_closing_channels();
        self.clear_pending_force_closing_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsResponse_PendingChannel {
    // message fields
    pub remote_node_pub: ::std::string::String,
    pub channel_point: ::std::string::String,
    pub capacity: i64,
    pub local_balance: i64,
    pub remote_balance: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsResponse_PendingChannel {}

impl PendingChannelsResponse_PendingChannel {
    pub fn new() -> PendingChannelsResponse_PendingChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsResponse_PendingChannel {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsResponse_PendingChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsResponse_PendingChannel,
        };
        unsafe {
            instance.get(PendingChannelsResponse_PendingChannel::new)
        }
    }

    // string remote_node_pub = 1;

    pub fn clear_remote_node_pub(&mut self) {
        self.remote_node_pub.clear();
    }

    // Param is passed by value, moved
    pub fn set_remote_node_pub(&mut self, v: ::std::string::String) {
        self.remote_node_pub = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_node_pub(&mut self) -> &mut ::std::string::String {
        &mut self.remote_node_pub
    }

    // Take field
    pub fn take_remote_node_pub(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remote_node_pub, ::std::string::String::new())
    }

    pub fn get_remote_node_pub(&self) -> &str {
        &self.remote_node_pub
    }

    fn get_remote_node_pub_for_reflect(&self) -> &::std::string::String {
        &self.remote_node_pub
    }

    fn mut_remote_node_pub_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.remote_node_pub
    }

    // string channel_point = 2;

    pub fn clear_channel_point(&mut self) {
        self.channel_point.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_point(&mut self, v: ::std::string::String) {
        self.channel_point = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_point(&mut self) -> &mut ::std::string::String {
        &mut self.channel_point
    }

    // Take field
    pub fn take_channel_point(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.channel_point, ::std::string::String::new())
    }

    pub fn get_channel_point(&self) -> &str {
        &self.channel_point
    }

    fn get_channel_point_for_reflect(&self) -> &::std::string::String {
        &self.channel_point
    }

    fn mut_channel_point_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.channel_point
    }

    // int64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: i64) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> i64 {
        self.capacity
    }

    fn get_capacity_for_reflect(&self) -> &i64 {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.capacity
    }

    // int64 local_balance = 4;

    pub fn clear_local_balance(&mut self) {
        self.local_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_local_balance(&mut self, v: i64) {
        self.local_balance = v;
    }

    pub fn get_local_balance(&self) -> i64 {
        self.local_balance
    }

    fn get_local_balance_for_reflect(&self) -> &i64 {
        &self.local_balance
    }

    fn mut_local_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.local_balance
    }

    // int64 remote_balance = 5;

    pub fn clear_remote_balance(&mut self) {
        self.remote_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_remote_balance(&mut self, v: i64) {
        self.remote_balance = v;
    }

    pub fn get_remote_balance(&self) -> i64 {
        self.remote_balance
    }

    fn get_remote_balance_for_reflect(&self) -> &i64 {
        &self.remote_balance
    }

    fn mut_remote_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.remote_balance
    }
}

impl ::protobuf::Message for PendingChannelsResponse_PendingChannel {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remote_node_pub)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.channel_point)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.capacity = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.local_balance = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.remote_balance = tmp;
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
        if !self.remote_node_pub.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.remote_node_pub);
        }
        if !self.channel_point.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.channel_point);
        }
        if self.capacity != 0 {
            my_size += ::protobuf::rt::value_size(3, self.capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.local_balance != 0 {
            my_size += ::protobuf::rt::value_size(4, self.local_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.remote_balance != 0 {
            my_size += ::protobuf::rt::value_size(5, self.remote_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.remote_node_pub.is_empty() {
            os.write_string(1, &self.remote_node_pub)?;
        }
        if !self.channel_point.is_empty() {
            os.write_string(2, &self.channel_point)?;
        }
        if self.capacity != 0 {
            os.write_int64(3, self.capacity)?;
        }
        if self.local_balance != 0 {
            os.write_int64(4, self.local_balance)?;
        }
        if self.remote_balance != 0 {
            os.write_int64(5, self.remote_balance)?;
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

impl ::protobuf::MessageStatic for PendingChannelsResponse_PendingChannel {
    fn new() -> PendingChannelsResponse_PendingChannel {
        PendingChannelsResponse_PendingChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsResponse_PendingChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "remote_node_pub",
                    PendingChannelsResponse_PendingChannel::get_remote_node_pub_for_reflect,
                    PendingChannelsResponse_PendingChannel::mut_remote_node_pub_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_point",
                    PendingChannelsResponse_PendingChannel::get_channel_point_for_reflect,
                    PendingChannelsResponse_PendingChannel::mut_channel_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "capacity",
                    PendingChannelsResponse_PendingChannel::get_capacity_for_reflect,
                    PendingChannelsResponse_PendingChannel::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "local_balance",
                    PendingChannelsResponse_PendingChannel::get_local_balance_for_reflect,
                    PendingChannelsResponse_PendingChannel::mut_local_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "remote_balance",
                    PendingChannelsResponse_PendingChannel::get_remote_balance_for_reflect,
                    PendingChannelsResponse_PendingChannel::mut_remote_balance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsResponse_PendingChannel>(
                    "PendingChannelsResponse_PendingChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsResponse_PendingChannel {
    fn clear(&mut self) {
        self.clear_remote_node_pub();
        self.clear_channel_point();
        self.clear_capacity();
        self.clear_local_balance();
        self.clear_remote_balance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsResponse_PendingChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsResponse_PendingChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsResponse_PendingOpenChannel {
    // message fields
    pub channel: ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel>,
    pub confirmation_height: u32,
    pub commit_fee: i64,
    pub commit_weight: i64,
    pub fee_per_kw: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsResponse_PendingOpenChannel {}

impl PendingChannelsResponse_PendingOpenChannel {
    pub fn new() -> PendingChannelsResponse_PendingOpenChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsResponse_PendingOpenChannel {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsResponse_PendingOpenChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsResponse_PendingOpenChannel,
        };
        unsafe {
            instance.get(PendingChannelsResponse_PendingOpenChannel::new)
        }
    }

    // .lnrpc.PendingChannelsResponse.PendingChannel channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: PendingChannelsResponse_PendingChannel) {
        self.channel = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut PendingChannelsResponse_PendingChannel {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> PendingChannelsResponse_PendingChannel {
        self.channel.take().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::new())
    }

    pub fn get_channel(&self) -> &PendingChannelsResponse_PendingChannel {
        self.channel.as_ref().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::default_instance())
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &mut self.channel
    }

    // uint32 confirmation_height = 2;

    pub fn clear_confirmation_height(&mut self) {
        self.confirmation_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_confirmation_height(&mut self, v: u32) {
        self.confirmation_height = v;
    }

    pub fn get_confirmation_height(&self) -> u32 {
        self.confirmation_height
    }

    fn get_confirmation_height_for_reflect(&self) -> &u32 {
        &self.confirmation_height
    }

    fn mut_confirmation_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.confirmation_height
    }

    // int64 commit_fee = 4;

    pub fn clear_commit_fee(&mut self) {
        self.commit_fee = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_fee(&mut self, v: i64) {
        self.commit_fee = v;
    }

    pub fn get_commit_fee(&self) -> i64 {
        self.commit_fee
    }

    fn get_commit_fee_for_reflect(&self) -> &i64 {
        &self.commit_fee
    }

    fn mut_commit_fee_for_reflect(&mut self) -> &mut i64 {
        &mut self.commit_fee
    }

    // int64 commit_weight = 5;

    pub fn clear_commit_weight(&mut self) {
        self.commit_weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_weight(&mut self, v: i64) {
        self.commit_weight = v;
    }

    pub fn get_commit_weight(&self) -> i64 {
        self.commit_weight
    }

    fn get_commit_weight_for_reflect(&self) -> &i64 {
        &self.commit_weight
    }

    fn mut_commit_weight_for_reflect(&mut self) -> &mut i64 {
        &mut self.commit_weight
    }

    // int64 fee_per_kw = 6;

    pub fn clear_fee_per_kw(&mut self) {
        self.fee_per_kw = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_per_kw(&mut self, v: i64) {
        self.fee_per_kw = v;
    }

    pub fn get_fee_per_kw(&self) -> i64 {
        self.fee_per_kw
    }

    fn get_fee_per_kw_for_reflect(&self) -> &i64 {
        &self.fee_per_kw
    }

    fn mut_fee_per_kw_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee_per_kw
    }
}

impl ::protobuf::Message for PendingChannelsResponse_PendingOpenChannel {
    fn is_initialized(&self) -> bool {
        for v in &self.channel {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.confirmation_height = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_fee = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_weight = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee_per_kw = tmp;
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
        if let Some(ref v) = self.channel.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.confirmation_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.confirmation_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_fee != 0 {
            my_size += ::protobuf::rt::value_size(4, self.commit_fee, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_weight != 0 {
            my_size += ::protobuf::rt::value_size(5, self.commit_weight, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_per_kw != 0 {
            my_size += ::protobuf::rt::value_size(6, self.fee_per_kw, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.channel.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.confirmation_height != 0 {
            os.write_uint32(2, self.confirmation_height)?;
        }
        if self.commit_fee != 0 {
            os.write_int64(4, self.commit_fee)?;
        }
        if self.commit_weight != 0 {
            os.write_int64(5, self.commit_weight)?;
        }
        if self.fee_per_kw != 0 {
            os.write_int64(6, self.fee_per_kw)?;
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

impl ::protobuf::MessageStatic for PendingChannelsResponse_PendingOpenChannel {
    fn new() -> PendingChannelsResponse_PendingOpenChannel {
        PendingChannelsResponse_PendingOpenChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsResponse_PendingOpenChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_PendingChannel>>(
                    "channel",
                    PendingChannelsResponse_PendingOpenChannel::get_channel_for_reflect,
                    PendingChannelsResponse_PendingOpenChannel::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "confirmation_height",
                    PendingChannelsResponse_PendingOpenChannel::get_confirmation_height_for_reflect,
                    PendingChannelsResponse_PendingOpenChannel::mut_confirmation_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_fee",
                    PendingChannelsResponse_PendingOpenChannel::get_commit_fee_for_reflect,
                    PendingChannelsResponse_PendingOpenChannel::mut_commit_fee_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_weight",
                    PendingChannelsResponse_PendingOpenChannel::get_commit_weight_for_reflect,
                    PendingChannelsResponse_PendingOpenChannel::mut_commit_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee_per_kw",
                    PendingChannelsResponse_PendingOpenChannel::get_fee_per_kw_for_reflect,
                    PendingChannelsResponse_PendingOpenChannel::mut_fee_per_kw_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsResponse_PendingOpenChannel>(
                    "PendingChannelsResponse_PendingOpenChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsResponse_PendingOpenChannel {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_confirmation_height();
        self.clear_commit_fee();
        self.clear_commit_weight();
        self.clear_fee_per_kw();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsResponse_PendingOpenChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsResponse_PendingOpenChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsResponse_ClosedChannel {
    // message fields
    pub channel: ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel>,
    pub closing_txid: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsResponse_ClosedChannel {}

impl PendingChannelsResponse_ClosedChannel {
    pub fn new() -> PendingChannelsResponse_ClosedChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsResponse_ClosedChannel {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsResponse_ClosedChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsResponse_ClosedChannel,
        };
        unsafe {
            instance.get(PendingChannelsResponse_ClosedChannel::new)
        }
    }

    // .lnrpc.PendingChannelsResponse.PendingChannel channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: PendingChannelsResponse_PendingChannel) {
        self.channel = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut PendingChannelsResponse_PendingChannel {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> PendingChannelsResponse_PendingChannel {
        self.channel.take().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::new())
    }

    pub fn get_channel(&self) -> &PendingChannelsResponse_PendingChannel {
        self.channel.as_ref().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::default_instance())
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &mut self.channel
    }

    // string closing_txid = 2;

    pub fn clear_closing_txid(&mut self) {
        self.closing_txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_closing_txid(&mut self, v: ::std::string::String) {
        self.closing_txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_closing_txid(&mut self) -> &mut ::std::string::String {
        &mut self.closing_txid
    }

    // Take field
    pub fn take_closing_txid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.closing_txid, ::std::string::String::new())
    }

    pub fn get_closing_txid(&self) -> &str {
        &self.closing_txid
    }

    fn get_closing_txid_for_reflect(&self) -> &::std::string::String {
        &self.closing_txid
    }

    fn mut_closing_txid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.closing_txid
    }
}

impl ::protobuf::Message for PendingChannelsResponse_ClosedChannel {
    fn is_initialized(&self) -> bool {
        for v in &self.channel {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.closing_txid)?;
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
        if let Some(ref v) = self.channel.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.closing_txid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.closing_txid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.channel.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.closing_txid.is_empty() {
            os.write_string(2, &self.closing_txid)?;
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

impl ::protobuf::MessageStatic for PendingChannelsResponse_ClosedChannel {
    fn new() -> PendingChannelsResponse_ClosedChannel {
        PendingChannelsResponse_ClosedChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsResponse_ClosedChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_PendingChannel>>(
                    "channel",
                    PendingChannelsResponse_ClosedChannel::get_channel_for_reflect,
                    PendingChannelsResponse_ClosedChannel::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "closing_txid",
                    PendingChannelsResponse_ClosedChannel::get_closing_txid_for_reflect,
                    PendingChannelsResponse_ClosedChannel::mut_closing_txid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsResponse_ClosedChannel>(
                    "PendingChannelsResponse_ClosedChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsResponse_ClosedChannel {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_closing_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsResponse_ClosedChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsResponse_ClosedChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PendingChannelsResponse_ForceClosedChannel {
    // message fields
    pub channel: ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel>,
    pub closing_txid: ::std::string::String,
    pub limbo_balance: i64,
    pub maturity_height: u32,
    pub blocks_til_maturity: i32,
    pub recovered_balance: i64,
    pub pending_htlcs: ::protobuf::RepeatedField<PendingHTLC>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PendingChannelsResponse_ForceClosedChannel {}

impl PendingChannelsResponse_ForceClosedChannel {
    pub fn new() -> PendingChannelsResponse_ForceClosedChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PendingChannelsResponse_ForceClosedChannel {
        static mut instance: ::protobuf::lazy::Lazy<PendingChannelsResponse_ForceClosedChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PendingChannelsResponse_ForceClosedChannel,
        };
        unsafe {
            instance.get(PendingChannelsResponse_ForceClosedChannel::new)
        }
    }

    // .lnrpc.PendingChannelsResponse.PendingChannel channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: PendingChannelsResponse_PendingChannel) {
        self.channel = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut PendingChannelsResponse_PendingChannel {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> PendingChannelsResponse_PendingChannel {
        self.channel.take().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::new())
    }

    pub fn get_channel(&self) -> &PendingChannelsResponse_PendingChannel {
        self.channel.as_ref().unwrap_or_else(|| PendingChannelsResponse_PendingChannel::default_instance())
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PendingChannelsResponse_PendingChannel> {
        &mut self.channel
    }

    // string closing_txid = 2;

    pub fn clear_closing_txid(&mut self) {
        self.closing_txid.clear();
    }

    // Param is passed by value, moved
    pub fn set_closing_txid(&mut self, v: ::std::string::String) {
        self.closing_txid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_closing_txid(&mut self) -> &mut ::std::string::String {
        &mut self.closing_txid
    }

    // Take field
    pub fn take_closing_txid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.closing_txid, ::std::string::String::new())
    }

    pub fn get_closing_txid(&self) -> &str {
        &self.closing_txid
    }

    fn get_closing_txid_for_reflect(&self) -> &::std::string::String {
        &self.closing_txid
    }

    fn mut_closing_txid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.closing_txid
    }

    // int64 limbo_balance = 3;

    pub fn clear_limbo_balance(&mut self) {
        self.limbo_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_limbo_balance(&mut self, v: i64) {
        self.limbo_balance = v;
    }

    pub fn get_limbo_balance(&self) -> i64 {
        self.limbo_balance
    }

    fn get_limbo_balance_for_reflect(&self) -> &i64 {
        &self.limbo_balance
    }

    fn mut_limbo_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.limbo_balance
    }

    // uint32 maturity_height = 4;

    pub fn clear_maturity_height(&mut self) {
        self.maturity_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_maturity_height(&mut self, v: u32) {
        self.maturity_height = v;
    }

    pub fn get_maturity_height(&self) -> u32 {
        self.maturity_height
    }

    fn get_maturity_height_for_reflect(&self) -> &u32 {
        &self.maturity_height
    }

    fn mut_maturity_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.maturity_height
    }

    // int32 blocks_til_maturity = 5;

    pub fn clear_blocks_til_maturity(&mut self) {
        self.blocks_til_maturity = 0;
    }

    // Param is passed by value, moved
    pub fn set_blocks_til_maturity(&mut self, v: i32) {
        self.blocks_til_maturity = v;
    }

    pub fn get_blocks_til_maturity(&self) -> i32 {
        self.blocks_til_maturity
    }

    fn get_blocks_til_maturity_for_reflect(&self) -> &i32 {
        &self.blocks_til_maturity
    }

    fn mut_blocks_til_maturity_for_reflect(&mut self) -> &mut i32 {
        &mut self.blocks_til_maturity
    }

    // int64 recovered_balance = 6;

    pub fn clear_recovered_balance(&mut self) {
        self.recovered_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_recovered_balance(&mut self, v: i64) {
        self.recovered_balance = v;
    }

    pub fn get_recovered_balance(&self) -> i64 {
        self.recovered_balance
    }

    fn get_recovered_balance_for_reflect(&self) -> &i64 {
        &self.recovered_balance
    }

    fn mut_recovered_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.recovered_balance
    }

    // repeated .lnrpc.PendingHTLC pending_htlcs = 8;

    pub fn clear_pending_htlcs(&mut self) {
        self.pending_htlcs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_htlcs(&mut self, v: ::protobuf::RepeatedField<PendingHTLC>) {
        self.pending_htlcs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_htlcs(&mut self) -> &mut ::protobuf::RepeatedField<PendingHTLC> {
        &mut self.pending_htlcs
    }

    // Take field
    pub fn take_pending_htlcs(&mut self) -> ::protobuf::RepeatedField<PendingHTLC> {
        ::std::mem::replace(&mut self.pending_htlcs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_htlcs(&self) -> &[PendingHTLC] {
        &self.pending_htlcs
    }

    fn get_pending_htlcs_for_reflect(&self) -> &::protobuf::RepeatedField<PendingHTLC> {
        &self.pending_htlcs
    }

    fn mut_pending_htlcs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PendingHTLC> {
        &mut self.pending_htlcs
    }
}

impl ::protobuf::Message for PendingChannelsResponse_ForceClosedChannel {
    fn is_initialized(&self) -> bool {
        for v in &self.channel {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pending_htlcs {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.closing_txid)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.limbo_balance = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maturity_height = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.blocks_til_maturity = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.recovered_balance = tmp;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_htlcs)?;
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
        if let Some(ref v) = self.channel.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.closing_txid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.closing_txid);
        }
        if self.limbo_balance != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limbo_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.maturity_height != 0 {
            my_size += ::protobuf::rt::value_size(4, self.maturity_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.blocks_til_maturity != 0 {
            my_size += ::protobuf::rt::value_size(5, self.blocks_til_maturity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.recovered_balance != 0 {
            my_size += ::protobuf::rt::value_size(6, self.recovered_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.pending_htlcs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.channel.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.closing_txid.is_empty() {
            os.write_string(2, &self.closing_txid)?;
        }
        if self.limbo_balance != 0 {
            os.write_int64(3, self.limbo_balance)?;
        }
        if self.maturity_height != 0 {
            os.write_uint32(4, self.maturity_height)?;
        }
        if self.blocks_til_maturity != 0 {
            os.write_int32(5, self.blocks_til_maturity)?;
        }
        if self.recovered_balance != 0 {
            os.write_int64(6, self.recovered_balance)?;
        }
        for v in &self.pending_htlcs {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PendingChannelsResponse_ForceClosedChannel {
    fn new() -> PendingChannelsResponse_ForceClosedChannel {
        PendingChannelsResponse_ForceClosedChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<PendingChannelsResponse_ForceClosedChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingChannelsResponse_PendingChannel>>(
                    "channel",
                    PendingChannelsResponse_ForceClosedChannel::get_channel_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "closing_txid",
                    PendingChannelsResponse_ForceClosedChannel::get_closing_txid_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_closing_txid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "limbo_balance",
                    PendingChannelsResponse_ForceClosedChannel::get_limbo_balance_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_limbo_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maturity_height",
                    PendingChannelsResponse_ForceClosedChannel::get_maturity_height_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_maturity_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "blocks_til_maturity",
                    PendingChannelsResponse_ForceClosedChannel::get_blocks_til_maturity_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_blocks_til_maturity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "recovered_balance",
                    PendingChannelsResponse_ForceClosedChannel::get_recovered_balance_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_recovered_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PendingHTLC>>(
                    "pending_htlcs",
                    PendingChannelsResponse_ForceClosedChannel::get_pending_htlcs_for_reflect,
                    PendingChannelsResponse_ForceClosedChannel::mut_pending_htlcs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PendingChannelsResponse_ForceClosedChannel>(
                    "PendingChannelsResponse_ForceClosedChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PendingChannelsResponse_ForceClosedChannel {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_closing_txid();
        self.clear_limbo_balance();
        self.clear_maturity_height();
        self.clear_blocks_til_maturity();
        self.clear_recovered_balance();
        self.clear_pending_htlcs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PendingChannelsResponse_ForceClosedChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PendingChannelsResponse_ForceClosedChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WalletBalanceRequest {
    // message fields
    pub witness_only: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WalletBalanceRequest {}

impl WalletBalanceRequest {
    pub fn new() -> WalletBalanceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WalletBalanceRequest {
        static mut instance: ::protobuf::lazy::Lazy<WalletBalanceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WalletBalanceRequest,
        };
        unsafe {
            instance.get(WalletBalanceRequest::new)
        }
    }

    // bool witness_only = 1;

    pub fn clear_witness_only(&mut self) {
        self.witness_only = false;
    }

    // Param is passed by value, moved
    pub fn set_witness_only(&mut self, v: bool) {
        self.witness_only = v;
    }

    pub fn get_witness_only(&self) -> bool {
        self.witness_only
    }

    fn get_witness_only_for_reflect(&self) -> &bool {
        &self.witness_only
    }

    fn mut_witness_only_for_reflect(&mut self) -> &mut bool {
        &mut self.witness_only
    }
}

impl ::protobuf::Message for WalletBalanceRequest {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.witness_only = tmp;
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
        if self.witness_only != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.witness_only != false {
            os.write_bool(1, self.witness_only)?;
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

impl ::protobuf::MessageStatic for WalletBalanceRequest {
    fn new() -> WalletBalanceRequest {
        WalletBalanceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WalletBalanceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "witness_only",
                    WalletBalanceRequest::get_witness_only_for_reflect,
                    WalletBalanceRequest::mut_witness_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WalletBalanceRequest>(
                    "WalletBalanceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WalletBalanceRequest {
    fn clear(&mut self) {
        self.clear_witness_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WalletBalanceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WalletBalanceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WalletBalanceResponse {
    // message fields
    pub total_balance: i64,
    pub confirmed_balance: i64,
    pub unconfirmed_balance: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WalletBalanceResponse {}

impl WalletBalanceResponse {
    pub fn new() -> WalletBalanceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WalletBalanceResponse {
        static mut instance: ::protobuf::lazy::Lazy<WalletBalanceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WalletBalanceResponse,
        };
        unsafe {
            instance.get(WalletBalanceResponse::new)
        }
    }

    // int64 total_balance = 1;

    pub fn clear_total_balance(&mut self) {
        self.total_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_balance(&mut self, v: i64) {
        self.total_balance = v;
    }

    pub fn get_total_balance(&self) -> i64 {
        self.total_balance
    }

    fn get_total_balance_for_reflect(&self) -> &i64 {
        &self.total_balance
    }

    fn mut_total_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_balance
    }

    // int64 confirmed_balance = 2;

    pub fn clear_confirmed_balance(&mut self) {
        self.confirmed_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_confirmed_balance(&mut self, v: i64) {
        self.confirmed_balance = v;
    }

    pub fn get_confirmed_balance(&self) -> i64 {
        self.confirmed_balance
    }

    fn get_confirmed_balance_for_reflect(&self) -> &i64 {
        &self.confirmed_balance
    }

    fn mut_confirmed_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.confirmed_balance
    }

    // int64 unconfirmed_balance = 3;

    pub fn clear_unconfirmed_balance(&mut self) {
        self.unconfirmed_balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_unconfirmed_balance(&mut self, v: i64) {
        self.unconfirmed_balance = v;
    }

    pub fn get_unconfirmed_balance(&self) -> i64 {
        self.unconfirmed_balance
    }

    fn get_unconfirmed_balance_for_reflect(&self) -> &i64 {
        &self.unconfirmed_balance
    }

    fn mut_unconfirmed_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.unconfirmed_balance
    }
}

impl ::protobuf::Message for WalletBalanceResponse {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int64()?;
                    self.total_balance = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.confirmed_balance = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.unconfirmed_balance = tmp;
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
        if self.total_balance != 0 {
            my_size += ::protobuf::rt::value_size(1, self.total_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.confirmed_balance != 0 {
            my_size += ::protobuf::rt::value_size(2, self.confirmed_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.unconfirmed_balance != 0 {
            my_size += ::protobuf::rt::value_size(3, self.unconfirmed_balance, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.total_balance != 0 {
            os.write_int64(1, self.total_balance)?;
        }
        if self.confirmed_balance != 0 {
            os.write_int64(2, self.confirmed_balance)?;
        }
        if self.unconfirmed_balance != 0 {
            os.write_int64(3, self.unconfirmed_balance)?;
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

impl ::protobuf::MessageStatic for WalletBalanceResponse {
    fn new() -> WalletBalanceResponse {
        WalletBalanceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WalletBalanceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_balance",
                    WalletBalanceResponse::get_total_balance_for_reflect,
                    WalletBalanceResponse::mut_total_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "confirmed_balance",
                    WalletBalanceResponse::get_confirmed_balance_for_reflect,
                    WalletBalanceResponse::mut_confirmed_balance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "unconfirmed_balance",
                    WalletBalanceResponse::get_unconfirmed_balance_for_reflect,
                    WalletBalanceResponse::mut_unconfirmed_balance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WalletBalanceResponse>(
                    "WalletBalanceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WalletBalanceResponse {
    fn clear(&mut self) {
        self.clear_total_balance();
        self.clear_confirmed_balance();
        self.clear_unconfirmed_balance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WalletBalanceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WalletBalanceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelBalanceRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelBalanceRequest {}

impl ChannelBalanceRequest {
    pub fn new() -> ChannelBalanceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelBalanceRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChannelBalanceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelBalanceRequest,
        };
        unsafe {
            instance.get(ChannelBalanceRequest::new)
        }
    }
}

impl ::protobuf::Message for ChannelBalanceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ChannelBalanceRequest {
    fn new() -> ChannelBalanceRequest {
        ChannelBalanceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelBalanceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ChannelBalanceRequest>(
                    "ChannelBalanceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelBalanceRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelBalanceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelBalanceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelBalanceResponse {
    // message fields
    pub balance: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelBalanceResponse {}

impl ChannelBalanceResponse {
    pub fn new() -> ChannelBalanceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelBalanceResponse {
        static mut instance: ::protobuf::lazy::Lazy<ChannelBalanceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelBalanceResponse,
        };
        unsafe {
            instance.get(ChannelBalanceResponse::new)
        }
    }

    // int64 balance = 1;

    pub fn clear_balance(&mut self) {
        self.balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_balance(&mut self, v: i64) {
        self.balance = v;
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    fn get_balance_for_reflect(&self) -> &i64 {
        &self.balance
    }

    fn mut_balance_for_reflect(&mut self) -> &mut i64 {
        &mut self.balance
    }
}

impl ::protobuf::Message for ChannelBalanceResponse {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int64()?;
                    self.balance = tmp;
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
        if self.balance != 0 {
            my_size += ::protobuf::rt::value_size(1, self.balance, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.balance != 0 {
            os.write_int64(1, self.balance)?;
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

impl ::protobuf::MessageStatic for ChannelBalanceResponse {
    fn new() -> ChannelBalanceResponse {
        ChannelBalanceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelBalanceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "balance",
                    ChannelBalanceResponse::get_balance_for_reflect,
                    ChannelBalanceResponse::mut_balance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelBalanceResponse>(
                    "ChannelBalanceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelBalanceResponse {
    fn clear(&mut self) {
        self.clear_balance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelBalanceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelBalanceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryRoutesRequest {
    // message fields
    pub pub_key: ::std::string::String,
    pub amt: i64,
    pub num_routes: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryRoutesRequest {}

impl QueryRoutesRequest {
    pub fn new() -> QueryRoutesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryRoutesRequest {
        static mut instance: ::protobuf::lazy::Lazy<QueryRoutesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryRoutesRequest,
        };
        unsafe {
            instance.get(QueryRoutesRequest::new)
        }
    }

    // string pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // int64 amt = 2;

    pub fn clear_amt(&mut self) {
        self.amt = 0;
    }

    // Param is passed by value, moved
    pub fn set_amt(&mut self, v: i64) {
        self.amt = v;
    }

    pub fn get_amt(&self) -> i64 {
        self.amt
    }

    fn get_amt_for_reflect(&self) -> &i64 {
        &self.amt
    }

    fn mut_amt_for_reflect(&mut self) -> &mut i64 {
        &mut self.amt
    }

    // int32 num_routes = 3;

    pub fn clear_num_routes(&mut self) {
        self.num_routes = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_routes(&mut self, v: i32) {
        self.num_routes = v;
    }

    pub fn get_num_routes(&self) -> i32 {
        self.num_routes
    }

    fn get_num_routes_for_reflect(&self) -> &i32 {
        &self.num_routes
    }

    fn mut_num_routes_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_routes
    }
}

impl ::protobuf::Message for QueryRoutesRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amt = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_routes = tmp;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pub_key);
        }
        if self.amt != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_routes != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_routes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_string(1, &self.pub_key)?;
        }
        if self.amt != 0 {
            os.write_int64(2, self.amt)?;
        }
        if self.num_routes != 0 {
            os.write_int32(3, self.num_routes)?;
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

impl ::protobuf::MessageStatic for QueryRoutesRequest {
    fn new() -> QueryRoutesRequest {
        QueryRoutesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryRoutesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    QueryRoutesRequest::get_pub_key_for_reflect,
                    QueryRoutesRequest::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amt",
                    QueryRoutesRequest::get_amt_for_reflect,
                    QueryRoutesRequest::mut_amt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_routes",
                    QueryRoutesRequest::get_num_routes_for_reflect,
                    QueryRoutesRequest::mut_num_routes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryRoutesRequest>(
                    "QueryRoutesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryRoutesRequest {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.clear_amt();
        self.clear_num_routes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryRoutesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryRoutesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryRoutesResponse {
    // message fields
    pub routes: ::protobuf::RepeatedField<Route>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryRoutesResponse {}

impl QueryRoutesResponse {
    pub fn new() -> QueryRoutesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryRoutesResponse {
        static mut instance: ::protobuf::lazy::Lazy<QueryRoutesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryRoutesResponse,
        };
        unsafe {
            instance.get(QueryRoutesResponse::new)
        }
    }

    // repeated .lnrpc.Route routes = 1;

    pub fn clear_routes(&mut self) {
        self.routes.clear();
    }

    // Param is passed by value, moved
    pub fn set_routes(&mut self, v: ::protobuf::RepeatedField<Route>) {
        self.routes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_routes(&mut self) -> &mut ::protobuf::RepeatedField<Route> {
        &mut self.routes
    }

    // Take field
    pub fn take_routes(&mut self) -> ::protobuf::RepeatedField<Route> {
        ::std::mem::replace(&mut self.routes, ::protobuf::RepeatedField::new())
    }

    pub fn get_routes(&self) -> &[Route] {
        &self.routes
    }

    fn get_routes_for_reflect(&self) -> &::protobuf::RepeatedField<Route> {
        &self.routes
    }

    fn mut_routes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Route> {
        &mut self.routes
    }
}

impl ::protobuf::Message for QueryRoutesResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.routes {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.routes)?;
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
        for value in &self.routes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.routes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for QueryRoutesResponse {
    fn new() -> QueryRoutesResponse {
        QueryRoutesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryRoutesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Route>>(
                    "routes",
                    QueryRoutesResponse::get_routes_for_reflect,
                    QueryRoutesResponse::mut_routes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryRoutesResponse>(
                    "QueryRoutesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryRoutesResponse {
    fn clear(&mut self) {
        self.clear_routes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryRoutesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryRoutesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Hop {
    // message fields
    pub chan_id: u64,
    pub chan_capacity: i64,
    pub amt_to_forward: i64,
    pub fee: i64,
    pub expiry: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Hop {}

impl Hop {
    pub fn new() -> Hop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Hop {
        static mut instance: ::protobuf::lazy::Lazy<Hop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Hop,
        };
        unsafe {
            instance.get(Hop::new)
        }
    }

    // uint64 chan_id = 1;

    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }

    fn get_chan_id_for_reflect(&self) -> &u64 {
        &self.chan_id
    }

    fn mut_chan_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.chan_id
    }

    // int64 chan_capacity = 2;

    pub fn clear_chan_capacity(&mut self) {
        self.chan_capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_capacity(&mut self, v: i64) {
        self.chan_capacity = v;
    }

    pub fn get_chan_capacity(&self) -> i64 {
        self.chan_capacity
    }

    fn get_chan_capacity_for_reflect(&self) -> &i64 {
        &self.chan_capacity
    }

    fn mut_chan_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.chan_capacity
    }

    // int64 amt_to_forward = 3;

    pub fn clear_amt_to_forward(&mut self) {
        self.amt_to_forward = 0;
    }

    // Param is passed by value, moved
    pub fn set_amt_to_forward(&mut self, v: i64) {
        self.amt_to_forward = v;
    }

    pub fn get_amt_to_forward(&self) -> i64 {
        self.amt_to_forward
    }

    fn get_amt_to_forward_for_reflect(&self) -> &i64 {
        &self.amt_to_forward
    }

    fn mut_amt_to_forward_for_reflect(&mut self) -> &mut i64 {
        &mut self.amt_to_forward
    }

    // int64 fee = 4;

    pub fn clear_fee(&mut self) {
        self.fee = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee(&mut self, v: i64) {
        self.fee = v;
    }

    pub fn get_fee(&self) -> i64 {
        self.fee
    }

    fn get_fee_for_reflect(&self) -> &i64 {
        &self.fee
    }

    fn mut_fee_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee
    }

    // uint32 expiry = 5;

    pub fn clear_expiry(&mut self) {
        self.expiry = 0;
    }

    // Param is passed by value, moved
    pub fn set_expiry(&mut self, v: u32) {
        self.expiry = v;
    }

    pub fn get_expiry(&self) -> u32 {
        self.expiry
    }

    fn get_expiry_for_reflect(&self) -> &u32 {
        &self.expiry
    }

    fn mut_expiry_for_reflect(&mut self) -> &mut u32 {
        &mut self.expiry
    }
}

impl ::protobuf::Message for Hop {
    fn is_initialized(&self) -> bool {
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
                    self.chan_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.chan_capacity = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.amt_to_forward = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.expiry = tmp;
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
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.chan_capacity != 0 {
            my_size += ::protobuf::rt::value_size(2, self.chan_capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.amt_to_forward != 0 {
            my_size += ::protobuf::rt::value_size(3, self.amt_to_forward, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee != 0 {
            my_size += ::protobuf::rt::value_size(4, self.fee, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.expiry != 0 {
            my_size += ::protobuf::rt::value_size(5, self.expiry, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chan_id != 0 {
            os.write_uint64(1, self.chan_id)?;
        }
        if self.chan_capacity != 0 {
            os.write_int64(2, self.chan_capacity)?;
        }
        if self.amt_to_forward != 0 {
            os.write_int64(3, self.amt_to_forward)?;
        }
        if self.fee != 0 {
            os.write_int64(4, self.fee)?;
        }
        if self.expiry != 0 {
            os.write_uint32(5, self.expiry)?;
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

impl ::protobuf::MessageStatic for Hop {
    fn new() -> Hop {
        Hop::new()
    }

    fn descriptor_static(_: ::std::option::Option<Hop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chan_id",
                    Hop::get_chan_id_for_reflect,
                    Hop::mut_chan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "chan_capacity",
                    Hop::get_chan_capacity_for_reflect,
                    Hop::mut_chan_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "amt_to_forward",
                    Hop::get_amt_to_forward_for_reflect,
                    Hop::mut_amt_to_forward_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee",
                    Hop::get_fee_for_reflect,
                    Hop::mut_fee_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "expiry",
                    Hop::get_expiry_for_reflect,
                    Hop::mut_expiry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Hop>(
                    "Hop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Hop {
    fn clear(&mut self) {
        self.clear_chan_id();
        self.clear_chan_capacity();
        self.clear_amt_to_forward();
        self.clear_fee();
        self.clear_expiry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Hop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Hop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Route {
    // message fields
    pub total_time_lock: u32,
    pub total_fees: i64,
    pub total_amt: i64,
    pub hops: ::protobuf::RepeatedField<Hop>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Route {}

impl Route {
    pub fn new() -> Route {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Route {
        static mut instance: ::protobuf::lazy::Lazy<Route> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Route,
        };
        unsafe {
            instance.get(Route::new)
        }
    }

    // uint32 total_time_lock = 1;

    pub fn clear_total_time_lock(&mut self) {
        self.total_time_lock = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_time_lock(&mut self, v: u32) {
        self.total_time_lock = v;
    }

    pub fn get_total_time_lock(&self) -> u32 {
        self.total_time_lock
    }

    fn get_total_time_lock_for_reflect(&self) -> &u32 {
        &self.total_time_lock
    }

    fn mut_total_time_lock_for_reflect(&mut self) -> &mut u32 {
        &mut self.total_time_lock
    }

    // int64 total_fees = 2;

    pub fn clear_total_fees(&mut self) {
        self.total_fees = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_fees(&mut self, v: i64) {
        self.total_fees = v;
    }

    pub fn get_total_fees(&self) -> i64 {
        self.total_fees
    }

    fn get_total_fees_for_reflect(&self) -> &i64 {
        &self.total_fees
    }

    fn mut_total_fees_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_fees
    }

    // int64 total_amt = 3;

    pub fn clear_total_amt(&mut self) {
        self.total_amt = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_amt(&mut self, v: i64) {
        self.total_amt = v;
    }

    pub fn get_total_amt(&self) -> i64 {
        self.total_amt
    }

    fn get_total_amt_for_reflect(&self) -> &i64 {
        &self.total_amt
    }

    fn mut_total_amt_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_amt
    }

    // repeated .lnrpc.Hop hops = 4;

    pub fn clear_hops(&mut self) {
        self.hops.clear();
    }

    // Param is passed by value, moved
    pub fn set_hops(&mut self, v: ::protobuf::RepeatedField<Hop>) {
        self.hops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hops(&mut self) -> &mut ::protobuf::RepeatedField<Hop> {
        &mut self.hops
    }

    // Take field
    pub fn take_hops(&mut self) -> ::protobuf::RepeatedField<Hop> {
        ::std::mem::replace(&mut self.hops, ::protobuf::RepeatedField::new())
    }

    pub fn get_hops(&self) -> &[Hop] {
        &self.hops
    }

    fn get_hops_for_reflect(&self) -> &::protobuf::RepeatedField<Hop> {
        &self.hops
    }

    fn mut_hops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Hop> {
        &mut self.hops
    }
}

impl ::protobuf::Message for Route {
    fn is_initialized(&self) -> bool {
        for v in &self.hops {
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
                    let tmp = is.read_uint32()?;
                    self.total_time_lock = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_fees = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_amt = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hops)?;
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
        if self.total_time_lock != 0 {
            my_size += ::protobuf::rt::value_size(1, self.total_time_lock, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_fees != 0 {
            my_size += ::protobuf::rt::value_size(2, self.total_fees, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_amt != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total_amt, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.hops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.total_time_lock != 0 {
            os.write_uint32(1, self.total_time_lock)?;
        }
        if self.total_fees != 0 {
            os.write_int64(2, self.total_fees)?;
        }
        if self.total_amt != 0 {
            os.write_int64(3, self.total_amt)?;
        }
        for v in &self.hops {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Route {
    fn new() -> Route {
        Route::new()
    }

    fn descriptor_static(_: ::std::option::Option<Route>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total_time_lock",
                    Route::get_total_time_lock_for_reflect,
                    Route::mut_total_time_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_fees",
                    Route::get_total_fees_for_reflect,
                    Route::mut_total_fees_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_amt",
                    Route::get_total_amt_for_reflect,
                    Route::mut_total_amt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Hop>>(
                    "hops",
                    Route::get_hops_for_reflect,
                    Route::mut_hops_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Route>(
                    "Route",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Route {
    fn clear(&mut self) {
        self.clear_total_time_lock();
        self.clear_total_fees();
        self.clear_total_amt();
        self.clear_hops();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Route {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Route {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeInfoRequest {
    // message fields
    pub pub_key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeInfoRequest {}

impl NodeInfoRequest {
    pub fn new() -> NodeInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<NodeInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeInfoRequest,
        };
        unsafe {
            instance.get(NodeInfoRequest::new)
        }
    }

    // string pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }
}

impl ::protobuf::Message for NodeInfoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pub_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_string(1, &self.pub_key)?;
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

impl ::protobuf::MessageStatic for NodeInfoRequest {
    fn new() -> NodeInfoRequest {
        NodeInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    NodeInfoRequest::get_pub_key_for_reflect,
                    NodeInfoRequest::mut_pub_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeInfoRequest>(
                    "NodeInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeInfoRequest {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeInfo {
    // message fields
    pub node: ::protobuf::SingularPtrField<LightningNode>,
    pub num_channels: u32,
    pub total_capacity: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeInfo {}

impl NodeInfo {
    pub fn new() -> NodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<NodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeInfo,
        };
        unsafe {
            instance.get(NodeInfo::new)
        }
    }

    // .lnrpc.LightningNode node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: LightningNode) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut LightningNode {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> LightningNode {
        self.node.take().unwrap_or_else(|| LightningNode::new())
    }

    pub fn get_node(&self) -> &LightningNode {
        self.node.as_ref().unwrap_or_else(|| LightningNode::default_instance())
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularPtrField<LightningNode> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LightningNode> {
        &mut self.node
    }

    // uint32 num_channels = 2;

    pub fn clear_num_channels(&mut self) {
        self.num_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_channels(&mut self, v: u32) {
        self.num_channels = v;
    }

    pub fn get_num_channels(&self) -> u32 {
        self.num_channels
    }

    fn get_num_channels_for_reflect(&self) -> &u32 {
        &self.num_channels
    }

    fn mut_num_channels_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_channels
    }

    // int64 total_capacity = 3;

    pub fn clear_total_capacity(&mut self) {
        self.total_capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_capacity(&mut self, v: i64) {
        self.total_capacity = v;
    }

    pub fn get_total_capacity(&self) -> i64 {
        self.total_capacity
    }

    fn get_total_capacity_for_reflect(&self) -> &i64 {
        &self.total_capacity
    }

    fn mut_total_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_capacity
    }
}

impl ::protobuf::Message for NodeInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_channels = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_capacity = tmp;
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
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.num_channels != 0 {
            my_size += ::protobuf::rt::value_size(2, self.num_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_capacity != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total_capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.num_channels != 0 {
            os.write_uint32(2, self.num_channels)?;
        }
        if self.total_capacity != 0 {
            os.write_int64(3, self.total_capacity)?;
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

impl ::protobuf::MessageStatic for NodeInfo {
    fn new() -> NodeInfo {
        NodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LightningNode>>(
                    "node",
                    NodeInfo::get_node_for_reflect,
                    NodeInfo::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_channels",
                    NodeInfo::get_num_channels_for_reflect,
                    NodeInfo::mut_num_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_capacity",
                    NodeInfo::get_total_capacity_for_reflect,
                    NodeInfo::mut_total_capacity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeInfo>(
                    "NodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeInfo {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_num_channels();
        self.clear_total_capacity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LightningNode {
    // message fields
    pub last_update: u32,
    pub pub_key: ::std::string::String,
    pub alias: ::std::string::String,
    pub addresses: ::protobuf::RepeatedField<NodeAddress>,
    pub color: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LightningNode {}

impl LightningNode {
    pub fn new() -> LightningNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LightningNode {
        static mut instance: ::protobuf::lazy::Lazy<LightningNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LightningNode,
        };
        unsafe {
            instance.get(LightningNode::new)
        }
    }

    // uint32 last_update = 1;

    pub fn clear_last_update(&mut self) {
        self.last_update = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_update(&mut self, v: u32) {
        self.last_update = v;
    }

    pub fn get_last_update(&self) -> u32 {
        self.last_update
    }

    fn get_last_update_for_reflect(&self) -> &u32 {
        &self.last_update
    }

    fn mut_last_update_for_reflect(&mut self) -> &mut u32 {
        &mut self.last_update
    }

    // string pub_key = 2;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // string alias = 3;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // repeated .lnrpc.NodeAddress addresses = 4;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<NodeAddress>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<NodeAddress> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<NodeAddress> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[NodeAddress] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<NodeAddress> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NodeAddress> {
        &mut self.addresses
    }

    // string color = 5;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: ::std::string::String) {
        self.color = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut ::std::string::String {
        &mut self.color
    }

    // Take field
    pub fn take_color(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.color, ::std::string::String::new())
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }

    fn get_color_for_reflect(&self) -> &::std::string::String {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.color
    }
}

impl ::protobuf::Message for LightningNode {
    fn is_initialized(&self) -> bool {
        for v in &self.addresses {
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
                    let tmp = is.read_uint32()?;
                    self.last_update = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.addresses)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.color)?;
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
        if self.last_update != 0 {
            my_size += ::protobuf::rt::value_size(1, self.last_update, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.pub_key);
        }
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.alias);
        }
        for value in &self.addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.color.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.color);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.last_update != 0 {
            os.write_uint32(1, self.last_update)?;
        }
        if !self.pub_key.is_empty() {
            os.write_string(2, &self.pub_key)?;
        }
        if !self.alias.is_empty() {
            os.write_string(3, &self.alias)?;
        }
        for v in &self.addresses {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.color.is_empty() {
            os.write_string(5, &self.color)?;
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

impl ::protobuf::MessageStatic for LightningNode {
    fn new() -> LightningNode {
        LightningNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<LightningNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_update",
                    LightningNode::get_last_update_for_reflect,
                    LightningNode::mut_last_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    LightningNode::get_pub_key_for_reflect,
                    LightningNode::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    LightningNode::get_alias_for_reflect,
                    LightningNode::mut_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NodeAddress>>(
                    "addresses",
                    LightningNode::get_addresses_for_reflect,
                    LightningNode::mut_addresses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "color",
                    LightningNode::get_color_for_reflect,
                    LightningNode::mut_color_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LightningNode>(
                    "LightningNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LightningNode {
    fn clear(&mut self) {
        self.clear_last_update();
        self.clear_pub_key();
        self.clear_alias();
        self.clear_addresses();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LightningNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LightningNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeAddress {
    // message fields
    pub network: ::std::string::String,
    pub addr: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeAddress {}

impl NodeAddress {
    pub fn new() -> NodeAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeAddress {
        static mut instance: ::protobuf::lazy::Lazy<NodeAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeAddress,
        };
        unsafe {
            instance.get(NodeAddress::new)
        }
    }

    // string network = 1;

    pub fn clear_network(&mut self) {
        self.network.clear();
    }

    // Param is passed by value, moved
    pub fn set_network(&mut self, v: ::std::string::String) {
        self.network = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network(&mut self) -> &mut ::std::string::String {
        &mut self.network
    }

    // Take field
    pub fn take_network(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.network, ::std::string::String::new())
    }

    pub fn get_network(&self) -> &str {
        &self.network
    }

    fn get_network_for_reflect(&self) -> &::std::string::String {
        &self.network
    }

    fn mut_network_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.network
    }

    // string addr = 2;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: ::std::string::String) {
        self.addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.addr, ::std::string::String::new())
    }

    pub fn get_addr(&self) -> &str {
        &self.addr
    }

    fn get_addr_for_reflect(&self) -> &::std::string::String {
        &self.addr
    }

    fn mut_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }
}

impl ::protobuf::Message for NodeAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.network)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.addr)?;
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
        if !self.network.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.network);
        }
        if !self.addr.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.network.is_empty() {
            os.write_string(1, &self.network)?;
        }
        if !self.addr.is_empty() {
            os.write_string(2, &self.addr)?;
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

impl ::protobuf::MessageStatic for NodeAddress {
    fn new() -> NodeAddress {
        NodeAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "network",
                    NodeAddress::get_network_for_reflect,
                    NodeAddress::mut_network_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addr",
                    NodeAddress::get_addr_for_reflect,
                    NodeAddress::mut_addr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeAddress>(
                    "NodeAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeAddress {
    fn clear(&mut self) {
        self.clear_network();
        self.clear_addr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoutingPolicy {
    // message fields
    pub time_lock_delta: u32,
    pub min_htlc: i64,
    pub fee_base_msat: i64,
    pub fee_rate_milli_msat: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoutingPolicy {}

impl RoutingPolicy {
    pub fn new() -> RoutingPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoutingPolicy {
        static mut instance: ::protobuf::lazy::Lazy<RoutingPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoutingPolicy,
        };
        unsafe {
            instance.get(RoutingPolicy::new)
        }
    }

    // uint32 time_lock_delta = 1;

    pub fn clear_time_lock_delta(&mut self) {
        self.time_lock_delta = 0;
    }

    // Param is passed by value, moved
    pub fn set_time_lock_delta(&mut self, v: u32) {
        self.time_lock_delta = v;
    }

    pub fn get_time_lock_delta(&self) -> u32 {
        self.time_lock_delta
    }

    fn get_time_lock_delta_for_reflect(&self) -> &u32 {
        &self.time_lock_delta
    }

    fn mut_time_lock_delta_for_reflect(&mut self) -> &mut u32 {
        &mut self.time_lock_delta
    }

    // int64 min_htlc = 2;

    pub fn clear_min_htlc(&mut self) {
        self.min_htlc = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_htlc(&mut self, v: i64) {
        self.min_htlc = v;
    }

    pub fn get_min_htlc(&self) -> i64 {
        self.min_htlc
    }

    fn get_min_htlc_for_reflect(&self) -> &i64 {
        &self.min_htlc
    }

    fn mut_min_htlc_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_htlc
    }

    // int64 fee_base_msat = 3;

    pub fn clear_fee_base_msat(&mut self) {
        self.fee_base_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_base_msat(&mut self, v: i64) {
        self.fee_base_msat = v;
    }

    pub fn get_fee_base_msat(&self) -> i64 {
        self.fee_base_msat
    }

    fn get_fee_base_msat_for_reflect(&self) -> &i64 {
        &self.fee_base_msat
    }

    fn mut_fee_base_msat_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee_base_msat
    }

    // int64 fee_rate_milli_msat = 4;

    pub fn clear_fee_rate_milli_msat(&mut self) {
        self.fee_rate_milli_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_rate_milli_msat(&mut self, v: i64) {
        self.fee_rate_milli_msat = v;
    }

    pub fn get_fee_rate_milli_msat(&self) -> i64 {
        self.fee_rate_milli_msat
    }

    fn get_fee_rate_milli_msat_for_reflect(&self) -> &i64 {
        &self.fee_rate_milli_msat
    }

    fn mut_fee_rate_milli_msat_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee_rate_milli_msat
    }
}

impl ::protobuf::Message for RoutingPolicy {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_uint32()?;
                    self.time_lock_delta = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.min_htlc = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee_base_msat = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee_rate_milli_msat = tmp;
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
        if self.time_lock_delta != 0 {
            my_size += ::protobuf::rt::value_size(1, self.time_lock_delta, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.min_htlc != 0 {
            my_size += ::protobuf::rt::value_size(2, self.min_htlc, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_base_msat != 0 {
            my_size += ::protobuf::rt::value_size(3, self.fee_base_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_rate_milli_msat != 0 {
            my_size += ::protobuf::rt::value_size(4, self.fee_rate_milli_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.time_lock_delta != 0 {
            os.write_uint32(1, self.time_lock_delta)?;
        }
        if self.min_htlc != 0 {
            os.write_int64(2, self.min_htlc)?;
        }
        if self.fee_base_msat != 0 {
            os.write_int64(3, self.fee_base_msat)?;
        }
        if self.fee_rate_milli_msat != 0 {
            os.write_int64(4, self.fee_rate_milli_msat)?;
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

impl ::protobuf::MessageStatic for RoutingPolicy {
    fn new() -> RoutingPolicy {
        RoutingPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoutingPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_lock_delta",
                    RoutingPolicy::get_time_lock_delta_for_reflect,
                    RoutingPolicy::mut_time_lock_delta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_htlc",
                    RoutingPolicy::get_min_htlc_for_reflect,
                    RoutingPolicy::mut_min_htlc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee_base_msat",
                    RoutingPolicy::get_fee_base_msat_for_reflect,
                    RoutingPolicy::mut_fee_base_msat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee_rate_milli_msat",
                    RoutingPolicy::get_fee_rate_milli_msat_for_reflect,
                    RoutingPolicy::mut_fee_rate_milli_msat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoutingPolicy>(
                    "RoutingPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoutingPolicy {
    fn clear(&mut self) {
        self.clear_time_lock_delta();
        self.clear_min_htlc();
        self.clear_fee_base_msat();
        self.clear_fee_rate_milli_msat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoutingPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoutingPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelEdge {
    // message fields
    pub channel_id: u64,
    pub chan_point: ::std::string::String,
    pub last_update: u32,
    pub node1_pub: ::std::string::String,
    pub node2_pub: ::std::string::String,
    pub capacity: i64,
    pub node1_policy: ::protobuf::SingularPtrField<RoutingPolicy>,
    pub node2_policy: ::protobuf::SingularPtrField<RoutingPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelEdge {}

impl ChannelEdge {
    pub fn new() -> ChannelEdge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelEdge {
        static mut instance: ::protobuf::lazy::Lazy<ChannelEdge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelEdge,
        };
        unsafe {
            instance.get(ChannelEdge::new)
        }
    }

    // uint64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = v;
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id
    }

    fn get_channel_id_for_reflect(&self) -> &u64 {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.channel_id
    }

    // string chan_point = 2;

    pub fn clear_chan_point(&mut self) {
        self.chan_point.clear();
    }

    // Param is passed by value, moved
    pub fn set_chan_point(&mut self, v: ::std::string::String) {
        self.chan_point = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chan_point(&mut self) -> &mut ::std::string::String {
        &mut self.chan_point
    }

    // Take field
    pub fn take_chan_point(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chan_point, ::std::string::String::new())
    }

    pub fn get_chan_point(&self) -> &str {
        &self.chan_point
    }

    fn get_chan_point_for_reflect(&self) -> &::std::string::String {
        &self.chan_point
    }

    fn mut_chan_point_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.chan_point
    }

    // uint32 last_update = 3;

    pub fn clear_last_update(&mut self) {
        self.last_update = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_update(&mut self, v: u32) {
        self.last_update = v;
    }

    pub fn get_last_update(&self) -> u32 {
        self.last_update
    }

    fn get_last_update_for_reflect(&self) -> &u32 {
        &self.last_update
    }

    fn mut_last_update_for_reflect(&mut self) -> &mut u32 {
        &mut self.last_update
    }

    // string node1_pub = 4;

    pub fn clear_node1_pub(&mut self) {
        self.node1_pub.clear();
    }

    // Param is passed by value, moved
    pub fn set_node1_pub(&mut self, v: ::std::string::String) {
        self.node1_pub = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node1_pub(&mut self) -> &mut ::std::string::String {
        &mut self.node1_pub
    }

    // Take field
    pub fn take_node1_pub(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node1_pub, ::std::string::String::new())
    }

    pub fn get_node1_pub(&self) -> &str {
        &self.node1_pub
    }

    fn get_node1_pub_for_reflect(&self) -> &::std::string::String {
        &self.node1_pub
    }

    fn mut_node1_pub_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node1_pub
    }

    // string node2_pub = 5;

    pub fn clear_node2_pub(&mut self) {
        self.node2_pub.clear();
    }

    // Param is passed by value, moved
    pub fn set_node2_pub(&mut self, v: ::std::string::String) {
        self.node2_pub = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node2_pub(&mut self) -> &mut ::std::string::String {
        &mut self.node2_pub
    }

    // Take field
    pub fn take_node2_pub(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node2_pub, ::std::string::String::new())
    }

    pub fn get_node2_pub(&self) -> &str {
        &self.node2_pub
    }

    fn get_node2_pub_for_reflect(&self) -> &::std::string::String {
        &self.node2_pub
    }

    fn mut_node2_pub_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node2_pub
    }

    // int64 capacity = 6;

    pub fn clear_capacity(&mut self) {
        self.capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: i64) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> i64 {
        self.capacity
    }

    fn get_capacity_for_reflect(&self) -> &i64 {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.capacity
    }

    // .lnrpc.RoutingPolicy node1_policy = 7;

    pub fn clear_node1_policy(&mut self) {
        self.node1_policy.clear();
    }

    pub fn has_node1_policy(&self) -> bool {
        self.node1_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node1_policy(&mut self, v: RoutingPolicy) {
        self.node1_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node1_policy(&mut self) -> &mut RoutingPolicy {
        if self.node1_policy.is_none() {
            self.node1_policy.set_default();
        }
        self.node1_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_node1_policy(&mut self) -> RoutingPolicy {
        self.node1_policy.take().unwrap_or_else(|| RoutingPolicy::new())
    }

    pub fn get_node1_policy(&self) -> &RoutingPolicy {
        self.node1_policy.as_ref().unwrap_or_else(|| RoutingPolicy::default_instance())
    }

    fn get_node1_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<RoutingPolicy> {
        &self.node1_policy
    }

    fn mut_node1_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RoutingPolicy> {
        &mut self.node1_policy
    }

    // .lnrpc.RoutingPolicy node2_policy = 8;

    pub fn clear_node2_policy(&mut self) {
        self.node2_policy.clear();
    }

    pub fn has_node2_policy(&self) -> bool {
        self.node2_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node2_policy(&mut self, v: RoutingPolicy) {
        self.node2_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node2_policy(&mut self) -> &mut RoutingPolicy {
        if self.node2_policy.is_none() {
            self.node2_policy.set_default();
        }
        self.node2_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_node2_policy(&mut self) -> RoutingPolicy {
        self.node2_policy.take().unwrap_or_else(|| RoutingPolicy::new())
    }

    pub fn get_node2_policy(&self) -> &RoutingPolicy {
        self.node2_policy.as_ref().unwrap_or_else(|| RoutingPolicy::default_instance())
    }

    fn get_node2_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<RoutingPolicy> {
        &self.node2_policy
    }

    fn mut_node2_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RoutingPolicy> {
        &mut self.node2_policy
    }
}

impl ::protobuf::Message for ChannelEdge {
    fn is_initialized(&self) -> bool {
        for v in &self.node1_policy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node2_policy {
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
                    self.channel_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chan_point)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.last_update = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node1_pub)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node2_pub)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.capacity = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node1_policy)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node2_policy)?;
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
        if self.channel_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.channel_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.chan_point.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.chan_point);
        }
        if self.last_update != 0 {
            my_size += ::protobuf::rt::value_size(3, self.last_update, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.node1_pub.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.node1_pub);
        }
        if !self.node2_pub.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.node2_pub);
        }
        if self.capacity != 0 {
            my_size += ::protobuf::rt::value_size(6, self.capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.node1_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.node2_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.channel_id != 0 {
            os.write_uint64(1, self.channel_id)?;
        }
        if !self.chan_point.is_empty() {
            os.write_string(2, &self.chan_point)?;
        }
        if self.last_update != 0 {
            os.write_uint32(3, self.last_update)?;
        }
        if !self.node1_pub.is_empty() {
            os.write_string(4, &self.node1_pub)?;
        }
        if !self.node2_pub.is_empty() {
            os.write_string(5, &self.node2_pub)?;
        }
        if self.capacity != 0 {
            os.write_int64(6, self.capacity)?;
        }
        if let Some(ref v) = self.node1_policy.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.node2_policy.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ChannelEdge {
    fn new() -> ChannelEdge {
        ChannelEdge::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelEdge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    ChannelEdge::get_channel_id_for_reflect,
                    ChannelEdge::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chan_point",
                    ChannelEdge::get_chan_point_for_reflect,
                    ChannelEdge::mut_chan_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_update",
                    ChannelEdge::get_last_update_for_reflect,
                    ChannelEdge::mut_last_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node1_pub",
                    ChannelEdge::get_node1_pub_for_reflect,
                    ChannelEdge::mut_node1_pub_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node2_pub",
                    ChannelEdge::get_node2_pub_for_reflect,
                    ChannelEdge::mut_node2_pub_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "capacity",
                    ChannelEdge::get_capacity_for_reflect,
                    ChannelEdge::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoutingPolicy>>(
                    "node1_policy",
                    ChannelEdge::get_node1_policy_for_reflect,
                    ChannelEdge::mut_node1_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoutingPolicy>>(
                    "node2_policy",
                    ChannelEdge::get_node2_policy_for_reflect,
                    ChannelEdge::mut_node2_policy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelEdge>(
                    "ChannelEdge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelEdge {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_chan_point();
        self.clear_last_update();
        self.clear_node1_pub();
        self.clear_node2_pub();
        self.clear_capacity();
        self.clear_node1_policy();
        self.clear_node2_policy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelEdge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelEdge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelGraphRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelGraphRequest {}

impl ChannelGraphRequest {
    pub fn new() -> ChannelGraphRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelGraphRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChannelGraphRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelGraphRequest,
        };
        unsafe {
            instance.get(ChannelGraphRequest::new)
        }
    }
}

impl ::protobuf::Message for ChannelGraphRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ChannelGraphRequest {
    fn new() -> ChannelGraphRequest {
        ChannelGraphRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelGraphRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ChannelGraphRequest>(
                    "ChannelGraphRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelGraphRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelGraphRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelGraphRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelGraph {
    // message fields
    pub nodes: ::protobuf::RepeatedField<LightningNode>,
    pub edges: ::protobuf::RepeatedField<ChannelEdge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelGraph {}

impl ChannelGraph {
    pub fn new() -> ChannelGraph {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelGraph {
        static mut instance: ::protobuf::lazy::Lazy<ChannelGraph> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelGraph,
        };
        unsafe {
            instance.get(ChannelGraph::new)
        }
    }

    // repeated .lnrpc.LightningNode nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<LightningNode>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<LightningNode> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<LightningNode> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[LightningNode] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<LightningNode> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LightningNode> {
        &mut self.nodes
    }

    // repeated .lnrpc.ChannelEdge edges = 2;

    pub fn clear_edges(&mut self) {
        self.edges.clear();
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: ::protobuf::RepeatedField<ChannelEdge>) {
        self.edges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut ::protobuf::RepeatedField<ChannelEdge> {
        &mut self.edges
    }

    // Take field
    pub fn take_edges(&mut self) -> ::protobuf::RepeatedField<ChannelEdge> {
        ::std::mem::replace(&mut self.edges, ::protobuf::RepeatedField::new())
    }

    pub fn get_edges(&self) -> &[ChannelEdge] {
        &self.edges
    }

    fn get_edges_for_reflect(&self) -> &::protobuf::RepeatedField<ChannelEdge> {
        &self.edges
    }

    fn mut_edges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChannelEdge> {
        &mut self.edges
    }
}

impl ::protobuf::Message for ChannelGraph {
    fn is_initialized(&self) -> bool {
        for v in &self.nodes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.edges {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.edges)?;
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
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.edges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.edges {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ChannelGraph {
    fn new() -> ChannelGraph {
        ChannelGraph::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelGraph>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LightningNode>>(
                    "nodes",
                    ChannelGraph::get_nodes_for_reflect,
                    ChannelGraph::mut_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelEdge>>(
                    "edges",
                    ChannelGraph::get_edges_for_reflect,
                    ChannelGraph::mut_edges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelGraph>(
                    "ChannelGraph",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelGraph {
    fn clear(&mut self) {
        self.clear_nodes();
        self.clear_edges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelGraph {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelGraph {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChanInfoRequest {
    // message fields
    pub chan_id: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChanInfoRequest {}

impl ChanInfoRequest {
    pub fn new() -> ChanInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChanInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChanInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChanInfoRequest,
        };
        unsafe {
            instance.get(ChanInfoRequest::new)
        }
    }

    // uint64 chan_id = 1;

    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }

    fn get_chan_id_for_reflect(&self) -> &u64 {
        &self.chan_id
    }

    fn mut_chan_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.chan_id
    }
}

impl ::protobuf::Message for ChanInfoRequest {
    fn is_initialized(&self) -> bool {
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
                    self.chan_id = tmp;
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
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chan_id != 0 {
            os.write_uint64(1, self.chan_id)?;
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

impl ::protobuf::MessageStatic for ChanInfoRequest {
    fn new() -> ChanInfoRequest {
        ChanInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChanInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chan_id",
                    ChanInfoRequest::get_chan_id_for_reflect,
                    ChanInfoRequest::mut_chan_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChanInfoRequest>(
                    "ChanInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChanInfoRequest {
    fn clear(&mut self) {
        self.clear_chan_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChanInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChanInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkInfoRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkInfoRequest {}

impl NetworkInfoRequest {
    pub fn new() -> NetworkInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<NetworkInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkInfoRequest,
        };
        unsafe {
            instance.get(NetworkInfoRequest::new)
        }
    }
}

impl ::protobuf::Message for NetworkInfoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for NetworkInfoRequest {
    fn new() -> NetworkInfoRequest {
        NetworkInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<NetworkInfoRequest>(
                    "NetworkInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkInfoRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkInfo {
    // message fields
    pub graph_diameter: u32,
    pub avg_out_degree: f64,
    pub max_out_degree: u32,
    pub num_nodes: u32,
    pub num_channels: u32,
    pub total_network_capacity: i64,
    pub avg_channel_size: f64,
    pub min_channel_size: i64,
    pub max_channel_size: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkInfo {}

impl NetworkInfo {
    pub fn new() -> NetworkInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkInfo {
        static mut instance: ::protobuf::lazy::Lazy<NetworkInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkInfo,
        };
        unsafe {
            instance.get(NetworkInfo::new)
        }
    }

    // uint32 graph_diameter = 1;

    pub fn clear_graph_diameter(&mut self) {
        self.graph_diameter = 0;
    }

    // Param is passed by value, moved
    pub fn set_graph_diameter(&mut self, v: u32) {
        self.graph_diameter = v;
    }

    pub fn get_graph_diameter(&self) -> u32 {
        self.graph_diameter
    }

    fn get_graph_diameter_for_reflect(&self) -> &u32 {
        &self.graph_diameter
    }

    fn mut_graph_diameter_for_reflect(&mut self) -> &mut u32 {
        &mut self.graph_diameter
    }

    // double avg_out_degree = 2;

    pub fn clear_avg_out_degree(&mut self) {
        self.avg_out_degree = 0.;
    }

    // Param is passed by value, moved
    pub fn set_avg_out_degree(&mut self, v: f64) {
        self.avg_out_degree = v;
    }

    pub fn get_avg_out_degree(&self) -> f64 {
        self.avg_out_degree
    }

    fn get_avg_out_degree_for_reflect(&self) -> &f64 {
        &self.avg_out_degree
    }

    fn mut_avg_out_degree_for_reflect(&mut self) -> &mut f64 {
        &mut self.avg_out_degree
    }

    // uint32 max_out_degree = 3;

    pub fn clear_max_out_degree(&mut self) {
        self.max_out_degree = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_out_degree(&mut self, v: u32) {
        self.max_out_degree = v;
    }

    pub fn get_max_out_degree(&self) -> u32 {
        self.max_out_degree
    }

    fn get_max_out_degree_for_reflect(&self) -> &u32 {
        &self.max_out_degree
    }

    fn mut_max_out_degree_for_reflect(&mut self) -> &mut u32 {
        &mut self.max_out_degree
    }

    // uint32 num_nodes = 4;

    pub fn clear_num_nodes(&mut self) {
        self.num_nodes = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_nodes(&mut self, v: u32) {
        self.num_nodes = v;
    }

    pub fn get_num_nodes(&self) -> u32 {
        self.num_nodes
    }

    fn get_num_nodes_for_reflect(&self) -> &u32 {
        &self.num_nodes
    }

    fn mut_num_nodes_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_nodes
    }

    // uint32 num_channels = 5;

    pub fn clear_num_channels(&mut self) {
        self.num_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_channels(&mut self, v: u32) {
        self.num_channels = v;
    }

    pub fn get_num_channels(&self) -> u32 {
        self.num_channels
    }

    fn get_num_channels_for_reflect(&self) -> &u32 {
        &self.num_channels
    }

    fn mut_num_channels_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_channels
    }

    // int64 total_network_capacity = 6;

    pub fn clear_total_network_capacity(&mut self) {
        self.total_network_capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_network_capacity(&mut self, v: i64) {
        self.total_network_capacity = v;
    }

    pub fn get_total_network_capacity(&self) -> i64 {
        self.total_network_capacity
    }

    fn get_total_network_capacity_for_reflect(&self) -> &i64 {
        &self.total_network_capacity
    }

    fn mut_total_network_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_network_capacity
    }

    // double avg_channel_size = 7;

    pub fn clear_avg_channel_size(&mut self) {
        self.avg_channel_size = 0.;
    }

    // Param is passed by value, moved
    pub fn set_avg_channel_size(&mut self, v: f64) {
        self.avg_channel_size = v;
    }

    pub fn get_avg_channel_size(&self) -> f64 {
        self.avg_channel_size
    }

    fn get_avg_channel_size_for_reflect(&self) -> &f64 {
        &self.avg_channel_size
    }

    fn mut_avg_channel_size_for_reflect(&mut self) -> &mut f64 {
        &mut self.avg_channel_size
    }

    // int64 min_channel_size = 8;

    pub fn clear_min_channel_size(&mut self) {
        self.min_channel_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_channel_size(&mut self, v: i64) {
        self.min_channel_size = v;
    }

    pub fn get_min_channel_size(&self) -> i64 {
        self.min_channel_size
    }

    fn get_min_channel_size_for_reflect(&self) -> &i64 {
        &self.min_channel_size
    }

    fn mut_min_channel_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_channel_size
    }

    // int64 max_channel_size = 9;

    pub fn clear_max_channel_size(&mut self) {
        self.max_channel_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_channel_size(&mut self, v: i64) {
        self.max_channel_size = v;
    }

    pub fn get_max_channel_size(&self) -> i64 {
        self.max_channel_size
    }

    fn get_max_channel_size_for_reflect(&self) -> &i64 {
        &self.max_channel_size
    }

    fn mut_max_channel_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_channel_size
    }
}

impl ::protobuf::Message for NetworkInfo {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_uint32()?;
                    self.graph_diameter = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.avg_out_degree = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_out_degree = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_nodes = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_channels = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_network_capacity = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.avg_channel_size = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.min_channel_size = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_channel_size = tmp;
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
        if self.graph_diameter != 0 {
            my_size += ::protobuf::rt::value_size(1, self.graph_diameter, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.avg_out_degree != 0. {
            my_size += 9;
        }
        if self.max_out_degree != 0 {
            my_size += ::protobuf::rt::value_size(3, self.max_out_degree, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_nodes != 0 {
            my_size += ::protobuf::rt::value_size(4, self.num_nodes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_channels != 0 {
            my_size += ::protobuf::rt::value_size(5, self.num_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_network_capacity != 0 {
            my_size += ::protobuf::rt::value_size(6, self.total_network_capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.avg_channel_size != 0. {
            my_size += 9;
        }
        if self.min_channel_size != 0 {
            my_size += ::protobuf::rt::value_size(8, self.min_channel_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_channel_size != 0 {
            my_size += ::protobuf::rt::value_size(9, self.max_channel_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.graph_diameter != 0 {
            os.write_uint32(1, self.graph_diameter)?;
        }
        if self.avg_out_degree != 0. {
            os.write_double(2, self.avg_out_degree)?;
        }
        if self.max_out_degree != 0 {
            os.write_uint32(3, self.max_out_degree)?;
        }
        if self.num_nodes != 0 {
            os.write_uint32(4, self.num_nodes)?;
        }
        if self.num_channels != 0 {
            os.write_uint32(5, self.num_channels)?;
        }
        if self.total_network_capacity != 0 {
            os.write_int64(6, self.total_network_capacity)?;
        }
        if self.avg_channel_size != 0. {
            os.write_double(7, self.avg_channel_size)?;
        }
        if self.min_channel_size != 0 {
            os.write_int64(8, self.min_channel_size)?;
        }
        if self.max_channel_size != 0 {
            os.write_int64(9, self.max_channel_size)?;
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

impl ::protobuf::MessageStatic for NetworkInfo {
    fn new() -> NetworkInfo {
        NetworkInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "graph_diameter",
                    NetworkInfo::get_graph_diameter_for_reflect,
                    NetworkInfo::mut_graph_diameter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "avg_out_degree",
                    NetworkInfo::get_avg_out_degree_for_reflect,
                    NetworkInfo::mut_avg_out_degree_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_out_degree",
                    NetworkInfo::get_max_out_degree_for_reflect,
                    NetworkInfo::mut_max_out_degree_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_nodes",
                    NetworkInfo::get_num_nodes_for_reflect,
                    NetworkInfo::mut_num_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_channels",
                    NetworkInfo::get_num_channels_for_reflect,
                    NetworkInfo::mut_num_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_network_capacity",
                    NetworkInfo::get_total_network_capacity_for_reflect,
                    NetworkInfo::mut_total_network_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "avg_channel_size",
                    NetworkInfo::get_avg_channel_size_for_reflect,
                    NetworkInfo::mut_avg_channel_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_channel_size",
                    NetworkInfo::get_min_channel_size_for_reflect,
                    NetworkInfo::mut_min_channel_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_channel_size",
                    NetworkInfo::get_max_channel_size_for_reflect,
                    NetworkInfo::mut_max_channel_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkInfo>(
                    "NetworkInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkInfo {
    fn clear(&mut self) {
        self.clear_graph_diameter();
        self.clear_avg_out_degree();
        self.clear_max_out_degree();
        self.clear_num_nodes();
        self.clear_num_channels();
        self.clear_total_network_capacity();
        self.clear_avg_channel_size();
        self.clear_min_channel_size();
        self.clear_max_channel_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StopRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StopRequest {}

impl StopRequest {
    pub fn new() -> StopRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StopRequest {
        static mut instance: ::protobuf::lazy::Lazy<StopRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StopRequest,
        };
        unsafe {
            instance.get(StopRequest::new)
        }
    }
}

impl ::protobuf::Message for StopRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for StopRequest {
    fn new() -> StopRequest {
        StopRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StopRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StopRequest>(
                    "StopRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StopRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StopRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StopResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StopResponse {}

impl StopResponse {
    pub fn new() -> StopResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StopResponse {
        static mut instance: ::protobuf::lazy::Lazy<StopResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StopResponse,
        };
        unsafe {
            instance.get(StopResponse::new)
        }
    }
}

impl ::protobuf::Message for StopResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for StopResponse {
    fn new() -> StopResponse {
        StopResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StopResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StopResponse>(
                    "StopResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StopResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StopResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTopologySubscription {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTopologySubscription {}

impl GraphTopologySubscription {
    pub fn new() -> GraphTopologySubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTopologySubscription {
        static mut instance: ::protobuf::lazy::Lazy<GraphTopologySubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTopologySubscription,
        };
        unsafe {
            instance.get(GraphTopologySubscription::new)
        }
    }
}

impl ::protobuf::Message for GraphTopologySubscription {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for GraphTopologySubscription {
    fn new() -> GraphTopologySubscription {
        GraphTopologySubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTopologySubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GraphTopologySubscription>(
                    "GraphTopologySubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTopologySubscription {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTopologySubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTopologySubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTopologyUpdate {
    // message fields
    pub node_updates: ::protobuf::RepeatedField<NodeUpdate>,
    pub channel_updates: ::protobuf::RepeatedField<ChannelEdgeUpdate>,
    pub closed_chans: ::protobuf::RepeatedField<ClosedChannelUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTopologyUpdate {}

impl GraphTopologyUpdate {
    pub fn new() -> GraphTopologyUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTopologyUpdate {
        static mut instance: ::protobuf::lazy::Lazy<GraphTopologyUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTopologyUpdate,
        };
        unsafe {
            instance.get(GraphTopologyUpdate::new)
        }
    }

    // repeated .lnrpc.NodeUpdate node_updates = 1;

    pub fn clear_node_updates(&mut self) {
        self.node_updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_updates(&mut self, v: ::protobuf::RepeatedField<NodeUpdate>) {
        self.node_updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_updates(&mut self) -> &mut ::protobuf::RepeatedField<NodeUpdate> {
        &mut self.node_updates
    }

    // Take field
    pub fn take_node_updates(&mut self) -> ::protobuf::RepeatedField<NodeUpdate> {
        ::std::mem::replace(&mut self.node_updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_updates(&self) -> &[NodeUpdate] {
        &self.node_updates
    }

    fn get_node_updates_for_reflect(&self) -> &::protobuf::RepeatedField<NodeUpdate> {
        &self.node_updates
    }

    fn mut_node_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NodeUpdate> {
        &mut self.node_updates
    }

    // repeated .lnrpc.ChannelEdgeUpdate channel_updates = 2;

    pub fn clear_channel_updates(&mut self) {
        self.channel_updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_updates(&mut self, v: ::protobuf::RepeatedField<ChannelEdgeUpdate>) {
        self.channel_updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channel_updates(&mut self) -> &mut ::protobuf::RepeatedField<ChannelEdgeUpdate> {
        &mut self.channel_updates
    }

    // Take field
    pub fn take_channel_updates(&mut self) -> ::protobuf::RepeatedField<ChannelEdgeUpdate> {
        ::std::mem::replace(&mut self.channel_updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_channel_updates(&self) -> &[ChannelEdgeUpdate] {
        &self.channel_updates
    }

    fn get_channel_updates_for_reflect(&self) -> &::protobuf::RepeatedField<ChannelEdgeUpdate> {
        &self.channel_updates
    }

    fn mut_channel_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChannelEdgeUpdate> {
        &mut self.channel_updates
    }

    // repeated .lnrpc.ClosedChannelUpdate closed_chans = 3;

    pub fn clear_closed_chans(&mut self) {
        self.closed_chans.clear();
    }

    // Param is passed by value, moved
    pub fn set_closed_chans(&mut self, v: ::protobuf::RepeatedField<ClosedChannelUpdate>) {
        self.closed_chans = v;
    }

    // Mutable pointer to the field.
    pub fn mut_closed_chans(&mut self) -> &mut ::protobuf::RepeatedField<ClosedChannelUpdate> {
        &mut self.closed_chans
    }

    // Take field
    pub fn take_closed_chans(&mut self) -> ::protobuf::RepeatedField<ClosedChannelUpdate> {
        ::std::mem::replace(&mut self.closed_chans, ::protobuf::RepeatedField::new())
    }

    pub fn get_closed_chans(&self) -> &[ClosedChannelUpdate] {
        &self.closed_chans
    }

    fn get_closed_chans_for_reflect(&self) -> &::protobuf::RepeatedField<ClosedChannelUpdate> {
        &self.closed_chans
    }

    fn mut_closed_chans_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClosedChannelUpdate> {
        &mut self.closed_chans
    }
}

impl ::protobuf::Message for GraphTopologyUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.node_updates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.channel_updates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.closed_chans {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_updates)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channel_updates)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.closed_chans)?;
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
        for value in &self.node_updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.channel_updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.closed_chans {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.node_updates {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.channel_updates {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.closed_chans {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GraphTopologyUpdate {
    fn new() -> GraphTopologyUpdate {
        GraphTopologyUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTopologyUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NodeUpdate>>(
                    "node_updates",
                    GraphTopologyUpdate::get_node_updates_for_reflect,
                    GraphTopologyUpdate::mut_node_updates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelEdgeUpdate>>(
                    "channel_updates",
                    GraphTopologyUpdate::get_channel_updates_for_reflect,
                    GraphTopologyUpdate::mut_channel_updates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClosedChannelUpdate>>(
                    "closed_chans",
                    GraphTopologyUpdate::get_closed_chans_for_reflect,
                    GraphTopologyUpdate::mut_closed_chans_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTopologyUpdate>(
                    "GraphTopologyUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTopologyUpdate {
    fn clear(&mut self) {
        self.clear_node_updates();
        self.clear_channel_updates();
        self.clear_closed_chans();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTopologyUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTopologyUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeUpdate {
    // message fields
    pub addresses: ::protobuf::RepeatedField<::std::string::String>,
    pub identity_key: ::std::string::String,
    pub global_features: ::std::vec::Vec<u8>,
    pub alias: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeUpdate {}

impl NodeUpdate {
    pub fn new() -> NodeUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeUpdate {
        static mut instance: ::protobuf::lazy::Lazy<NodeUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeUpdate,
        };
        unsafe {
            instance.get(NodeUpdate::new)
        }
    }

    // repeated string addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[::std::string::String] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // string identity_key = 2;

    pub fn clear_identity_key(&mut self) {
        self.identity_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_identity_key(&mut self, v: ::std::string::String) {
        self.identity_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identity_key(&mut self) -> &mut ::std::string::String {
        &mut self.identity_key
    }

    // Take field
    pub fn take_identity_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.identity_key, ::std::string::String::new())
    }

    pub fn get_identity_key(&self) -> &str {
        &self.identity_key
    }

    fn get_identity_key_for_reflect(&self) -> &::std::string::String {
        &self.identity_key
    }

    fn mut_identity_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.identity_key
    }

    // bytes global_features = 3;

    pub fn clear_global_features(&mut self) {
        self.global_features.clear();
    }

    // Param is passed by value, moved
    pub fn set_global_features(&mut self, v: ::std::vec::Vec<u8>) {
        self.global_features = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_global_features(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.global_features
    }

    // Take field
    pub fn take_global_features(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.global_features, ::std::vec::Vec::new())
    }

    pub fn get_global_features(&self) -> &[u8] {
        &self.global_features
    }

    fn get_global_features_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.global_features
    }

    fn mut_global_features_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.global_features
    }

    // string alias = 4;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }
}

impl ::protobuf::Message for NodeUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.addresses)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.identity_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.global_features)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
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
        for value in &self.addresses {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if !self.identity_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.identity_key);
        }
        if !self.global_features.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.global_features);
        }
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.alias);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
            os.write_string(1, &v)?;
        };
        if !self.identity_key.is_empty() {
            os.write_string(2, &self.identity_key)?;
        }
        if !self.global_features.is_empty() {
            os.write_bytes(3, &self.global_features)?;
        }
        if !self.alias.is_empty() {
            os.write_string(4, &self.alias)?;
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

impl ::protobuf::MessageStatic for NodeUpdate {
    fn new() -> NodeUpdate {
        NodeUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addresses",
                    NodeUpdate::get_addresses_for_reflect,
                    NodeUpdate::mut_addresses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identity_key",
                    NodeUpdate::get_identity_key_for_reflect,
                    NodeUpdate::mut_identity_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "global_features",
                    NodeUpdate::get_global_features_for_reflect,
                    NodeUpdate::mut_global_features_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    NodeUpdate::get_alias_for_reflect,
                    NodeUpdate::mut_alias_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeUpdate>(
                    "NodeUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeUpdate {
    fn clear(&mut self) {
        self.clear_addresses();
        self.clear_identity_key();
        self.clear_global_features();
        self.clear_alias();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelEdgeUpdate {
    // message fields
    pub chan_id: u64,
    pub chan_point: ::protobuf::SingularPtrField<ChannelPoint>,
    pub capacity: i64,
    pub routing_policy: ::protobuf::SingularPtrField<RoutingPolicy>,
    pub advertising_node: ::std::string::String,
    pub connecting_node: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelEdgeUpdate {}

impl ChannelEdgeUpdate {
    pub fn new() -> ChannelEdgeUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelEdgeUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ChannelEdgeUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelEdgeUpdate,
        };
        unsafe {
            instance.get(ChannelEdgeUpdate::new)
        }
    }

    // uint64 chan_id = 1;

    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }

    fn get_chan_id_for_reflect(&self) -> &u64 {
        &self.chan_id
    }

    fn mut_chan_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.chan_id
    }

    // .lnrpc.ChannelPoint chan_point = 2;

    pub fn clear_chan_point(&mut self) {
        self.chan_point.clear();
    }

    pub fn has_chan_point(&self) -> bool {
        self.chan_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chan_point(&mut self, v: ChannelPoint) {
        self.chan_point = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chan_point(&mut self) -> &mut ChannelPoint {
        if self.chan_point.is_none() {
            self.chan_point.set_default();
        }
        self.chan_point.as_mut().unwrap()
    }

    // Take field
    pub fn take_chan_point(&mut self) -> ChannelPoint {
        self.chan_point.take().unwrap_or_else(|| ChannelPoint::new())
    }

    pub fn get_chan_point(&self) -> &ChannelPoint {
        self.chan_point.as_ref().unwrap_or_else(|| ChannelPoint::default_instance())
    }

    fn get_chan_point_for_reflect(&self) -> &::protobuf::SingularPtrField<ChannelPoint> {
        &self.chan_point
    }

    fn mut_chan_point_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChannelPoint> {
        &mut self.chan_point
    }

    // int64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: i64) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> i64 {
        self.capacity
    }

    fn get_capacity_for_reflect(&self) -> &i64 {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.capacity
    }

    // .lnrpc.RoutingPolicy routing_policy = 4;

    pub fn clear_routing_policy(&mut self) {
        self.routing_policy.clear();
    }

    pub fn has_routing_policy(&self) -> bool {
        self.routing_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing_policy(&mut self, v: RoutingPolicy) {
        self.routing_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routing_policy(&mut self) -> &mut RoutingPolicy {
        if self.routing_policy.is_none() {
            self.routing_policy.set_default();
        }
        self.routing_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_routing_policy(&mut self) -> RoutingPolicy {
        self.routing_policy.take().unwrap_or_else(|| RoutingPolicy::new())
    }

    pub fn get_routing_policy(&self) -> &RoutingPolicy {
        self.routing_policy.as_ref().unwrap_or_else(|| RoutingPolicy::default_instance())
    }

    fn get_routing_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<RoutingPolicy> {
        &self.routing_policy
    }

    fn mut_routing_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RoutingPolicy> {
        &mut self.routing_policy
    }

    // string advertising_node = 5;

    pub fn clear_advertising_node(&mut self) {
        self.advertising_node.clear();
    }

    // Param is passed by value, moved
    pub fn set_advertising_node(&mut self, v: ::std::string::String) {
        self.advertising_node = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_advertising_node(&mut self) -> &mut ::std::string::String {
        &mut self.advertising_node
    }

    // Take field
    pub fn take_advertising_node(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.advertising_node, ::std::string::String::new())
    }

    pub fn get_advertising_node(&self) -> &str {
        &self.advertising_node
    }

    fn get_advertising_node_for_reflect(&self) -> &::std::string::String {
        &self.advertising_node
    }

    fn mut_advertising_node_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.advertising_node
    }

    // string connecting_node = 6;

    pub fn clear_connecting_node(&mut self) {
        self.connecting_node.clear();
    }

    // Param is passed by value, moved
    pub fn set_connecting_node(&mut self, v: ::std::string::String) {
        self.connecting_node = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connecting_node(&mut self) -> &mut ::std::string::String {
        &mut self.connecting_node
    }

    // Take field
    pub fn take_connecting_node(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.connecting_node, ::std::string::String::new())
    }

    pub fn get_connecting_node(&self) -> &str {
        &self.connecting_node
    }

    fn get_connecting_node_for_reflect(&self) -> &::std::string::String {
        &self.connecting_node
    }

    fn mut_connecting_node_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.connecting_node
    }
}

impl ::protobuf::Message for ChannelEdgeUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.chan_point {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.routing_policy {
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
                    self.chan_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chan_point)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.capacity = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.routing_policy)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.advertising_node)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.connecting_node)?;
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
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.chan_point.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.capacity != 0 {
            my_size += ::protobuf::rt::value_size(3, self.capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.routing_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.advertising_node.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.advertising_node);
        }
        if !self.connecting_node.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.connecting_node);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chan_id != 0 {
            os.write_uint64(1, self.chan_id)?;
        }
        if let Some(ref v) = self.chan_point.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.capacity != 0 {
            os.write_int64(3, self.capacity)?;
        }
        if let Some(ref v) = self.routing_policy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.advertising_node.is_empty() {
            os.write_string(5, &self.advertising_node)?;
        }
        if !self.connecting_node.is_empty() {
            os.write_string(6, &self.connecting_node)?;
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

impl ::protobuf::MessageStatic for ChannelEdgeUpdate {
    fn new() -> ChannelEdgeUpdate {
        ChannelEdgeUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelEdgeUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chan_id",
                    ChannelEdgeUpdate::get_chan_id_for_reflect,
                    ChannelEdgeUpdate::mut_chan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelPoint>>(
                    "chan_point",
                    ChannelEdgeUpdate::get_chan_point_for_reflect,
                    ChannelEdgeUpdate::mut_chan_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "capacity",
                    ChannelEdgeUpdate::get_capacity_for_reflect,
                    ChannelEdgeUpdate::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoutingPolicy>>(
                    "routing_policy",
                    ChannelEdgeUpdate::get_routing_policy_for_reflect,
                    ChannelEdgeUpdate::mut_routing_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "advertising_node",
                    ChannelEdgeUpdate::get_advertising_node_for_reflect,
                    ChannelEdgeUpdate::mut_advertising_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "connecting_node",
                    ChannelEdgeUpdate::get_connecting_node_for_reflect,
                    ChannelEdgeUpdate::mut_connecting_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelEdgeUpdate>(
                    "ChannelEdgeUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelEdgeUpdate {
    fn clear(&mut self) {
        self.clear_chan_id();
        self.clear_chan_point();
        self.clear_capacity();
        self.clear_routing_policy();
        self.clear_advertising_node();
        self.clear_connecting_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelEdgeUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelEdgeUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClosedChannelUpdate {
    // message fields
    pub chan_id: u64,
    pub capacity: i64,
    pub closed_height: u32,
    pub chan_point: ::protobuf::SingularPtrField<ChannelPoint>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClosedChannelUpdate {}

impl ClosedChannelUpdate {
    pub fn new() -> ClosedChannelUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClosedChannelUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ClosedChannelUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClosedChannelUpdate,
        };
        unsafe {
            instance.get(ClosedChannelUpdate::new)
        }
    }

    // uint64 chan_id = 1;

    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }

    fn get_chan_id_for_reflect(&self) -> &u64 {
        &self.chan_id
    }

    fn mut_chan_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.chan_id
    }

    // int64 capacity = 2;

    pub fn clear_capacity(&mut self) {
        self.capacity = 0;
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: i64) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> i64 {
        self.capacity
    }

    fn get_capacity_for_reflect(&self) -> &i64 {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut i64 {
        &mut self.capacity
    }

    // uint32 closed_height = 3;

    pub fn clear_closed_height(&mut self) {
        self.closed_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_closed_height(&mut self, v: u32) {
        self.closed_height = v;
    }

    pub fn get_closed_height(&self) -> u32 {
        self.closed_height
    }

    fn get_closed_height_for_reflect(&self) -> &u32 {
        &self.closed_height
    }

    fn mut_closed_height_for_reflect(&mut self) -> &mut u32 {
        &mut self.closed_height
    }

    // .lnrpc.ChannelPoint chan_point = 4;

    pub fn clear_chan_point(&mut self) {
        self.chan_point.clear();
    }

    pub fn has_chan_point(&self) -> bool {
        self.chan_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chan_point(&mut self, v: ChannelPoint) {
        self.chan_point = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chan_point(&mut self) -> &mut ChannelPoint {
        if self.chan_point.is_none() {
            self.chan_point.set_default();
        }
        self.chan_point.as_mut().unwrap()
    }

    // Take field
    pub fn take_chan_point(&mut self) -> ChannelPoint {
        self.chan_point.take().unwrap_or_else(|| ChannelPoint::new())
    }

    pub fn get_chan_point(&self) -> &ChannelPoint {
        self.chan_point.as_ref().unwrap_or_else(|| ChannelPoint::default_instance())
    }

    fn get_chan_point_for_reflect(&self) -> &::protobuf::SingularPtrField<ChannelPoint> {
        &self.chan_point
    }

    fn mut_chan_point_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChannelPoint> {
        &mut self.chan_point
    }
}

impl ::protobuf::Message for ClosedChannelUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.chan_point {
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
                    self.chan_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.capacity = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.closed_height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chan_point)?;
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
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.capacity != 0 {
            my_size += ::protobuf::rt::value_size(2, self.capacity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.closed_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.closed_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.chan_point.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chan_id != 0 {
            os.write_uint64(1, self.chan_id)?;
        }
        if self.capacity != 0 {
            os.write_int64(2, self.capacity)?;
        }
        if self.closed_height != 0 {
            os.write_uint32(3, self.closed_height)?;
        }
        if let Some(ref v) = self.chan_point.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ClosedChannelUpdate {
    fn new() -> ClosedChannelUpdate {
        ClosedChannelUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClosedChannelUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chan_id",
                    ClosedChannelUpdate::get_chan_id_for_reflect,
                    ClosedChannelUpdate::mut_chan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "capacity",
                    ClosedChannelUpdate::get_capacity_for_reflect,
                    ClosedChannelUpdate::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "closed_height",
                    ClosedChannelUpdate::get_closed_height_for_reflect,
                    ClosedChannelUpdate::mut_closed_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelPoint>>(
                    "chan_point",
                    ClosedChannelUpdate::get_chan_point_for_reflect,
                    ClosedChannelUpdate::mut_chan_point_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClosedChannelUpdate>(
                    "ClosedChannelUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClosedChannelUpdate {
    fn clear(&mut self) {
        self.clear_chan_id();
        self.clear_capacity();
        self.clear_closed_height();
        self.clear_chan_point();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClosedChannelUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClosedChannelUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Invoice {
    // message fields
    pub memo: ::std::string::String,
    pub receipt: ::std::vec::Vec<u8>,
    pub r_preimage: ::std::vec::Vec<u8>,
    pub r_hash: ::std::vec::Vec<u8>,
    pub value: i64,
    pub settled: bool,
    pub creation_date: i64,
    pub settle_date: i64,
    pub payment_request: ::std::string::String,
    pub description_hash: ::std::vec::Vec<u8>,
    pub expiry: i64,
    pub fallback_addr: ::std::string::String,
    pub cltv_expiry: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Invoice {}

impl Invoice {
    pub fn new() -> Invoice {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Invoice {
        static mut instance: ::protobuf::lazy::Lazy<Invoice> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Invoice,
        };
        unsafe {
            instance.get(Invoice::new)
        }
    }

    // string memo = 1;

    pub fn clear_memo(&mut self) {
        self.memo.clear();
    }

    // Param is passed by value, moved
    pub fn set_memo(&mut self, v: ::std::string::String) {
        self.memo = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_memo(&mut self) -> &mut ::std::string::String {
        &mut self.memo
    }

    // Take field
    pub fn take_memo(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.memo, ::std::string::String::new())
    }

    pub fn get_memo(&self) -> &str {
        &self.memo
    }

    fn get_memo_for_reflect(&self) -> &::std::string::String {
        &self.memo
    }

    fn mut_memo_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.memo
    }

    // bytes receipt = 2;

    pub fn clear_receipt(&mut self) {
        self.receipt.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipt(&mut self, v: ::std::vec::Vec<u8>) {
        self.receipt = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipt(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipt
    }

    // Take field
    pub fn take_receipt(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.receipt, ::std::vec::Vec::new())
    }

    pub fn get_receipt(&self) -> &[u8] {
        &self.receipt
    }

    fn get_receipt_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.receipt
    }

    fn mut_receipt_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipt
    }

    // bytes r_preimage = 3;

    pub fn clear_r_preimage(&mut self) {
        self.r_preimage.clear();
    }

    // Param is passed by value, moved
    pub fn set_r_preimage(&mut self, v: ::std::vec::Vec<u8>) {
        self.r_preimage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r_preimage(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_preimage
    }

    // Take field
    pub fn take_r_preimage(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.r_preimage, ::std::vec::Vec::new())
    }

    pub fn get_r_preimage(&self) -> &[u8] {
        &self.r_preimage
    }

    fn get_r_preimage_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.r_preimage
    }

    fn mut_r_preimage_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_preimage
    }

    // bytes r_hash = 4;

    pub fn clear_r_hash(&mut self) {
        self.r_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_r_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.r_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }

    // Take field
    pub fn take_r_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.r_hash, ::std::vec::Vec::new())
    }

    pub fn get_r_hash(&self) -> &[u8] {
        &self.r_hash
    }

    fn get_r_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.r_hash
    }

    fn mut_r_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }

    // int64 value = 5;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.value
    }

    // bool settled = 6;

    pub fn clear_settled(&mut self) {
        self.settled = false;
    }

    // Param is passed by value, moved
    pub fn set_settled(&mut self, v: bool) {
        self.settled = v;
    }

    pub fn get_settled(&self) -> bool {
        self.settled
    }

    fn get_settled_for_reflect(&self) -> &bool {
        &self.settled
    }

    fn mut_settled_for_reflect(&mut self) -> &mut bool {
        &mut self.settled
    }

    // int64 creation_date = 7;

    pub fn clear_creation_date(&mut self) {
        self.creation_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_creation_date(&mut self, v: i64) {
        self.creation_date = v;
    }

    pub fn get_creation_date(&self) -> i64 {
        self.creation_date
    }

    fn get_creation_date_for_reflect(&self) -> &i64 {
        &self.creation_date
    }

    fn mut_creation_date_for_reflect(&mut self) -> &mut i64 {
        &mut self.creation_date
    }

    // int64 settle_date = 8;

    pub fn clear_settle_date(&mut self) {
        self.settle_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_settle_date(&mut self, v: i64) {
        self.settle_date = v;
    }

    pub fn get_settle_date(&self) -> i64 {
        self.settle_date
    }

    fn get_settle_date_for_reflect(&self) -> &i64 {
        &self.settle_date
    }

    fn mut_settle_date_for_reflect(&mut self) -> &mut i64 {
        &mut self.settle_date
    }

    // string payment_request = 9;

    pub fn clear_payment_request(&mut self) {
        self.payment_request.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_request(&mut self, v: ::std::string::String) {
        self.payment_request = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_request(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }

    // Take field
    pub fn take_payment_request(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_request, ::std::string::String::new())
    }

    pub fn get_payment_request(&self) -> &str {
        &self.payment_request
    }

    fn get_payment_request_for_reflect(&self) -> &::std::string::String {
        &self.payment_request
    }

    fn mut_payment_request_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }

    // bytes description_hash = 10;

    pub fn clear_description_hash(&mut self) {
        self.description_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_description_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.description_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.description_hash
    }

    // Take field
    pub fn take_description_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.description_hash, ::std::vec::Vec::new())
    }

    pub fn get_description_hash(&self) -> &[u8] {
        &self.description_hash
    }

    fn get_description_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.description_hash
    }

    fn mut_description_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.description_hash
    }

    // int64 expiry = 11;

    pub fn clear_expiry(&mut self) {
        self.expiry = 0;
    }

    // Param is passed by value, moved
    pub fn set_expiry(&mut self, v: i64) {
        self.expiry = v;
    }

    pub fn get_expiry(&self) -> i64 {
        self.expiry
    }

    fn get_expiry_for_reflect(&self) -> &i64 {
        &self.expiry
    }

    fn mut_expiry_for_reflect(&mut self) -> &mut i64 {
        &mut self.expiry
    }

    // string fallback_addr = 12;

    pub fn clear_fallback_addr(&mut self) {
        self.fallback_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_fallback_addr(&mut self, v: ::std::string::String) {
        self.fallback_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fallback_addr(&mut self) -> &mut ::std::string::String {
        &mut self.fallback_addr
    }

    // Take field
    pub fn take_fallback_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.fallback_addr, ::std::string::String::new())
    }

    pub fn get_fallback_addr(&self) -> &str {
        &self.fallback_addr
    }

    fn get_fallback_addr_for_reflect(&self) -> &::std::string::String {
        &self.fallback_addr
    }

    fn mut_fallback_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.fallback_addr
    }

    // uint64 cltv_expiry = 13;

    pub fn clear_cltv_expiry(&mut self) {
        self.cltv_expiry = 0;
    }

    // Param is passed by value, moved
    pub fn set_cltv_expiry(&mut self, v: u64) {
        self.cltv_expiry = v;
    }

    pub fn get_cltv_expiry(&self) -> u64 {
        self.cltv_expiry
    }

    fn get_cltv_expiry_for_reflect(&self) -> &u64 {
        &self.cltv_expiry
    }

    fn mut_cltv_expiry_for_reflect(&mut self) -> &mut u64 {
        &mut self.cltv_expiry
    }
}

impl ::protobuf::Message for Invoice {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.memo)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.receipt)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.r_preimage)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.r_hash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.value = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.settled = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.creation_date = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.settle_date = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_request)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.description_hash)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expiry = tmp;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.fallback_addr)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cltv_expiry = tmp;
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
        if !self.memo.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.memo);
        }
        if !self.receipt.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.receipt);
        }
        if !self.r_preimage.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.r_preimage);
        }
        if !self.r_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.r_hash);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(5, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.settled != false {
            my_size += 2;
        }
        if self.creation_date != 0 {
            my_size += ::protobuf::rt::value_size(7, self.creation_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.settle_date != 0 {
            my_size += ::protobuf::rt::value_size(8, self.settle_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.payment_request.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.payment_request);
        }
        if !self.description_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.description_hash);
        }
        if self.expiry != 0 {
            my_size += ::protobuf::rt::value_size(11, self.expiry, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.fallback_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.fallback_addr);
        }
        if self.cltv_expiry != 0 {
            my_size += ::protobuf::rt::value_size(13, self.cltv_expiry, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.memo.is_empty() {
            os.write_string(1, &self.memo)?;
        }
        if !self.receipt.is_empty() {
            os.write_bytes(2, &self.receipt)?;
        }
        if !self.r_preimage.is_empty() {
            os.write_bytes(3, &self.r_preimage)?;
        }
        if !self.r_hash.is_empty() {
            os.write_bytes(4, &self.r_hash)?;
        }
        if self.value != 0 {
            os.write_int64(5, self.value)?;
        }
        if self.settled != false {
            os.write_bool(6, self.settled)?;
        }
        if self.creation_date != 0 {
            os.write_int64(7, self.creation_date)?;
        }
        if self.settle_date != 0 {
            os.write_int64(8, self.settle_date)?;
        }
        if !self.payment_request.is_empty() {
            os.write_string(9, &self.payment_request)?;
        }
        if !self.description_hash.is_empty() {
            os.write_bytes(10, &self.description_hash)?;
        }
        if self.expiry != 0 {
            os.write_int64(11, self.expiry)?;
        }
        if !self.fallback_addr.is_empty() {
            os.write_string(12, &self.fallback_addr)?;
        }
        if self.cltv_expiry != 0 {
            os.write_uint64(13, self.cltv_expiry)?;
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

impl ::protobuf::MessageStatic for Invoice {
    fn new() -> Invoice {
        Invoice::new()
    }

    fn descriptor_static(_: ::std::option::Option<Invoice>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "memo",
                    Invoice::get_memo_for_reflect,
                    Invoice::mut_memo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "receipt",
                    Invoice::get_receipt_for_reflect,
                    Invoice::mut_receipt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "r_preimage",
                    Invoice::get_r_preimage_for_reflect,
                    Invoice::mut_r_preimage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "r_hash",
                    Invoice::get_r_hash_for_reflect,
                    Invoice::mut_r_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    Invoice::get_value_for_reflect,
                    Invoice::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "settled",
                    Invoice::get_settled_for_reflect,
                    Invoice::mut_settled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "creation_date",
                    Invoice::get_creation_date_for_reflect,
                    Invoice::mut_creation_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "settle_date",
                    Invoice::get_settle_date_for_reflect,
                    Invoice::mut_settle_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_request",
                    Invoice::get_payment_request_for_reflect,
                    Invoice::mut_payment_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "description_hash",
                    Invoice::get_description_hash_for_reflect,
                    Invoice::mut_description_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expiry",
                    Invoice::get_expiry_for_reflect,
                    Invoice::mut_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fallback_addr",
                    Invoice::get_fallback_addr_for_reflect,
                    Invoice::mut_fallback_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cltv_expiry",
                    Invoice::get_cltv_expiry_for_reflect,
                    Invoice::mut_cltv_expiry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Invoice>(
                    "Invoice",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Invoice {
    fn clear(&mut self) {
        self.clear_memo();
        self.clear_receipt();
        self.clear_r_preimage();
        self.clear_r_hash();
        self.clear_value();
        self.clear_settled();
        self.clear_creation_date();
        self.clear_settle_date();
        self.clear_payment_request();
        self.clear_description_hash();
        self.clear_expiry();
        self.clear_fallback_addr();
        self.clear_cltv_expiry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Invoice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Invoice {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddInvoiceResponse {
    // message fields
    pub r_hash: ::std::vec::Vec<u8>,
    pub payment_request: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddInvoiceResponse {}

impl AddInvoiceResponse {
    pub fn new() -> AddInvoiceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddInvoiceResponse {
        static mut instance: ::protobuf::lazy::Lazy<AddInvoiceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddInvoiceResponse,
        };
        unsafe {
            instance.get(AddInvoiceResponse::new)
        }
    }

    // bytes r_hash = 1;

    pub fn clear_r_hash(&mut self) {
        self.r_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_r_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.r_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }

    // Take field
    pub fn take_r_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.r_hash, ::std::vec::Vec::new())
    }

    pub fn get_r_hash(&self) -> &[u8] {
        &self.r_hash
    }

    fn get_r_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.r_hash
    }

    fn mut_r_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }

    // string payment_request = 2;

    pub fn clear_payment_request(&mut self) {
        self.payment_request.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_request(&mut self, v: ::std::string::String) {
        self.payment_request = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_request(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }

    // Take field
    pub fn take_payment_request(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_request, ::std::string::String::new())
    }

    pub fn get_payment_request(&self) -> &str {
        &self.payment_request
    }

    fn get_payment_request_for_reflect(&self) -> &::std::string::String {
        &self.payment_request
    }

    fn mut_payment_request_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_request
    }
}

impl ::protobuf::Message for AddInvoiceResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.r_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_request)?;
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
        if !self.r_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.r_hash);
        }
        if !self.payment_request.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.payment_request);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.r_hash.is_empty() {
            os.write_bytes(1, &self.r_hash)?;
        }
        if !self.payment_request.is_empty() {
            os.write_string(2, &self.payment_request)?;
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

impl ::protobuf::MessageStatic for AddInvoiceResponse {
    fn new() -> AddInvoiceResponse {
        AddInvoiceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddInvoiceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "r_hash",
                    AddInvoiceResponse::get_r_hash_for_reflect,
                    AddInvoiceResponse::mut_r_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_request",
                    AddInvoiceResponse::get_payment_request_for_reflect,
                    AddInvoiceResponse::mut_payment_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddInvoiceResponse>(
                    "AddInvoiceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddInvoiceResponse {
    fn clear(&mut self) {
        self.clear_r_hash();
        self.clear_payment_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddInvoiceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddInvoiceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PaymentHash {
    // message fields
    pub r_hash_str: ::std::string::String,
    pub r_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PaymentHash {}

impl PaymentHash {
    pub fn new() -> PaymentHash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PaymentHash {
        static mut instance: ::protobuf::lazy::Lazy<PaymentHash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PaymentHash,
        };
        unsafe {
            instance.get(PaymentHash::new)
        }
    }

    // string r_hash_str = 1;

    pub fn clear_r_hash_str(&mut self) {
        self.r_hash_str.clear();
    }

    // Param is passed by value, moved
    pub fn set_r_hash_str(&mut self, v: ::std::string::String) {
        self.r_hash_str = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r_hash_str(&mut self) -> &mut ::std::string::String {
        &mut self.r_hash_str
    }

    // Take field
    pub fn take_r_hash_str(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.r_hash_str, ::std::string::String::new())
    }

    pub fn get_r_hash_str(&self) -> &str {
        &self.r_hash_str
    }

    fn get_r_hash_str_for_reflect(&self) -> &::std::string::String {
        &self.r_hash_str
    }

    fn mut_r_hash_str_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.r_hash_str
    }

    // bytes r_hash = 2;

    pub fn clear_r_hash(&mut self) {
        self.r_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_r_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.r_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }

    // Take field
    pub fn take_r_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.r_hash, ::std::vec::Vec::new())
    }

    pub fn get_r_hash(&self) -> &[u8] {
        &self.r_hash
    }

    fn get_r_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.r_hash
    }

    fn mut_r_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.r_hash
    }
}

impl ::protobuf::Message for PaymentHash {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.r_hash_str)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.r_hash)?;
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
        if !self.r_hash_str.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.r_hash_str);
        }
        if !self.r_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.r_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.r_hash_str.is_empty() {
            os.write_string(1, &self.r_hash_str)?;
        }
        if !self.r_hash.is_empty() {
            os.write_bytes(2, &self.r_hash)?;
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

impl ::protobuf::MessageStatic for PaymentHash {
    fn new() -> PaymentHash {
        PaymentHash::new()
    }

    fn descriptor_static(_: ::std::option::Option<PaymentHash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "r_hash_str",
                    PaymentHash::get_r_hash_str_for_reflect,
                    PaymentHash::mut_r_hash_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "r_hash",
                    PaymentHash::get_r_hash_for_reflect,
                    PaymentHash::mut_r_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PaymentHash>(
                    "PaymentHash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PaymentHash {
    fn clear(&mut self) {
        self.clear_r_hash_str();
        self.clear_r_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PaymentHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PaymentHash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListInvoiceRequest {
    // message fields
    pub pending_only: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListInvoiceRequest {}

impl ListInvoiceRequest {
    pub fn new() -> ListInvoiceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListInvoiceRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListInvoiceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListInvoiceRequest,
        };
        unsafe {
            instance.get(ListInvoiceRequest::new)
        }
    }

    // bool pending_only = 1;

    pub fn clear_pending_only(&mut self) {
        self.pending_only = false;
    }

    // Param is passed by value, moved
    pub fn set_pending_only(&mut self, v: bool) {
        self.pending_only = v;
    }

    pub fn get_pending_only(&self) -> bool {
        self.pending_only
    }

    fn get_pending_only_for_reflect(&self) -> &bool {
        &self.pending_only
    }

    fn mut_pending_only_for_reflect(&mut self) -> &mut bool {
        &mut self.pending_only
    }
}

impl ::protobuf::Message for ListInvoiceRequest {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.pending_only = tmp;
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
        if self.pending_only != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.pending_only != false {
            os.write_bool(1, self.pending_only)?;
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

impl ::protobuf::MessageStatic for ListInvoiceRequest {
    fn new() -> ListInvoiceRequest {
        ListInvoiceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListInvoiceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pending_only",
                    ListInvoiceRequest::get_pending_only_for_reflect,
                    ListInvoiceRequest::mut_pending_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListInvoiceRequest>(
                    "ListInvoiceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListInvoiceRequest {
    fn clear(&mut self) {
        self.clear_pending_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListInvoiceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListInvoiceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListInvoiceResponse {
    // message fields
    pub invoices: ::protobuf::RepeatedField<Invoice>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListInvoiceResponse {}

impl ListInvoiceResponse {
    pub fn new() -> ListInvoiceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListInvoiceResponse {
        static mut instance: ::protobuf::lazy::Lazy<ListInvoiceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListInvoiceResponse,
        };
        unsafe {
            instance.get(ListInvoiceResponse::new)
        }
    }

    // repeated .lnrpc.Invoice invoices = 1;

    pub fn clear_invoices(&mut self) {
        self.invoices.clear();
    }

    // Param is passed by value, moved
    pub fn set_invoices(&mut self, v: ::protobuf::RepeatedField<Invoice>) {
        self.invoices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invoices(&mut self) -> &mut ::protobuf::RepeatedField<Invoice> {
        &mut self.invoices
    }

    // Take field
    pub fn take_invoices(&mut self) -> ::protobuf::RepeatedField<Invoice> {
        ::std::mem::replace(&mut self.invoices, ::protobuf::RepeatedField::new())
    }

    pub fn get_invoices(&self) -> &[Invoice] {
        &self.invoices
    }

    fn get_invoices_for_reflect(&self) -> &::protobuf::RepeatedField<Invoice> {
        &self.invoices
    }

    fn mut_invoices_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Invoice> {
        &mut self.invoices
    }
}

impl ::protobuf::Message for ListInvoiceResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.invoices {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invoices)?;
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
        for value in &self.invoices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.invoices {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListInvoiceResponse {
    fn new() -> ListInvoiceResponse {
        ListInvoiceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListInvoiceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Invoice>>(
                    "invoices",
                    ListInvoiceResponse::get_invoices_for_reflect,
                    ListInvoiceResponse::mut_invoices_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListInvoiceResponse>(
                    "ListInvoiceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListInvoiceResponse {
    fn clear(&mut self) {
        self.clear_invoices();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListInvoiceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListInvoiceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InvoiceSubscription {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InvoiceSubscription {}

impl InvoiceSubscription {
    pub fn new() -> InvoiceSubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InvoiceSubscription {
        static mut instance: ::protobuf::lazy::Lazy<InvoiceSubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InvoiceSubscription,
        };
        unsafe {
            instance.get(InvoiceSubscription::new)
        }
    }
}

impl ::protobuf::Message for InvoiceSubscription {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for InvoiceSubscription {
    fn new() -> InvoiceSubscription {
        InvoiceSubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<InvoiceSubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<InvoiceSubscription>(
                    "InvoiceSubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InvoiceSubscription {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InvoiceSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InvoiceSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Payment {
    // message fields
    pub payment_hash: ::std::string::String,
    pub value: i64,
    pub creation_date: i64,
    pub path: ::protobuf::RepeatedField<::std::string::String>,
    pub fee: i64,
    pub payment_preimage: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Payment {}

impl Payment {
    pub fn new() -> Payment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Payment {
        static mut instance: ::protobuf::lazy::Lazy<Payment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Payment,
        };
        unsafe {
            instance.get(Payment::new)
        }
    }

    // string payment_hash = 1;

    pub fn clear_payment_hash(&mut self) {
        self.payment_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_hash(&mut self, v: ::std::string::String) {
        self.payment_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_hash(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash
    }

    // Take field
    pub fn take_payment_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_hash, ::std::string::String::new())
    }

    pub fn get_payment_hash(&self) -> &str {
        &self.payment_hash
    }

    fn get_payment_hash_for_reflect(&self) -> &::std::string::String {
        &self.payment_hash
    }

    fn mut_payment_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash
    }

    // int64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.value
    }

    // int64 creation_date = 3;

    pub fn clear_creation_date(&mut self) {
        self.creation_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_creation_date(&mut self, v: i64) {
        self.creation_date = v;
    }

    pub fn get_creation_date(&self) -> i64 {
        self.creation_date
    }

    fn get_creation_date_for_reflect(&self) -> &i64 {
        &self.creation_date
    }

    fn mut_creation_date_for_reflect(&mut self) -> &mut i64 {
        &mut self.creation_date
    }

    // repeated string path = 4;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.path, ::protobuf::RepeatedField::new())
    }

    pub fn get_path(&self) -> &[::std::string::String] {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.path
    }

    // int64 fee = 5;

    pub fn clear_fee(&mut self) {
        self.fee = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee(&mut self, v: i64) {
        self.fee = v;
    }

    pub fn get_fee(&self) -> i64 {
        self.fee
    }

    fn get_fee_for_reflect(&self) -> &i64 {
        &self.fee
    }

    fn mut_fee_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee
    }

    // string payment_preimage = 6;

    pub fn clear_payment_preimage(&mut self) {
        self.payment_preimage.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_preimage(&mut self, v: ::std::string::String) {
        self.payment_preimage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_preimage(&mut self) -> &mut ::std::string::String {
        &mut self.payment_preimage
    }

    // Take field
    pub fn take_payment_preimage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_preimage, ::std::string::String::new())
    }

    pub fn get_payment_preimage(&self) -> &str {
        &self.payment_preimage
    }

    fn get_payment_preimage_for_reflect(&self) -> &::std::string::String {
        &self.payment_preimage
    }

    fn mut_payment_preimage_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_preimage
    }
}

impl ::protobuf::Message for Payment {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.value = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.creation_date = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.path)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_preimage)?;
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
        if !self.payment_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.payment_hash);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.creation_date != 0 {
            my_size += ::protobuf::rt::value_size(3, self.creation_date, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.fee != 0 {
            my_size += ::protobuf::rt::value_size(5, self.fee, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.payment_preimage.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.payment_preimage);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.payment_hash.is_empty() {
            os.write_string(1, &self.payment_hash)?;
        }
        if self.value != 0 {
            os.write_int64(2, self.value)?;
        }
        if self.creation_date != 0 {
            os.write_int64(3, self.creation_date)?;
        }
        for v in &self.path {
            os.write_string(4, &v)?;
        };
        if self.fee != 0 {
            os.write_int64(5, self.fee)?;
        }
        if !self.payment_preimage.is_empty() {
            os.write_string(6, &self.payment_preimage)?;
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

impl ::protobuf::MessageStatic for Payment {
    fn new() -> Payment {
        Payment::new()
    }

    fn descriptor_static(_: ::std::option::Option<Payment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_hash",
                    Payment::get_payment_hash_for_reflect,
                    Payment::mut_payment_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    Payment::get_value_for_reflect,
                    Payment::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "creation_date",
                    Payment::get_creation_date_for_reflect,
                    Payment::mut_creation_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    Payment::get_path_for_reflect,
                    Payment::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee",
                    Payment::get_fee_for_reflect,
                    Payment::mut_fee_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_preimage",
                    Payment::get_payment_preimage_for_reflect,
                    Payment::mut_payment_preimage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Payment>(
                    "Payment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Payment {
    fn clear(&mut self) {
        self.clear_payment_hash();
        self.clear_value();
        self.clear_creation_date();
        self.clear_path();
        self.clear_fee();
        self.clear_payment_preimage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListPaymentsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListPaymentsRequest {}

impl ListPaymentsRequest {
    pub fn new() -> ListPaymentsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListPaymentsRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListPaymentsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListPaymentsRequest,
        };
        unsafe {
            instance.get(ListPaymentsRequest::new)
        }
    }
}

impl ::protobuf::Message for ListPaymentsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ListPaymentsRequest {
    fn new() -> ListPaymentsRequest {
        ListPaymentsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListPaymentsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListPaymentsRequest>(
                    "ListPaymentsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListPaymentsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListPaymentsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListPaymentsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListPaymentsResponse {
    // message fields
    pub payments: ::protobuf::RepeatedField<Payment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListPaymentsResponse {}

impl ListPaymentsResponse {
    pub fn new() -> ListPaymentsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListPaymentsResponse {
        static mut instance: ::protobuf::lazy::Lazy<ListPaymentsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListPaymentsResponse,
        };
        unsafe {
            instance.get(ListPaymentsResponse::new)
        }
    }

    // repeated .lnrpc.Payment payments = 1;

    pub fn clear_payments(&mut self) {
        self.payments.clear();
    }

    // Param is passed by value, moved
    pub fn set_payments(&mut self, v: ::protobuf::RepeatedField<Payment>) {
        self.payments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_payments(&mut self) -> &mut ::protobuf::RepeatedField<Payment> {
        &mut self.payments
    }

    // Take field
    pub fn take_payments(&mut self) -> ::protobuf::RepeatedField<Payment> {
        ::std::mem::replace(&mut self.payments, ::protobuf::RepeatedField::new())
    }

    pub fn get_payments(&self) -> &[Payment] {
        &self.payments
    }

    fn get_payments_for_reflect(&self) -> &::protobuf::RepeatedField<Payment> {
        &self.payments
    }

    fn mut_payments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Payment> {
        &mut self.payments
    }
}

impl ::protobuf::Message for ListPaymentsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.payments {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.payments)?;
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
        for value in &self.payments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.payments {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListPaymentsResponse {
    fn new() -> ListPaymentsResponse {
        ListPaymentsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListPaymentsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payment>>(
                    "payments",
                    ListPaymentsResponse::get_payments_for_reflect,
                    ListPaymentsResponse::mut_payments_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListPaymentsResponse>(
                    "ListPaymentsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListPaymentsResponse {
    fn clear(&mut self) {
        self.clear_payments();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListPaymentsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListPaymentsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteAllPaymentsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteAllPaymentsRequest {}

impl DeleteAllPaymentsRequest {
    pub fn new() -> DeleteAllPaymentsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteAllPaymentsRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteAllPaymentsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteAllPaymentsRequest,
        };
        unsafe {
            instance.get(DeleteAllPaymentsRequest::new)
        }
    }
}

impl ::protobuf::Message for DeleteAllPaymentsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for DeleteAllPaymentsRequest {
    fn new() -> DeleteAllPaymentsRequest {
        DeleteAllPaymentsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteAllPaymentsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteAllPaymentsRequest>(
                    "DeleteAllPaymentsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteAllPaymentsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteAllPaymentsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteAllPaymentsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteAllPaymentsResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteAllPaymentsResponse {}

impl DeleteAllPaymentsResponse {
    pub fn new() -> DeleteAllPaymentsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteAllPaymentsResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteAllPaymentsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteAllPaymentsResponse,
        };
        unsafe {
            instance.get(DeleteAllPaymentsResponse::new)
        }
    }
}

impl ::protobuf::Message for DeleteAllPaymentsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for DeleteAllPaymentsResponse {
    fn new() -> DeleteAllPaymentsResponse {
        DeleteAllPaymentsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteAllPaymentsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteAllPaymentsResponse>(
                    "DeleteAllPaymentsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteAllPaymentsResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteAllPaymentsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteAllPaymentsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugLevelRequest {
    // message fields
    pub show: bool,
    pub level_spec: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugLevelRequest {}

impl DebugLevelRequest {
    pub fn new() -> DebugLevelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugLevelRequest {
        static mut instance: ::protobuf::lazy::Lazy<DebugLevelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugLevelRequest,
        };
        unsafe {
            instance.get(DebugLevelRequest::new)
        }
    }

    // bool show = 1;

    pub fn clear_show(&mut self) {
        self.show = false;
    }

    // Param is passed by value, moved
    pub fn set_show(&mut self, v: bool) {
        self.show = v;
    }

    pub fn get_show(&self) -> bool {
        self.show
    }

    fn get_show_for_reflect(&self) -> &bool {
        &self.show
    }

    fn mut_show_for_reflect(&mut self) -> &mut bool {
        &mut self.show
    }

    // string level_spec = 2;

    pub fn clear_level_spec(&mut self) {
        self.level_spec.clear();
    }

    // Param is passed by value, moved
    pub fn set_level_spec(&mut self, v: ::std::string::String) {
        self.level_spec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_level_spec(&mut self) -> &mut ::std::string::String {
        &mut self.level_spec
    }

    // Take field
    pub fn take_level_spec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.level_spec, ::std::string::String::new())
    }

    pub fn get_level_spec(&self) -> &str {
        &self.level_spec
    }

    fn get_level_spec_for_reflect(&self) -> &::std::string::String {
        &self.level_spec
    }

    fn mut_level_spec_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.level_spec
    }
}

impl ::protobuf::Message for DebugLevelRequest {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.show = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.level_spec)?;
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
        if self.show != false {
            my_size += 2;
        }
        if !self.level_spec.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.level_spec);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.show != false {
            os.write_bool(1, self.show)?;
        }
        if !self.level_spec.is_empty() {
            os.write_string(2, &self.level_spec)?;
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

impl ::protobuf::MessageStatic for DebugLevelRequest {
    fn new() -> DebugLevelRequest {
        DebugLevelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugLevelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show",
                    DebugLevelRequest::get_show_for_reflect,
                    DebugLevelRequest::mut_show_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "level_spec",
                    DebugLevelRequest::get_level_spec_for_reflect,
                    DebugLevelRequest::mut_level_spec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugLevelRequest>(
                    "DebugLevelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugLevelRequest {
    fn clear(&mut self) {
        self.clear_show();
        self.clear_level_spec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugLevelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugLevelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugLevelResponse {
    // message fields
    pub sub_systems: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugLevelResponse {}

impl DebugLevelResponse {
    pub fn new() -> DebugLevelResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugLevelResponse {
        static mut instance: ::protobuf::lazy::Lazy<DebugLevelResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugLevelResponse,
        };
        unsafe {
            instance.get(DebugLevelResponse::new)
        }
    }

    // string sub_systems = 1;

    pub fn clear_sub_systems(&mut self) {
        self.sub_systems.clear();
    }

    // Param is passed by value, moved
    pub fn set_sub_systems(&mut self, v: ::std::string::String) {
        self.sub_systems = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_systems(&mut self) -> &mut ::std::string::String {
        &mut self.sub_systems
    }

    // Take field
    pub fn take_sub_systems(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sub_systems, ::std::string::String::new())
    }

    pub fn get_sub_systems(&self) -> &str {
        &self.sub_systems
    }

    fn get_sub_systems_for_reflect(&self) -> &::std::string::String {
        &self.sub_systems
    }

    fn mut_sub_systems_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sub_systems
    }
}

impl ::protobuf::Message for DebugLevelResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sub_systems)?;
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
        if !self.sub_systems.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.sub_systems);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.sub_systems.is_empty() {
            os.write_string(1, &self.sub_systems)?;
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

impl ::protobuf::MessageStatic for DebugLevelResponse {
    fn new() -> DebugLevelResponse {
        DebugLevelResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugLevelResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sub_systems",
                    DebugLevelResponse::get_sub_systems_for_reflect,
                    DebugLevelResponse::mut_sub_systems_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugLevelResponse>(
                    "DebugLevelResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugLevelResponse {
    fn clear(&mut self) {
        self.clear_sub_systems();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugLevelResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugLevelResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PayReqString {
    // message fields
    pub pay_req: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PayReqString {}

impl PayReqString {
    pub fn new() -> PayReqString {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PayReqString {
        static mut instance: ::protobuf::lazy::Lazy<PayReqString> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PayReqString,
        };
        unsafe {
            instance.get(PayReqString::new)
        }
    }

    // string pay_req = 1;

    pub fn clear_pay_req(&mut self) {
        self.pay_req.clear();
    }

    // Param is passed by value, moved
    pub fn set_pay_req(&mut self, v: ::std::string::String) {
        self.pay_req = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pay_req(&mut self) -> &mut ::std::string::String {
        &mut self.pay_req
    }

    // Take field
    pub fn take_pay_req(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pay_req, ::std::string::String::new())
    }

    pub fn get_pay_req(&self) -> &str {
        &self.pay_req
    }

    fn get_pay_req_for_reflect(&self) -> &::std::string::String {
        &self.pay_req
    }

    fn mut_pay_req_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pay_req
    }
}

impl ::protobuf::Message for PayReqString {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pay_req)?;
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
        if !self.pay_req.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pay_req);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pay_req.is_empty() {
            os.write_string(1, &self.pay_req)?;
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

impl ::protobuf::MessageStatic for PayReqString {
    fn new() -> PayReqString {
        PayReqString::new()
    }

    fn descriptor_static(_: ::std::option::Option<PayReqString>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pay_req",
                    PayReqString::get_pay_req_for_reflect,
                    PayReqString::mut_pay_req_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PayReqString>(
                    "PayReqString",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PayReqString {
    fn clear(&mut self) {
        self.clear_pay_req();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PayReqString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PayReqString {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PayReq {
    // message fields
    pub destination: ::std::string::String,
    pub payment_hash: ::std::string::String,
    pub num_satoshis: i64,
    pub timestamp: i64,
    pub expiry: i64,
    pub description: ::std::string::String,
    pub description_hash: ::std::string::String,
    pub fallback_addr: ::std::string::String,
    pub cltv_expiry: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PayReq {}

impl PayReq {
    pub fn new() -> PayReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PayReq {
        static mut instance: ::protobuf::lazy::Lazy<PayReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PayReq,
        };
        unsafe {
            instance.get(PayReq::new)
        }
    }

    // string destination = 1;

    pub fn clear_destination(&mut self) {
        self.destination.clear();
    }

    // Param is passed by value, moved
    pub fn set_destination(&mut self, v: ::std::string::String) {
        self.destination = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination(&mut self) -> &mut ::std::string::String {
        &mut self.destination
    }

    // Take field
    pub fn take_destination(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.destination, ::std::string::String::new())
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    fn get_destination_for_reflect(&self) -> &::std::string::String {
        &self.destination
    }

    fn mut_destination_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.destination
    }

    // string payment_hash = 2;

    pub fn clear_payment_hash(&mut self) {
        self.payment_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_payment_hash(&mut self, v: ::std::string::String) {
        self.payment_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payment_hash(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash
    }

    // Take field
    pub fn take_payment_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payment_hash, ::std::string::String::new())
    }

    pub fn get_payment_hash(&self) -> &str {
        &self.payment_hash
    }

    fn get_payment_hash_for_reflect(&self) -> &::std::string::String {
        &self.payment_hash
    }

    fn mut_payment_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payment_hash
    }

    // int64 num_satoshis = 3;

    pub fn clear_num_satoshis(&mut self) {
        self.num_satoshis = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_satoshis(&mut self, v: i64) {
        self.num_satoshis = v;
    }

    pub fn get_num_satoshis(&self) -> i64 {
        self.num_satoshis
    }

    fn get_num_satoshis_for_reflect(&self) -> &i64 {
        &self.num_satoshis
    }

    fn mut_num_satoshis_for_reflect(&mut self) -> &mut i64 {
        &mut self.num_satoshis
    }

    // int64 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &i64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.timestamp
    }

    // int64 expiry = 5;

    pub fn clear_expiry(&mut self) {
        self.expiry = 0;
    }

    // Param is passed by value, moved
    pub fn set_expiry(&mut self, v: i64) {
        self.expiry = v;
    }

    pub fn get_expiry(&self) -> i64 {
        self.expiry
    }

    fn get_expiry_for_reflect(&self) -> &i64 {
        &self.expiry
    }

    fn mut_expiry_for_reflect(&mut self) -> &mut i64 {
        &mut self.expiry
    }

    // string description = 6;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // string description_hash = 7;

    pub fn clear_description_hash(&mut self) {
        self.description_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_description_hash(&mut self, v: ::std::string::String) {
        self.description_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description_hash(&mut self) -> &mut ::std::string::String {
        &mut self.description_hash
    }

    // Take field
    pub fn take_description_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description_hash, ::std::string::String::new())
    }

    pub fn get_description_hash(&self) -> &str {
        &self.description_hash
    }

    fn get_description_hash_for_reflect(&self) -> &::std::string::String {
        &self.description_hash
    }

    fn mut_description_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description_hash
    }

    // string fallback_addr = 8;

    pub fn clear_fallback_addr(&mut self) {
        self.fallback_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_fallback_addr(&mut self, v: ::std::string::String) {
        self.fallback_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fallback_addr(&mut self) -> &mut ::std::string::String {
        &mut self.fallback_addr
    }

    // Take field
    pub fn take_fallback_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.fallback_addr, ::std::string::String::new())
    }

    pub fn get_fallback_addr(&self) -> &str {
        &self.fallback_addr
    }

    fn get_fallback_addr_for_reflect(&self) -> &::std::string::String {
        &self.fallback_addr
    }

    fn mut_fallback_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.fallback_addr
    }

    // int64 cltv_expiry = 9;

    pub fn clear_cltv_expiry(&mut self) {
        self.cltv_expiry = 0;
    }

    // Param is passed by value, moved
    pub fn set_cltv_expiry(&mut self, v: i64) {
        self.cltv_expiry = v;
    }

    pub fn get_cltv_expiry(&self) -> i64 {
        self.cltv_expiry
    }

    fn get_cltv_expiry_for_reflect(&self) -> &i64 {
        &self.cltv_expiry
    }

    fn mut_cltv_expiry_for_reflect(&mut self) -> &mut i64 {
        &mut self.cltv_expiry
    }
}

impl ::protobuf::Message for PayReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.destination)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payment_hash)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.num_satoshis = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expiry = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description_hash)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.fallback_addr)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.cltv_expiry = tmp;
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
        if !self.destination.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.destination);
        }
        if !self.payment_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.payment_hash);
        }
        if self.num_satoshis != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_satoshis, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(4, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.expiry != 0 {
            my_size += ::protobuf::rt::value_size(5, self.expiry, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.description);
        }
        if !self.description_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.description_hash);
        }
        if !self.fallback_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.fallback_addr);
        }
        if self.cltv_expiry != 0 {
            my_size += ::protobuf::rt::value_size(9, self.cltv_expiry, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.destination.is_empty() {
            os.write_string(1, &self.destination)?;
        }
        if !self.payment_hash.is_empty() {
            os.write_string(2, &self.payment_hash)?;
        }
        if self.num_satoshis != 0 {
            os.write_int64(3, self.num_satoshis)?;
        }
        if self.timestamp != 0 {
            os.write_int64(4, self.timestamp)?;
        }
        if self.expiry != 0 {
            os.write_int64(5, self.expiry)?;
        }
        if !self.description.is_empty() {
            os.write_string(6, &self.description)?;
        }
        if !self.description_hash.is_empty() {
            os.write_string(7, &self.description_hash)?;
        }
        if !self.fallback_addr.is_empty() {
            os.write_string(8, &self.fallback_addr)?;
        }
        if self.cltv_expiry != 0 {
            os.write_int64(9, self.cltv_expiry)?;
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

impl ::protobuf::MessageStatic for PayReq {
    fn new() -> PayReq {
        PayReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<PayReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "destination",
                    PayReq::get_destination_for_reflect,
                    PayReq::mut_destination_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payment_hash",
                    PayReq::get_payment_hash_for_reflect,
                    PayReq::mut_payment_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "num_satoshis",
                    PayReq::get_num_satoshis_for_reflect,
                    PayReq::mut_num_satoshis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    PayReq::get_timestamp_for_reflect,
                    PayReq::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expiry",
                    PayReq::get_expiry_for_reflect,
                    PayReq::mut_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    PayReq::get_description_for_reflect,
                    PayReq::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description_hash",
                    PayReq::get_description_hash_for_reflect,
                    PayReq::mut_description_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fallback_addr",
                    PayReq::get_fallback_addr_for_reflect,
                    PayReq::mut_fallback_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "cltv_expiry",
                    PayReq::get_cltv_expiry_for_reflect,
                    PayReq::mut_cltv_expiry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PayReq>(
                    "PayReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PayReq {
    fn clear(&mut self) {
        self.clear_destination();
        self.clear_payment_hash();
        self.clear_num_satoshis();
        self.clear_timestamp();
        self.clear_expiry();
        self.clear_description();
        self.clear_description_hash();
        self.clear_fallback_addr();
        self.clear_cltv_expiry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PayReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PayReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeeReportRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeeReportRequest {}

impl FeeReportRequest {
    pub fn new() -> FeeReportRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeeReportRequest {
        static mut instance: ::protobuf::lazy::Lazy<FeeReportRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeeReportRequest,
        };
        unsafe {
            instance.get(FeeReportRequest::new)
        }
    }
}

impl ::protobuf::Message for FeeReportRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for FeeReportRequest {
    fn new() -> FeeReportRequest {
        FeeReportRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeeReportRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<FeeReportRequest>(
                    "FeeReportRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeeReportRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeeReportRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeeReportRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelFeeReport {
    // message fields
    pub chan_point: ::std::string::String,
    pub base_fee_msat: i64,
    pub fee_per_mil: i64,
    pub fee_rate: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelFeeReport {}

impl ChannelFeeReport {
    pub fn new() -> ChannelFeeReport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelFeeReport {
        static mut instance: ::protobuf::lazy::Lazy<ChannelFeeReport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelFeeReport,
        };
        unsafe {
            instance.get(ChannelFeeReport::new)
        }
    }

    // string chan_point = 1;

    pub fn clear_chan_point(&mut self) {
        self.chan_point.clear();
    }

    // Param is passed by value, moved
    pub fn set_chan_point(&mut self, v: ::std::string::String) {
        self.chan_point = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chan_point(&mut self) -> &mut ::std::string::String {
        &mut self.chan_point
    }

    // Take field
    pub fn take_chan_point(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chan_point, ::std::string::String::new())
    }

    pub fn get_chan_point(&self) -> &str {
        &self.chan_point
    }

    fn get_chan_point_for_reflect(&self) -> &::std::string::String {
        &self.chan_point
    }

    fn mut_chan_point_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.chan_point
    }

    // int64 base_fee_msat = 2;

    pub fn clear_base_fee_msat(&mut self) {
        self.base_fee_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_base_fee_msat(&mut self, v: i64) {
        self.base_fee_msat = v;
    }

    pub fn get_base_fee_msat(&self) -> i64 {
        self.base_fee_msat
    }

    fn get_base_fee_msat_for_reflect(&self) -> &i64 {
        &self.base_fee_msat
    }

    fn mut_base_fee_msat_for_reflect(&mut self) -> &mut i64 {
        &mut self.base_fee_msat
    }

    // int64 fee_per_mil = 3;

    pub fn clear_fee_per_mil(&mut self) {
        self.fee_per_mil = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_per_mil(&mut self, v: i64) {
        self.fee_per_mil = v;
    }

    pub fn get_fee_per_mil(&self) -> i64 {
        self.fee_per_mil
    }

    fn get_fee_per_mil_for_reflect(&self) -> &i64 {
        &self.fee_per_mil
    }

    fn mut_fee_per_mil_for_reflect(&mut self) -> &mut i64 {
        &mut self.fee_per_mil
    }

    // double fee_rate = 4;

    pub fn clear_fee_rate(&mut self) {
        self.fee_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_fee_rate(&mut self, v: f64) {
        self.fee_rate = v;
    }

    pub fn get_fee_rate(&self) -> f64 {
        self.fee_rate
    }

    fn get_fee_rate_for_reflect(&self) -> &f64 {
        &self.fee_rate
    }

    fn mut_fee_rate_for_reflect(&mut self) -> &mut f64 {
        &mut self.fee_rate
    }
}

impl ::protobuf::Message for ChannelFeeReport {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chan_point)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.base_fee_msat = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fee_per_mil = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.fee_rate = tmp;
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
        if !self.chan_point.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.chan_point);
        }
        if self.base_fee_msat != 0 {
            my_size += ::protobuf::rt::value_size(2, self.base_fee_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_per_mil != 0 {
            my_size += ::protobuf::rt::value_size(3, self.fee_per_mil, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_rate != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.chan_point.is_empty() {
            os.write_string(1, &self.chan_point)?;
        }
        if self.base_fee_msat != 0 {
            os.write_int64(2, self.base_fee_msat)?;
        }
        if self.fee_per_mil != 0 {
            os.write_int64(3, self.fee_per_mil)?;
        }
        if self.fee_rate != 0. {
            os.write_double(4, self.fee_rate)?;
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

impl ::protobuf::MessageStatic for ChannelFeeReport {
    fn new() -> ChannelFeeReport {
        ChannelFeeReport::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelFeeReport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chan_point",
                    ChannelFeeReport::get_chan_point_for_reflect,
                    ChannelFeeReport::mut_chan_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "base_fee_msat",
                    ChannelFeeReport::get_base_fee_msat_for_reflect,
                    ChannelFeeReport::mut_base_fee_msat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fee_per_mil",
                    ChannelFeeReport::get_fee_per_mil_for_reflect,
                    ChannelFeeReport::mut_fee_per_mil_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "fee_rate",
                    ChannelFeeReport::get_fee_rate_for_reflect,
                    ChannelFeeReport::mut_fee_rate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelFeeReport>(
                    "ChannelFeeReport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelFeeReport {
    fn clear(&mut self) {
        self.clear_chan_point();
        self.clear_base_fee_msat();
        self.clear_fee_per_mil();
        self.clear_fee_rate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelFeeReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelFeeReport {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeeReportResponse {
    // message fields
    pub channel_fees: ::protobuf::RepeatedField<ChannelFeeReport>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeeReportResponse {}

impl FeeReportResponse {
    pub fn new() -> FeeReportResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeeReportResponse {
        static mut instance: ::protobuf::lazy::Lazy<FeeReportResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeeReportResponse,
        };
        unsafe {
            instance.get(FeeReportResponse::new)
        }
    }

    // repeated .lnrpc.ChannelFeeReport channel_fees = 1;

    pub fn clear_channel_fees(&mut self) {
        self.channel_fees.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_fees(&mut self, v: ::protobuf::RepeatedField<ChannelFeeReport>) {
        self.channel_fees = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channel_fees(&mut self) -> &mut ::protobuf::RepeatedField<ChannelFeeReport> {
        &mut self.channel_fees
    }

    // Take field
    pub fn take_channel_fees(&mut self) -> ::protobuf::RepeatedField<ChannelFeeReport> {
        ::std::mem::replace(&mut self.channel_fees, ::protobuf::RepeatedField::new())
    }

    pub fn get_channel_fees(&self) -> &[ChannelFeeReport] {
        &self.channel_fees
    }

    fn get_channel_fees_for_reflect(&self) -> &::protobuf::RepeatedField<ChannelFeeReport> {
        &self.channel_fees
    }

    fn mut_channel_fees_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChannelFeeReport> {
        &mut self.channel_fees
    }
}

impl ::protobuf::Message for FeeReportResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.channel_fees {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channel_fees)?;
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
        for value in &self.channel_fees {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.channel_fees {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for FeeReportResponse {
    fn new() -> FeeReportResponse {
        FeeReportResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeeReportResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelFeeReport>>(
                    "channel_fees",
                    FeeReportResponse::get_channel_fees_for_reflect,
                    FeeReportResponse::mut_channel_fees_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeeReportResponse>(
                    "FeeReportResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeeReportResponse {
    fn clear(&mut self) {
        self.clear_channel_fees();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeeReportResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeeReportResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PolicyUpdateRequest {
    // message fields
    pub base_fee_msat: i64,
    pub fee_rate: f64,
    pub time_lock_delta: u32,
    // message oneof groups
    scope: ::std::option::Option<PolicyUpdateRequest_oneof_scope>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PolicyUpdateRequest {}

#[derive(Clone,PartialEq)]
pub enum PolicyUpdateRequest_oneof_scope {
    global(bool),
    chan_point(ChannelPoint),
}

impl PolicyUpdateRequest {
    pub fn new() -> PolicyUpdateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PolicyUpdateRequest {
        static mut instance: ::protobuf::lazy::Lazy<PolicyUpdateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PolicyUpdateRequest,
        };
        unsafe {
            instance.get(PolicyUpdateRequest::new)
        }
    }

    // bool global = 1;

    pub fn clear_global(&mut self) {
        self.scope = ::std::option::Option::None;
    }

    pub fn has_global(&self) -> bool {
        match self.scope {
            ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::global(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_global(&mut self, v: bool) {
        self.scope = ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::global(v))
    }

    pub fn get_global(&self) -> bool {
        match self.scope {
            ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::global(v)) => v,
            _ => false,
        }
    }

    // .lnrpc.ChannelPoint chan_point = 2;

    pub fn clear_chan_point(&mut self) {
        self.scope = ::std::option::Option::None;
    }

    pub fn has_chan_point(&self) -> bool {
        match self.scope {
            ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chan_point(&mut self, v: ChannelPoint) {
        self.scope = ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chan_point(&mut self) -> &mut ChannelPoint {
        if let ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(_)) = self.scope {
        } else {
            self.scope = ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(ChannelPoint::new()));
        }
        match self.scope {
            ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chan_point(&mut self) -> ChannelPoint {
        if self.has_chan_point() {
            match self.scope.take() {
                ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(v)) => v,
                _ => panic!(),
            }
        } else {
            ChannelPoint::new()
        }
    }

    pub fn get_chan_point(&self) -> &ChannelPoint {
        match self.scope {
            ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(ref v)) => v,
            _ => ChannelPoint::default_instance(),
        }
    }

    // int64 base_fee_msat = 3;

    pub fn clear_base_fee_msat(&mut self) {
        self.base_fee_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_base_fee_msat(&mut self, v: i64) {
        self.base_fee_msat = v;
    }

    pub fn get_base_fee_msat(&self) -> i64 {
        self.base_fee_msat
    }

    fn get_base_fee_msat_for_reflect(&self) -> &i64 {
        &self.base_fee_msat
    }

    fn mut_base_fee_msat_for_reflect(&mut self) -> &mut i64 {
        &mut self.base_fee_msat
    }

    // double fee_rate = 4;

    pub fn clear_fee_rate(&mut self) {
        self.fee_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_fee_rate(&mut self, v: f64) {
        self.fee_rate = v;
    }

    pub fn get_fee_rate(&self) -> f64 {
        self.fee_rate
    }

    fn get_fee_rate_for_reflect(&self) -> &f64 {
        &self.fee_rate
    }

    fn mut_fee_rate_for_reflect(&mut self) -> &mut f64 {
        &mut self.fee_rate
    }

    // uint32 time_lock_delta = 5;

    pub fn clear_time_lock_delta(&mut self) {
        self.time_lock_delta = 0;
    }

    // Param is passed by value, moved
    pub fn set_time_lock_delta(&mut self, v: u32) {
        self.time_lock_delta = v;
    }

    pub fn get_time_lock_delta(&self) -> u32 {
        self.time_lock_delta
    }

    fn get_time_lock_delta_for_reflect(&self) -> &u32 {
        &self.time_lock_delta
    }

    fn mut_time_lock_delta_for_reflect(&mut self) -> &mut u32 {
        &mut self.time_lock_delta
    }
}

impl ::protobuf::Message for PolicyUpdateRequest {
    fn is_initialized(&self) -> bool {
        if let Some(PolicyUpdateRequest_oneof_scope::chan_point(ref v)) = self.scope {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    self.scope = ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::global(is.read_bool()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.scope = ::std::option::Option::Some(PolicyUpdateRequest_oneof_scope::chan_point(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.base_fee_msat = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.fee_rate = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_lock_delta = tmp;
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
        if self.base_fee_msat != 0 {
            my_size += ::protobuf::rt::value_size(3, self.base_fee_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_rate != 0. {
            my_size += 9;
        }
        if self.time_lock_delta != 0 {
            my_size += ::protobuf::rt::value_size(5, self.time_lock_delta, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.scope {
            match v {
                &PolicyUpdateRequest_oneof_scope::global(v) => {
                    my_size += 2;
                },
                &PolicyUpdateRequest_oneof_scope::chan_point(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.base_fee_msat != 0 {
            os.write_int64(3, self.base_fee_msat)?;
        }
        if self.fee_rate != 0. {
            os.write_double(4, self.fee_rate)?;
        }
        if self.time_lock_delta != 0 {
            os.write_uint32(5, self.time_lock_delta)?;
        }
        if let ::std::option::Option::Some(ref v) = self.scope {
            match v {
                &PolicyUpdateRequest_oneof_scope::global(v) => {
                    os.write_bool(1, v)?;
                },
                &PolicyUpdateRequest_oneof_scope::chan_point(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for PolicyUpdateRequest {
    fn new() -> PolicyUpdateRequest {
        PolicyUpdateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PolicyUpdateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "global",
                    PolicyUpdateRequest::has_global,
                    PolicyUpdateRequest::get_global,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ChannelPoint>(
                    "chan_point",
                    PolicyUpdateRequest::has_chan_point,
                    PolicyUpdateRequest::get_chan_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "base_fee_msat",
                    PolicyUpdateRequest::get_base_fee_msat_for_reflect,
                    PolicyUpdateRequest::mut_base_fee_msat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "fee_rate",
                    PolicyUpdateRequest::get_fee_rate_for_reflect,
                    PolicyUpdateRequest::mut_fee_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_lock_delta",
                    PolicyUpdateRequest::get_time_lock_delta_for_reflect,
                    PolicyUpdateRequest::mut_time_lock_delta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PolicyUpdateRequest>(
                    "PolicyUpdateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PolicyUpdateRequest {
    fn clear(&mut self) {
        self.clear_global();
        self.clear_chan_point();
        self.clear_base_fee_msat();
        self.clear_fee_rate();
        self.clear_time_lock_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PolicyUpdateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PolicyUpdateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PolicyUpdateResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PolicyUpdateResponse {}

impl PolicyUpdateResponse {
    pub fn new() -> PolicyUpdateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PolicyUpdateResponse {
        static mut instance: ::protobuf::lazy::Lazy<PolicyUpdateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PolicyUpdateResponse,
        };
        unsafe {
            instance.get(PolicyUpdateResponse::new)
        }
    }
}

impl ::protobuf::Message for PolicyUpdateResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for PolicyUpdateResponse {
    fn new() -> PolicyUpdateResponse {
        PolicyUpdateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PolicyUpdateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PolicyUpdateResponse>(
                    "PolicyUpdateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PolicyUpdateResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PolicyUpdateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PolicyUpdateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetSPVProofRequest {
    // message fields
    pub tx_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSPVProofRequest {}

impl GetSPVProofRequest {
    pub fn new() -> GetSPVProofRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSPVProofRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetSPVProofRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSPVProofRequest,
        };
        unsafe {
            instance.get(GetSPVProofRequest::new)
        }
    }

    // bytes tx_hash = 1;

    pub fn clear_tx_hash(&mut self) {
        self.tx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }

    // Take field
    pub fn take_tx_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx_hash, ::std::vec::Vec::new())
    }

    pub fn get_tx_hash(&self) -> &[u8] {
        &self.tx_hash
    }

    fn get_tx_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx_hash
    }

    fn mut_tx_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }
}

impl ::protobuf::Message for GetSPVProofRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_hash)?;
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
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_hash.is_empty() {
            os.write_bytes(1, &self.tx_hash)?;
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

impl ::protobuf::MessageStatic for GetSPVProofRequest {
    fn new() -> GetSPVProofRequest {
        GetSPVProofRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSPVProofRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    GetSPVProofRequest::get_tx_hash_for_reflect,
                    GetSPVProofRequest::mut_tx_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetSPVProofRequest>(
                    "GetSPVProofRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSPVProofRequest {
    fn clear(&mut self) {
        self.clear_tx_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetSPVProofRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSPVProofRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetSPVProofResponse {
    // message fields
    pub serialized_full_spv_proof: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSPVProofResponse {}

impl GetSPVProofResponse {
    pub fn new() -> GetSPVProofResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSPVProofResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetSPVProofResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSPVProofResponse,
        };
        unsafe {
            instance.get(GetSPVProofResponse::new)
        }
    }

    // bytes serialized_full_spv_proof = 1;

    pub fn clear_serialized_full_spv_proof(&mut self) {
        self.serialized_full_spv_proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_serialized_full_spv_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.serialized_full_spv_proof = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_full_spv_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_full_spv_proof
    }

    // Take field
    pub fn take_serialized_full_spv_proof(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.serialized_full_spv_proof, ::std::vec::Vec::new())
    }

    pub fn get_serialized_full_spv_proof(&self) -> &[u8] {
        &self.serialized_full_spv_proof
    }

    fn get_serialized_full_spv_proof_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.serialized_full_spv_proof
    }

    fn mut_serialized_full_spv_proof_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_full_spv_proof
    }
}

impl ::protobuf::Message for GetSPVProofResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.serialized_full_spv_proof)?;
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
        if !self.serialized_full_spv_proof.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.serialized_full_spv_proof);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.serialized_full_spv_proof.is_empty() {
            os.write_bytes(1, &self.serialized_full_spv_proof)?;
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

impl ::protobuf::MessageStatic for GetSPVProofResponse {
    fn new() -> GetSPVProofResponse {
        GetSPVProofResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSPVProofResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "serialized_full_spv_proof",
                    GetSPVProofResponse::get_serialized_full_spv_proof_for_reflect,
                    GetSPVProofResponse::mut_serialized_full_spv_proof_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetSPVProofResponse>(
                    "GetSPVProofResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSPVProofResponse {
    fn clear(&mut self) {
        self.clear_serialized_full_spv_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetSPVProofResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSPVProofResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifySPVProofRequest {
    // message fields
    pub serialized_full_spv_proof: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifySPVProofRequest {}

impl VerifySPVProofRequest {
    pub fn new() -> VerifySPVProofRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifySPVProofRequest {
        static mut instance: ::protobuf::lazy::Lazy<VerifySPVProofRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifySPVProofRequest,
        };
        unsafe {
            instance.get(VerifySPVProofRequest::new)
        }
    }

    // bytes serialized_full_spv_proof = 1;

    pub fn clear_serialized_full_spv_proof(&mut self) {
        self.serialized_full_spv_proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_serialized_full_spv_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.serialized_full_spv_proof = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_full_spv_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_full_spv_proof
    }

    // Take field
    pub fn take_serialized_full_spv_proof(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.serialized_full_spv_proof, ::std::vec::Vec::new())
    }

    pub fn get_serialized_full_spv_proof(&self) -> &[u8] {
        &self.serialized_full_spv_proof
    }

    fn get_serialized_full_spv_proof_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.serialized_full_spv_proof
    }

    fn mut_serialized_full_spv_proof_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_full_spv_proof
    }
}

impl ::protobuf::Message for VerifySPVProofRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.serialized_full_spv_proof)?;
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
        if !self.serialized_full_spv_proof.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.serialized_full_spv_proof);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.serialized_full_spv_proof.is_empty() {
            os.write_bytes(1, &self.serialized_full_spv_proof)?;
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

impl ::protobuf::MessageStatic for VerifySPVProofRequest {
    fn new() -> VerifySPVProofRequest {
        VerifySPVProofRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifySPVProofRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "serialized_full_spv_proof",
                    VerifySPVProofRequest::get_serialized_full_spv_proof_for_reflect,
                    VerifySPVProofRequest::mut_serialized_full_spv_proof_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifySPVProofRequest>(
                    "VerifySPVProofRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifySPVProofRequest {
    fn clear(&mut self) {
        self.clear_serialized_full_spv_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifySPVProofRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifySPVProofRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifySPVProofResponse {
    // message fields
    pub verification_status: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifySPVProofResponse {}

impl VerifySPVProofResponse {
    pub fn new() -> VerifySPVProofResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifySPVProofResponse {
        static mut instance: ::protobuf::lazy::Lazy<VerifySPVProofResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifySPVProofResponse,
        };
        unsafe {
            instance.get(VerifySPVProofResponse::new)
        }
    }

    // bool verification_status = 1;

    pub fn clear_verification_status(&mut self) {
        self.verification_status = false;
    }

    // Param is passed by value, moved
    pub fn set_verification_status(&mut self, v: bool) {
        self.verification_status = v;
    }

    pub fn get_verification_status(&self) -> bool {
        self.verification_status
    }

    fn get_verification_status_for_reflect(&self) -> &bool {
        &self.verification_status
    }

    fn mut_verification_status_for_reflect(&mut self) -> &mut bool {
        &mut self.verification_status
    }
}

impl ::protobuf::Message for VerifySPVProofResponse {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.verification_status = tmp;
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
        if self.verification_status != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.verification_status != false {
            os.write_bool(1, self.verification_status)?;
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

impl ::protobuf::MessageStatic for VerifySPVProofResponse {
    fn new() -> VerifySPVProofResponse {
        VerifySPVProofResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifySPVProofResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "verification_status",
                    VerifySPVProofResponse::get_verification_status_for_reflect,
                    VerifySPVProofResponse::mut_verification_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifySPVProofResponse>(
                    "VerifySPVProofResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifySPVProofResponse {
    fn clear(&mut self) {
        self.clear_verification_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifySPVProofResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifySPVProofResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\trpc.proto\x12\x05lnrpc\x1a\x1cgoogle/api/annotations.proto\"1\n\x13C\
    reateWalletRequest\x12\x1a\n\x08password\x18\x01\x20\x01(\x0cR\x08passwo\
    rd\"\x16\n\x14CreateWalletResponse\"1\n\x13UnlockWalletRequest\x12\x1a\n\
    \x08password\x18\x01\x20\x01(\x0cR\x08password\"\x16\n\x14UnlockWalletRe\
    sponse\"\x99\x02\n\x0bTransaction\x12\x18\n\x07tx_hash\x18\x01\x20\x01(\
    \tR\x07tx_hash\x12\x16\n\x06amount\x18\x02\x20\x01(\x03R\x06amount\x12,\
    \n\x11num_confirmations\x18\x03\x20\x01(\x05R\x11num_confirmations\x12\
    \x1e\n\nblock_hash\x18\x04\x20\x01(\tR\nblock_hash\x12\"\n\x0cblock_heig\
    ht\x18\x05\x20\x01(\x05R\x0cblock_height\x12\x1e\n\ntime_stamp\x18\x06\
    \x20\x01(\x03R\ntime_stamp\x12\x1e\n\ntotal_fees\x18\x07\x20\x01(\x03R\n\
    total_fees\x12&\n\x0edest_addresses\x18\x08\x20\x03(\tR\x0edest_addresse\
    s\"\x18\n\x16GetTransactionsRequest\"L\n\x12TransactionDetails\x126\n\
    \x0ctransactions\x18\x01\x20\x03(\x0b2\x12.lnrpc.TransactionR\x0ctransac\
    tions\"\xfa\x01\n\x0bSendRequest\x12\x12\n\x04dest\x18\x01\x20\x01(\x0cR\
    \x04dest\x12\x1f\n\x0bdest_string\x18\x02\x20\x01(\tR\ndestString\x12\
    \x10\n\x03amt\x18\x03\x20\x01(\x03R\x03amt\x12!\n\x0cpayment_hash\x18\
    \x04\x20\x01(\x0cR\x0bpaymentHash\x12.\n\x13payment_hash_string\x18\x05\
    \x20\x01(\tR\x11paymentHashString\x12'\n\x0fpayment_request\x18\x06\x20\
    \x01(\tR\x0epaymentRequest\x12(\n\x10final_cltv_delta\x18\x07\x20\x01(\
    \x05R\x0efinalCltvDelta\"\x94\x01\n\x0cSendResponse\x12$\n\rpayment_erro\
    r\x18\x01\x20\x01(\tR\rpayment_error\x12*\n\x10payment_preimage\x18\x02\
    \x20\x01(\x0cR\x10payment_preimage\x122\n\rpayment_route\x18\x03\x20\x01\
    (\x0b2\x0c.lnrpc.RouteR\rpayment_route\"\xa2\x01\n\x0cChannelPoint\x120\
    \n\x12funding_txid_bytes\x18\x01\x20\x01(\x0cH\0R\x12funding_txid_bytes\
    \x12,\n\x10funding_txid_str\x18\x02\x20\x01(\tH\0R\x10funding_txid_str\
    \x12\"\n\x0coutput_index\x18\x03\x20\x01(\rR\x0coutput_indexB\x0e\n\x0cf\
    unding_txid\">\n\x10LightningAddress\x12\x16\n\x06pubkey\x18\x01\x20\x01\
    (\tR\x06pubkey\x12\x12\n\x04host\x18\x02\x20\x01(\tR\x04host\"\xe3\x01\n\
    \x0fSendManyRequest\x12L\n\x0cAddrToAmount\x18\x01\x20\x03(\x0b2(.lnrpc.\
    SendManyRequest.AddrToAmountEntryR\x0cAddrToAmount\x12\x1f\n\x0btarget_c\
    onf\x18\x03\x20\x01(\x05R\ntargetConf\x12\x20\n\x0csat_per_byte\x18\x05\
    \x20\x01(\x03R\nsatPerByte\x1a?\n\x11AddrToAmountEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x03R\x05\
    value:\x028\x01\"&\n\x10SendManyResponse\x12\x12\n\x04txid\x18\x01\x20\
    \x01(\tR\x04txid\"\x81\x01\n\x10SendCoinsRequest\x12\x12\n\x04addr\x18\
    \x01\x20\x01(\tR\x04addr\x12\x16\n\x06amount\x18\x02\x20\x01(\x03R\x06am\
    ount\x12\x1f\n\x0btarget_conf\x18\x03\x20\x01(\x05R\ntargetConf\x12\x20\
    \n\x0csat_per_byte\x18\x05\x20\x01(\x03R\nsatPerByte\"'\n\x11SendCoinsRe\
    sponse\x12\x12\n\x04txid\x18\x01\x20\x01(\tR\x04txid\"\x9e\x01\n\x11NewA\
    ddressRequest\x128\n\x04type\x18\x01\x20\x01(\x0e2$.lnrpc.NewAddressRequ\
    est.AddressTypeR\x04type\"O\n\x0bAddressType\x12\x17\n\x13WITNESS_PUBKEY\
    _HASH\x10\0\x12\x16\n\x12NESTED_PUBKEY_HASH\x10\x01\x12\x0f\n\x0bPUBKEY_\
    HASH\x10\x02\"\x1a\n\x18NewWitnessAddressRequest\".\n\x12NewAddressRespo\
    nse\x12\x18\n\x07address\x18\x01\x20\x01(\tR\x07address\"&\n\x12SignMess\
    ageRequest\x12\x10\n\x03msg\x18\x01\x20\x01(\x0cR\x03msg\"3\n\x13SignMes\
    sageResponse\x12\x1c\n\tsignature\x18\x01\x20\x01(\tR\tsignature\"F\n\
    \x14VerifyMessageRequest\x12\x10\n\x03msg\x18\x01\x20\x01(\x0cR\x03msg\
    \x12\x1c\n\tsignature\x18\x02\x20\x01(\tR\tsignature\"E\n\x15VerifyMessa\
    geResponse\x12\x14\n\x05valid\x18\x01\x20\x01(\x08R\x05valid\x12\x16\n\
    \x06pubkey\x18\x02\x20\x01(\tR\x06pubkey\"U\n\x12ConnectPeerRequest\x12+\
    \n\x04addr\x18\x01\x20\x01(\x0b2\x17.lnrpc.LightningAddressR\x04addr\x12\
    \x12\n\x04perm\x18\x02\x20\x01(\x08R\x04perm\"\x15\n\x13ConnectPeerRespo\
    nse\"1\n\x15DisconnectPeerRequest\x12\x18\n\x07pub_key\x18\x01\x20\x01(\
    \tR\x07pub_key\"\x18\n\x16DisconnectPeerResponse\"\x86\x01\n\x04HTLC\x12\
    \x1a\n\x08incoming\x18\x01\x20\x01(\x08R\x08incoming\x12\x16\n\x06amount\
    \x18\x02\x20\x01(\x03R\x06amount\x12\x1c\n\thash_lock\x18\x03\x20\x01(\
    \x0cR\thash_lock\x12,\n\x11expiration_height\x18\x04\x20\x01(\rR\x11expi\
    ration_height\"\xea\x04\n\rActiveChannel\x12\x16\n\x06active\x18\x01\x20\
    \x01(\x08R\x06active\x12$\n\rremote_pubkey\x18\x02\x20\x01(\tR\rremote_p\
    ubkey\x12$\n\rchannel_point\x18\x03\x20\x01(\tR\rchannel_point\x12\x18\n\
    \x07chan_id\x18\x04\x20\x01(\x04R\x07chan_id\x12\x1a\n\x08capacity\x18\
    \x05\x20\x01(\x03R\x08capacity\x12$\n\rlocal_balance\x18\x06\x20\x01(\
    \x03R\rlocal_balance\x12&\n\x0eremote_balance\x18\x07\x20\x01(\x03R\x0er\
    emote_balance\x12\x1e\n\ncommit_fee\x18\x08\x20\x01(\x03R\ncommit_fee\
    \x12$\n\rcommit_weight\x18\t\x20\x01(\x03R\rcommit_weight\x12\x1e\n\nfee\
    _per_kw\x18\n\x20\x01(\x03R\nfee_per_kw\x12,\n\x11unsettled_balance\x18\
    \x0b\x20\x01(\x03R\x11unsettled_balance\x120\n\x13total_satoshis_sent\
    \x18\x0c\x20\x01(\x03R\x13total_satoshis_sent\x128\n\x17total_satoshis_r\
    eceived\x18\r\x20\x01(\x03R\x17total_satoshis_received\x12\x20\n\x0bnum_\
    updates\x18\x0e\x20\x01(\x04R\x0bnum_updates\x121\n\rpending_htlcs\x18\
    \x0f\x20\x03(\x0b2\x0b.lnrpc.HTLCR\rpending_htlcs\x12\x1c\n\tcsv_delay\
    \x18\x10\x20\x01(\rR\tcsv_delay\"\x15\n\x13ListChannelsRequest\"H\n\x14L\
    istChannelsResponse\x120\n\x08channels\x18\x0b\x20\x03(\x0b2\x14.lnrpc.A\
    ctiveChannelR\x08channels\"\xea\x01\n\x04Peer\x12\x18\n\x07pub_key\x18\
    \x01\x20\x01(\tR\x07pub_key\x12\x18\n\x07address\x18\x03\x20\x01(\tR\x07\
    address\x12\x1e\n\nbytes_sent\x18\x04\x20\x01(\x04R\nbytes_sent\x12\x1e\
    \n\nbytes_recv\x18\x05\x20\x01(\x04R\nbytes_recv\x12\x1a\n\x08sat_sent\
    \x18\x06\x20\x01(\x03R\x08sat_sent\x12\x1a\n\x08sat_recv\x18\x07\x20\x01\
    (\x03R\x08sat_recv\x12\x18\n\x07inbound\x18\x08\x20\x01(\x08R\x07inbound\
    \x12\x1c\n\tping_time\x18\t\x20\x01(\x03R\tping_time\"\x12\n\x10ListPeer\
    sRequest\"6\n\x11ListPeersResponse\x12!\n\x05peers\x18\x01\x20\x03(\x0b2\
    \x0b.lnrpc.PeerR\x05peers\"\x10\n\x0eGetInfoRequest\"\xbf\x03\n\x0fGetIn\
    foResponse\x12(\n\x0fidentity_pubkey\x18\x01\x20\x01(\tR\x0fidentity_pub\
    key\x12\x14\n\x05alias\x18\x02\x20\x01(\tR\x05alias\x122\n\x14num_pendin\
    g_channels\x18\x03\x20\x01(\rR\x14num_pending_channels\x120\n\x13num_act\
    ive_channels\x18\x04\x20\x01(\rR\x13num_active_channels\x12\x1c\n\tnum_p\
    eers\x18\x05\x20\x01(\rR\tnum_peers\x12\"\n\x0cblock_height\x18\x06\x20\
    \x01(\rR\x0cblock_height\x12\x1e\n\nblock_hash\x18\x08\x20\x01(\tR\nbloc\
    k_hash\x12(\n\x0fsynced_to_chain\x18\t\x20\x01(\x08R\x0fsynced_to_chain\
    \x12\x18\n\x07testnet\x18\n\x20\x01(\x08R\x07testnet\x12\x16\n\x06chains\
    \x18\x0b\x20\x03(\tR\x06chains\x12\x12\n\x04uris\x18\x0c\x20\x03(\tR\x04\
    uris\x124\n\x15best_header_timestamp\x18\r\x20\x01(\x03R\x15best_header_\
    timestamp\"z\n\x12ConfirmationUpdate\x12\x1b\n\tblock_sha\x18\x01\x20\
    \x01(\x0cR\x08blockSha\x12!\n\x0cblock_height\x18\x02\x20\x01(\x05R\x0bb\
    lockHeight\x12$\n\x0enum_confs_left\x18\x03\x20\x01(\rR\x0cnumConfsLeft\
    \"N\n\x11ChannelOpenUpdate\x129\n\rchannel_point\x18\x01\x20\x01(\x0b2\
    \x13.lnrpc.ChannelPointR\rchannel_point\"R\n\x12ChannelCloseUpdate\x12\"\
    \n\x0cclosing_txid\x18\x01\x20\x01(\x0cR\x0cclosing_txid\x12\x18\n\x07su\
    ccess\x18\x02\x20\x01(\x08R\x07success\"\xa8\x01\n\x13CloseChannelReques\
    t\x128\n\rchannel_point\x18\x01\x20\x01(\x0b2\x13.lnrpc.ChannelPointR\
    \x0cchannelPoint\x12\x14\n\x05force\x18\x02\x20\x01(\x08R\x05force\x12\
    \x1f\n\x0btarget_conf\x18\x03\x20\x01(\x05R\ntargetConf\x12\x20\n\x0csat\
    _per_byte\x18\x04\x20\x01(\x03R\nsatPerByte\"\xd9\x01\n\x11CloseStatusUp\
    date\x12<\n\rclose_pending\x18\x01\x20\x01(\x0b2\x14.lnrpc.PendingUpdate\
    H\0R\rclose_pending\x12?\n\x0cconfirmation\x18\x02\x20\x01(\x0b2\x19.lnr\
    pc.ConfirmationUpdateH\0R\x0cconfirmation\x12;\n\nchan_close\x18\x03\x20\
    \x01(\x0b2\x19.lnrpc.ChannelCloseUpdateH\0R\nchan_closeB\x08\n\x06update\
    \"G\n\rPendingUpdate\x12\x12\n\x04txid\x18\x01\x20\x01(\x0cR\x04txid\x12\
    \"\n\x0coutput_index\x18\x02\x20\x01(\rR\x0coutput_index\"\xb9\x02\n\x12\
    OpenChannelRequest\x12\x20\n\x0bnode_pubkey\x18\x02\x20\x01(\x0cR\x0bnod\
    e_pubkey\x12.\n\x12node_pubkey_string\x18\x03\x20\x01(\tR\x12node_pubkey\
    _string\x122\n\x14local_funding_amount\x18\x04\x20\x01(\x03R\x14local_fu\
    nding_amount\x12\x1a\n\x08push_sat\x18\x05\x20\x01(\x03R\x08push_sat\x12\
    \x1f\n\x0btarget_conf\x18\x06\x20\x01(\x05R\ntargetConf\x12\x20\n\x0csat\
    _per_byte\x18\x07\x20\x01(\x03R\nsatPerByte\x12\x18\n\x07private\x18\x08\
    \x20\x01(\x08R\x07private\x12$\n\rmin_htlc_msat\x18\t\x20\x01(\x03R\rmin\
    _htlc_msat\"\xd3\x01\n\x10OpenStatusUpdate\x12:\n\x0cchan_pending\x18\
    \x01\x20\x01(\x0b2\x14.lnrpc.PendingUpdateH\0R\x0cchan_pending\x12?\n\
    \x0cconfirmation\x18\x02\x20\x01(\x0b2\x19.lnrpc.ConfirmationUpdateH\0R\
    \x0cconfirmation\x128\n\tchan_open\x18\x03\x20\x01(\x0b2\x18.lnrpc.Chann\
    elOpenUpdateH\0R\tchan_openB\x08\n\x06update\"\xcf\x01\n\x0bPendingHTLC\
    \x12\x1a\n\x08incoming\x18\x01\x20\x01(\x08R\x08incoming\x12\x16\n\x06am\
    ount\x18\x02\x20\x01(\x03R\x06amount\x12\x1a\n\x08outpoint\x18\x03\x20\
    \x01(\tR\x08outpoint\x12(\n\x0fmaturity_height\x18\x04\x20\x01(\rR\x0fma\
    turity_height\x120\n\x13blocks_til_maturity\x18\x05\x20\x01(\x05R\x13blo\
    cks_til_maturity\x12\x14\n\x05stage\x18\x06\x20\x01(\rR\x05stage\"\x18\n\
    \x16PendingChannelsRequest\"\xca\n\n\x17PendingChannelsResponse\x120\n\
    \x13total_limbo_balance\x18\x01\x20\x01(\x03R\x13total_limbo_balance\x12\
    g\n\x15pending_open_channels\x18\x02\x20\x03(\x0b21.lnrpc.PendingChannel\
    sResponse.PendingOpenChannelR\x15pending_open_channels\x12h\n\x18pending\
    _closing_channels\x18\x03\x20\x03(\x0b2,.lnrpc.PendingChannelsResponse.C\
    losedChannelR\x18pending_closing_channels\x12y\n\x1epending_force_closin\
    g_channels\x18\x04\x20\x03(\x0b21.lnrpc.PendingChannelsResponse.ForceClo\
    sedChannelR\x1epending_force_closing_channels\x1a\xca\x01\n\x0ePendingCh\
    annel\x12(\n\x0fremote_node_pub\x18\x01\x20\x01(\tR\x0fremote_node_pub\
    \x12$\n\rchannel_point\x18\x02\x20\x01(\tR\rchannel_point\x12\x1a\n\x08c\
    apacity\x18\x03\x20\x01(\x03R\x08capacity\x12$\n\rlocal_balance\x18\x04\
    \x20\x01(\x03R\rlocal_balance\x12&\n\x0eremote_balance\x18\x05\x20\x01(\
    \x03R\x0eremote_balance\x1a\xf5\x01\n\x12PendingOpenChannel\x12G\n\x07ch\
    annel\x18\x01\x20\x01(\x0b2-.lnrpc.PendingChannelsResponse.PendingChanne\
    lR\x07channel\x120\n\x13confirmation_height\x18\x02\x20\x01(\rR\x13confi\
    rmation_height\x12\x1e\n\ncommit_fee\x18\x04\x20\x01(\x03R\ncommit_fee\
    \x12$\n\rcommit_weight\x18\x05\x20\x01(\x03R\rcommit_weight\x12\x1e\n\nf\
    ee_per_kw\x18\x06\x20\x01(\x03R\nfee_per_kw\x1a|\n\rClosedChannel\x12G\n\
    \x07channel\x18\x01\x20\x01(\x0b2-.lnrpc.PendingChannelsResponse.Pending\
    ChannelR\x07channel\x12\"\n\x0cclosing_txid\x18\x02\x20\x01(\tR\x0cclosi\
    ng_txid\x1a\xeb\x02\n\x12ForceClosedChannel\x12G\n\x07channel\x18\x01\
    \x20\x01(\x0b2-.lnrpc.PendingChannelsResponse.PendingChannelR\x07channel\
    \x12\"\n\x0cclosing_txid\x18\x02\x20\x01(\tR\x0cclosing_txid\x12$\n\rlim\
    bo_balance\x18\x03\x20\x01(\x03R\rlimbo_balance\x12(\n\x0fmaturity_heigh\
    t\x18\x04\x20\x01(\rR\x0fmaturity_height\x120\n\x13blocks_til_maturity\
    \x18\x05\x20\x01(\x05R\x13blocks_til_maturity\x12,\n\x11recovered_balanc\
    e\x18\x06\x20\x01(\x03R\x11recovered_balance\x128\n\rpending_htlcs\x18\
    \x08\x20\x03(\x0b2\x12.lnrpc.PendingHTLCR\rpending_htlcs\"9\n\x14WalletB\
    alanceRequest\x12!\n\x0cwitness_only\x18\x01\x20\x01(\x08R\x0bwitnessOnl\
    y\"\x9d\x01\n\x15WalletBalanceResponse\x12$\n\rtotal_balance\x18\x01\x20\
    \x01(\x03R\rtotal_balance\x12,\n\x11confirmed_balance\x18\x02\x20\x01(\
    \x03R\x11confirmed_balance\x120\n\x13unconfirmed_balance\x18\x03\x20\x01\
    (\x03R\x13unconfirmed_balance\"\x17\n\x15ChannelBalanceRequest\"2\n\x16C\
    hannelBalanceResponse\x12\x18\n\x07balance\x18\x01\x20\x01(\x03R\x07bala\
    nce\"^\n\x12QueryRoutesRequest\x12\x17\n\x07pub_key\x18\x01\x20\x01(\tR\
    \x06pubKey\x12\x10\n\x03amt\x18\x02\x20\x01(\x03R\x03amt\x12\x1d\n\nnum_\
    routes\x18\x03\x20\x01(\x05R\tnumRoutes\";\n\x13QueryRoutesResponse\x12$\
    \n\x06routes\x18\x01\x20\x03(\x0b2\x0c.lnrpc.RouteR\x06routes\"\x97\x01\
    \n\x03Hop\x12\x18\n\x07chan_id\x18\x01\x20\x01(\x04R\x07chan_id\x12$\n\r\
    chan_capacity\x18\x02\x20\x01(\x03R\rchan_capacity\x12&\n\x0eamt_to_forw\
    ard\x18\x03\x20\x01(\x03R\x0eamt_to_forward\x12\x10\n\x03fee\x18\x04\x20\
    \x01(\x03R\x03fee\x12\x16\n\x06expiry\x18\x05\x20\x01(\rR\x06expiry\"\
    \x8f\x01\n\x05Route\x12(\n\x0ftotal_time_lock\x18\x01\x20\x01(\rR\x0ftot\
    al_time_lock\x12\x1e\n\ntotal_fees\x18\x02\x20\x01(\x03R\ntotal_fees\x12\
    \x1c\n\ttotal_amt\x18\x03\x20\x01(\x03R\ttotal_amt\x12\x1e\n\x04hops\x18\
    \x04\x20\x03(\x0b2\n.lnrpc.HopR\x04hops\"*\n\x0fNodeInfoRequest\x12\x17\
    \n\x07pub_key\x18\x01\x20\x01(\tR\x06pubKey\"\x80\x01\n\x08NodeInfo\x12(\
    \n\x04node\x18\x01\x20\x01(\x0b2\x14.lnrpc.LightningNodeR\x04node\x12\"\
    \n\x0cnum_channels\x18\x02\x20\x01(\rR\x0cnum_channels\x12&\n\x0etotal_c\
    apacity\x18\x03\x20\x01(\x03R\x0etotal_capacity\"\xa9\x01\n\rLightningNo\
    de\x12\x20\n\x0blast_update\x18\x01\x20\x01(\rR\x0blast_update\x12\x18\n\
    \x07pub_key\x18\x02\x20\x01(\tR\x07pub_key\x12\x14\n\x05alias\x18\x03\
    \x20\x01(\tR\x05alias\x120\n\taddresses\x18\x04\x20\x03(\x0b2\x12.lnrpc.\
    NodeAddressR\taddresses\x12\x14\n\x05color\x18\x05\x20\x01(\tR\x05color\
    \";\n\x0bNodeAddress\x12\x18\n\x07network\x18\x01\x20\x01(\tR\x07network\
    \x12\x12\n\x04addr\x18\x02\x20\x01(\tR\x04addr\"\xad\x01\n\rRoutingPolic\
    y\x12(\n\x0ftime_lock_delta\x18\x01\x20\x01(\rR\x0ftime_lock_delta\x12\
    \x1a\n\x08min_htlc\x18\x02\x20\x01(\x03R\x08min_htlc\x12$\n\rfee_base_ms\
    at\x18\x03\x20\x01(\x03R\rfee_base_msat\x120\n\x13fee_rate_milli_msat\
    \x18\x04\x20\x01(\x03R\x13fee_rate_milli_msat\"\xbb\x02\n\x0bChannelEdge\
    \x12\x1e\n\nchannel_id\x18\x01\x20\x01(\x04R\nchannel_id\x12\x1e\n\nchan\
    _point\x18\x02\x20\x01(\tR\nchan_point\x12\x20\n\x0blast_update\x18\x03\
    \x20\x01(\rR\x0blast_update\x12\x1c\n\tnode1_pub\x18\x04\x20\x01(\tR\tno\
    de1_pub\x12\x1c\n\tnode2_pub\x18\x05\x20\x01(\tR\tnode2_pub\x12\x1a\n\
    \x08capacity\x18\x06\x20\x01(\x03R\x08capacity\x128\n\x0cnode1_policy\
    \x18\x07\x20\x01(\x0b2\x14.lnrpc.RoutingPolicyR\x0cnode1_policy\x128\n\
    \x0cnode2_policy\x18\x08\x20\x01(\x0b2\x14.lnrpc.RoutingPolicyR\x0cnode2\
    _policy\"\x15\n\x13ChannelGraphRequest\"d\n\x0cChannelGraph\x12*\n\x05no\
    des\x18\x01\x20\x03(\x0b2\x14.lnrpc.LightningNodeR\x05nodes\x12(\n\x05ed\
    ges\x18\x02\x20\x03(\x0b2\x12.lnrpc.ChannelEdgeR\x05edges\"*\n\x0fChanIn\
    foRequest\x12\x17\n\x07chan_id\x18\x01\x20\x01(\x04R\x06chanId\"\x14\n\
    \x12NetworkInfoRequest\"\x83\x03\n\x0bNetworkInfo\x12&\n\x0egraph_diamet\
    er\x18\x01\x20\x01(\rR\x0egraph_diameter\x12&\n\x0eavg_out_degree\x18\
    \x02\x20\x01(\x01R\x0eavg_out_degree\x12&\n\x0emax_out_degree\x18\x03\
    \x20\x01(\rR\x0emax_out_degree\x12\x1c\n\tnum_nodes\x18\x04\x20\x01(\rR\
    \tnum_nodes\x12\"\n\x0cnum_channels\x18\x05\x20\x01(\rR\x0cnum_channels\
    \x126\n\x16total_network_capacity\x18\x06\x20\x01(\x03R\x16total_network\
    _capacity\x12*\n\x10avg_channel_size\x18\x07\x20\x01(\x01R\x10avg_channe\
    l_size\x12*\n\x10min_channel_size\x18\x08\x20\x01(\x03R\x10min_channel_s\
    ize\x12*\n\x10max_channel_size\x18\t\x20\x01(\x03R\x10max_channel_size\"\
    \r\n\x0bStopRequest\"\x0e\n\x0cStopResponse\"\x1b\n\x19GraphTopologySubs\
    cription\"\xcd\x01\n\x13GraphTopologyUpdate\x124\n\x0cnode_updates\x18\
    \x01\x20\x03(\x0b2\x11.lnrpc.NodeUpdateR\x0bnodeUpdates\x12A\n\x0fchanne\
    l_updates\x18\x02\x20\x03(\x0b2\x18.lnrpc.ChannelEdgeUpdateR\x0echannelU\
    pdates\x12=\n\x0cclosed_chans\x18\x03\x20\x03(\x0b2\x1a.lnrpc.ClosedChan\
    nelUpdateR\x0bclosedChans\"\x8c\x01\n\nNodeUpdate\x12\x1c\n\taddresses\
    \x18\x01\x20\x03(\tR\taddresses\x12!\n\x0cidentity_key\x18\x02\x20\x01(\
    \tR\x0bidentityKey\x12'\n\x0fglobal_features\x18\x03\x20\x01(\x0cR\x0egl\
    obalFeatures\x12\x14\n\x05alias\x18\x04\x20\x01(\tR\x05alias\"\x8d\x02\n\
    \x11ChannelEdgeUpdate\x12\x17\n\x07chan_id\x18\x01\x20\x01(\x04R\x06chan\
    Id\x122\n\nchan_point\x18\x02\x20\x01(\x0b2\x13.lnrpc.ChannelPointR\tcha\
    nPoint\x12\x1a\n\x08capacity\x18\x03\x20\x01(\x03R\x08capacity\x12;\n\
    \x0erouting_policy\x18\x04\x20\x01(\x0b2\x14.lnrpc.RoutingPolicyR\rrouti\
    ngPolicy\x12)\n\x10advertising_node\x18\x05\x20\x01(\tR\x0fadvertisingNo\
    de\x12'\n\x0fconnecting_node\x18\x06\x20\x01(\tR\x0econnectingNode\"\xa3\
    \x01\n\x13ClosedChannelUpdate\x12\x17\n\x07chan_id\x18\x01\x20\x01(\x04R\
    \x06chanId\x12\x1a\n\x08capacity\x18\x02\x20\x01(\x03R\x08capacity\x12#\
    \n\rclosed_height\x18\x03\x20\x01(\rR\x0cclosedHeight\x122\n\nchan_point\
    \x18\x04\x20\x01(\x0b2\x13.lnrpc.ChannelPointR\tchanPoint\"\x9d\x03\n\
    \x07Invoice\x12\x12\n\x04memo\x18\x01\x20\x01(\tR\x04memo\x12\x18\n\x07r\
    eceipt\x18\x02\x20\x01(\x0cR\x07receipt\x12\x1e\n\nr_preimage\x18\x03\
    \x20\x01(\x0cR\nr_preimage\x12\x16\n\x06r_hash\x18\x04\x20\x01(\x0cR\x06\
    r_hash\x12\x14\n\x05value\x18\x05\x20\x01(\x03R\x05value\x12\x18\n\x07se\
    ttled\x18\x06\x20\x01(\x08R\x07settled\x12$\n\rcreation_date\x18\x07\x20\
    \x01(\x03R\rcreation_date\x12\x20\n\x0bsettle_date\x18\x08\x20\x01(\x03R\
    \x0bsettle_date\x12(\n\x0fpayment_request\x18\t\x20\x01(\tR\x0fpayment_r\
    equest\x12*\n\x10description_hash\x18\n\x20\x01(\x0cR\x10description_has\
    h\x12\x16\n\x06expiry\x18\x0b\x20\x01(\x03R\x06expiry\x12$\n\rfallback_a\
    ddr\x18\x0c\x20\x01(\tR\rfallback_addr\x12\x20\n\x0bcltv_expiry\x18\r\
    \x20\x01(\x04R\x0bcltv_expiry\"V\n\x12AddInvoiceResponse\x12\x16\n\x06r_\
    hash\x18\x01\x20\x01(\x0cR\x06r_hash\x12(\n\x0fpayment_request\x18\x02\
    \x20\x01(\tR\x0fpayment_request\"E\n\x0bPaymentHash\x12\x1e\n\nr_hash_st\
    r\x18\x01\x20\x01(\tR\nr_hash_str\x12\x16\n\x06r_hash\x18\x02\x20\x01(\
    \x0cR\x06r_hash\"7\n\x12ListInvoiceRequest\x12!\n\x0cpending_only\x18\
    \x01\x20\x01(\x08R\x0bpendingOnly\"A\n\x13ListInvoiceResponse\x12*\n\x08\
    invoices\x18\x01\x20\x03(\x0b2\x0e.lnrpc.InvoiceR\x08invoices\"\x15\n\
    \x13InvoiceSubscription\"\xbb\x01\n\x07Payment\x12\"\n\x0cpayment_hash\
    \x18\x01\x20\x01(\tR\x0cpayment_hash\x12\x14\n\x05value\x18\x02\x20\x01(\
    \x03R\x05value\x12$\n\rcreation_date\x18\x03\x20\x01(\x03R\rcreation_dat\
    e\x12\x12\n\x04path\x18\x04\x20\x03(\tR\x04path\x12\x10\n\x03fee\x18\x05\
    \x20\x01(\x03R\x03fee\x12*\n\x10payment_preimage\x18\x06\x20\x01(\tR\x10\
    payment_preimage\"\x15\n\x13ListPaymentsRequest\"B\n\x14ListPaymentsResp\
    onse\x12*\n\x08payments\x18\x01\x20\x03(\x0b2\x0e.lnrpc.PaymentR\x08paym\
    ents\"\x1a\n\x18DeleteAllPaymentsRequest\"\x1b\n\x19DeleteAllPaymentsRes\
    ponse\"F\n\x11DebugLevelRequest\x12\x12\n\x04show\x18\x01\x20\x01(\x08R\
    \x04show\x12\x1d\n\nlevel_spec\x18\x02\x20\x01(\tR\tlevelSpec\"6\n\x12De\
    bugLevelResponse\x12\x20\n\x0bsub_systems\x18\x01\x20\x01(\tR\x0bsub_sys\
    tems\"'\n\x0cPayReqString\x12\x17\n\x07pay_req\x18\x01\x20\x01(\tR\x06pa\
    yReq\"\xbe\x02\n\x06PayReq\x12\x20\n\x0bdestination\x18\x01\x20\x01(\tR\
    \x0bdestination\x12\"\n\x0cpayment_hash\x18\x02\x20\x01(\tR\x0cpayment_h\
    ash\x12\"\n\x0cnum_satoshis\x18\x03\x20\x01(\x03R\x0cnum_satoshis\x12\
    \x1c\n\ttimestamp\x18\x04\x20\x01(\x03R\ttimestamp\x12\x16\n\x06expiry\
    \x18\x05\x20\x01(\x03R\x06expiry\x12\x20\n\x0bdescription\x18\x06\x20\
    \x01(\tR\x0bdescription\x12*\n\x10description_hash\x18\x07\x20\x01(\tR\
    \x10description_hash\x12$\n\rfallback_addr\x18\x08\x20\x01(\tR\rfallback\
    _addr\x12\x20\n\x0bcltv_expiry\x18\t\x20\x01(\x03R\x0bcltv_expiry\"\x12\
    \n\x10FeeReportRequest\"\x99\x01\n\x10ChannelFeeReport\x12!\n\nchan_poin\
    t\x18\x01\x20\x01(\tR\rchannel_point\x12$\n\rbase_fee_msat\x18\x02\x20\
    \x01(\x03R\rbase_fee_msat\x12\x20\n\x0bfee_per_mil\x18\x03\x20\x01(\x03R\
    \x0bfee_per_mil\x12\x1a\n\x08fee_rate\x18\x04\x20\x01(\x01R\x08fee_rate\
    \"P\n\x11FeeReportResponse\x12;\n\x0cchannel_fees\x18\x01\x20\x03(\x0b2\
    \x17.lnrpc.ChannelFeeReportR\x0cchannel_fees\"\xdb\x01\n\x13PolicyUpdate\
    Request\x12\x18\n\x06global\x18\x01\x20\x01(\x08H\0R\x06global\x125\n\nc\
    han_point\x18\x02\x20\x01(\x0b2\x13.lnrpc.ChannelPointH\0R\nchan_point\
    \x12$\n\rbase_fee_msat\x18\x03\x20\x01(\x03R\rbase_fee_msat\x12\x1a\n\
    \x08fee_rate\x18\x04\x20\x01(\x01R\x08fee_rate\x12(\n\x0ftime_lock_delta\
    \x18\x05\x20\x01(\rR\x0ftime_lock_deltaB\x07\n\x05scope\"\x16\n\x14Polic\
    yUpdateResponse\".\n\x12GetSPVProofRequest\x12\x18\n\x07tx_hash\x18\x01\
    \x20\x01(\x0cR\x07tx_hash\"S\n\x13GetSPVProofResponse\x12<\n\x19serializ\
    ed_full_spv_proof\x18\x01\x20\x01(\x0cR\x19serialized_full_spv_proof\"U\
    \n\x15VerifySPVProofRequest\x12<\n\x19serialized_full_spv_proof\x18\x01\
    \x20\x01(\x0cR\x19serialized_full_spv_proof\"J\n\x16VerifySPVProofRespon\
    se\x120\n\x13verification_status\x18\x01\x20\x01(\x08R\x13verification_s\
    tatus2\xdc\x01\n\x0eWalletUnlocker\x12d\n\x0cCreateWallet\x12\x1a.lnrpc.\
    CreateWalletRequest\x1a\x1b.lnrpc.CreateWalletResponse\"\x1b\x82\xd3\xe4\
    \x93\x02\x15\"\x10/v1/createwallet:\x01*\x12d\n\x0cUnlockWallet\x12\x1a.\
    lnrpc.UnlockWalletRequest\x1a\x1b.lnrpc.UnlockWalletResponse\"\x1b\x82\
    \xd3\xe4\x93\x02\x15\"\x10/v1/unlockwallet:\x01*2\x88\x1c\n\tLightning\
    \x12j\n\rWalletBalance\x12\x1b.lnrpc.WalletBalanceRequest\x1a\x1c.lnrpc.\
    WalletBalanceResponse\"\x1e\x82\xd3\xe4\x93\x02\x18\x12\x16/v1/balance/b\
    lockchain\x12k\n\x0eChannelBalance\x12\x1c.lnrpc.ChannelBalanceRequest\
    \x1a\x1d.lnrpc.ChannelBalanceResponse\"\x1c\x82\xd3\xe4\x93\x02\x16\x12\
    \x14/v1/balance/channels\x12e\n\x0fGetTransactions\x12\x1d.lnrpc.GetTran\
    sactionsRequest\x1a\x19.lnrpc.TransactionDetails\"\x18\x82\xd3\xe4\x93\
    \x02\x12\x12\x10/v1/transactions\x12[\n\tSendCoins\x12\x17.lnrpc.SendCoi\
    nsRequest\x1a\x18.lnrpc.SendCoinsResponse\"\x1b\x82\xd3\xe4\x93\x02\x15\
    \"\x10/v1/transactions:\x01*\x12L\n\x15SubscribeTransactions\x12\x1d.lnr\
    pc.GetTransactionsRequest\x1a\x12.lnrpc.Transaction0\x01\x12;\n\x08SendM\
    any\x12\x16.lnrpc.SendManyRequest\x1a\x17.lnrpc.SendManyResponse\x12A\n\
    \nNewAddress\x12\x18.lnrpc.NewAddressRequest\x1a\x19.lnrpc.NewAddressRes\
    ponse\x12g\n\x11NewWitnessAddress\x12\x1f.lnrpc.NewWitnessAddressRequest\
    \x1a\x19.lnrpc.NewAddressResponse\"\x16\x82\xd3\xe4\x93\x02\x10\x12\x0e/\
    v1/newaddress\x12D\n\x0bSignMessage\x12\x19.lnrpc.SignMessageRequest\x1a\
    \x1a.lnrpc.SignMessageResponse\x12J\n\rVerifyMessage\x12\x1b.lnrpc.Verif\
    yMessageRequest\x1a\x1c.lnrpc.VerifyMessageResponse\x12Z\n\x0bConnectPee\
    r\x12\x19.lnrpc.ConnectPeerRequest\x1a\x1a.lnrpc.ConnectPeerResponse\"\
    \x14\x82\xd3\xe4\x93\x02\x0e\"\t/v1/peers:\x01*\x12j\n\x0eDisconnectPeer\
    \x12\x1c.lnrpc.DisconnectPeerRequest\x1a\x1d.lnrpc.DisconnectPeerRespons\
    e\"\x1b\x82\xd3\xe4\x93\x02\x15*\x13/v1/peers/{pub_key}\x12Q\n\tListPeer\
    s\x12\x17.lnrpc.ListPeersRequest\x1a\x18.lnrpc.ListPeersResponse\"\x11\
    \x82\xd3\xe4\x93\x02\x0b\x12\t/v1/peers\x12M\n\x07GetInfo\x12\x15.lnrpc.\
    GetInfoRequest\x1a\x16.lnrpc.GetInfoResponse\"\x13\x82\xd3\xe4\x93\x02\r\
    \x12\x0b/v1/getinfo\x12n\n\x0fPendingChannels\x12\x1d.lnrpc.PendingChann\
    elsRequest\x1a\x1e.lnrpc.PendingChannelsResponse\"\x1c\x82\xd3\xe4\x93\
    \x02\x16\x12\x14/v1/channels/pending\x12]\n\x0cListChannels\x12\x1a.lnrp\
    c.ListChannelsRequest\x1a\x1b.lnrpc.ListChannelsResponse\"\x14\x82\xd3\
    \xe4\x93\x02\x0e\x12\x0c/v1/channels\x12Z\n\x0fOpenChannelSync\x12\x19.l\
    nrpc.OpenChannelRequest\x1a\x13.lnrpc.ChannelPoint\"\x17\x82\xd3\xe4\x93\
    \x02\x11\"\x0c/v1/channels:\x01*\x12C\n\x0bOpenChannel\x12\x19.lnrpc.Ope\
    nChannelRequest\x1a\x17.lnrpc.OpenStatusUpdate0\x01\x12\x9a\x01\n\x0cClo\
    seChannel\x12\x1a.lnrpc.CloseChannelRequest\x1a\x18.lnrpc.CloseStatusUpd\
    ate\"R\x82\xd3\xe4\x93\x02L*J/v1/channels/{channel_point.funding_txid_st\
    r}/{channel_point.output_index}0\x01\x12:\n\x0bSendPayment\x12\x12.lnrpc\
    .SendRequest\x1a\x13.lnrpc.SendResponse(\x010\x01\x12`\n\x0fSendPaymentS\
    ync\x12\x12.lnrpc.SendRequest\x1a\x13.lnrpc.SendResponse\"$\x82\xd3\xe4\
    \x93\x02\x1e\"\x19/v1/channels/transactions:\x01*\x12P\n\nAddInvoice\x12\
    \x0e.lnrpc.Invoice\x1a\x19.lnrpc.AddInvoiceResponse\"\x17\x82\xd3\xe4\
    \x93\x02\x11\"\x0c/v1/invoices:\x01*\x12[\n\x0cListInvoices\x12\x19.lnrp\
    c.ListInvoiceRequest\x1a\x1a.lnrpc.ListInvoiceResponse\"\x14\x82\xd3\xe4\
    \x93\x02\x0e\x12\x0c/v1/invoices\x12U\n\rLookupInvoice\x12\x12.lnrpc.Pay\
    mentHash\x1a\x0e.lnrpc.Invoice\"\x20\x82\xd3\xe4\x93\x02\x1a\x12\x18/v1/\
    invoice/{r_hash_str}\x12a\n\x11SubscribeInvoices\x12\x1a.lnrpc.InvoiceSu\
    bscription\x1a\x0e.lnrpc.Invoice\"\x1e\x82\xd3\xe4\x93\x02\x18\x12\x16/v\
    1/invoices/subscribe0\x01\x12P\n\x0cDecodePayReq\x12\x13.lnrpc.PayReqStr\
    ing\x1a\r.lnrpc.PayReq\"\x1c\x82\xd3\xe4\x93\x02\x16\x12\x14/v1/payreq/{\
    pay_req}\x12]\n\x0cListPayments\x12\x1a.lnrpc.ListPaymentsRequest\x1a\
    \x1b.lnrpc.ListPaymentsResponse\"\x14\x82\xd3\xe4\x93\x02\x0e\x12\x0c/v1\
    /payments\x12l\n\x11DeleteAllPayments\x12\x1f.lnrpc.DeleteAllPaymentsReq\
    uest\x1a\x20.lnrpc.DeleteAllPaymentsResponse\"\x14\x82\xd3\xe4\x93\x02\
    \x0e*\x0c/v1/payments\x12S\n\rDescribeGraph\x12\x1a.lnrpc.ChannelGraphRe\
    quest\x1a\x13.lnrpc.ChannelGraph\"\x11\x82\xd3\xe4\x93\x02\x0b\x12\t/v1/\
    graph\x12[\n\x0bGetChanInfo\x12\x16.lnrpc.ChanInfoRequest\x1a\x12.lnrpc.\
    ChannelEdge\"\x20\x82\xd3\xe4\x93\x02\x1a\x12\x18/v1/graph/edge/{chan_id\
    }\x12X\n\x0bGetNodeInfo\x12\x16.lnrpc.NodeInfoRequest\x1a\x0f.lnrpc.Node\
    Info\"\x20\x82\xd3\xe4\x93\x02\x1a\x12\x18/v1/graph/node/{pub_key}\x12n\
    \n\x0bQueryRoutes\x12\x19.lnrpc.QueryRoutesRequest\x1a\x1a.lnrpc.QueryRo\
    utesResponse\"(\x82\xd3\xe4\x93\x02\"\x12\x20/v1/graph/routes/{pub_key}/\
    {amt}\x12W\n\x0eGetNetworkInfo\x12\x19.lnrpc.NetworkInfoRequest\x1a\x12.\
    lnrpc.NetworkInfo\"\x16\x82\xd3\xe4\x93\x02\x10\x12\x0e/v1/graph/info\
    \x125\n\nStopDaemon\x12\x12.lnrpc.StopRequest\x1a\x13.lnrpc.StopResponse\
    \x12W\n\x15SubscribeChannelGraph\x12\x20.lnrpc.GraphTopologySubscription\
    \x1a\x1a.lnrpc.GraphTopologyUpdate0\x01\x12A\n\nDebugLevel\x12\x18.lnrpc\
    .DebugLevelRequest\x1a\x19.lnrpc.DebugLevelResponse\x12P\n\tFeeReport\
    \x12\x17.lnrpc.FeeReportRequest\x1a\x18.lnrpc.FeeReportResponse\"\x10\
    \x82\xd3\xe4\x93\x02\n\x12\x08/v1/fees\x12i\n\x13UpdateChannelPolicy\x12\
    \x1a.lnrpc.PolicyUpdateRequest\x1a\x1b.lnrpc.PolicyUpdateResponse\"\x19\
    \x82\xd3\xe4\x93\x02\x13\"\x0e/v1/chanpolicy:\x01*\x12D\n\x0bGetSPVProof\
    \x12\x19.lnrpc.GetSPVProofRequest\x1a\x1a.lnrpc.GetSPVProofResponse\x12M\
    \n\x0eVerifySPVProof\x12\x1c.lnrpc.VerifySPVProofRequest\x1a\x1d.lnrpc.V\
    erifySPVProofResponseb\x06proto3\
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
