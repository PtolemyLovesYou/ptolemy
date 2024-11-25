# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: observer.proto
# Protobuf Python Version: 5.28.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    28,
    1,
    '',
    'observer.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0eobserver.proto\x12\x08observer\"3\n\x0ePublishRequest\x12!\n\x07records\x18\x01 \x03(\x0b\x32\x10.observer.Record\"G\n\x0fPublishResponse\x12\x12\n\nsuccessful\x18\x01 \x01(\x08\x12\x14\n\x07message\x18\x02 \x01(\tH\x00\x88\x01\x01\x42\n\n\x08_message\"\xee\x03\n\x06Record\x12\x1c\n\x04tier\x18\x01 \x01(\x0e\x32\x0e.observer.Tier\x12#\n\x08log_type\x18\x02 \x01(\x0e\x32\x11.observer.LogType\x12\x11\n\tparent_id\x18\x03 \x01(\t\x12\n\n\x02id\x18\x04 \x01(\t\x12\x11\n\x04name\x18\x05 \x01(\tH\x00\x88\x01\x01\x12\x17\n\nparameters\x18\x06 \x01(\tH\x01\x88\x01\x01\x12\x14\n\x07version\x18\x07 \x01(\tH\x02\x88\x01\x01\x12\x18\n\x0b\x65nvironment\x18\x08 \x01(\tH\x03\x88\x01\x01\x12\x17\n\nstart_time\x18\t \x01(\tH\x04\x88\x01\x01\x12\x15\n\x08\x65nd_time\x18\n \x01(\tH\x05\x88\x01\x01\x12\x17\n\nerror_type\x18\x0b \x01(\tH\x06\x88\x01\x01\x12\x1a\n\rerror_content\x18\x0c \x01(\tH\x07\x88\x01\x01\x12\x17\n\nfield_name\x18\r \x01(\tH\x08\x88\x01\x01\x12\x18\n\x0b\x66ield_value\x18\x0e \x01(\tH\t\x88\x01\x01\x42\x07\n\x05_nameB\r\n\x0b_parametersB\n\n\x08_versionB\x0e\n\x0c_environmentB\r\n\x0b_start_timeB\x0b\n\t_end_timeB\r\n\x0b_error_typeB\x10\n\x0e_error_contentB\r\n\x0b_field_nameB\x0e\n\x0c_field_value*i\n\x07LogType\x12\x17\n\x13UNDECLARED_LOG_TYPE\x10\x00\x12\t\n\x05\x45VENT\x10\x01\x12\x0b\n\x07RUNTIME\x10\x02\x12\t\n\x05INPUT\x10\x03\x12\n\n\x06OUTPUT\x10\x04\x12\x08\n\x04TIER\x10\x05\x12\x0c\n\x08METADATA\x10\x06*W\n\x04Tier\x12\x13\n\x0fUNDECLARED_TIER\x10\x00\x12\n\n\x06SYSTEM\x10\x01\x12\r\n\tSUBSYSTEM\x10\x02\x12\r\n\tCOMPONENT\x10\x03\x12\x10\n\x0cSUBCOMPONENT\x10\x04\x32J\n\x08Observer\x12>\n\x07Publish\x12\x18.observer.PublishRequest\x1a\x19.observer.PublishResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'observer_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_LOGTYPE']._serialized_start=651
  _globals['_LOGTYPE']._serialized_end=756
  _globals['_TIER']._serialized_start=758
  _globals['_TIER']._serialized_end=845
  _globals['_PUBLISHREQUEST']._serialized_start=28
  _globals['_PUBLISHREQUEST']._serialized_end=79
  _globals['_PUBLISHRESPONSE']._serialized_start=81
  _globals['_PUBLISHRESPONSE']._serialized_end=152
  _globals['_RECORD']._serialized_start=155
  _globals['_RECORD']._serialized_end=649
  _globals['_OBSERVER']._serialized_start=847
  _globals['_OBSERVER']._serialized_end=921
# @@protoc_insertion_point(module_scope)
