use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_uchar, c_void, c_long, c_ulong};
use std::ptr::{null_mut, null};
use std::mem::{size_of, zeroed};
use std::process;
use libc::{FILE, stdin, stdout, stderr, fprintf, printf, sprintf, memset, strcmp};

type YamlCharT = c_uchar;
type YamlIntT = c_int;
type YamlSizeT = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlVersionDirective {
    major: YamlIntT,
    minor: YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlTagDirective {
    handle: *mut YamlCharT,
    prefix: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlMark {
    index: YamlSizeT,
    line: YamlSizeT,
    column: YamlSizeT,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlEncoding {
    Any = 0,
    Utf8 = 1,
    Utf16Le = 2,
    Utf16Be = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlBreak {
    Any = 0,
    Cr = 1,
    Ln = 2,
    CrLn = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlErrorType {
    NoError = 0,
    MemoryError = 1,
    ReaderError = 2,
    ScannerError = 3,
    ParserError = 4,
    ComposerError = 5,
    WriterError = 6,
    EmitterError = 7,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlScalarStyle {
    Any = 0,
    Plain = 1,
    SingleQuoted = 2,
    DoubleQuoted = 3,
    Literal = 4,
    Folded = 5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlSequenceStyle {
    Any = 0,
    Block = 1,
    Flow = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlMappingStyle {
    Any = 0,
    Block = 1,
    Flow = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlEvent {
    type_: YamlEventType,
    data: YamlEventData,
    start_mark: YamlMark,
    end_mark: YamlMark,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlEventType {
    NoEvent = 0,
    StreamStart = 1,
    StreamEnd = 2,
    DocumentStart = 3,
    DocumentEnd = 4,
    Alias = 5,
    Scalar = 6,
    SequenceStart = 7,
    SequenceEnd = 8,
    MappingStart = 9,
    MappingEnd = 10,
}

#[repr(C)]
union YamlEventData {
    stream_start: YamlStreamStartEventData,
    document_start: YamlDocumentStartEventData,
    document_end: YamlDocumentEndEventData,
    alias: YamlAliasEventData,
    scalar: YamlScalarEventData,
    sequence_start: YamlSequenceStartEventData,
    mapping_start: YamlMappingStartEventData,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlStreamStartEventData {
    encoding: YamlEncoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlDocumentStartEventData {
    version_directive: *mut YamlVersionDirective,
    tag_directives: YamlTagDirectiveRange,
    implicit: YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlTagDirectiveRange {
    start: *mut YamlTagDirective,
    end: *mut YamlTagDirective,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlDocumentEndEventData {
    implicit: YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlAliasEventData {
    anchor: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlScalarEventData {
    anchor: *mut YamlCharT,
    tag: *mut YamlCharT,
    value: *mut YamlCharT,
    length: YamlSizeT,
    plain_implicit: YamlIntT,
    quoted_implicit: YamlIntT,
    style: YamlScalarStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlSequenceStartEventData {
    anchor: *mut YamlCharT,
    tag: *mut YamlCharT,
    implicit: YamlIntT,
    style: YamlSequenceStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlMappingStartEventData {
    anchor: *mut YamlCharT,
    tag: *mut YamlCharT,
    implicit: YamlIntT,
    style: YamlMappingStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlDocument {
    nodes: YamlNodeRange,
    version_directive: *mut YamlVersionDirective,
    tag_directives: YamlTagDirectiveRange,
    start_implicit: YamlIntT,
    end_implicit: YamlIntT,
    start_mark: YamlMark,
    end_mark: YamlMark,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlNodeRange {
    start: *mut YamlNode,
    end: *mut YamlNode,
    top: *mut YamlNode,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlNode {
    type_: YamlNodeType,
    tag: *mut YamlCharT,
    data: YamlNodeData,
    start_mark: YamlMark,
    end_mark: YamlMark,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlNodeType {
    NoNode = 0,
    Scalar = 1,
    Sequence = 2,
    Mapping = 3,
}

#[repr(C)]
union YamlNodeData {
    scalar: YamlScalarNodeData,
    sequence: YamlSequenceNodeData,
    mapping: YamlMappingNodeData,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlScalarNodeData {
    value: *mut YamlCharT,
    length: YamlSizeT,
    style: YamlScalarStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlSequenceNodeData {
    items: YamlNodeItemRange,
    style: YamlSequenceStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlNodeItemRange {
    start: *mut YamlIntT,
    end: *mut YamlIntT,
    top: *mut YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlMappingNodeData {
    pairs: YamlNodePairRange,
    style: YamlMappingStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlNodePairRange {
    start: *mut YamlNodePair,
    end: *mut YamlNodePair,
    top: *mut YamlNodePair,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlNodePair {
    key: YamlIntT,
    value: YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlParser {
    error: YamlErrorType,
    problem: *const c_char,
    problem_offset: YamlSizeT,
    problem_value: YamlIntT,
    problem_mark: YamlMark,
    context: *const c_char,
    context_mark: YamlMark,
    read_handler: Option<unsafe extern "C" fn(*mut c_void, *mut c_uchar, YamlSizeT, *mut YamlSizeT) -> YamlIntT>,
    read_handler_data: *mut c_void,
    input: YamlParserInput,
    eof: YamlIntT,
    buffer: YamlCharBuffer,
    unread: YamlSizeT,
    raw_buffer: YamlUCharBuffer,
    encoding: YamlEncoding,
    offset: YamlSizeT,
    mark: YamlMark,
    stream_start_produced: YamlIntT,
    stream_end_produced: YamlIntT,
    flow_level: YamlIntT,
    tokens: YamlTokenBuffer,
    tokens_parsed: YamlSizeT,
    token_available: YamlIntT,
    indents: YamlIntBuffer,
    indent: YamlIntT,
    simple_key_allowed: YamlIntT,
    simple_keys: YamlSimpleKeyBuffer,
    states: YamlParserStateBuffer,
    state: YamlParserState,
    marks: YamlMarkBuffer,
    tag_directives: YamlTagDirectiveBuffer,
    aliases: YamlAliasDataBuffer,
    document: *mut YamlDocument,
}

#[repr(C)]
union YamlParserInput {
    string: YamlStringInput,
    file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlStringInput {
    start: *const c_uchar,
    end: *const c_uchar,
    current: *const c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlCharBuffer {
    start: *mut YamlCharT,
    end: *mut YamlCharT,
    pointer: *mut YamlCharT,
    last: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlUCharBuffer {
    start: *mut c_uchar,
    end: *mut c_uchar,
    pointer: *mut c_uchar,
    last: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlTokenBuffer {
    start: *mut YamlToken,
    end: *mut YamlToken,
    head: *mut YamlToken,
    tail: *mut YamlToken,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlToken {
    type_: YamlTokenType,
    data: YamlTokenData,
    start_mark: YamlMark,
    end_mark: YamlMark,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlTokenType {
    NoToken = 0,
    StreamStart = 1,
    StreamEnd = 2,
    VersionDirective = 3,
    TagDirective = 4,
    DocumentStart = 5,
    DocumentEnd = 6,
    BlockSequenceStart = 7,
    BlockMappingStart = 8,
    BlockEnd = 9,
    FlowSequenceStart = 10,
    FlowSequenceEnd = 11,
    FlowMappingStart = 12,
    FlowMappingEnd = 13,
    BlockEntry = 14,
    FlowEntry = 15,
    Key = 16,
    Value = 17,
    Alias = 18,
    Anchor = 19,
    Tag = 20,
    Scalar = 21,
}

#[repr(C)]
union YamlTokenData {
    stream_start: YamlStreamStartTokenData,
    alias: YamlAliasTokenData,
    anchor: YamlAnchorTokenData,
    tag: YamlTagTokenData,
    scalar: YamlScalarTokenData,
    version_directive: YamlVersionDirective,
    tag_directive: YamlTagDirective,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlStreamStartTokenData {
    encoding: YamlEncoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlAliasTokenData {
    value: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlAnchorTokenData {
    value: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlTagTokenData {
    handle: *mut YamlCharT,
    suffix: *mut YamlCharT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlScalarTokenData {
    value: *mut YamlCharT,
    length: YamlSizeT,
    style: YamlScalarStyle,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlIntBuffer {
    start: *mut YamlIntT,
    end: *mut YamlIntT,
    top: *mut YamlIntT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlSimpleKeyBuffer {
    start: *mut YamlSimpleKey,
    end: *mut YamlSimpleKey,
    top: *mut YamlSimpleKey,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlSimpleKey {
    possible: YamlIntT,
    required: YamlIntT,
    token_number: YamlSizeT,
    mark: YamlMark,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlParserStateBuffer {
    start: *mut YamlParserState,
    end: *mut YamlParserState,
    top: *mut YamlParserState,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlParserState {
    StreamStart = 0,
    ImplicitDocumentStart = 1,
    DocumentStart = 2,
    DocumentContent = 3,
    DocumentEnd = 4,
    BlockNode = 5,
    BlockNodeOrIndentlessSequence = 6,
    FlowNode = 7,
    BlockSequenceFirstEntry = 8,
    BlockSequenceEntry = 9,
    IndentlessSequenceEntry = 10,
    BlockMappingFirstKey = 11,
    BlockMappingKey = 12,
    BlockMappingValue = 13,
    FlowSequenceFirstEntry = 14,
    FlowSequenceEntry = 15,
    FlowSequenceEntryMappingKey = 16,
    FlowSequenceEntryMappingValue = 17,
    FlowSequenceEntryMappingEnd = 18,
    FlowMappingFirstKey = 19,
    FlowMappingKey = 20,
    FlowMappingValue = 21,
    FlowMappingEmptyValue = 22,
    End = 23,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlMarkBuffer {
    start: *mut YamlMark,
    end: *mut YamlMark,
    top: *mut YamlMark,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlTagDirectiveBuffer {
    start: *mut YamlTagDirective,
    end: *mut YamlTagDirective,
    top: *mut YamlTagDirective,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlAliasDataBuffer {
    start: *mut YamlAliasData,
    end: *mut YamlAliasData,
    top: *mut YamlAliasData,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlAliasData {
    anchor: *mut YamlCharT,
    index: YamlIntT,
    mark: YamlMark,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlEmitter {
    error: YamlErrorType,
    problem: *const c_char,
    write_handler: Option<unsafe extern "C" fn(*mut c_void, *mut c_uchar, YamlSizeT) -> YamlIntT>,
    write_handler_data: *mut c_void,
    output: YamlEmitterOutput,
    buffer: YamlCharBuffer,
    raw_buffer: YamlUCharBuffer,
    encoding: YamlEncoding,
    canonical: YamlIntT,
    best_indent: YamlIntT,
    best_width: YamlIntT,
    unicode: YamlIntT,
    line_break: YamlBreak,
    states: YamlEmitterStateBuffer,
    state: YamlEmitterState,
    events: YamlEventBuffer,
    indents: YamlIntBuffer,
    tag_directives: YamlTagDirectiveBuffer,
    indent: YamlIntT,
    flow_level: YamlIntT,
    root_context: YamlIntT,
    sequence_context: YamlIntT,
    mapping_context: YamlIntT,
    simple_key_context: YamlIntT,
    line: YamlIntT,
    column: YamlIntT,
    whitespace: YamlIntT,
    indention: YamlIntT,
    open_ended: YamlIntT,
    anchor_data: YamlAnchorData,
    tag_data: YamlTagData,
    scalar_data: YamlScalarData,
    opened: YamlIntT,
    closed: YamlIntT,
    anchors: *mut YamlAnchors,
    last_anchor_id: YamlIntT,
    document: *mut YamlDocument,
}

#[repr(C)]
union YamlEmitterOutput {
    string: YamlStringOutput,
    file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlStringOutput {
    buffer: *mut c_uchar,
    size: YamlSizeT,
    size_written: *mut YamlSizeT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlEmitterStateBuffer {
    start: *mut YamlEmitterState,
    end: *mut YamlEmitterState,
    top: *mut YamlEmitterState,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum YamlEmitterState {
    StreamStart = 0,
    FirstDocumentStart = 1,
    DocumentStart = 2,
    DocumentContent = 3,
    DocumentEnd = 4,
    FlowSequenceFirstItem = 5,
    FlowSequenceItem = 6,
    FlowMappingFirstKey = 7,
    FlowMappingKey = 8,
    FlowMappingSimpleValue = 9,
    FlowMappingValue = 10,
    BlockSequenceFirstItem = 11,
    BlockSequenceItem = 12,
    BlockMappingFirstKey = 13,
    BlockMappingKey = 14,
    BlockMappingSimpleValue = 15,
    BlockMappingValue = 16,
    End = 17,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YamlEventBuffer {
    start: *mut YamlEvent,
    end: *mut YamlEvent,
    head: *mut YamlEvent,
    tail: *mut