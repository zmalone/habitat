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
pub struct ApplicationEnvironment {
    // message fields
    application: ::protobuf::SingularField<::std::string::String>,
    environment: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApplicationEnvironment {}

impl ApplicationEnvironment {
    pub fn new() -> ApplicationEnvironment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApplicationEnvironment {
        static mut instance: ::protobuf::lazy::Lazy<ApplicationEnvironment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApplicationEnvironment,
        };
        unsafe {
            instance.get(ApplicationEnvironment::new)
        }
    }

    // required string application = 1;

    pub fn clear_application(&mut self) {
        self.application.clear();
    }

    pub fn has_application(&self) -> bool {
        self.application.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application(&mut self, v: ::std::string::String) {
        self.application = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application(&mut self) -> &mut ::std::string::String {
        if self.application.is_none() {
            self.application.set_default();
        }
        self.application.as_mut().unwrap()
    }

    // Take field
    pub fn take_application(&mut self) -> ::std::string::String {
        self.application.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_application(&self) -> &str {
        match self.application.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_application_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.application
    }

    fn mut_application_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.application
    }

    // optional string environment = 2;

    pub fn clear_environment(&mut self) {
        self.environment.clear();
    }

    pub fn has_environment(&self) -> bool {
        self.environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_environment(&mut self, v: ::std::string::String) {
        self.environment = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_environment(&mut self) -> &mut ::std::string::String {
        if self.environment.is_none() {
            self.environment.set_default();
        }
        self.environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_environment(&mut self) -> ::std::string::String {
        self.environment.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_environment(&self) -> &str {
        match self.environment.as_ref() {
            Some(v) => &v,
            None => "default",
        }
    }

    fn get_environment_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.environment
    }

    fn mut_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.environment
    }
}

impl ::protobuf::Message for ApplicationEnvironment {
    fn is_initialized(&self) -> bool {
        if self.application.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.application)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.environment)?;
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
        if let Some(ref v) = self.application.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.environment.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.application.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.environment.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for ApplicationEnvironment {
    fn new() -> ApplicationEnvironment {
        ApplicationEnvironment::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApplicationEnvironment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "application",
                    ApplicationEnvironment::get_application_for_reflect,
                    ApplicationEnvironment::mut_application_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "environment",
                    ApplicationEnvironment::get_environment_for_reflect,
                    ApplicationEnvironment::mut_environment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApplicationEnvironment>(
                    "ApplicationEnvironment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApplicationEnvironment {
    fn clear(&mut self) {
        self.clear_application();
        self.clear_environment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApplicationEnvironment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApplicationEnvironment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageIdent {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    release: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageIdent {}

impl PackageIdent {
    pub fn new() -> PackageIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageIdent {
        static mut instance: ::protobuf::lazy::Lazy<PackageIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageIdent,
        };
        unsafe {
            instance.get(PackageIdent::new)
        }
    }

    // optional string origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut ::std::string::String {
        if self.origin.is_none() {
            self.origin.set_default();
        }
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin(&self) -> &str {
        match self.origin.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin
    }

    // optional string name = 2;

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

    // optional string version = 3;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }

    // optional string release = 4;

    pub fn clear_release(&mut self) {
        self.release.clear();
    }

    pub fn has_release(&self) -> bool {
        self.release.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release(&mut self, v: ::std::string::String) {
        self.release = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_release(&mut self) -> &mut ::std::string::String {
        if self.release.is_none() {
            self.release.set_default();
        }
        self.release.as_mut().unwrap()
    }

    // Take field
    pub fn take_release(&mut self) -> ::std::string::String {
        self.release.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_release(&self) -> &str {
        match self.release.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_release_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.release
    }

    fn mut_release_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.release
    }
}

impl ::protobuf::Message for PackageIdent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.release)?;
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
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.release.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.release.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for PackageIdent {
    fn new() -> PackageIdent {
        PackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    PackageIdent::get_origin_for_reflect,
                    PackageIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PackageIdent::get_name_for_reflect,
                    PackageIdent::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PackageIdent::get_version_for_reflect,
                    PackageIdent::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "release",
                    PackageIdent::get_release_for_reflect,
                    PackageIdent::mut_release_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageIdent>(
                    "PackageIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProcessStatus {
    // message fields
    elapsed: ::std::option::Option<i64>,
    pid: ::std::option::Option<u32>,
    state: ::std::option::Option<ProcessState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProcessStatus {}

impl ProcessStatus {
    pub fn new() -> ProcessStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProcessStatus {
        static mut instance: ::protobuf::lazy::Lazy<ProcessStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProcessStatus,
        };
        unsafe {
            instance.get(ProcessStatus::new)
        }
    }

    // optional int64 elapsed = 1;

    pub fn clear_elapsed(&mut self) {
        self.elapsed = ::std::option::Option::None;
    }

    pub fn has_elapsed(&self) -> bool {
        self.elapsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_elapsed(&mut self, v: i64) {
        self.elapsed = ::std::option::Option::Some(v);
    }

    pub fn get_elapsed(&self) -> i64 {
        self.elapsed.unwrap_or(0)
    }

    fn get_elapsed_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.elapsed
    }

    fn mut_elapsed_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.elapsed
    }

    // optional uint32 pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: u32) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid(&self) -> u32 {
        self.pid.unwrap_or(0)
    }

    fn get_pid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pid
    }

    fn mut_pid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pid
    }

    // optional .ProcessState state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ProcessState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> ProcessState {
        self.state.unwrap_or(ProcessState::Down)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<ProcessState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<ProcessState> {
        &mut self.state
    }
}

impl ::protobuf::Message for ProcessStatus {
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
                    self.elapsed = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.pid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 3, &mut self.unknown_fields)?
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
        if let Some(v) = self.elapsed {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.pid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.elapsed {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.pid {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for ProcessStatus {
    fn new() -> ProcessStatus {
        ProcessStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProcessStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "elapsed",
                    ProcessStatus::get_elapsed_for_reflect,
                    ProcessStatus::mut_elapsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pid",
                    ProcessStatus::get_pid_for_reflect,
                    ProcessStatus::mut_pid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessState>>(
                    "state",
                    ProcessStatus::get_state_for_reflect,
                    ProcessStatus::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProcessStatus>(
                    "ProcessStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProcessStatus {
    fn clear(&mut self) {
        self.clear_elapsed();
        self.clear_pid();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProcessStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProcessStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceBind {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    service_group: ::protobuf::SingularPtrField<ServiceGroup>,
    service_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceBind {}

impl ServiceBind {
    pub fn new() -> ServiceBind {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceBind {
        static mut instance: ::protobuf::lazy::Lazy<ServiceBind> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceBind,
        };
        unsafe {
            instance.get(ServiceBind::new)
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

    // required .ServiceGroup service_group = 2;

    pub fn clear_service_group(&mut self) {
        self.service_group.clear();
    }

    pub fn has_service_group(&self) -> bool {
        self.service_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_group(&mut self, v: ServiceGroup) {
        self.service_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_group(&mut self) -> &mut ServiceGroup {
        if self.service_group.is_none() {
            self.service_group.set_default();
        }
        self.service_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_group(&mut self) -> ServiceGroup {
        self.service_group.take().unwrap_or_else(|| ServiceGroup::new())
    }

    pub fn get_service_group(&self) -> &ServiceGroup {
        self.service_group.as_ref().unwrap_or_else(|| ServiceGroup::default_instance())
    }

    fn get_service_group_for_reflect(&self) -> &::protobuf::SingularPtrField<ServiceGroup> {
        &self.service_group
    }

    fn mut_service_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServiceGroup> {
        &mut self.service_group
    }

    // optional string service_name = 3;

    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    pub fn has_service_name(&self) -> bool {
        self.service_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        if self.service_name.is_none() {
            self.service_name.set_default();
        }
        self.service_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        self.service_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service_name(&self) -> &str {
        match self.service_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service_name
    }

    fn mut_service_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service_name
    }
}

impl ::protobuf::Message for ServiceBind {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.service_group.is_none() {
            return false;
        }
        for v in &self.service_group {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.service_group)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service_name)?;
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
        if let Some(ref v) = self.service_group.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.service_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.service_group.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.service_name.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for ServiceBind {
    fn new() -> ServiceBind {
        ServiceBind::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceBind>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServiceBind::get_name_for_reflect,
                    ServiceBind::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceGroup>>(
                    "service_group",
                    ServiceBind::get_service_group_for_reflect,
                    ServiceBind::mut_service_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_name",
                    ServiceBind::get_service_name_for_reflect,
                    ServiceBind::mut_service_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceBind>(
                    "ServiceBind",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceBind {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_service_group();
        self.clear_service_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceBind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceBind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceCfg {
    // message fields
    format: ::std::option::Option<ServiceCfg_Format>,
    default: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceCfg {}

impl ServiceCfg {
    pub fn new() -> ServiceCfg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceCfg {
        static mut instance: ::protobuf::lazy::Lazy<ServiceCfg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceCfg,
        };
        unsafe {
            instance.get(ServiceCfg::new)
        }
    }

    // optional .ServiceCfg.Format format = 1;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: ServiceCfg_Format) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> ServiceCfg_Format {
        self.format.unwrap_or(ServiceCfg_Format::TOML)
    }

    fn get_format_for_reflect(&self) -> &::std::option::Option<ServiceCfg_Format> {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::option::Option<ServiceCfg_Format> {
        &mut self.format
    }

    // optional string default = 2;

    pub fn clear_default(&mut self) {
        self.default.clear();
    }

    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default(&mut self, v: ::std::string::String) {
        self.default = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default(&mut self) -> &mut ::std::string::String {
        if self.default.is_none() {
            self.default.set_default();
        }
        self.default.as_mut().unwrap()
    }

    // Take field
    pub fn take_default(&mut self) -> ::std::string::String {
        self.default.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_default(&self) -> &str {
        match self.default.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_default_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.default
    }

    fn mut_default_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.default
    }
}

impl ::protobuf::Message for ServiceCfg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.format, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.default)?;
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
        if let Some(v) = self.format {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.default.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.format {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.default.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for ServiceCfg {
    fn new() -> ServiceCfg {
        ServiceCfg::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceCfg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServiceCfg_Format>>(
                    "format",
                    ServiceCfg::get_format_for_reflect,
                    ServiceCfg::mut_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "default",
                    ServiceCfg::get_default_for_reflect,
                    ServiceCfg::mut_default_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceCfg>(
                    "ServiceCfg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceCfg {
    fn clear(&mut self) {
        self.clear_format();
        self.clear_default();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceCfg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceCfg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServiceCfg_Format {
    TOML = 0,
}

impl ::protobuf::ProtobufEnum for ServiceCfg_Format {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServiceCfg_Format> {
        match value {
            0 => ::std::option::Option::Some(ServiceCfg_Format::TOML),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServiceCfg_Format] = &[
            ServiceCfg_Format::TOML,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ServiceCfg_Format>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ServiceCfg_Format", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ServiceCfg_Format {
}

impl ::protobuf::reflect::ProtobufValue for ServiceCfg_Format {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceGroup {
    // message fields
    service: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    application_environment: ::protobuf::SingularPtrField<ApplicationEnvironment>,
    organization: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceGroup {}

impl ServiceGroup {
    pub fn new() -> ServiceGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceGroup {
        static mut instance: ::protobuf::lazy::Lazy<ServiceGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceGroup,
        };
        unsafe {
            instance.get(ServiceGroup::new)
        }
    }

    // required string service = 1;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }

    // optional string group = 2;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "default",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional .ApplicationEnvironment application_environment = 3;

    pub fn clear_application_environment(&mut self) {
        self.application_environment.clear();
    }

    pub fn has_application_environment(&self) -> bool {
        self.application_environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_environment(&mut self, v: ApplicationEnvironment) {
        self.application_environment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_environment(&mut self) -> &mut ApplicationEnvironment {
        if self.application_environment.is_none() {
            self.application_environment.set_default();
        }
        self.application_environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_environment(&mut self) -> ApplicationEnvironment {
        self.application_environment.take().unwrap_or_else(|| ApplicationEnvironment::new())
    }

    pub fn get_application_environment(&self) -> &ApplicationEnvironment {
        self.application_environment.as_ref().unwrap_or_else(|| ApplicationEnvironment::default_instance())
    }

    fn get_application_environment_for_reflect(&self) -> &::protobuf::SingularPtrField<ApplicationEnvironment> {
        &self.application_environment
    }

    fn mut_application_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApplicationEnvironment> {
        &mut self.application_environment
    }

    // optional string organization = 4;

    pub fn clear_organization(&mut self) {
        self.organization.clear();
    }

    pub fn has_organization(&self) -> bool {
        self.organization.is_some()
    }

    // Param is passed by value, moved
    pub fn set_organization(&mut self, v: ::std::string::String) {
        self.organization = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_organization(&mut self) -> &mut ::std::string::String {
        if self.organization.is_none() {
            self.organization.set_default();
        }
        self.organization.as_mut().unwrap()
    }

    // Take field
    pub fn take_organization(&mut self) -> ::std::string::String {
        self.organization.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_organization(&self) -> &str {
        match self.organization.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_organization_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.organization
    }

    fn mut_organization_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.organization
    }
}

impl ::protobuf::Message for ServiceGroup {
    fn is_initialized(&self) -> bool {
        if self.service.is_none() {
            return false;
        }
        for v in &self.application_environment {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.application_environment)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.organization)?;
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
        if let Some(ref v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.organization.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.service.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.organization.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for ServiceGroup {
    fn new() -> ServiceGroup {
        ServiceGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    ServiceGroup::get_service_for_reflect,
                    ServiceGroup::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ServiceGroup::get_group_for_reflect,
                    ServiceGroup::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApplicationEnvironment>>(
                    "application_environment",
                    ServiceGroup::get_application_environment_for_reflect,
                    ServiceGroup::mut_application_environment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "organization",
                    ServiceGroup::get_organization_for_reflect,
                    ServiceGroup::mut_organization_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceGroup>(
                    "ServiceGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceGroup {
    fn clear(&mut self) {
        self.clear_service();
        self.clear_group();
        self.clear_application_environment();
        self.clear_organization();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceSpec {
    // message fields
    ident: ::protobuf::SingularPtrField<PackageIdent>,
    group: ::protobuf::SingularField<::std::string::String>,
    application_environment: ::protobuf::SingularPtrField<ApplicationEnvironment>,
    bldr_url: ::protobuf::SingularField<::std::string::String>,
    channel: ::protobuf::SingularField<::std::string::String>,
    topology: ::std::option::Option<Topology>,
    update_strategy: ::std::option::Option<UpdateStrategy>,
    binds: ::protobuf::RepeatedField<ServiceBind>,
    binding_mode: ::std::option::Option<BindingMode>,
    config_from: ::protobuf::SingularField<::std::string::String>,
    desired_state: ::std::option::Option<ProcessState>,
    svc_encrypted_password: ::protobuf::SingularField<::std::string::String>,
    composite: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceSpec {}

impl ServiceSpec {
    pub fn new() -> ServiceSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceSpec {
        static mut instance: ::protobuf::lazy::Lazy<ServiceSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceSpec,
        };
        unsafe {
            instance.get(ServiceSpec::new)
        }
    }

    // required .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> PackageIdent {
        self.ident.take().unwrap_or_else(|| PackageIdent::new())
    }

    pub fn get_ident(&self) -> &PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PackageIdent> {
        &mut self.ident
    }

    // optional string group = 2;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "default",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional .ApplicationEnvironment application_environment = 3;

    pub fn clear_application_environment(&mut self) {
        self.application_environment.clear();
    }

    pub fn has_application_environment(&self) -> bool {
        self.application_environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_environment(&mut self, v: ApplicationEnvironment) {
        self.application_environment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_environment(&mut self) -> &mut ApplicationEnvironment {
        if self.application_environment.is_none() {
            self.application_environment.set_default();
        }
        self.application_environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_environment(&mut self) -> ApplicationEnvironment {
        self.application_environment.take().unwrap_or_else(|| ApplicationEnvironment::new())
    }

    pub fn get_application_environment(&self) -> &ApplicationEnvironment {
        self.application_environment.as_ref().unwrap_or_else(|| ApplicationEnvironment::default_instance())
    }

    fn get_application_environment_for_reflect(&self) -> &::protobuf::SingularPtrField<ApplicationEnvironment> {
        &self.application_environment
    }

    fn mut_application_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApplicationEnvironment> {
        &mut self.application_environment
    }

    // optional string bldr_url = 4;

    pub fn clear_bldr_url(&mut self) {
        self.bldr_url.clear();
    }

    pub fn has_bldr_url(&self) -> bool {
        self.bldr_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bldr_url(&mut self, v: ::std::string::String) {
        self.bldr_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bldr_url(&mut self) -> &mut ::std::string::String {
        if self.bldr_url.is_none() {
            self.bldr_url.set_default();
        }
        self.bldr_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_bldr_url(&mut self) -> ::std::string::String {
        self.bldr_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_bldr_url(&self) -> &str {
        match self.bldr_url.as_ref() {
            Some(v) => &v,
            None => "https://bldr.habitat.sh",
        }
    }

    fn get_bldr_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.bldr_url
    }

    fn mut_bldr_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.bldr_url
    }

    // optional string channel = 5;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ::std::string::String) {
        self.channel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut ::std::string::String {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> ::std::string::String {
        self.channel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel(&self) -> &str {
        match self.channel.as_ref() {
            Some(v) => &v,
            None => "stable",
        }
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel
    }

    // optional .Topology topology = 6;

    pub fn clear_topology(&mut self) {
        self.topology = ::std::option::Option::None;
    }

    pub fn has_topology(&self) -> bool {
        self.topology.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topology(&mut self, v: Topology) {
        self.topology = ::std::option::Option::Some(v);
    }

    pub fn get_topology(&self) -> Topology {
        self.topology.unwrap_or(Topology::Standalone)
    }

    fn get_topology_for_reflect(&self) -> &::std::option::Option<Topology> {
        &self.topology
    }

    fn mut_topology_for_reflect(&mut self) -> &mut ::std::option::Option<Topology> {
        &mut self.topology
    }

    // optional .UpdateStrategy update_strategy = 7;

    pub fn clear_update_strategy(&mut self) {
        self.update_strategy = ::std::option::Option::None;
    }

    pub fn has_update_strategy(&self) -> bool {
        self.update_strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_strategy(&mut self, v: UpdateStrategy) {
        self.update_strategy = ::std::option::Option::Some(v);
    }

    pub fn get_update_strategy(&self) -> UpdateStrategy {
        self.update_strategy.unwrap_or(UpdateStrategy::None)
    }

    fn get_update_strategy_for_reflect(&self) -> &::std::option::Option<UpdateStrategy> {
        &self.update_strategy
    }

    fn mut_update_strategy_for_reflect(&mut self) -> &mut ::std::option::Option<UpdateStrategy> {
        &mut self.update_strategy
    }

    // repeated .ServiceBind binds = 8;

    pub fn clear_binds(&mut self) {
        self.binds.clear();
    }

    // Param is passed by value, moved
    pub fn set_binds(&mut self, v: ::protobuf::RepeatedField<ServiceBind>) {
        self.binds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_binds(&mut self) -> &mut ::protobuf::RepeatedField<ServiceBind> {
        &mut self.binds
    }

    // Take field
    pub fn take_binds(&mut self) -> ::protobuf::RepeatedField<ServiceBind> {
        ::std::mem::replace(&mut self.binds, ::protobuf::RepeatedField::new())
    }

    pub fn get_binds(&self) -> &[ServiceBind] {
        &self.binds
    }

    fn get_binds_for_reflect(&self) -> &::protobuf::RepeatedField<ServiceBind> {
        &self.binds
    }

    fn mut_binds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ServiceBind> {
        &mut self.binds
    }

    // optional .BindingMode binding_mode = 9;

    pub fn clear_binding_mode(&mut self) {
        self.binding_mode = ::std::option::Option::None;
    }

    pub fn has_binding_mode(&self) -> bool {
        self.binding_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_binding_mode(&mut self, v: BindingMode) {
        self.binding_mode = ::std::option::Option::Some(v);
    }

    pub fn get_binding_mode(&self) -> BindingMode {
        self.binding_mode.unwrap_or(BindingMode::Relaxed)
    }

    fn get_binding_mode_for_reflect(&self) -> &::std::option::Option<BindingMode> {
        &self.binding_mode
    }

    fn mut_binding_mode_for_reflect(&mut self) -> &mut ::std::option::Option<BindingMode> {
        &mut self.binding_mode
    }

    // optional string config_from = 10;

    pub fn clear_config_from(&mut self) {
        self.config_from.clear();
    }

    pub fn has_config_from(&self) -> bool {
        self.config_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_from(&mut self, v: ::std::string::String) {
        self.config_from = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_from(&mut self) -> &mut ::std::string::String {
        if self.config_from.is_none() {
            self.config_from.set_default();
        }
        self.config_from.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_from(&mut self) -> ::std::string::String {
        self.config_from.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_config_from(&self) -> &str {
        match self.config_from.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_config_from_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.config_from
    }

    fn mut_config_from_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.config_from
    }

    // optional .ProcessState desired_state = 11;

    pub fn clear_desired_state(&mut self) {
        self.desired_state = ::std::option::Option::None;
    }

    pub fn has_desired_state(&self) -> bool {
        self.desired_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desired_state(&mut self, v: ProcessState) {
        self.desired_state = ::std::option::Option::Some(v);
    }

    pub fn get_desired_state(&self) -> ProcessState {
        self.desired_state.unwrap_or(ProcessState::Up)
    }

    fn get_desired_state_for_reflect(&self) -> &::std::option::Option<ProcessState> {
        &self.desired_state
    }

    fn mut_desired_state_for_reflect(&mut self) -> &mut ::std::option::Option<ProcessState> {
        &mut self.desired_state
    }

    // optional string svc_encrypted_password = 12;

    pub fn clear_svc_encrypted_password(&mut self) {
        self.svc_encrypted_password.clear();
    }

    pub fn has_svc_encrypted_password(&self) -> bool {
        self.svc_encrypted_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_svc_encrypted_password(&mut self, v: ::std::string::String) {
        self.svc_encrypted_password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_svc_encrypted_password(&mut self) -> &mut ::std::string::String {
        if self.svc_encrypted_password.is_none() {
            self.svc_encrypted_password.set_default();
        }
        self.svc_encrypted_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_svc_encrypted_password(&mut self) -> ::std::string::String {
        self.svc_encrypted_password.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_svc_encrypted_password(&self) -> &str {
        match self.svc_encrypted_password.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_svc_encrypted_password_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.svc_encrypted_password
    }

    fn mut_svc_encrypted_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.svc_encrypted_password
    }

    // optional string composite = 13;

    pub fn clear_composite(&mut self) {
        self.composite.clear();
    }

    pub fn has_composite(&self) -> bool {
        self.composite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_composite(&mut self, v: ::std::string::String) {
        self.composite = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_composite(&mut self) -> &mut ::std::string::String {
        if self.composite.is_none() {
            self.composite.set_default();
        }
        self.composite.as_mut().unwrap()
    }

    // Take field
    pub fn take_composite(&mut self) -> ::std::string::String {
        self.composite.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_composite(&self) -> &str {
        match self.composite.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_composite_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.composite
    }

    fn mut_composite_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.composite
    }
}

impl ::protobuf::Message for ServiceSpec {
    fn is_initialized(&self) -> bool {
        if self.ident.is_none() {
            return false;
        }
        for v in &self.ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.application_environment {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.binds {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.application_environment)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.bldr_url)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel)?;
                },
                6 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.topology, 6, &mut self.unknown_fields)?
                },
                7 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.update_strategy, 7, &mut self.unknown_fields)?
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.binds)?;
                },
                9 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.binding_mode, 9, &mut self.unknown_fields)?
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config_from)?;
                },
                11 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.desired_state, 11, &mut self.unknown_fields)?
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.svc_encrypted_password)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.composite)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.channel.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.topology {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(v) = self.update_strategy {
            my_size += ::protobuf::rt::enum_size(7, v);
        }
        for value in &self.binds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.binding_mode {
            my_size += ::protobuf::rt::enum_size(9, v);
        }
        if let Some(ref v) = self.config_from.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(v) = self.desired_state {
            my_size += ::protobuf::rt::enum_size(11, v);
        }
        if let Some(ref v) = self.svc_encrypted_password.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(ref v) = self.composite.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.channel.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.topology {
            os.write_enum(6, v.value())?;
        }
        if let Some(v) = self.update_strategy {
            os.write_enum(7, v.value())?;
        }
        for v in &self.binds {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.binding_mode {
            os.write_enum(9, v.value())?;
        }
        if let Some(ref v) = self.config_from.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(v) = self.desired_state {
            os.write_enum(11, v.value())?;
        }
        if let Some(ref v) = self.svc_encrypted_password.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(ref v) = self.composite.as_ref() {
            os.write_string(13, &v)?;
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

impl ::protobuf::MessageStatic for ServiceSpec {
    fn new() -> ServiceSpec {
        ServiceSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "ident",
                    ServiceSpec::get_ident_for_reflect,
                    ServiceSpec::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ServiceSpec::get_group_for_reflect,
                    ServiceSpec::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApplicationEnvironment>>(
                    "application_environment",
                    ServiceSpec::get_application_environment_for_reflect,
                    ServiceSpec::mut_application_environment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "bldr_url",
                    ServiceSpec::get_bldr_url_for_reflect,
                    ServiceSpec::mut_bldr_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel",
                    ServiceSpec::get_channel_for_reflect,
                    ServiceSpec::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Topology>>(
                    "topology",
                    ServiceSpec::get_topology_for_reflect,
                    ServiceSpec::mut_topology_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<UpdateStrategy>>(
                    "update_strategy",
                    ServiceSpec::get_update_strategy_for_reflect,
                    ServiceSpec::mut_update_strategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceBind>>(
                    "binds",
                    ServiceSpec::get_binds_for_reflect,
                    ServiceSpec::mut_binds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<BindingMode>>(
                    "binding_mode",
                    ServiceSpec::get_binding_mode_for_reflect,
                    ServiceSpec::mut_binding_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config_from",
                    ServiceSpec::get_config_from_for_reflect,
                    ServiceSpec::mut_config_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessState>>(
                    "desired_state",
                    ServiceSpec::get_desired_state_for_reflect,
                    ServiceSpec::mut_desired_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "svc_encrypted_password",
                    ServiceSpec::get_svc_encrypted_password_for_reflect,
                    ServiceSpec::mut_svc_encrypted_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "composite",
                    ServiceSpec::get_composite_for_reflect,
                    ServiceSpec::mut_composite_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceSpec>(
                    "ServiceSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceSpec {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_group();
        self.clear_application_environment();
        self.clear_bldr_url();
        self.clear_channel();
        self.clear_topology();
        self.clear_update_strategy();
        self.clear_binds();
        self.clear_binding_mode();
        self.clear_config_from();
        self.clear_desired_state();
        self.clear_svc_encrypted_password();
        self.clear_composite();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceStatus {
    // message fields
    ident: ::protobuf::SingularPtrField<PackageIdent>,
    process: ::protobuf::SingularPtrField<ProcessStatus>,
    service_group: ::protobuf::SingularPtrField<ServiceGroup>,
    composite: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceStatus {}

impl ServiceStatus {
    pub fn new() -> ServiceStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceStatus {
        static mut instance: ::protobuf::lazy::Lazy<ServiceStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceStatus,
        };
        unsafe {
            instance.get(ServiceStatus::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> PackageIdent {
        self.ident.take().unwrap_or_else(|| PackageIdent::new())
    }

    pub fn get_ident(&self) -> &PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PackageIdent> {
        &mut self.ident
    }

    // optional .ProcessStatus process = 2;

    pub fn clear_process(&mut self) {
        self.process.clear();
    }

    pub fn has_process(&self) -> bool {
        self.process.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process(&mut self, v: ProcessStatus) {
        self.process = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_process(&mut self) -> &mut ProcessStatus {
        if self.process.is_none() {
            self.process.set_default();
        }
        self.process.as_mut().unwrap()
    }

    // Take field
    pub fn take_process(&mut self) -> ProcessStatus {
        self.process.take().unwrap_or_else(|| ProcessStatus::new())
    }

    pub fn get_process(&self) -> &ProcessStatus {
        self.process.as_ref().unwrap_or_else(|| ProcessStatus::default_instance())
    }

    fn get_process_for_reflect(&self) -> &::protobuf::SingularPtrField<ProcessStatus> {
        &self.process
    }

    fn mut_process_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ProcessStatus> {
        &mut self.process
    }

    // optional .ServiceGroup service_group = 3;

    pub fn clear_service_group(&mut self) {
        self.service_group.clear();
    }

    pub fn has_service_group(&self) -> bool {
        self.service_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_group(&mut self, v: ServiceGroup) {
        self.service_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_group(&mut self) -> &mut ServiceGroup {
        if self.service_group.is_none() {
            self.service_group.set_default();
        }
        self.service_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_group(&mut self) -> ServiceGroup {
        self.service_group.take().unwrap_or_else(|| ServiceGroup::new())
    }

    pub fn get_service_group(&self) -> &ServiceGroup {
        self.service_group.as_ref().unwrap_or_else(|| ServiceGroup::default_instance())
    }

    fn get_service_group_for_reflect(&self) -> &::protobuf::SingularPtrField<ServiceGroup> {
        &self.service_group
    }

    fn mut_service_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServiceGroup> {
        &mut self.service_group
    }

    // optional string composite = 4;

    pub fn clear_composite(&mut self) {
        self.composite.clear();
    }

    pub fn has_composite(&self) -> bool {
        self.composite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_composite(&mut self, v: ::std::string::String) {
        self.composite = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_composite(&mut self) -> &mut ::std::string::String {
        if self.composite.is_none() {
            self.composite.set_default();
        }
        self.composite.as_mut().unwrap()
    }

    // Take field
    pub fn take_composite(&mut self) -> ::std::string::String {
        self.composite.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_composite(&self) -> &str {
        match self.composite.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_composite_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.composite
    }

    fn mut_composite_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.composite
    }
}

impl ::protobuf::Message for ServiceStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.process {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.service_group {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.process)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.service_group)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.composite)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.process.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.service_group.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.composite.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.process.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.service_group.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.composite.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for ServiceStatus {
    fn new() -> ServiceStatus {
        ServiceStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "ident",
                    ServiceStatus::get_ident_for_reflect,
                    ServiceStatus::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProcessStatus>>(
                    "process",
                    ServiceStatus::get_process_for_reflect,
                    ServiceStatus::mut_process_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceGroup>>(
                    "service_group",
                    ServiceStatus::get_service_group_for_reflect,
                    ServiceStatus::mut_service_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "composite",
                    ServiceStatus::get_composite_for_reflect,
                    ServiceStatus::mut_composite_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceStatus>(
                    "ServiceStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceStatus {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_process();
        self.clear_service_group();
        self.clear_composite();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InstallSource {
    Ident = 0,
    Archive = 1,
}

impl ::protobuf::ProtobufEnum for InstallSource {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InstallSource> {
        match value {
            0 => ::std::option::Option::Some(InstallSource::Ident),
            1 => ::std::option::Option::Some(InstallSource::Archive),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InstallSource] = &[
            InstallSource::Ident,
            InstallSource::Archive,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<InstallSource>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InstallSource", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InstallSource {
}

impl ::protobuf::reflect::ProtobufValue for InstallSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProcessState {
    Down = 0,
    Up = 1,
}

impl ::protobuf::ProtobufEnum for ProcessState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProcessState> {
        match value {
            0 => ::std::option::Option::Some(ProcessState::Down),
            1 => ::std::option::Option::Some(ProcessState::Up),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProcessState] = &[
            ProcessState::Down,
            ProcessState::Up,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ProcessState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProcessState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProcessState {
}

impl ::protobuf::reflect::ProtobufValue for ProcessState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Topology {
    Standalone = 0,
    Leader = 1,
}

impl ::protobuf::ProtobufEnum for Topology {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Topology> {
        match value {
            0 => ::std::option::Option::Some(Topology::Standalone),
            1 => ::std::option::Option::Some(Topology::Leader),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Topology] = &[
            Topology::Standalone,
            Topology::Leader,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Topology>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Topology", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Topology {
}

impl ::protobuf::reflect::ProtobufValue for Topology {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UpdateStrategy {
    None = 0,
    AtOnce = 1,
    Rolling = 2,
}

impl ::protobuf::ProtobufEnum for UpdateStrategy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UpdateStrategy> {
        match value {
            0 => ::std::option::Option::Some(UpdateStrategy::None),
            1 => ::std::option::Option::Some(UpdateStrategy::AtOnce),
            2 => ::std::option::Option::Some(UpdateStrategy::Rolling),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UpdateStrategy] = &[
            UpdateStrategy::None,
            UpdateStrategy::AtOnce,
            UpdateStrategy::Rolling,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<UpdateStrategy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UpdateStrategy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UpdateStrategy {
}

impl ::protobuf::reflect::ProtobufValue for UpdateStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BindingMode {
    Relaxed = 0,
    Strict = 1,
}

impl ::protobuf::ProtobufEnum for BindingMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BindingMode> {
        match value {
            0 => ::std::option::Option::Some(BindingMode::Relaxed),
            1 => ::std::option::Option::Some(BindingMode::Strict),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BindingMode] = &[
            BindingMode::Relaxed,
            BindingMode::Strict,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BindingMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BindingMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BindingMode {
}

impl ::protobuf::reflect::ProtobufValue for BindingMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0btypes.proto\"e\n\x16ApplicationEnvironment\x12\x20\n\x0bapplicatio\
    n\x18\x01\x20\x02(\tR\x0bapplication\x12)\n\x0benvironment\x18\x02\x20\
    \x01(\t:\x07defaultR\x0benvironment\"n\n\x0cPackageIdent\x12\x16\n\x06or\
    igin\x18\x01\x20\x01(\tR\x06origin\x12\x12\n\x04name\x18\x02\x20\x01(\tR\
    \x04name\x12\x18\n\x07version\x18\x03\x20\x01(\tR\x07version\x12\x18\n\
    \x07release\x18\x04\x20\x01(\tR\x07release\"`\n\rProcessStatus\x12\x18\n\
    \x07elapsed\x18\x01\x20\x01(\x03R\x07elapsed\x12\x10\n\x03pid\x18\x02\
    \x20\x01(\rR\x03pid\x12#\n\x05state\x18\x03\x20\x01(\x0e2\r.ProcessState\
    R\x05state\"x\n\x0bServiceBind\x12\x12\n\x04name\x18\x01\x20\x02(\tR\x04\
    name\x122\n\rservice_group\x18\x02\x20\x02(\x0b2\r.ServiceGroupR\x0cserv\
    iceGroup\x12!\n\x0cservice_name\x18\x03\x20\x01(\tR\x0bserviceName\"l\n\
    \nServiceCfg\x120\n\x06format\x18\x01\x20\x01(\x0e2\x12.ServiceCfg.Forma\
    t:\x04TOMLR\x06format\x12\x18\n\x07default\x18\x02\x20\x01(\tR\x07defaul\
    t\"\x12\n\x06Format\x12\x08\n\x04TOML\x10\0\"\xbd\x01\n\x0cServiceGroup\
    \x12\x18\n\x07service\x18\x01\x20\x02(\tR\x07service\x12\x1d\n\x05group\
    \x18\x02\x20\x01(\t:\x07defaultR\x05group\x12P\n\x17application_environm\
    ent\x18\x03\x20\x01(\x0b2\x17.ApplicationEnvironmentR\x16applicationEnvi\
    ronment\x12\"\n\x0corganization\x18\x04\x20\x01(\tR\x0corganization\"\
    \xf7\x04\n\x0bServiceSpec\x12#\n\x05ident\x18\x01\x20\x02(\x0b2\r.Packag\
    eIdentR\x05ident\x12\x1d\n\x05group\x18\x02\x20\x01(\t:\x07defaultR\x05g\
    roup\x12P\n\x17application_environment\x18\x03\x20\x01(\x0b2\x17.Applica\
    tionEnvironmentR\x16applicationEnvironment\x122\n\x08bldr_url\x18\x04\
    \x20\x01(\t:\x17https://bldr.habitat.shR\x07bldrUrl\x12\x20\n\x07channel\
    \x18\x05\x20\x01(\t:\x06stableR\x07channel\x121\n\x08topology\x18\x06\
    \x20\x01(\x0e2\t.Topology:\nStandaloneR\x08topology\x12>\n\x0fupdate_str\
    ategy\x18\x07\x20\x01(\x0e2\x0f.UpdateStrategy:\x04NoneR\x0eupdateStrate\
    gy\x12\"\n\x05binds\x18\x08\x20\x03(\x0b2\x0c.ServiceBindR\x05binds\x128\
    \n\x0cbinding_mode\x18\t\x20\x01(\x0e2\x0c.BindingMode:\x07RelaxedR\x0bb\
    indingMode\x12\x1f\n\x0bconfig_from\x18\n\x20\x01(\tR\nconfigFrom\x126\n\
    \rdesired_state\x18\x0b\x20\x01(\x0e2\r.ProcessState:\x02UpR\x0cdesiredS\
    tate\x124\n\x16svc_encrypted_password\x18\x0c\x20\x01(\tR\x14svcEncrypte\
    dPassword\x12\x1c\n\tcomposite\x18\r\x20\x01(\tR\tcomposite\"\xb0\x01\n\
    \rServiceStatus\x12#\n\x05ident\x18\x01\x20\x01(\x0b2\r.PackageIdentR\
    \x05ident\x12(\n\x07process\x18\x02\x20\x01(\x0b2\x0e.ProcessStatusR\x07\
    process\x122\n\rservice_group\x18\x03\x20\x01(\x0b2\r.ServiceGroupR\x0cs\
    erviceGroup\x12\x1c\n\tcomposite\x18\x04\x20\x01(\tR\tcomposite*'\n\rIns\
    tallSource\x12\t\n\x05Ident\x10\0\x12\x0b\n\x07Archive\x10\x01*\x20\n\
    \x0cProcessState\x12\x08\n\x04Down\x10\0\x12\x06\n\x02Up\x10\x01*&\n\x08\
    Topology\x12\x0e\n\nStandalone\x10\0\x12\n\n\x06Leader\x10\x01*3\n\x0eUp\
    dateStrategy\x12\x08\n\x04None\x10\0\x12\n\n\x06AtOnce\x10\x01\x12\x0b\n\
    \x07Rolling\x10\x02*&\n\x0bBindingMode\x12\x0b\n\x07Relaxed\x10\0\x12\n\
    \n\x06Strict\x10\x01\
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
