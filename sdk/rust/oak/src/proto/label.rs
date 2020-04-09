// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `oak/proto/label.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Label {
    // message fields
    pub secrecy_tags: ::protobuf::RepeatedField<Tag>,
    pub integrity_tags: ::protobuf::RepeatedField<Tag>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Label {
    fn default() -> &'a Label {
        <Label as ::protobuf::Message>::default_instance()
    }
}

impl Label {
    pub fn new() -> Label {
        ::std::default::Default::default()
    }

    // repeated .oak.label.Tag secrecy_tags = 1;


    pub fn get_secrecy_tags(&self) -> &[Tag] {
        &self.secrecy_tags
    }
    pub fn clear_secrecy_tags(&mut self) {
        self.secrecy_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_secrecy_tags(&mut self, v: ::protobuf::RepeatedField<Tag>) {
        self.secrecy_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_secrecy_tags(&mut self) -> &mut ::protobuf::RepeatedField<Tag> {
        &mut self.secrecy_tags
    }

    // Take field
    pub fn take_secrecy_tags(&mut self) -> ::protobuf::RepeatedField<Tag> {
        ::std::mem::replace(&mut self.secrecy_tags, ::protobuf::RepeatedField::new())
    }

    // repeated .oak.label.Tag integrity_tags = 2;


    pub fn get_integrity_tags(&self) -> &[Tag] {
        &self.integrity_tags
    }
    pub fn clear_integrity_tags(&mut self) {
        self.integrity_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_integrity_tags(&mut self, v: ::protobuf::RepeatedField<Tag>) {
        self.integrity_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_integrity_tags(&mut self) -> &mut ::protobuf::RepeatedField<Tag> {
        &mut self.integrity_tags
    }

    // Take field
    pub fn take_integrity_tags(&mut self) -> ::protobuf::RepeatedField<Tag> {
        ::std::mem::replace(&mut self.integrity_tags, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Label {
    fn is_initialized(&self) -> bool {
        for v in &self.secrecy_tags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.integrity_tags {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.secrecy_tags)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.integrity_tags)?;
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
        for value in &self.secrecy_tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.integrity_tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.secrecy_tags {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.integrity_tags {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Label {
        Label::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tag>>(
                    "secrecy_tags",
                    |m: &Label| { &m.secrecy_tags },
                    |m: &mut Label| { &mut m.secrecy_tags },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tag>>(
                    "integrity_tags",
                    |m: &Label| { &m.integrity_tags },
                    |m: &mut Label| { &mut m.integrity_tags },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Label>(
                    "Label",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Label {
        static mut instance: ::protobuf::lazy::Lazy<Label> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Label,
        };
        unsafe {
            instance.get(Label::new)
        }
    }
}

impl ::protobuf::Clear for Label {
    fn clear(&mut self) {
        self.secrecy_tags.clear();
        self.integrity_tags.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Label {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Label {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tag {
    // message oneof groups
    pub tag: ::std::option::Option<Tag_oneof_tag>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Tag {
    fn default() -> &'a Tag {
        <Tag as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Tag_oneof_tag {
    grpc_tag(GrpcTag),
    web_assembly_module_tag(WebAssemblyModuleTag),
    tls_endpoint_tag(TlsEndpointTag),
}

impl Tag {
    pub fn new() -> Tag {
        ::std::default::Default::default()
    }

    // .oak.label.GrpcTag grpc_tag = 1;


    pub fn get_grpc_tag(&self) -> &GrpcTag {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(ref v)) => v,
            _ => GrpcTag::default_instance(),
        }
    }
    pub fn clear_grpc_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_grpc_tag(&self) -> bool {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_grpc_tag(&mut self, v: GrpcTag) {
        self.tag = ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(v))
    }

    // Mutable pointer to the field.
    pub fn mut_grpc_tag(&mut self) -> &mut GrpcTag {
        if let ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(_)) = self.tag {
        } else {
            self.tag = ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(GrpcTag::new()));
        }
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_grpc_tag(&mut self) -> GrpcTag {
        if self.has_grpc_tag() {
            match self.tag.take() {
                ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(v)) => v,
                _ => panic!(),
            }
        } else {
            GrpcTag::new()
        }
    }

    // .oak.label.WebAssemblyModuleTag web_assembly_module_tag = 2;


    pub fn get_web_assembly_module_tag(&self) -> &WebAssemblyModuleTag {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(ref v)) => v,
            _ => WebAssemblyModuleTag::default_instance(),
        }
    }
    pub fn clear_web_assembly_module_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_web_assembly_module_tag(&self) -> bool {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_web_assembly_module_tag(&mut self, v: WebAssemblyModuleTag) {
        self.tag = ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(v))
    }

    // Mutable pointer to the field.
    pub fn mut_web_assembly_module_tag(&mut self) -> &mut WebAssemblyModuleTag {
        if let ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(_)) = self.tag {
        } else {
            self.tag = ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(WebAssemblyModuleTag::new()));
        }
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_web_assembly_module_tag(&mut self) -> WebAssemblyModuleTag {
        if self.has_web_assembly_module_tag() {
            match self.tag.take() {
                ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(v)) => v,
                _ => panic!(),
            }
        } else {
            WebAssemblyModuleTag::new()
        }
    }

    // .oak.label.TlsEndpointTag tls_endpoint_tag = 3;


    pub fn get_tls_endpoint_tag(&self) -> &TlsEndpointTag {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(ref v)) => v,
            _ => TlsEndpointTag::default_instance(),
        }
    }
    pub fn clear_tls_endpoint_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_tls_endpoint_tag(&self) -> bool {
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tls_endpoint_tag(&mut self, v: TlsEndpointTag) {
        self.tag = ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tls_endpoint_tag(&mut self) -> &mut TlsEndpointTag {
        if let ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(_)) = self.tag {
        } else {
            self.tag = ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(TlsEndpointTag::new()));
        }
        match self.tag {
            ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tls_endpoint_tag(&mut self) -> TlsEndpointTag {
        if self.has_tls_endpoint_tag() {
            match self.tag.take() {
                ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(v)) => v,
                _ => panic!(),
            }
        } else {
            TlsEndpointTag::new()
        }
    }
}

impl ::protobuf::Message for Tag {
    fn is_initialized(&self) -> bool {
        if let Some(Tag_oneof_tag::grpc_tag(ref v)) = self.tag {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Tag_oneof_tag::web_assembly_module_tag(ref v)) = self.tag {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Tag_oneof_tag::tls_endpoint_tag(ref v)) = self.tag {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.tag = ::std::option::Option::Some(Tag_oneof_tag::grpc_tag(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.tag = ::std::option::Option::Some(Tag_oneof_tag::web_assembly_module_tag(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.tag = ::std::option::Option::Some(Tag_oneof_tag::tls_endpoint_tag(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.tag {
            match v {
                &Tag_oneof_tag::grpc_tag(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Tag_oneof_tag::web_assembly_module_tag(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Tag_oneof_tag::tls_endpoint_tag(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.tag {
            match v {
                &Tag_oneof_tag::grpc_tag(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Tag_oneof_tag::web_assembly_module_tag(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Tag_oneof_tag::tls_endpoint_tag(ref v) => {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Tag {
        Tag::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GrpcTag>(
                    "grpc_tag",
                    Tag::has_grpc_tag,
                    Tag::get_grpc_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WebAssemblyModuleTag>(
                    "web_assembly_module_tag",
                    Tag::has_web_assembly_module_tag,
                    Tag::get_web_assembly_module_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TlsEndpointTag>(
                    "tls_endpoint_tag",
                    Tag::has_tls_endpoint_tag,
                    Tag::get_tls_endpoint_tag,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tag>(
                    "Tag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Tag {
        static mut instance: ::protobuf::lazy::Lazy<Tag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tag,
        };
        unsafe {
            instance.get(Tag::new)
        }
    }
}

impl ::protobuf::Clear for Tag {
    fn clear(&mut self) {
        self.tag = ::std::option::Option::None;
        self.tag = ::std::option::Option::None;
        self.tag = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GrpcTag {
    // message fields
    pub authorization_bearer_token_hmac: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GrpcTag {
    fn default() -> &'a GrpcTag {
        <GrpcTag as ::protobuf::Message>::default_instance()
    }
}

impl GrpcTag {
    pub fn new() -> GrpcTag {
        ::std::default::Default::default()
    }

    // bytes authorization_bearer_token_hmac = 1;


    pub fn get_authorization_bearer_token_hmac(&self) -> &[u8] {
        &self.authorization_bearer_token_hmac
    }
    pub fn clear_authorization_bearer_token_hmac(&mut self) {
        self.authorization_bearer_token_hmac.clear();
    }

    // Param is passed by value, moved
    pub fn set_authorization_bearer_token_hmac(&mut self, v: ::std::vec::Vec<u8>) {
        self.authorization_bearer_token_hmac = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_authorization_bearer_token_hmac(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.authorization_bearer_token_hmac
    }

    // Take field
    pub fn take_authorization_bearer_token_hmac(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.authorization_bearer_token_hmac, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for GrpcTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.authorization_bearer_token_hmac)?;
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
        if !self.authorization_bearer_token_hmac.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.authorization_bearer_token_hmac);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.authorization_bearer_token_hmac.is_empty() {
            os.write_bytes(1, &self.authorization_bearer_token_hmac)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GrpcTag {
        GrpcTag::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "authorization_bearer_token_hmac",
                    |m: &GrpcTag| { &m.authorization_bearer_token_hmac },
                    |m: &mut GrpcTag| { &mut m.authorization_bearer_token_hmac },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GrpcTag>(
                    "GrpcTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GrpcTag {
        static mut instance: ::protobuf::lazy::Lazy<GrpcTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GrpcTag,
        };
        unsafe {
            instance.get(GrpcTag::new)
        }
    }
}

impl ::protobuf::Clear for GrpcTag {
    fn clear(&mut self) {
        self.authorization_bearer_token_hmac.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WebAssemblyModuleTag {
    // message fields
    pub module_attestation: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WebAssemblyModuleTag {
    fn default() -> &'a WebAssemblyModuleTag {
        <WebAssemblyModuleTag as ::protobuf::Message>::default_instance()
    }
}

impl WebAssemblyModuleTag {
    pub fn new() -> WebAssemblyModuleTag {
        ::std::default::Default::default()
    }

    // bytes module_attestation = 1;


    pub fn get_module_attestation(&self) -> &[u8] {
        &self.module_attestation
    }
    pub fn clear_module_attestation(&mut self) {
        self.module_attestation.clear();
    }

    // Param is passed by value, moved
    pub fn set_module_attestation(&mut self, v: ::std::vec::Vec<u8>) {
        self.module_attestation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_module_attestation(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.module_attestation
    }

    // Take field
    pub fn take_module_attestation(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.module_attestation, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for WebAssemblyModuleTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.module_attestation)?;
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
        if !self.module_attestation.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.module_attestation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.module_attestation.is_empty() {
            os.write_bytes(1, &self.module_attestation)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WebAssemblyModuleTag {
        WebAssemblyModuleTag::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "module_attestation",
                    |m: &WebAssemblyModuleTag| { &m.module_attestation },
                    |m: &mut WebAssemblyModuleTag| { &mut m.module_attestation },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WebAssemblyModuleTag>(
                    "WebAssemblyModuleTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static WebAssemblyModuleTag {
        static mut instance: ::protobuf::lazy::Lazy<WebAssemblyModuleTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WebAssemblyModuleTag,
        };
        unsafe {
            instance.get(WebAssemblyModuleTag::new)
        }
    }
}

impl ::protobuf::Clear for WebAssemblyModuleTag {
    fn clear(&mut self) {
        self.module_attestation.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WebAssemblyModuleTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WebAssemblyModuleTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TlsEndpointTag {
    // message fields
    pub certificate_subject_alternative_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TlsEndpointTag {
    fn default() -> &'a TlsEndpointTag {
        <TlsEndpointTag as ::protobuf::Message>::default_instance()
    }
}

impl TlsEndpointTag {
    pub fn new() -> TlsEndpointTag {
        ::std::default::Default::default()
    }

    // string certificate_subject_alternative_name = 1;


    pub fn get_certificate_subject_alternative_name(&self) -> &str {
        &self.certificate_subject_alternative_name
    }
    pub fn clear_certificate_subject_alternative_name(&mut self) {
        self.certificate_subject_alternative_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_certificate_subject_alternative_name(&mut self, v: ::std::string::String) {
        self.certificate_subject_alternative_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_certificate_subject_alternative_name(&mut self) -> &mut ::std::string::String {
        &mut self.certificate_subject_alternative_name
    }

    // Take field
    pub fn take_certificate_subject_alternative_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.certificate_subject_alternative_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for TlsEndpointTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.certificate_subject_alternative_name)?;
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
        if !self.certificate_subject_alternative_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.certificate_subject_alternative_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.certificate_subject_alternative_name.is_empty() {
            os.write_string(1, &self.certificate_subject_alternative_name)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TlsEndpointTag {
        TlsEndpointTag::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "certificate_subject_alternative_name",
                    |m: &TlsEndpointTag| { &m.certificate_subject_alternative_name },
                    |m: &mut TlsEndpointTag| { &mut m.certificate_subject_alternative_name },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TlsEndpointTag>(
                    "TlsEndpointTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TlsEndpointTag {
        static mut instance: ::protobuf::lazy::Lazy<TlsEndpointTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TlsEndpointTag,
        };
        unsafe {
            instance.get(TlsEndpointTag::new)
        }
    }
}

impl ::protobuf::Clear for TlsEndpointTag {
    fn clear(&mut self) {
        self.certificate_subject_alternative_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TlsEndpointTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TlsEndpointTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15oak/proto/label.proto\x12\toak.label\"q\n\x05Label\x121\n\x0csecre\
    cy_tags\x18\x01\x20\x03(\x0b2\x0e.oak.label.TagR\x0bsecrecyTags\x125\n\
    \x0eintegrity_tags\x18\x02\x20\x03(\x0b2\x0e.oak.label.TagR\rintegrityTa\
    gs\"\xde\x01\n\x03Tag\x12/\n\x08grpc_tag\x18\x01\x20\x01(\x0b2\x12.oak.l\
    abel.GrpcTagH\0R\x07grpcTag\x12X\n\x17web_assembly_module_tag\x18\x02\
    \x20\x01(\x0b2\x1f.oak.label.WebAssemblyModuleTagH\0R\x14webAssemblyModu\
    leTag\x12E\n\x10tls_endpoint_tag\x18\x03\x20\x01(\x0b2\x19.oak.label.Tls\
    EndpointTagH\0R\x0etlsEndpointTagB\x05\n\x03tag\"P\n\x07GrpcTag\x12E\n\
    \x1fauthorization_bearer_token_hmac\x18\x01\x20\x01(\x0cR\x1cauthorizati\
    onBearerTokenHmac\"E\n\x14WebAssemblyModuleTag\x12-\n\x12module_attestat\
    ion\x18\x01\x20\x01(\x0cR\x11moduleAttestation\"a\n\x0eTlsEndpointTag\
    \x12O\n$certificate_subject_alternative_name\x18\x01\x20\x01(\tR!certifi\
    cateSubjectAlternativeNameb\x06proto3\
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