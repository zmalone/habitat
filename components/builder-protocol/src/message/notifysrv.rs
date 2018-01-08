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
pub struct NotificationCreate {
    // message fields
    notification: ::protobuf::SingularPtrField<Notification>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NotificationCreate {}

impl NotificationCreate {
    pub fn new() -> NotificationCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotificationCreate {
        static mut instance: ::protobuf::lazy::Lazy<NotificationCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotificationCreate,
        };
        unsafe {
            instance.get(NotificationCreate::new)
        }
    }

    // optional .notifysrv.Notification notification = 1;

    pub fn clear_notification(&mut self) {
        self.notification.clear();
    }

    pub fn has_notification(&self) -> bool {
        self.notification.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notification(&mut self, v: Notification) {
        self.notification = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_notification(&mut self) -> &mut Notification {
        if self.notification.is_none() {
            self.notification.set_default();
        }
        self.notification.as_mut().unwrap()
    }

    // Take field
    pub fn take_notification(&mut self) -> Notification {
        self.notification.take().unwrap_or_else(|| Notification::new())
    }

    pub fn get_notification(&self) -> &Notification {
        self.notification.as_ref().unwrap_or_else(|| Notification::default_instance())
    }

    fn get_notification_for_reflect(&self) -> &::protobuf::SingularPtrField<Notification> {
        &self.notification
    }

    fn mut_notification_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Notification> {
        &mut self.notification
    }
}

impl ::protobuf::Message for NotificationCreate {
    fn is_initialized(&self) -> bool {
        for v in &self.notification {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.notification)?;
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
        if let Some(ref v) = self.notification.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.notification.as_ref() {
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

impl ::protobuf::MessageStatic for NotificationCreate {
    fn new() -> NotificationCreate {
        NotificationCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotificationCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Notification>>(
                    "notification",
                    NotificationCreate::get_notification_for_reflect,
                    NotificationCreate::mut_notification_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotificationCreate>(
                    "NotificationCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotificationCreate {
    fn clear(&mut self) {
        self.clear_notification();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NotificationCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NotificationCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Notification {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    category: ::std::option::Option<NotificationCategory>,
    data: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Notification {}

impl Notification {
    pub fn new() -> Notification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Notification {
        static mut instance: ::protobuf::lazy::Lazy<Notification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Notification,
        };
        unsafe {
            instance.get(Notification::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional uint64 account_id = 3;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional .notifysrv.NotificationCategory category = 4;

    pub fn clear_category(&mut self) {
        self.category = ::std::option::Option::None;
    }

    pub fn has_category(&self) -> bool {
        self.category.is_some()
    }

    // Param is passed by value, moved
    pub fn set_category(&mut self, v: NotificationCategory) {
        self.category = ::std::option::Option::Some(v);
    }

    pub fn get_category(&self) -> NotificationCategory {
        self.category.unwrap_or(NotificationCategory::Info)
    }

    fn get_category_for_reflect(&self) -> &::std::option::Option<NotificationCategory> {
        &self.category
    }

    fn mut_category_for_reflect(&mut self) -> &mut ::std::option::Option<NotificationCategory> {
        &mut self.category
    }

    // optional string data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        self.data.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data(&self) -> &str {
        match self.data.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data
    }
}

impl ::protobuf::Message for Notification {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.category = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.category {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.category {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for Notification {
    fn new() -> Notification {
        Notification::new()
    }

    fn descriptor_static(_: ::std::option::Option<Notification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Notification::get_id_for_reflect,
                    Notification::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    Notification::get_origin_id_for_reflect,
                    Notification::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    Notification::get_account_id_for_reflect,
                    Notification::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NotificationCategory>>(
                    "category",
                    Notification::get_category_for_reflect,
                    Notification::mut_category_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data",
                    Notification::get_data_for_reflect,
                    Notification::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Notification>(
                    "Notification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Notification {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_account_id();
        self.clear_category();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Notification {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NotificationCategory {
    Info = 1,
    Error = 2,
}

impl ::protobuf::ProtobufEnum for NotificationCategory {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NotificationCategory> {
        match value {
            1 => ::std::option::Option::Some(NotificationCategory::Info),
            2 => ::std::option::Option::Some(NotificationCategory::Error),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NotificationCategory] = &[
            NotificationCategory::Info,
            NotificationCategory::Error,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NotificationCategory>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NotificationCategory", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NotificationCategory {
}

impl ::protobuf::reflect::ProtobufValue for NotificationCategory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19protocols/notifysrv.proto\x12\tnotifysrv\"Q\n\x12NotificationCreat\
    e\x12;\n\x0cnotification\x18\x01\x20\x01(\x0b2\x17.notifysrv.Notificatio\
    nR\x0cnotification\"\xab\x01\n\x0cNotification\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x04R\x02id\x12\x1b\n\torigin_id\x18\x02\x20\x01(\x04R\x08origi\
    nId\x12\x1d\n\naccount_id\x18\x03\x20\x01(\x04R\taccountId\x12;\n\x08cat\
    egory\x18\x04\x20\x01(\x0e2\x1f.notifysrv.NotificationCategoryR\x08categ\
    ory\x12\x12\n\x04data\x18\x05\x20\x01(\tR\x04data*+\n\x14NotificationCat\
    egory\x12\x08\n\x04Info\x10\x01\x12\t\n\x05Error\x10\x02J\xd4\x04\n\x06\
    \x12\x04\0\0\x12\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x01\x08\x11\n\n\n\x02\x04\0\x12\x04\x03\0\x05\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x03\x08\x1a\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02)\n\x0c\
    \n\x05\x04\0\x02\0\x04\x12\x03\x04\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x04\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x18$\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x04'(\n\n\n\x02\x04\x01\x12\x04\x07\0\r\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x08\x02\x19\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\
    \x05\x04\x01\x02\0\x05\x12\x03\x08\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x08\x12\x14\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x17\x18\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x02\x20\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x03\t\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\t\x0b\x11\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\t\x12\x1b\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03\t\x1e\x1f\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\n\x02!\n\
    \x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x02\x05\x12\x03\n\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x12\
    \x1c\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\n\x1f\x20\n\x0b\n\x04\x04\
    \x01\x02\x03\x12\x03\x0b\x02-\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\
    \x0b\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x0b\x0b\x1f\n\x0c\n\
    \x05\x04\x01\x02\x03\x01\x12\x03\x0b\x20(\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03\x0b+,\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x0c\x02\x1b\n\x0c\
    \n\x05\x04\x01\x02\x04\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\x01\x02\x04\
    \x05\x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x0c\x12\
    \x16\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0c\x19\x1a\n\n\n\x02\x05\0\
    \x12\x04\x0f\0\x12\x01\n\n\n\x03\x05\0\x01\x12\x03\x0f\x05\x19\n\x0b\n\
    \x04\x05\0\x02\0\x12\x03\x10\x02\x0b\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\
    \x10\x02\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x10\t\n\n\x0b\n\x04\x05\
    \0\x02\x01\x12\x03\x11\x02\x0c\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x11\
    \x02\x07\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x11\n\x0b\
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
