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
//! Generated file from `google/api/expr/v1alpha1/eval.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct EvalState {
    // message fields
    pub values: ::protobuf::RepeatedField<ExprValue>,
    pub results: ::protobuf::RepeatedField<EvalState_Result>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EvalState {
    fn default() -> &'a EvalState {
        <EvalState as ::protobuf::Message>::default_instance()
    }
}

impl EvalState {
    pub fn new() -> EvalState {
        ::std::default::Default::default()
    }

    // repeated .google.api.expr.v1alpha1.ExprValue values = 1;


    pub fn get_values(&self) -> &[ExprValue] {
        &self.values
    }
    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<ExprValue>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<ExprValue> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<ExprValue> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    // repeated .google.api.expr.v1alpha1.EvalState.Result results = 3;


    pub fn get_results(&self) -> &[EvalState_Result] {
        &self.results
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    // Param is passed by value, moved
    pub fn set_results(&mut self, v: ::protobuf::RepeatedField<EvalState_Result>) {
        self.results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_results(&mut self) -> &mut ::protobuf::RepeatedField<EvalState_Result> {
        &mut self.results
    }

    // Take field
    pub fn take_results(&mut self) -> ::protobuf::RepeatedField<EvalState_Result> {
        ::std::mem::replace(&mut self.results, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for EvalState {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.results {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.results)?;
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
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.results {
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

    fn new() -> EvalState {
        EvalState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExprValue>>(
                    "values",
                    |m: &EvalState| { &m.values },
                    |m: &mut EvalState| { &mut m.values },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EvalState_Result>>(
                    "results",
                    |m: &EvalState| { &m.results },
                    |m: &mut EvalState| { &mut m.results },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EvalState>(
                    "EvalState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EvalState {
        static mut instance: ::protobuf::lazy::Lazy<EvalState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvalState,
        };
        unsafe {
            instance.get(EvalState::new)
        }
    }
}

impl ::protobuf::Clear for EvalState {
    fn clear(&mut self) {
        self.values.clear();
        self.results.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EvalState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvalState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EvalState_Result {
    // message fields
    pub expr: i64,
    pub value: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EvalState_Result {
    fn default() -> &'a EvalState_Result {
        <EvalState_Result as ::protobuf::Message>::default_instance()
    }
}

impl EvalState_Result {
    pub fn new() -> EvalState_Result {
        ::std::default::Default::default()
    }

    // int64 expr = 1;


    pub fn get_expr(&self) -> i64 {
        self.expr
    }
    pub fn clear_expr(&mut self) {
        self.expr = 0;
    }

    // Param is passed by value, moved
    pub fn set_expr(&mut self, v: i64) {
        self.expr = v;
    }

    // int64 value = 2;


    pub fn get_value(&self) -> i64 {
        self.value
    }
    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }
}

impl ::protobuf::Message for EvalState_Result {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expr = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.value = tmp;
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
        if self.expr != 0 {
            my_size += ::protobuf::rt::value_size(1, self.expr, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.expr != 0 {
            os.write_int64(1, self.expr)?;
        }
        if self.value != 0 {
            os.write_int64(2, self.value)?;
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

    fn new() -> EvalState_Result {
        EvalState_Result::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expr",
                    |m: &EvalState_Result| { &m.expr },
                    |m: &mut EvalState_Result| { &mut m.expr },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    |m: &EvalState_Result| { &m.value },
                    |m: &mut EvalState_Result| { &mut m.value },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EvalState_Result>(
                    "EvalState_Result",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EvalState_Result {
        static mut instance: ::protobuf::lazy::Lazy<EvalState_Result> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvalState_Result,
        };
        unsafe {
            instance.get(EvalState_Result::new)
        }
    }
}

impl ::protobuf::Clear for EvalState_Result {
    fn clear(&mut self) {
        self.expr = 0;
        self.value = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EvalState_Result {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvalState_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExprValue {
    // message oneof groups
    pub kind: ::std::option::Option<ExprValue_oneof_kind>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExprValue {
    fn default() -> &'a ExprValue {
        <ExprValue as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum ExprValue_oneof_kind {
    value(super::value::Value),
    error(ErrorSet),
    unknown(UnknownSet),
}

impl ExprValue {
    pub fn new() -> ExprValue {
        ::std::default::Default::default()
    }

    // .google.api.expr.v1alpha1.Value value = 1;


    pub fn get_value(&self) -> &super::value::Value {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::value(ref v)) => v,
            _ => super::value::Value::default_instance(),
        }
    }
    pub fn clear_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::value::Value) {
        self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut super::value::Value {
        if let ::std::option::Option::Some(ExprValue_oneof_kind::value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::value(super::value::Value::new()));
        }
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_value(&mut self) -> super::value::Value {
        if self.has_value() {
            match self.kind.take() {
                ::std::option::Option::Some(ExprValue_oneof_kind::value(v)) => v,
                _ => panic!(),
            }
        } else {
            super::value::Value::new()
        }
    }

    // .google.api.expr.v1alpha1.ErrorSet error = 2;


    pub fn get_error(&self) -> &ErrorSet {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::error(ref v)) => v,
            _ => ErrorSet::default_instance(),
        }
    }
    pub fn clear_error(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ErrorSet) {
        self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ErrorSet {
        if let ::std::option::Option::Some(ExprValue_oneof_kind::error(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::error(ErrorSet::new()));
        }
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ErrorSet {
        if self.has_error() {
            match self.kind.take() {
                ::std::option::Option::Some(ExprValue_oneof_kind::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ErrorSet::new()
        }
    }

    // .google.api.expr.v1alpha1.UnknownSet unknown = 3;


    pub fn get_unknown(&self) -> &UnknownSet {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::unknown(ref v)) => v,
            _ => UnknownSet::default_instance(),
        }
    }
    pub fn clear_unknown(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_unknown(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::unknown(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unknown(&mut self, v: UnknownSet) {
        self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::unknown(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unknown(&mut self) -> &mut UnknownSet {
        if let ::std::option::Option::Some(ExprValue_oneof_kind::unknown(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::unknown(UnknownSet::new()));
        }
        match self.kind {
            ::std::option::Option::Some(ExprValue_oneof_kind::unknown(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unknown(&mut self) -> UnknownSet {
        if self.has_unknown() {
            match self.kind.take() {
                ::std::option::Option::Some(ExprValue_oneof_kind::unknown(v)) => v,
                _ => panic!(),
            }
        } else {
            UnknownSet::new()
        }
    }
}

impl ::protobuf::Message for ExprValue {
    fn is_initialized(&self) -> bool {
        if let Some(ExprValue_oneof_kind::value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ExprValue_oneof_kind::error(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ExprValue_oneof_kind::unknown(ref v)) = self.kind {
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
                    self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::value(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::error(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(ExprValue_oneof_kind::unknown(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &ExprValue_oneof_kind::value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ExprValue_oneof_kind::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ExprValue_oneof_kind::unknown(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &ExprValue_oneof_kind::value(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ExprValue_oneof_kind::error(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ExprValue_oneof_kind::unknown(ref v) => {
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

    fn new() -> ExprValue {
        ExprValue::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::value::Value>(
                    "value",
                    ExprValue::has_value,
                    ExprValue::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ErrorSet>(
                    "error",
                    ExprValue::has_error,
                    ExprValue::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, UnknownSet>(
                    "unknown",
                    ExprValue::has_unknown,
                    ExprValue::get_unknown,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExprValue>(
                    "ExprValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ExprValue {
        static mut instance: ::protobuf::lazy::Lazy<ExprValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExprValue,
        };
        unsafe {
            instance.get(ExprValue::new)
        }
    }
}

impl ::protobuf::Clear for ExprValue {
    fn clear(&mut self) {
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExprValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExprValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErrorSet {
    // message fields
    pub errors: ::protobuf::RepeatedField<super::status::Status>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ErrorSet {
    fn default() -> &'a ErrorSet {
        <ErrorSet as ::protobuf::Message>::default_instance()
    }
}

impl ErrorSet {
    pub fn new() -> ErrorSet {
        ::std::default::Default::default()
    }

    // repeated .google.rpc.Status errors = 1;


    pub fn get_errors(&self) -> &[super::status::Status] {
        &self.errors
    }
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<super::status::Status>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<super::status::Status> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<super::status::Status> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ErrorSet {
    fn is_initialized(&self) -> bool {
        for v in &self.errors {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors)?;
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
        for value in &self.errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.errors {
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

    fn new() -> ErrorSet {
        ErrorSet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::status::Status>>(
                    "errors",
                    |m: &ErrorSet| { &m.errors },
                    |m: &mut ErrorSet| { &mut m.errors },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorSet>(
                    "ErrorSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ErrorSet {
        static mut instance: ::protobuf::lazy::Lazy<ErrorSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorSet,
        };
        unsafe {
            instance.get(ErrorSet::new)
        }
    }
}

impl ::protobuf::Clear for ErrorSet {
    fn clear(&mut self) {
        self.errors.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErrorSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnknownSet {
    // message fields
    pub exprs: ::std::vec::Vec<i64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnknownSet {
    fn default() -> &'a UnknownSet {
        <UnknownSet as ::protobuf::Message>::default_instance()
    }
}

impl UnknownSet {
    pub fn new() -> UnknownSet {
        ::std::default::Default::default()
    }

    // repeated int64 exprs = 1;


    pub fn get_exprs(&self) -> &[i64] {
        &self.exprs
    }
    pub fn clear_exprs(&mut self) {
        self.exprs.clear();
    }

    // Param is passed by value, moved
    pub fn set_exprs(&mut self, v: ::std::vec::Vec<i64>) {
        self.exprs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_exprs(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.exprs
    }

    // Take field
    pub fn take_exprs(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.exprs, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for UnknownSet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.exprs)?;
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
        for value in &self.exprs {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.exprs {
            os.write_int64(1, *v)?;
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

    fn new() -> UnknownSet {
        UnknownSet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "exprs",
                    |m: &UnknownSet| { &m.exprs },
                    |m: &mut UnknownSet| { &mut m.exprs },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnknownSet>(
                    "UnknownSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static UnknownSet {
        static mut instance: ::protobuf::lazy::Lazy<UnknownSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnknownSet,
        };
        unsafe {
            instance.get(UnknownSet::new)
        }
    }
}

impl ::protobuf::Clear for UnknownSet {
    fn clear(&mut self) {
        self.exprs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnknownSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnknownSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#google/api/expr/v1alpha1/eval.proto\x12\x18google.api.expr.v1alpha1\
    \x1a$google/api/expr/v1alpha1/value.proto\x1a\x17google/rpc/status.proto\
    \"\xc2\x01\n\tEvalState\x12;\n\x06values\x18\x01\x20\x03(\x0b2#.google.a\
    pi.expr.v1alpha1.ExprValueR\x06values\x12D\n\x07results\x18\x03\x20\x03(\
    \x0b2*.google.api.expr.v1alpha1.EvalState.ResultR\x07results\x1a2\n\x06R\
    esult\x12\x12\n\x04expr\x18\x01\x20\x01(\x03R\x04expr\x12\x14\n\x05value\
    \x18\x02\x20\x01(\x03R\x05value\"\xca\x01\n\tExprValue\x127\n\x05value\
    \x18\x01\x20\x01(\x0b2\x1f.google.api.expr.v1alpha1.ValueH\0R\x05value\
    \x12:\n\x05error\x18\x02\x20\x01(\x0b2\".google.api.expr.v1alpha1.ErrorS\
    etH\0R\x05error\x12@\n\x07unknown\x18\x03\x20\x01(\x0b2$.google.api.expr\
    .v1alpha1.UnknownSetH\0R\x07unknownB\x06\n\x04kind\"6\n\x08ErrorSet\x12*\
    \n\x06errors\x18\x01\x20\x03(\x0b2\x12.google.rpc.StatusR\x06errors\"\"\
    \n\nUnknownSet\x12\x14\n\x05exprs\x18\x01\x20\x03(\x03R\x05exprsBl\n\x1c\
    com.google.api.expr.v1alpha1B\tEvalProtoP\x01Z<google.golang.org/genprot\
    o/googleapis/api/expr/v1alpha1;expr\xf8\x01\x01b\x06proto3\
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