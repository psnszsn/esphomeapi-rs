// @generated, do not edit
#![warn(rust_2018_idioms)]
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LegacyCoverState(i32);
impl LegacyCoverState {
  pub const LEGACY_COVER_STATE_OPEN: LegacyCoverState = LegacyCoverState(0);
  pub const LEGACY_COVER_STATE_CLOSED: LegacyCoverState = LegacyCoverState(1);
  pub const KNOWN_VARIANTS: [LegacyCoverState; 2] = [LegacyCoverState::LEGACY_COVER_STATE_OPEN, LegacyCoverState::LEGACY_COVER_STATE_CLOSED];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<LegacyCoverState_Closed> {
    match self {
      LegacyCoverState::LEGACY_COVER_STATE_OPEN => Some(LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN),
      LegacyCoverState::LEGACY_COVER_STATE_CLOSED => Some(LegacyCoverState_Closed::LEGACY_COVER_STATE_CLOSED),
      _ => None,
    }
  }
}
impl ::std::default::Default for LegacyCoverState {
  fn default() -> Self {
    LegacyCoverState::LEGACY_COVER_STATE_OPEN
  }
}
impl From<LegacyCoverState> for i32 {
  fn from(v: LegacyCoverState) -> i32 {
    v.0
  }
}
impl From<i32> for LegacyCoverState {
  fn from(v: i32) -> LegacyCoverState {
    LegacyCoverState(v)
  }
}
impl From<LegacyCoverState_Closed> for LegacyCoverState {
  fn from(v: LegacyCoverState_Closed) -> LegacyCoverState {
    LegacyCoverState(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for LegacyCoverState {
}
impl ::pb_jelly::OpenProtoEnum for LegacyCoverState {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      LegacyCoverState::LEGACY_COVER_STATE_OPEN => Some("LEGACY_COVER_STATE_OPEN"),
      LegacyCoverState::LEGACY_COVER_STATE_CLOSED => Some("LEGACY_COVER_STATE_CLOSED"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      LegacyCoverState::LEGACY_COVER_STATE_OPEN => true,
      LegacyCoverState::LEGACY_COVER_STATE_CLOSED => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for LegacyCoverState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum LegacyCoverState_Closed {
  LEGACY_COVER_STATE_OPEN = 0,
  LEGACY_COVER_STATE_CLOSED = 1,
}
impl LegacyCoverState_Closed {
  pub const KNOWN_VARIANTS: [LegacyCoverState_Closed; 2] = [LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN, LegacyCoverState_Closed::LEGACY_COVER_STATE_CLOSED];
}
impl ::std::default::Default for LegacyCoverState_Closed {
  fn default() -> Self {
    LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN
  }
}
impl From<LegacyCoverState_Closed> for i32 {
  fn from(v: LegacyCoverState_Closed) -> i32 {
    match v {
      LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN => 0,
      LegacyCoverState_Closed::LEGACY_COVER_STATE_CLOSED => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for LegacyCoverState_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN),
      1 => Ok(LegacyCoverState_Closed::LEGACY_COVER_STATE_CLOSED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for LegacyCoverState_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for LegacyCoverState_Closed {
  fn name(self) -> &'static str {
    match self {
      LegacyCoverState_Closed::LEGACY_COVER_STATE_OPEN => "LEGACY_COVER_STATE_OPEN",
      LegacyCoverState_Closed::LEGACY_COVER_STATE_CLOSED => "LEGACY_COVER_STATE_CLOSED",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CoverOperation(i32);
impl CoverOperation {
  pub const COVER_OPERATION_IDLE: CoverOperation = CoverOperation(0);
  pub const COVER_OPERATION_IS_OPENING: CoverOperation = CoverOperation(1);
  pub const COVER_OPERATION_IS_CLOSING: CoverOperation = CoverOperation(2);
  pub const KNOWN_VARIANTS: [CoverOperation; 3] = [CoverOperation::COVER_OPERATION_IDLE, CoverOperation::COVER_OPERATION_IS_OPENING, CoverOperation::COVER_OPERATION_IS_CLOSING];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<CoverOperation_Closed> {
    match self {
      CoverOperation::COVER_OPERATION_IDLE => Some(CoverOperation_Closed::COVER_OPERATION_IDLE),
      CoverOperation::COVER_OPERATION_IS_OPENING => Some(CoverOperation_Closed::COVER_OPERATION_IS_OPENING),
      CoverOperation::COVER_OPERATION_IS_CLOSING => Some(CoverOperation_Closed::COVER_OPERATION_IS_CLOSING),
      _ => None,
    }
  }
}
impl ::std::default::Default for CoverOperation {
  fn default() -> Self {
    CoverOperation::COVER_OPERATION_IDLE
  }
}
impl From<CoverOperation> for i32 {
  fn from(v: CoverOperation) -> i32 {
    v.0
  }
}
impl From<i32> for CoverOperation {
  fn from(v: i32) -> CoverOperation {
    CoverOperation(v)
  }
}
impl From<CoverOperation_Closed> for CoverOperation {
  fn from(v: CoverOperation_Closed) -> CoverOperation {
    CoverOperation(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for CoverOperation {
}
impl ::pb_jelly::OpenProtoEnum for CoverOperation {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      CoverOperation::COVER_OPERATION_IDLE => Some("COVER_OPERATION_IDLE"),
      CoverOperation::COVER_OPERATION_IS_OPENING => Some("COVER_OPERATION_IS_OPENING"),
      CoverOperation::COVER_OPERATION_IS_CLOSING => Some("COVER_OPERATION_IS_CLOSING"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      CoverOperation::COVER_OPERATION_IDLE => true,
      CoverOperation::COVER_OPERATION_IS_OPENING => true,
      CoverOperation::COVER_OPERATION_IS_CLOSING => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for CoverOperation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum CoverOperation_Closed {
  COVER_OPERATION_IDLE = 0,
  COVER_OPERATION_IS_OPENING = 1,
  COVER_OPERATION_IS_CLOSING = 2,
}
impl CoverOperation_Closed {
  pub const KNOWN_VARIANTS: [CoverOperation_Closed; 3] = [CoverOperation_Closed::COVER_OPERATION_IDLE, CoverOperation_Closed::COVER_OPERATION_IS_OPENING, CoverOperation_Closed::COVER_OPERATION_IS_CLOSING];
}
impl ::std::default::Default for CoverOperation_Closed {
  fn default() -> Self {
    CoverOperation_Closed::COVER_OPERATION_IDLE
  }
}
impl From<CoverOperation_Closed> for i32 {
  fn from(v: CoverOperation_Closed) -> i32 {
    match v {
      CoverOperation_Closed::COVER_OPERATION_IDLE => 0,
      CoverOperation_Closed::COVER_OPERATION_IS_OPENING => 1,
      CoverOperation_Closed::COVER_OPERATION_IS_CLOSING => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for CoverOperation_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(CoverOperation_Closed::COVER_OPERATION_IDLE),
      1 => Ok(CoverOperation_Closed::COVER_OPERATION_IS_OPENING),
      2 => Ok(CoverOperation_Closed::COVER_OPERATION_IS_CLOSING),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for CoverOperation_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for CoverOperation_Closed {
  fn name(self) -> &'static str {
    match self {
      CoverOperation_Closed::COVER_OPERATION_IDLE => "COVER_OPERATION_IDLE",
      CoverOperation_Closed::COVER_OPERATION_IS_OPENING => "COVER_OPERATION_IS_OPENING",
      CoverOperation_Closed::COVER_OPERATION_IS_CLOSING => "COVER_OPERATION_IS_CLOSING",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LegacyCoverCommand(i32);
impl LegacyCoverCommand {
  pub const LEGACY_COVER_COMMAND_OPEN: LegacyCoverCommand = LegacyCoverCommand(0);
  pub const LEGACY_COVER_COMMAND_CLOSE: LegacyCoverCommand = LegacyCoverCommand(1);
  pub const LEGACY_COVER_COMMAND_STOP: LegacyCoverCommand = LegacyCoverCommand(2);
  pub const KNOWN_VARIANTS: [LegacyCoverCommand; 3] = [LegacyCoverCommand::LEGACY_COVER_COMMAND_OPEN, LegacyCoverCommand::LEGACY_COVER_COMMAND_CLOSE, LegacyCoverCommand::LEGACY_COVER_COMMAND_STOP];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<LegacyCoverCommand_Closed> {
    match self {
      LegacyCoverCommand::LEGACY_COVER_COMMAND_OPEN => Some(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN),
      LegacyCoverCommand::LEGACY_COVER_COMMAND_CLOSE => Some(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_CLOSE),
      LegacyCoverCommand::LEGACY_COVER_COMMAND_STOP => Some(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_STOP),
      _ => None,
    }
  }
}
impl ::std::default::Default for LegacyCoverCommand {
  fn default() -> Self {
    LegacyCoverCommand::LEGACY_COVER_COMMAND_OPEN
  }
}
impl From<LegacyCoverCommand> for i32 {
  fn from(v: LegacyCoverCommand) -> i32 {
    v.0
  }
}
impl From<i32> for LegacyCoverCommand {
  fn from(v: i32) -> LegacyCoverCommand {
    LegacyCoverCommand(v)
  }
}
impl From<LegacyCoverCommand_Closed> for LegacyCoverCommand {
  fn from(v: LegacyCoverCommand_Closed) -> LegacyCoverCommand {
    LegacyCoverCommand(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for LegacyCoverCommand {
}
impl ::pb_jelly::OpenProtoEnum for LegacyCoverCommand {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      LegacyCoverCommand::LEGACY_COVER_COMMAND_OPEN => Some("LEGACY_COVER_COMMAND_OPEN"),
      LegacyCoverCommand::LEGACY_COVER_COMMAND_CLOSE => Some("LEGACY_COVER_COMMAND_CLOSE"),
      LegacyCoverCommand::LEGACY_COVER_COMMAND_STOP => Some("LEGACY_COVER_COMMAND_STOP"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      LegacyCoverCommand::LEGACY_COVER_COMMAND_OPEN => true,
      LegacyCoverCommand::LEGACY_COVER_COMMAND_CLOSE => true,
      LegacyCoverCommand::LEGACY_COVER_COMMAND_STOP => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for LegacyCoverCommand {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum LegacyCoverCommand_Closed {
  LEGACY_COVER_COMMAND_OPEN = 0,
  LEGACY_COVER_COMMAND_CLOSE = 1,
  LEGACY_COVER_COMMAND_STOP = 2,
}
impl LegacyCoverCommand_Closed {
  pub const KNOWN_VARIANTS: [LegacyCoverCommand_Closed; 3] = [LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN, LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_CLOSE, LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_STOP];
}
impl ::std::default::Default for LegacyCoverCommand_Closed {
  fn default() -> Self {
    LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN
  }
}
impl From<LegacyCoverCommand_Closed> for i32 {
  fn from(v: LegacyCoverCommand_Closed) -> i32 {
    match v {
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN => 0,
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_CLOSE => 1,
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_STOP => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for LegacyCoverCommand_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN),
      1 => Ok(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_CLOSE),
      2 => Ok(LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_STOP),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for LegacyCoverCommand_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for LegacyCoverCommand_Closed {
  fn name(self) -> &'static str {
    match self {
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_OPEN => "LEGACY_COVER_COMMAND_OPEN",
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_CLOSE => "LEGACY_COVER_COMMAND_CLOSE",
      LegacyCoverCommand_Closed::LEGACY_COVER_COMMAND_STOP => "LEGACY_COVER_COMMAND_STOP",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FanSpeed(i32);
impl FanSpeed {
  pub const FAN_SPEED_LOW: FanSpeed = FanSpeed(0);
  pub const FAN_SPEED_MEDIUM: FanSpeed = FanSpeed(1);
  pub const FAN_SPEED_HIGH: FanSpeed = FanSpeed(2);
  pub const KNOWN_VARIANTS: [FanSpeed; 3] = [FanSpeed::FAN_SPEED_LOW, FanSpeed::FAN_SPEED_MEDIUM, FanSpeed::FAN_SPEED_HIGH];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FanSpeed_Closed> {
    match self {
      FanSpeed::FAN_SPEED_LOW => Some(FanSpeed_Closed::FAN_SPEED_LOW),
      FanSpeed::FAN_SPEED_MEDIUM => Some(FanSpeed_Closed::FAN_SPEED_MEDIUM),
      FanSpeed::FAN_SPEED_HIGH => Some(FanSpeed_Closed::FAN_SPEED_HIGH),
      _ => None,
    }
  }
}
impl ::std::default::Default for FanSpeed {
  fn default() -> Self {
    FanSpeed::FAN_SPEED_LOW
  }
}
impl From<FanSpeed> for i32 {
  fn from(v: FanSpeed) -> i32 {
    v.0
  }
}
impl From<i32> for FanSpeed {
  fn from(v: i32) -> FanSpeed {
    FanSpeed(v)
  }
}
impl From<FanSpeed_Closed> for FanSpeed {
  fn from(v: FanSpeed_Closed) -> FanSpeed {
    FanSpeed(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FanSpeed {
}
impl ::pb_jelly::OpenProtoEnum for FanSpeed {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FanSpeed::FAN_SPEED_LOW => Some("FAN_SPEED_LOW"),
      FanSpeed::FAN_SPEED_MEDIUM => Some("FAN_SPEED_MEDIUM"),
      FanSpeed::FAN_SPEED_HIGH => Some("FAN_SPEED_HIGH"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FanSpeed::FAN_SPEED_LOW => true,
      FanSpeed::FAN_SPEED_MEDIUM => true,
      FanSpeed::FAN_SPEED_HIGH => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FanSpeed {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FanSpeed_Closed {
  FAN_SPEED_LOW = 0,
  FAN_SPEED_MEDIUM = 1,
  FAN_SPEED_HIGH = 2,
}
impl FanSpeed_Closed {
  pub const KNOWN_VARIANTS: [FanSpeed_Closed; 3] = [FanSpeed_Closed::FAN_SPEED_LOW, FanSpeed_Closed::FAN_SPEED_MEDIUM, FanSpeed_Closed::FAN_SPEED_HIGH];
}
impl ::std::default::Default for FanSpeed_Closed {
  fn default() -> Self {
    FanSpeed_Closed::FAN_SPEED_LOW
  }
}
impl From<FanSpeed_Closed> for i32 {
  fn from(v: FanSpeed_Closed) -> i32 {
    match v {
      FanSpeed_Closed::FAN_SPEED_LOW => 0,
      FanSpeed_Closed::FAN_SPEED_MEDIUM => 1,
      FanSpeed_Closed::FAN_SPEED_HIGH => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FanSpeed_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FanSpeed_Closed::FAN_SPEED_LOW),
      1 => Ok(FanSpeed_Closed::FAN_SPEED_MEDIUM),
      2 => Ok(FanSpeed_Closed::FAN_SPEED_HIGH),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FanSpeed_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FanSpeed_Closed {
  fn name(self) -> &'static str {
    match self {
      FanSpeed_Closed::FAN_SPEED_LOW => "FAN_SPEED_LOW",
      FanSpeed_Closed::FAN_SPEED_MEDIUM => "FAN_SPEED_MEDIUM",
      FanSpeed_Closed::FAN_SPEED_HIGH => "FAN_SPEED_HIGH",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FanDirection(i32);
impl FanDirection {
  pub const FAN_DIRECTION_FORWARD: FanDirection = FanDirection(0);
  pub const FAN_DIRECTION_REVERSE: FanDirection = FanDirection(1);
  pub const KNOWN_VARIANTS: [FanDirection; 2] = [FanDirection::FAN_DIRECTION_FORWARD, FanDirection::FAN_DIRECTION_REVERSE];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FanDirection_Closed> {
    match self {
      FanDirection::FAN_DIRECTION_FORWARD => Some(FanDirection_Closed::FAN_DIRECTION_FORWARD),
      FanDirection::FAN_DIRECTION_REVERSE => Some(FanDirection_Closed::FAN_DIRECTION_REVERSE),
      _ => None,
    }
  }
}
impl ::std::default::Default for FanDirection {
  fn default() -> Self {
    FanDirection::FAN_DIRECTION_FORWARD
  }
}
impl From<FanDirection> for i32 {
  fn from(v: FanDirection) -> i32 {
    v.0
  }
}
impl From<i32> for FanDirection {
  fn from(v: i32) -> FanDirection {
    FanDirection(v)
  }
}
impl From<FanDirection_Closed> for FanDirection {
  fn from(v: FanDirection_Closed) -> FanDirection {
    FanDirection(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FanDirection {
}
impl ::pb_jelly::OpenProtoEnum for FanDirection {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FanDirection::FAN_DIRECTION_FORWARD => Some("FAN_DIRECTION_FORWARD"),
      FanDirection::FAN_DIRECTION_REVERSE => Some("FAN_DIRECTION_REVERSE"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FanDirection::FAN_DIRECTION_FORWARD => true,
      FanDirection::FAN_DIRECTION_REVERSE => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FanDirection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FanDirection_Closed {
  FAN_DIRECTION_FORWARD = 0,
  FAN_DIRECTION_REVERSE = 1,
}
impl FanDirection_Closed {
  pub const KNOWN_VARIANTS: [FanDirection_Closed; 2] = [FanDirection_Closed::FAN_DIRECTION_FORWARD, FanDirection_Closed::FAN_DIRECTION_REVERSE];
}
impl ::std::default::Default for FanDirection_Closed {
  fn default() -> Self {
    FanDirection_Closed::FAN_DIRECTION_FORWARD
  }
}
impl From<FanDirection_Closed> for i32 {
  fn from(v: FanDirection_Closed) -> i32 {
    match v {
      FanDirection_Closed::FAN_DIRECTION_FORWARD => 0,
      FanDirection_Closed::FAN_DIRECTION_REVERSE => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FanDirection_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FanDirection_Closed::FAN_DIRECTION_FORWARD),
      1 => Ok(FanDirection_Closed::FAN_DIRECTION_REVERSE),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FanDirection_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FanDirection_Closed {
  fn name(self) -> &'static str {
    match self {
      FanDirection_Closed::FAN_DIRECTION_FORWARD => "FAN_DIRECTION_FORWARD",
      FanDirection_Closed::FAN_DIRECTION_REVERSE => "FAN_DIRECTION_REVERSE",
    }
  }
}

/// ==================== SUBSCRIBE LOGS ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LogLevel(i32);
impl LogLevel {
  pub const LOG_LEVEL_NONE: LogLevel = LogLevel(0);
  pub const LOG_LEVEL_ERROR: LogLevel = LogLevel(1);
  pub const LOG_LEVEL_WARN: LogLevel = LogLevel(2);
  pub const LOG_LEVEL_INFO: LogLevel = LogLevel(3);
  pub const LOG_LEVEL_DEBUG: LogLevel = LogLevel(4);
  pub const LOG_LEVEL_VERBOSE: LogLevel = LogLevel(5);
  pub const LOG_LEVEL_VERY_VERBOSE: LogLevel = LogLevel(6);
  pub const KNOWN_VARIANTS: [LogLevel; 7] = [LogLevel::LOG_LEVEL_NONE, LogLevel::LOG_LEVEL_ERROR, LogLevel::LOG_LEVEL_WARN, LogLevel::LOG_LEVEL_INFO, LogLevel::LOG_LEVEL_DEBUG, LogLevel::LOG_LEVEL_VERBOSE, LogLevel::LOG_LEVEL_VERY_VERBOSE];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<LogLevel_Closed> {
    match self {
      LogLevel::LOG_LEVEL_NONE => Some(LogLevel_Closed::LOG_LEVEL_NONE),
      LogLevel::LOG_LEVEL_ERROR => Some(LogLevel_Closed::LOG_LEVEL_ERROR),
      LogLevel::LOG_LEVEL_WARN => Some(LogLevel_Closed::LOG_LEVEL_WARN),
      LogLevel::LOG_LEVEL_INFO => Some(LogLevel_Closed::LOG_LEVEL_INFO),
      LogLevel::LOG_LEVEL_DEBUG => Some(LogLevel_Closed::LOG_LEVEL_DEBUG),
      LogLevel::LOG_LEVEL_VERBOSE => Some(LogLevel_Closed::LOG_LEVEL_VERBOSE),
      LogLevel::LOG_LEVEL_VERY_VERBOSE => Some(LogLevel_Closed::LOG_LEVEL_VERY_VERBOSE),
      _ => None,
    }
  }
}
impl ::std::default::Default for LogLevel {
  fn default() -> Self {
    LogLevel::LOG_LEVEL_NONE
  }
}
impl From<LogLevel> for i32 {
  fn from(v: LogLevel) -> i32 {
    v.0
  }
}
impl From<i32> for LogLevel {
  fn from(v: i32) -> LogLevel {
    LogLevel(v)
  }
}
impl From<LogLevel_Closed> for LogLevel {
  fn from(v: LogLevel_Closed) -> LogLevel {
    LogLevel(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for LogLevel {
}
impl ::pb_jelly::OpenProtoEnum for LogLevel {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      LogLevel::LOG_LEVEL_NONE => Some("LOG_LEVEL_NONE"),
      LogLevel::LOG_LEVEL_ERROR => Some("LOG_LEVEL_ERROR"),
      LogLevel::LOG_LEVEL_WARN => Some("LOG_LEVEL_WARN"),
      LogLevel::LOG_LEVEL_INFO => Some("LOG_LEVEL_INFO"),
      LogLevel::LOG_LEVEL_DEBUG => Some("LOG_LEVEL_DEBUG"),
      LogLevel::LOG_LEVEL_VERBOSE => Some("LOG_LEVEL_VERBOSE"),
      LogLevel::LOG_LEVEL_VERY_VERBOSE => Some("LOG_LEVEL_VERY_VERBOSE"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      LogLevel::LOG_LEVEL_NONE => true,
      LogLevel::LOG_LEVEL_ERROR => true,
      LogLevel::LOG_LEVEL_WARN => true,
      LogLevel::LOG_LEVEL_INFO => true,
      LogLevel::LOG_LEVEL_DEBUG => true,
      LogLevel::LOG_LEVEL_VERBOSE => true,
      LogLevel::LOG_LEVEL_VERY_VERBOSE => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for LogLevel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// ==================== SUBSCRIBE LOGS ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum LogLevel_Closed {
  LOG_LEVEL_NONE = 0,
  LOG_LEVEL_ERROR = 1,
  LOG_LEVEL_WARN = 2,
  LOG_LEVEL_INFO = 3,
  LOG_LEVEL_DEBUG = 4,
  LOG_LEVEL_VERBOSE = 5,
  LOG_LEVEL_VERY_VERBOSE = 6,
}
impl LogLevel_Closed {
  pub const KNOWN_VARIANTS: [LogLevel_Closed; 7] = [LogLevel_Closed::LOG_LEVEL_NONE, LogLevel_Closed::LOG_LEVEL_ERROR, LogLevel_Closed::LOG_LEVEL_WARN, LogLevel_Closed::LOG_LEVEL_INFO, LogLevel_Closed::LOG_LEVEL_DEBUG, LogLevel_Closed::LOG_LEVEL_VERBOSE, LogLevel_Closed::LOG_LEVEL_VERY_VERBOSE];
}
impl ::std::default::Default for LogLevel_Closed {
  fn default() -> Self {
    LogLevel_Closed::LOG_LEVEL_NONE
  }
}
impl From<LogLevel_Closed> for i32 {
  fn from(v: LogLevel_Closed) -> i32 {
    match v {
      LogLevel_Closed::LOG_LEVEL_NONE => 0,
      LogLevel_Closed::LOG_LEVEL_ERROR => 1,
      LogLevel_Closed::LOG_LEVEL_WARN => 2,
      LogLevel_Closed::LOG_LEVEL_INFO => 3,
      LogLevel_Closed::LOG_LEVEL_DEBUG => 4,
      LogLevel_Closed::LOG_LEVEL_VERBOSE => 5,
      LogLevel_Closed::LOG_LEVEL_VERY_VERBOSE => 6,
    }
  }
}
impl ::std::convert::TryFrom<i32> for LogLevel_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(LogLevel_Closed::LOG_LEVEL_NONE),
      1 => Ok(LogLevel_Closed::LOG_LEVEL_ERROR),
      2 => Ok(LogLevel_Closed::LOG_LEVEL_WARN),
      3 => Ok(LogLevel_Closed::LOG_LEVEL_INFO),
      4 => Ok(LogLevel_Closed::LOG_LEVEL_DEBUG),
      5 => Ok(LogLevel_Closed::LOG_LEVEL_VERBOSE),
      6 => Ok(LogLevel_Closed::LOG_LEVEL_VERY_VERBOSE),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for LogLevel_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for LogLevel_Closed {
  fn name(self) -> &'static str {
    match self {
      LogLevel_Closed::LOG_LEVEL_NONE => "LOG_LEVEL_NONE",
      LogLevel_Closed::LOG_LEVEL_ERROR => "LOG_LEVEL_ERROR",
      LogLevel_Closed::LOG_LEVEL_WARN => "LOG_LEVEL_WARN",
      LogLevel_Closed::LOG_LEVEL_INFO => "LOG_LEVEL_INFO",
      LogLevel_Closed::LOG_LEVEL_DEBUG => "LOG_LEVEL_DEBUG",
      LogLevel_Closed::LOG_LEVEL_VERBOSE => "LOG_LEVEL_VERBOSE",
      LogLevel_Closed::LOG_LEVEL_VERY_VERBOSE => "LOG_LEVEL_VERY_VERBOSE",
    }
  }
}

/// ==================== USER-DEFINES SERVICES ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ServiceArgType(i32);
impl ServiceArgType {
  pub const SERVICE_ARG_TYPE_BOOL: ServiceArgType = ServiceArgType(0);
  pub const SERVICE_ARG_TYPE_INT: ServiceArgType = ServiceArgType(1);
  pub const SERVICE_ARG_TYPE_FLOAT: ServiceArgType = ServiceArgType(2);
  pub const SERVICE_ARG_TYPE_STRING: ServiceArgType = ServiceArgType(3);
  pub const SERVICE_ARG_TYPE_BOOL_ARRAY: ServiceArgType = ServiceArgType(4);
  pub const SERVICE_ARG_TYPE_INT_ARRAY: ServiceArgType = ServiceArgType(5);
  pub const SERVICE_ARG_TYPE_FLOAT_ARRAY: ServiceArgType = ServiceArgType(6);
  pub const SERVICE_ARG_TYPE_STRING_ARRAY: ServiceArgType = ServiceArgType(7);
  pub const KNOWN_VARIANTS: [ServiceArgType; 8] = [ServiceArgType::SERVICE_ARG_TYPE_BOOL, ServiceArgType::SERVICE_ARG_TYPE_INT, ServiceArgType::SERVICE_ARG_TYPE_FLOAT, ServiceArgType::SERVICE_ARG_TYPE_STRING, ServiceArgType::SERVICE_ARG_TYPE_BOOL_ARRAY, ServiceArgType::SERVICE_ARG_TYPE_INT_ARRAY, ServiceArgType::SERVICE_ARG_TYPE_FLOAT_ARRAY, ServiceArgType::SERVICE_ARG_TYPE_STRING_ARRAY];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<ServiceArgType_Closed> {
    match self {
      ServiceArgType::SERVICE_ARG_TYPE_BOOL => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL),
      ServiceArgType::SERVICE_ARG_TYPE_INT => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_INT),
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT),
      ServiceArgType::SERVICE_ARG_TYPE_STRING => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING),
      ServiceArgType::SERVICE_ARG_TYPE_BOOL_ARRAY => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL_ARRAY),
      ServiceArgType::SERVICE_ARG_TYPE_INT_ARRAY => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_INT_ARRAY),
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT_ARRAY => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT_ARRAY),
      ServiceArgType::SERVICE_ARG_TYPE_STRING_ARRAY => Some(ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING_ARRAY),
      _ => None,
    }
  }
}
impl ::std::default::Default for ServiceArgType {
  fn default() -> Self {
    ServiceArgType::SERVICE_ARG_TYPE_BOOL
  }
}
impl From<ServiceArgType> for i32 {
  fn from(v: ServiceArgType) -> i32 {
    v.0
  }
}
impl From<i32> for ServiceArgType {
  fn from(v: i32) -> ServiceArgType {
    ServiceArgType(v)
  }
}
impl From<ServiceArgType_Closed> for ServiceArgType {
  fn from(v: ServiceArgType_Closed) -> ServiceArgType {
    ServiceArgType(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ServiceArgType {
}
impl ::pb_jelly::OpenProtoEnum for ServiceArgType {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      ServiceArgType::SERVICE_ARG_TYPE_BOOL => Some("SERVICE_ARG_TYPE_BOOL"),
      ServiceArgType::SERVICE_ARG_TYPE_INT => Some("SERVICE_ARG_TYPE_INT"),
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT => Some("SERVICE_ARG_TYPE_FLOAT"),
      ServiceArgType::SERVICE_ARG_TYPE_STRING => Some("SERVICE_ARG_TYPE_STRING"),
      ServiceArgType::SERVICE_ARG_TYPE_BOOL_ARRAY => Some("SERVICE_ARG_TYPE_BOOL_ARRAY"),
      ServiceArgType::SERVICE_ARG_TYPE_INT_ARRAY => Some("SERVICE_ARG_TYPE_INT_ARRAY"),
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT_ARRAY => Some("SERVICE_ARG_TYPE_FLOAT_ARRAY"),
      ServiceArgType::SERVICE_ARG_TYPE_STRING_ARRAY => Some("SERVICE_ARG_TYPE_STRING_ARRAY"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      ServiceArgType::SERVICE_ARG_TYPE_BOOL => true,
      ServiceArgType::SERVICE_ARG_TYPE_INT => true,
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT => true,
      ServiceArgType::SERVICE_ARG_TYPE_STRING => true,
      ServiceArgType::SERVICE_ARG_TYPE_BOOL_ARRAY => true,
      ServiceArgType::SERVICE_ARG_TYPE_INT_ARRAY => true,
      ServiceArgType::SERVICE_ARG_TYPE_FLOAT_ARRAY => true,
      ServiceArgType::SERVICE_ARG_TYPE_STRING_ARRAY => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for ServiceArgType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// ==================== USER-DEFINES SERVICES ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ServiceArgType_Closed {
  SERVICE_ARG_TYPE_BOOL = 0,
  SERVICE_ARG_TYPE_INT = 1,
  SERVICE_ARG_TYPE_FLOAT = 2,
  SERVICE_ARG_TYPE_STRING = 3,
  SERVICE_ARG_TYPE_BOOL_ARRAY = 4,
  SERVICE_ARG_TYPE_INT_ARRAY = 5,
  SERVICE_ARG_TYPE_FLOAT_ARRAY = 6,
  SERVICE_ARG_TYPE_STRING_ARRAY = 7,
}
impl ServiceArgType_Closed {
  pub const KNOWN_VARIANTS: [ServiceArgType_Closed; 8] = [ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL, ServiceArgType_Closed::SERVICE_ARG_TYPE_INT, ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT, ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING, ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL_ARRAY, ServiceArgType_Closed::SERVICE_ARG_TYPE_INT_ARRAY, ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT_ARRAY, ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING_ARRAY];
}
impl ::std::default::Default for ServiceArgType_Closed {
  fn default() -> Self {
    ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL
  }
}
impl From<ServiceArgType_Closed> for i32 {
  fn from(v: ServiceArgType_Closed) -> i32 {
    match v {
      ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL => 0,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_INT => 1,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT => 2,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING => 3,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL_ARRAY => 4,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_INT_ARRAY => 5,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT_ARRAY => 6,
      ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING_ARRAY => 7,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ServiceArgType_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL),
      1 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_INT),
      2 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT),
      3 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING),
      4 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL_ARRAY),
      5 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_INT_ARRAY),
      6 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT_ARRAY),
      7 => Ok(ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING_ARRAY),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ServiceArgType_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ServiceArgType_Closed {
  fn name(self) -> &'static str {
    match self {
      ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL => "SERVICE_ARG_TYPE_BOOL",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_INT => "SERVICE_ARG_TYPE_INT",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT => "SERVICE_ARG_TYPE_FLOAT",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING => "SERVICE_ARG_TYPE_STRING",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_BOOL_ARRAY => "SERVICE_ARG_TYPE_BOOL_ARRAY",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_INT_ARRAY => "SERVICE_ARG_TYPE_INT_ARRAY",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_FLOAT_ARRAY => "SERVICE_ARG_TYPE_FLOAT_ARRAY",
      ServiceArgType_Closed::SERVICE_ARG_TYPE_STRING_ARRAY => "SERVICE_ARG_TYPE_STRING_ARRAY",
    }
  }
}

/// ==================== CLIMATE ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClimateMode(i32);
impl ClimateMode {
  pub const CLIMATE_MODE_OFF: ClimateMode = ClimateMode(0);
  pub const CLIMATE_MODE_AUTO: ClimateMode = ClimateMode(1);
  pub const CLIMATE_MODE_COOL: ClimateMode = ClimateMode(2);
  pub const CLIMATE_MODE_HEAT: ClimateMode = ClimateMode(3);
  pub const CLIMATE_MODE_FAN_ONLY: ClimateMode = ClimateMode(4);
  pub const CLIMATE_MODE_DRY: ClimateMode = ClimateMode(5);
  pub const KNOWN_VARIANTS: [ClimateMode; 6] = [ClimateMode::CLIMATE_MODE_OFF, ClimateMode::CLIMATE_MODE_AUTO, ClimateMode::CLIMATE_MODE_COOL, ClimateMode::CLIMATE_MODE_HEAT, ClimateMode::CLIMATE_MODE_FAN_ONLY, ClimateMode::CLIMATE_MODE_DRY];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<ClimateMode_Closed> {
    match self {
      ClimateMode::CLIMATE_MODE_OFF => Some(ClimateMode_Closed::CLIMATE_MODE_OFF),
      ClimateMode::CLIMATE_MODE_AUTO => Some(ClimateMode_Closed::CLIMATE_MODE_AUTO),
      ClimateMode::CLIMATE_MODE_COOL => Some(ClimateMode_Closed::CLIMATE_MODE_COOL),
      ClimateMode::CLIMATE_MODE_HEAT => Some(ClimateMode_Closed::CLIMATE_MODE_HEAT),
      ClimateMode::CLIMATE_MODE_FAN_ONLY => Some(ClimateMode_Closed::CLIMATE_MODE_FAN_ONLY),
      ClimateMode::CLIMATE_MODE_DRY => Some(ClimateMode_Closed::CLIMATE_MODE_DRY),
      _ => None,
    }
  }
}
impl ::std::default::Default for ClimateMode {
  fn default() -> Self {
    ClimateMode::CLIMATE_MODE_OFF
  }
}
impl From<ClimateMode> for i32 {
  fn from(v: ClimateMode) -> i32 {
    v.0
  }
}
impl From<i32> for ClimateMode {
  fn from(v: i32) -> ClimateMode {
    ClimateMode(v)
  }
}
impl From<ClimateMode_Closed> for ClimateMode {
  fn from(v: ClimateMode_Closed) -> ClimateMode {
    ClimateMode(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ClimateMode {
}
impl ::pb_jelly::OpenProtoEnum for ClimateMode {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      ClimateMode::CLIMATE_MODE_OFF => Some("CLIMATE_MODE_OFF"),
      ClimateMode::CLIMATE_MODE_AUTO => Some("CLIMATE_MODE_AUTO"),
      ClimateMode::CLIMATE_MODE_COOL => Some("CLIMATE_MODE_COOL"),
      ClimateMode::CLIMATE_MODE_HEAT => Some("CLIMATE_MODE_HEAT"),
      ClimateMode::CLIMATE_MODE_FAN_ONLY => Some("CLIMATE_MODE_FAN_ONLY"),
      ClimateMode::CLIMATE_MODE_DRY => Some("CLIMATE_MODE_DRY"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      ClimateMode::CLIMATE_MODE_OFF => true,
      ClimateMode::CLIMATE_MODE_AUTO => true,
      ClimateMode::CLIMATE_MODE_COOL => true,
      ClimateMode::CLIMATE_MODE_HEAT => true,
      ClimateMode::CLIMATE_MODE_FAN_ONLY => true,
      ClimateMode::CLIMATE_MODE_DRY => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for ClimateMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// ==================== CLIMATE ====================
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ClimateMode_Closed {
  CLIMATE_MODE_OFF = 0,
  CLIMATE_MODE_AUTO = 1,
  CLIMATE_MODE_COOL = 2,
  CLIMATE_MODE_HEAT = 3,
  CLIMATE_MODE_FAN_ONLY = 4,
  CLIMATE_MODE_DRY = 5,
}
impl ClimateMode_Closed {
  pub const KNOWN_VARIANTS: [ClimateMode_Closed; 6] = [ClimateMode_Closed::CLIMATE_MODE_OFF, ClimateMode_Closed::CLIMATE_MODE_AUTO, ClimateMode_Closed::CLIMATE_MODE_COOL, ClimateMode_Closed::CLIMATE_MODE_HEAT, ClimateMode_Closed::CLIMATE_MODE_FAN_ONLY, ClimateMode_Closed::CLIMATE_MODE_DRY];
}
impl ::std::default::Default for ClimateMode_Closed {
  fn default() -> Self {
    ClimateMode_Closed::CLIMATE_MODE_OFF
  }
}
impl From<ClimateMode_Closed> for i32 {
  fn from(v: ClimateMode_Closed) -> i32 {
    match v {
      ClimateMode_Closed::CLIMATE_MODE_OFF => 0,
      ClimateMode_Closed::CLIMATE_MODE_AUTO => 1,
      ClimateMode_Closed::CLIMATE_MODE_COOL => 2,
      ClimateMode_Closed::CLIMATE_MODE_HEAT => 3,
      ClimateMode_Closed::CLIMATE_MODE_FAN_ONLY => 4,
      ClimateMode_Closed::CLIMATE_MODE_DRY => 5,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ClimateMode_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ClimateMode_Closed::CLIMATE_MODE_OFF),
      1 => Ok(ClimateMode_Closed::CLIMATE_MODE_AUTO),
      2 => Ok(ClimateMode_Closed::CLIMATE_MODE_COOL),
      3 => Ok(ClimateMode_Closed::CLIMATE_MODE_HEAT),
      4 => Ok(ClimateMode_Closed::CLIMATE_MODE_FAN_ONLY),
      5 => Ok(ClimateMode_Closed::CLIMATE_MODE_DRY),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ClimateMode_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ClimateMode_Closed {
  fn name(self) -> &'static str {
    match self {
      ClimateMode_Closed::CLIMATE_MODE_OFF => "CLIMATE_MODE_OFF",
      ClimateMode_Closed::CLIMATE_MODE_AUTO => "CLIMATE_MODE_AUTO",
      ClimateMode_Closed::CLIMATE_MODE_COOL => "CLIMATE_MODE_COOL",
      ClimateMode_Closed::CLIMATE_MODE_HEAT => "CLIMATE_MODE_HEAT",
      ClimateMode_Closed::CLIMATE_MODE_FAN_ONLY => "CLIMATE_MODE_FAN_ONLY",
      ClimateMode_Closed::CLIMATE_MODE_DRY => "CLIMATE_MODE_DRY",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClimateFanMode(i32);
impl ClimateFanMode {
  pub const CLIMATE_FAN_ON: ClimateFanMode = ClimateFanMode(0);
  pub const CLIMATE_FAN_OFF: ClimateFanMode = ClimateFanMode(1);
  pub const CLIMATE_FAN_AUTO: ClimateFanMode = ClimateFanMode(2);
  pub const CLIMATE_FAN_LOW: ClimateFanMode = ClimateFanMode(3);
  pub const CLIMATE_FAN_MEDIUM: ClimateFanMode = ClimateFanMode(4);
  pub const CLIMATE_FAN_HIGH: ClimateFanMode = ClimateFanMode(5);
  pub const CLIMATE_FAN_MIDDLE: ClimateFanMode = ClimateFanMode(6);
  pub const CLIMATE_FAN_FOCUS: ClimateFanMode = ClimateFanMode(7);
  pub const CLIMATE_FAN_DIFFUSE: ClimateFanMode = ClimateFanMode(8);
  pub const KNOWN_VARIANTS: [ClimateFanMode; 9] = [ClimateFanMode::CLIMATE_FAN_ON, ClimateFanMode::CLIMATE_FAN_OFF, ClimateFanMode::CLIMATE_FAN_AUTO, ClimateFanMode::CLIMATE_FAN_LOW, ClimateFanMode::CLIMATE_FAN_MEDIUM, ClimateFanMode::CLIMATE_FAN_HIGH, ClimateFanMode::CLIMATE_FAN_MIDDLE, ClimateFanMode::CLIMATE_FAN_FOCUS, ClimateFanMode::CLIMATE_FAN_DIFFUSE];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<ClimateFanMode_Closed> {
    match self {
      ClimateFanMode::CLIMATE_FAN_ON => Some(ClimateFanMode_Closed::CLIMATE_FAN_ON),
      ClimateFanMode::CLIMATE_FAN_OFF => Some(ClimateFanMode_Closed::CLIMATE_FAN_OFF),
      ClimateFanMode::CLIMATE_FAN_AUTO => Some(ClimateFanMode_Closed::CLIMATE_FAN_AUTO),
      ClimateFanMode::CLIMATE_FAN_LOW => Some(ClimateFanMode_Closed::CLIMATE_FAN_LOW),
      ClimateFanMode::CLIMATE_FAN_MEDIUM => Some(ClimateFanMode_Closed::CLIMATE_FAN_MEDIUM),
      ClimateFanMode::CLIMATE_FAN_HIGH => Some(ClimateFanMode_Closed::CLIMATE_FAN_HIGH),
      ClimateFanMode::CLIMATE_FAN_MIDDLE => Some(ClimateFanMode_Closed::CLIMATE_FAN_MIDDLE),
      ClimateFanMode::CLIMATE_FAN_FOCUS => Some(ClimateFanMode_Closed::CLIMATE_FAN_FOCUS),
      ClimateFanMode::CLIMATE_FAN_DIFFUSE => Some(ClimateFanMode_Closed::CLIMATE_FAN_DIFFUSE),
      _ => None,
    }
  }
}
impl ::std::default::Default for ClimateFanMode {
  fn default() -> Self {
    ClimateFanMode::CLIMATE_FAN_ON
  }
}
impl From<ClimateFanMode> for i32 {
  fn from(v: ClimateFanMode) -> i32 {
    v.0
  }
}
impl From<i32> for ClimateFanMode {
  fn from(v: i32) -> ClimateFanMode {
    ClimateFanMode(v)
  }
}
impl From<ClimateFanMode_Closed> for ClimateFanMode {
  fn from(v: ClimateFanMode_Closed) -> ClimateFanMode {
    ClimateFanMode(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ClimateFanMode {
}
impl ::pb_jelly::OpenProtoEnum for ClimateFanMode {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      ClimateFanMode::CLIMATE_FAN_ON => Some("CLIMATE_FAN_ON"),
      ClimateFanMode::CLIMATE_FAN_OFF => Some("CLIMATE_FAN_OFF"),
      ClimateFanMode::CLIMATE_FAN_AUTO => Some("CLIMATE_FAN_AUTO"),
      ClimateFanMode::CLIMATE_FAN_LOW => Some("CLIMATE_FAN_LOW"),
      ClimateFanMode::CLIMATE_FAN_MEDIUM => Some("CLIMATE_FAN_MEDIUM"),
      ClimateFanMode::CLIMATE_FAN_HIGH => Some("CLIMATE_FAN_HIGH"),
      ClimateFanMode::CLIMATE_FAN_MIDDLE => Some("CLIMATE_FAN_MIDDLE"),
      ClimateFanMode::CLIMATE_FAN_FOCUS => Some("CLIMATE_FAN_FOCUS"),
      ClimateFanMode::CLIMATE_FAN_DIFFUSE => Some("CLIMATE_FAN_DIFFUSE"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      ClimateFanMode::CLIMATE_FAN_ON => true,
      ClimateFanMode::CLIMATE_FAN_OFF => true,
      ClimateFanMode::CLIMATE_FAN_AUTO => true,
      ClimateFanMode::CLIMATE_FAN_LOW => true,
      ClimateFanMode::CLIMATE_FAN_MEDIUM => true,
      ClimateFanMode::CLIMATE_FAN_HIGH => true,
      ClimateFanMode::CLIMATE_FAN_MIDDLE => true,
      ClimateFanMode::CLIMATE_FAN_FOCUS => true,
      ClimateFanMode::CLIMATE_FAN_DIFFUSE => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for ClimateFanMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ClimateFanMode_Closed {
  CLIMATE_FAN_ON = 0,
  CLIMATE_FAN_OFF = 1,
  CLIMATE_FAN_AUTO = 2,
  CLIMATE_FAN_LOW = 3,
  CLIMATE_FAN_MEDIUM = 4,
  CLIMATE_FAN_HIGH = 5,
  CLIMATE_FAN_MIDDLE = 6,
  CLIMATE_FAN_FOCUS = 7,
  CLIMATE_FAN_DIFFUSE = 8,
}
impl ClimateFanMode_Closed {
  pub const KNOWN_VARIANTS: [ClimateFanMode_Closed; 9] = [ClimateFanMode_Closed::CLIMATE_FAN_ON, ClimateFanMode_Closed::CLIMATE_FAN_OFF, ClimateFanMode_Closed::CLIMATE_FAN_AUTO, ClimateFanMode_Closed::CLIMATE_FAN_LOW, ClimateFanMode_Closed::CLIMATE_FAN_MEDIUM, ClimateFanMode_Closed::CLIMATE_FAN_HIGH, ClimateFanMode_Closed::CLIMATE_FAN_MIDDLE, ClimateFanMode_Closed::CLIMATE_FAN_FOCUS, ClimateFanMode_Closed::CLIMATE_FAN_DIFFUSE];
}
impl ::std::default::Default for ClimateFanMode_Closed {
  fn default() -> Self {
    ClimateFanMode_Closed::CLIMATE_FAN_ON
  }
}
impl From<ClimateFanMode_Closed> for i32 {
  fn from(v: ClimateFanMode_Closed) -> i32 {
    match v {
      ClimateFanMode_Closed::CLIMATE_FAN_ON => 0,
      ClimateFanMode_Closed::CLIMATE_FAN_OFF => 1,
      ClimateFanMode_Closed::CLIMATE_FAN_AUTO => 2,
      ClimateFanMode_Closed::CLIMATE_FAN_LOW => 3,
      ClimateFanMode_Closed::CLIMATE_FAN_MEDIUM => 4,
      ClimateFanMode_Closed::CLIMATE_FAN_HIGH => 5,
      ClimateFanMode_Closed::CLIMATE_FAN_MIDDLE => 6,
      ClimateFanMode_Closed::CLIMATE_FAN_FOCUS => 7,
      ClimateFanMode_Closed::CLIMATE_FAN_DIFFUSE => 8,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ClimateFanMode_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_ON),
      1 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_OFF),
      2 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_AUTO),
      3 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_LOW),
      4 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_MEDIUM),
      5 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_HIGH),
      6 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_MIDDLE),
      7 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_FOCUS),
      8 => Ok(ClimateFanMode_Closed::CLIMATE_FAN_DIFFUSE),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ClimateFanMode_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ClimateFanMode_Closed {
  fn name(self) -> &'static str {
    match self {
      ClimateFanMode_Closed::CLIMATE_FAN_ON => "CLIMATE_FAN_ON",
      ClimateFanMode_Closed::CLIMATE_FAN_OFF => "CLIMATE_FAN_OFF",
      ClimateFanMode_Closed::CLIMATE_FAN_AUTO => "CLIMATE_FAN_AUTO",
      ClimateFanMode_Closed::CLIMATE_FAN_LOW => "CLIMATE_FAN_LOW",
      ClimateFanMode_Closed::CLIMATE_FAN_MEDIUM => "CLIMATE_FAN_MEDIUM",
      ClimateFanMode_Closed::CLIMATE_FAN_HIGH => "CLIMATE_FAN_HIGH",
      ClimateFanMode_Closed::CLIMATE_FAN_MIDDLE => "CLIMATE_FAN_MIDDLE",
      ClimateFanMode_Closed::CLIMATE_FAN_FOCUS => "CLIMATE_FAN_FOCUS",
      ClimateFanMode_Closed::CLIMATE_FAN_DIFFUSE => "CLIMATE_FAN_DIFFUSE",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClimateSwingMode(i32);
impl ClimateSwingMode {
  pub const CLIMATE_SWING_OFF: ClimateSwingMode = ClimateSwingMode(0);
  pub const CLIMATE_SWING_BOTH: ClimateSwingMode = ClimateSwingMode(1);
  pub const CLIMATE_SWING_VERTICAL: ClimateSwingMode = ClimateSwingMode(2);
  pub const CLIMATE_SWINT_HORIZONTAL: ClimateSwingMode = ClimateSwingMode(3);
  pub const KNOWN_VARIANTS: [ClimateSwingMode; 4] = [ClimateSwingMode::CLIMATE_SWING_OFF, ClimateSwingMode::CLIMATE_SWING_BOTH, ClimateSwingMode::CLIMATE_SWING_VERTICAL, ClimateSwingMode::CLIMATE_SWINT_HORIZONTAL];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<ClimateSwingMode_Closed> {
    match self {
      ClimateSwingMode::CLIMATE_SWING_OFF => Some(ClimateSwingMode_Closed::CLIMATE_SWING_OFF),
      ClimateSwingMode::CLIMATE_SWING_BOTH => Some(ClimateSwingMode_Closed::CLIMATE_SWING_BOTH),
      ClimateSwingMode::CLIMATE_SWING_VERTICAL => Some(ClimateSwingMode_Closed::CLIMATE_SWING_VERTICAL),
      ClimateSwingMode::CLIMATE_SWINT_HORIZONTAL => Some(ClimateSwingMode_Closed::CLIMATE_SWINT_HORIZONTAL),
      _ => None,
    }
  }
}
impl ::std::default::Default for ClimateSwingMode {
  fn default() -> Self {
    ClimateSwingMode::CLIMATE_SWING_OFF
  }
}
impl From<ClimateSwingMode> for i32 {
  fn from(v: ClimateSwingMode) -> i32 {
    v.0
  }
}
impl From<i32> for ClimateSwingMode {
  fn from(v: i32) -> ClimateSwingMode {
    ClimateSwingMode(v)
  }
}
impl From<ClimateSwingMode_Closed> for ClimateSwingMode {
  fn from(v: ClimateSwingMode_Closed) -> ClimateSwingMode {
    ClimateSwingMode(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ClimateSwingMode {
}
impl ::pb_jelly::OpenProtoEnum for ClimateSwingMode {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      ClimateSwingMode::CLIMATE_SWING_OFF => Some("CLIMATE_SWING_OFF"),
      ClimateSwingMode::CLIMATE_SWING_BOTH => Some("CLIMATE_SWING_BOTH"),
      ClimateSwingMode::CLIMATE_SWING_VERTICAL => Some("CLIMATE_SWING_VERTICAL"),
      ClimateSwingMode::CLIMATE_SWINT_HORIZONTAL => Some("CLIMATE_SWINT_HORIZONTAL"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      ClimateSwingMode::CLIMATE_SWING_OFF => true,
      ClimateSwingMode::CLIMATE_SWING_BOTH => true,
      ClimateSwingMode::CLIMATE_SWING_VERTICAL => true,
      ClimateSwingMode::CLIMATE_SWINT_HORIZONTAL => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for ClimateSwingMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ClimateSwingMode_Closed {
  CLIMATE_SWING_OFF = 0,
  CLIMATE_SWING_BOTH = 1,
  CLIMATE_SWING_VERTICAL = 2,
  CLIMATE_SWINT_HORIZONTAL = 3,
}
impl ClimateSwingMode_Closed {
  pub const KNOWN_VARIANTS: [ClimateSwingMode_Closed; 4] = [ClimateSwingMode_Closed::CLIMATE_SWING_OFF, ClimateSwingMode_Closed::CLIMATE_SWING_BOTH, ClimateSwingMode_Closed::CLIMATE_SWING_VERTICAL, ClimateSwingMode_Closed::CLIMATE_SWINT_HORIZONTAL];
}
impl ::std::default::Default for ClimateSwingMode_Closed {
  fn default() -> Self {
    ClimateSwingMode_Closed::CLIMATE_SWING_OFF
  }
}
impl From<ClimateSwingMode_Closed> for i32 {
  fn from(v: ClimateSwingMode_Closed) -> i32 {
    match v {
      ClimateSwingMode_Closed::CLIMATE_SWING_OFF => 0,
      ClimateSwingMode_Closed::CLIMATE_SWING_BOTH => 1,
      ClimateSwingMode_Closed::CLIMATE_SWING_VERTICAL => 2,
      ClimateSwingMode_Closed::CLIMATE_SWINT_HORIZONTAL => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ClimateSwingMode_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ClimateSwingMode_Closed::CLIMATE_SWING_OFF),
      1 => Ok(ClimateSwingMode_Closed::CLIMATE_SWING_BOTH),
      2 => Ok(ClimateSwingMode_Closed::CLIMATE_SWING_VERTICAL),
      3 => Ok(ClimateSwingMode_Closed::CLIMATE_SWINT_HORIZONTAL),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ClimateSwingMode_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ClimateSwingMode_Closed {
  fn name(self) -> &'static str {
    match self {
      ClimateSwingMode_Closed::CLIMATE_SWING_OFF => "CLIMATE_SWING_OFF",
      ClimateSwingMode_Closed::CLIMATE_SWING_BOTH => "CLIMATE_SWING_BOTH",
      ClimateSwingMode_Closed::CLIMATE_SWING_VERTICAL => "CLIMATE_SWING_VERTICAL",
      ClimateSwingMode_Closed::CLIMATE_SWINT_HORIZONTAL => "CLIMATE_SWINT_HORIZONTAL",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClimateAction(i32);
impl ClimateAction {
  pub const CLIMATE_ACTION_OFF: ClimateAction = ClimateAction(0);
  /// values same as mode for readability
  pub const CLIMATE_ACTION_COOLING: ClimateAction = ClimateAction(2);
  pub const CLIMATE_ACTION_HEATING: ClimateAction = ClimateAction(3);
  pub const CLIMATE_ACTION_IDLE: ClimateAction = ClimateAction(4);
  pub const CLIMATE_ACTION_DRYING: ClimateAction = ClimateAction(5);
  pub const CLIMATE_ACTION_FAN: ClimateAction = ClimateAction(6);
  pub const KNOWN_VARIANTS: [ClimateAction; 6] = [ClimateAction::CLIMATE_ACTION_OFF, ClimateAction::CLIMATE_ACTION_COOLING, ClimateAction::CLIMATE_ACTION_HEATING, ClimateAction::CLIMATE_ACTION_IDLE, ClimateAction::CLIMATE_ACTION_DRYING, ClimateAction::CLIMATE_ACTION_FAN];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<ClimateAction_Closed> {
    match self {
      ClimateAction::CLIMATE_ACTION_OFF => Some(ClimateAction_Closed::CLIMATE_ACTION_OFF),
      ClimateAction::CLIMATE_ACTION_COOLING => Some(ClimateAction_Closed::CLIMATE_ACTION_COOLING),
      ClimateAction::CLIMATE_ACTION_HEATING => Some(ClimateAction_Closed::CLIMATE_ACTION_HEATING),
      ClimateAction::CLIMATE_ACTION_IDLE => Some(ClimateAction_Closed::CLIMATE_ACTION_IDLE),
      ClimateAction::CLIMATE_ACTION_DRYING => Some(ClimateAction_Closed::CLIMATE_ACTION_DRYING),
      ClimateAction::CLIMATE_ACTION_FAN => Some(ClimateAction_Closed::CLIMATE_ACTION_FAN),
      _ => None,
    }
  }
}
impl ::std::default::Default for ClimateAction {
  fn default() -> Self {
    ClimateAction::CLIMATE_ACTION_OFF
  }
}
impl From<ClimateAction> for i32 {
  fn from(v: ClimateAction) -> i32 {
    v.0
  }
}
impl From<i32> for ClimateAction {
  fn from(v: i32) -> ClimateAction {
    ClimateAction(v)
  }
}
impl From<ClimateAction_Closed> for ClimateAction {
  fn from(v: ClimateAction_Closed) -> ClimateAction {
    ClimateAction(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ClimateAction {
}
impl ::pb_jelly::OpenProtoEnum for ClimateAction {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      ClimateAction::CLIMATE_ACTION_OFF => Some("CLIMATE_ACTION_OFF"),
      ClimateAction::CLIMATE_ACTION_COOLING => Some("CLIMATE_ACTION_COOLING"),
      ClimateAction::CLIMATE_ACTION_HEATING => Some("CLIMATE_ACTION_HEATING"),
      ClimateAction::CLIMATE_ACTION_IDLE => Some("CLIMATE_ACTION_IDLE"),
      ClimateAction::CLIMATE_ACTION_DRYING => Some("CLIMATE_ACTION_DRYING"),
      ClimateAction::CLIMATE_ACTION_FAN => Some("CLIMATE_ACTION_FAN"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      ClimateAction::CLIMATE_ACTION_OFF => true,
      ClimateAction::CLIMATE_ACTION_COOLING => true,
      ClimateAction::CLIMATE_ACTION_HEATING => true,
      ClimateAction::CLIMATE_ACTION_IDLE => true,
      ClimateAction::CLIMATE_ACTION_DRYING => true,
      ClimateAction::CLIMATE_ACTION_FAN => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for ClimateAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb_jelly::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ClimateAction_Closed {
  CLIMATE_ACTION_OFF = 0,
  /// values same as mode for readability
  CLIMATE_ACTION_COOLING = 2,
  CLIMATE_ACTION_HEATING = 3,
  CLIMATE_ACTION_IDLE = 4,
  CLIMATE_ACTION_DRYING = 5,
  CLIMATE_ACTION_FAN = 6,
}
impl ClimateAction_Closed {
  pub const KNOWN_VARIANTS: [ClimateAction_Closed; 6] = [ClimateAction_Closed::CLIMATE_ACTION_OFF, ClimateAction_Closed::CLIMATE_ACTION_COOLING, ClimateAction_Closed::CLIMATE_ACTION_HEATING, ClimateAction_Closed::CLIMATE_ACTION_IDLE, ClimateAction_Closed::CLIMATE_ACTION_DRYING, ClimateAction_Closed::CLIMATE_ACTION_FAN];
}
impl ::std::default::Default for ClimateAction_Closed {
  fn default() -> Self {
    ClimateAction_Closed::CLIMATE_ACTION_OFF
  }
}
impl From<ClimateAction_Closed> for i32 {
  fn from(v: ClimateAction_Closed) -> i32 {
    match v {
      ClimateAction_Closed::CLIMATE_ACTION_OFF => 0,
      ClimateAction_Closed::CLIMATE_ACTION_COOLING => 2,
      ClimateAction_Closed::CLIMATE_ACTION_HEATING => 3,
      ClimateAction_Closed::CLIMATE_ACTION_IDLE => 4,
      ClimateAction_Closed::CLIMATE_ACTION_DRYING => 5,
      ClimateAction_Closed::CLIMATE_ACTION_FAN => 6,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ClimateAction_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ClimateAction_Closed::CLIMATE_ACTION_OFF),
      2 => Ok(ClimateAction_Closed::CLIMATE_ACTION_COOLING),
      3 => Ok(ClimateAction_Closed::CLIMATE_ACTION_HEATING),
      4 => Ok(ClimateAction_Closed::CLIMATE_ACTION_IDLE),
      5 => Ok(ClimateAction_Closed::CLIMATE_ACTION_DRYING),
      6 => Ok(ClimateAction_Closed::CLIMATE_ACTION_FAN),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ClimateAction_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ClimateAction_Closed {
  fn name(self) -> &'static str {
    match self {
      ClimateAction_Closed::CLIMATE_ACTION_OFF => "CLIMATE_ACTION_OFF",
      ClimateAction_Closed::CLIMATE_ACTION_COOLING => "CLIMATE_ACTION_COOLING",
      ClimateAction_Closed::CLIMATE_ACTION_HEATING => "CLIMATE_ACTION_HEATING",
      ClimateAction_Closed::CLIMATE_ACTION_IDLE => "CLIMATE_ACTION_IDLE",
      ClimateAction_Closed::CLIMATE_ACTION_DRYING => "CLIMATE_ACTION_DRYING",
      ClimateAction_Closed::CLIMATE_ACTION_FAN => "CLIMATE_ACTION_FAN",
    }
  }
}

// ==================== BASE PACKETS ====================

// The Home Assistant protocol is structured as a simple
// TCP socket with short binary messages encoded in the protocol buffers format
// First, a message in this protocol has a specific format:
//  * VarInt denoting the size of the message object. (type is not part of this)
//  * VarInt denoting the type of message.
//  * The message object encoded as a ProtoBuf message

// The connection is established in 4 steps:
//  * First, the client connects to the server and sends a "Hello Request" identifying itself
//  * The server responds with a "Hello Response" and selects the protocol version
//  * After receiving this message, the client attempts to authenticate itself using
//    the password and a "Connect Request"
//  * The server responds with a "Connect Response" and notifies of invalid password.
// If anything in this initial process fails, the connection must immediately closed
// by both sides and _no_ disconnection message is to be sent.

/// Message sent at the beginning of each connection
/// Can only be sent by the client and only at the beginning of the connection
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloRequest {
  /// Description of client (like User Agent)
  /// For example "Home Assistant"
  /// Not strictly necessary to send but nice for debugging
  /// purposes.
  pub client_info: ::std::string::String,
}
impl ::std::default::Default for HelloRequest {
  fn default() -> Self {
    HelloRequest {
      client_info: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HelloRequest_default: HelloRequest = HelloRequest::default();
}
impl ::pb_jelly::Message for HelloRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut client_info_size = 0;
    if self.client_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.client_info;
      let l = ::pb_jelly::Message::compute_size(val);
      client_info_size += ::pb_jelly::wire_format::serialized_length(1);
      client_info_size += ::pb_jelly::varint::serialized_length(l as u64);
      client_info_size += l;
    }
    size += client_info_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.client_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.client_info;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.client_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.client_info;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HelloRequest", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.client_info = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for HelloRequest {
  const NAME: &'static str = "HelloRequest";
  const FULL_NAME: &'static str = "espapi.HelloRequest";
}

/// Confirmation of successful connection request.
/// Can only be sent by the server and only at the beginning of the connection
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloResponse {
  /// The version of the API to use. The _client_ (for example Home Assistant) needs to check
  /// for compatibility and if necessary adopt to an older API.
  /// Major is for breaking changes in the base protocol - a mismatch will lead to immediate disconnect_client_
  /// Minor is for breaking changes in individual messages - a mismatch will lead to a warning message
  pub api_version_major: u32,
  pub api_version_minor: u32,
  /// A string identifying the server (ESP); like client info this may be empty
  /// and only exists for debugging/logging purposes.
  /// For example "ESPHome v1.10.0 on ESP8266"
  pub server_info: ::std::string::String,
}
impl ::std::default::Default for HelloResponse {
  fn default() -> Self {
    HelloResponse {
      api_version_major: ::std::default::Default::default(),
      api_version_minor: ::std::default::Default::default(),
      server_info: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HelloResponse_default: HelloResponse = HelloResponse::default();
}
impl ::pb_jelly::Message for HelloResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut api_version_major_size = 0;
    if self.api_version_major != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_major;
      let l = ::pb_jelly::Message::compute_size(val);
      api_version_major_size += ::pb_jelly::wire_format::serialized_length(1);
      api_version_major_size += l;
    }
    size += api_version_major_size;
    let mut api_version_minor_size = 0;
    if self.api_version_minor != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_minor;
      let l = ::pb_jelly::Message::compute_size(val);
      api_version_minor_size += ::pb_jelly::wire_format::serialized_length(2);
      api_version_minor_size += l;
    }
    size += api_version_minor_size;
    let mut server_info_size = 0;
    if self.server_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.server_info;
      let l = ::pb_jelly::Message::compute_size(val);
      server_info_size += ::pb_jelly::wire_format::serialized_length(3);
      server_info_size += ::pb_jelly::varint::serialized_length(l as u64);
      server_info_size += l;
    }
    size += server_info_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.api_version_major != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_major;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.api_version_minor != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_minor;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.server_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.server_info;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.api_version_major != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_major;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.api_version_minor != <u32 as ::std::default::Default>::default() {
      let val = &self.api_version_minor;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.server_info != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.server_info;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "HelloResponse", 1)?;
          let mut val: u32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.api_version_major = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "HelloResponse", 2)?;
          let mut val: u32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.api_version_minor = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HelloResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.server_info = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for HelloResponse {
  const NAME: &'static str = "HelloResponse";
  const FULL_NAME: &'static str = "espapi.HelloResponse";
}

/// Message sent at the beginning of each connection to authenticate the client
/// Can only be sent by the client and only at the beginning of the connection
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConnectRequest {
  /// The password to log in with
  pub password: ::std::string::String,
}
impl ::std::default::Default for ConnectRequest {
  fn default() -> Self {
    ConnectRequest {
      password: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ConnectRequest_default: ConnectRequest = ConnectRequest::default();
}
impl ::pb_jelly::Message for ConnectRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut password_size = 0;
    if self.password != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.password;
      let l = ::pb_jelly::Message::compute_size(val);
      password_size += ::pb_jelly::wire_format::serialized_length(1);
      password_size += ::pb_jelly::varint::serialized_length(l as u64);
      password_size += l;
    }
    size += password_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.password != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.password;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.password != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.password;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ConnectRequest", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ConnectRequest {
  const NAME: &'static str = "ConnectRequest";
  const FULL_NAME: &'static str = "espapi.ConnectRequest";
}

/// Confirmation of successful connection. After this the connection is available for all traffic.
/// Can only be sent by the server and only at the beginning of the connection
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConnectResponse {
  pub invalid_password: bool,
}
impl ::std::default::Default for ConnectResponse {
  fn default() -> Self {
    ConnectResponse {
      invalid_password: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ConnectResponse_default: ConnectResponse = ConnectResponse::default();
}
impl ::pb_jelly::Message for ConnectResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut invalid_password_size = 0;
    if self.invalid_password != <bool as ::std::default::Default>::default() {
      let val = &self.invalid_password;
      let l = ::pb_jelly::Message::compute_size(val);
      invalid_password_size += ::pb_jelly::wire_format::serialized_length(1);
      invalid_password_size += l;
    }
    size += invalid_password_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.invalid_password != <bool as ::std::default::Default>::default() {
      let val = &self.invalid_password;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.invalid_password != <bool as ::std::default::Default>::default() {
      let val = &self.invalid_password;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ConnectResponse", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.invalid_password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ConnectResponse {
  const NAME: &'static str = "ConnectResponse";
  const FULL_NAME: &'static str = "espapi.ConnectResponse";
}

/// Request to close the connection.
/// Can be sent by both the client and server
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisconnectRequest {
}
impl ::std::default::Default for DisconnectRequest {
  fn default() -> Self {
    DisconnectRequest {
    }
  }
}
lazy_static! {
  pub static ref DisconnectRequest_default: DisconnectRequest = DisconnectRequest::default();
}
impl ::pb_jelly::Message for DisconnectRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for DisconnectRequest {
  const NAME: &'static str = "DisconnectRequest";
  const FULL_NAME: &'static str = "espapi.DisconnectRequest";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisconnectResponse {
}
impl ::std::default::Default for DisconnectResponse {
  fn default() -> Self {
    DisconnectResponse {
    }
  }
}
lazy_static! {
  pub static ref DisconnectResponse_default: DisconnectResponse = DisconnectResponse::default();
}
impl ::pb_jelly::Message for DisconnectResponse {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for DisconnectResponse {
  const NAME: &'static str = "DisconnectResponse";
  const FULL_NAME: &'static str = "espapi.DisconnectResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PingRequest {
}
impl ::std::default::Default for PingRequest {
  fn default() -> Self {
    PingRequest {
    }
  }
}
lazy_static! {
  pub static ref PingRequest_default: PingRequest = PingRequest::default();
}
impl ::pb_jelly::Message for PingRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for PingRequest {
  const NAME: &'static str = "PingRequest";
  const FULL_NAME: &'static str = "espapi.PingRequest";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PingResponse {
}
impl ::std::default::Default for PingResponse {
  fn default() -> Self {
    PingResponse {
    }
  }
}
lazy_static! {
  pub static ref PingResponse_default: PingResponse = PingResponse::default();
}
impl ::pb_jelly::Message for PingResponse {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for PingResponse {
  const NAME: &'static str = "PingResponse";
  const FULL_NAME: &'static str = "espapi.PingResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceInfoRequest {
}
impl ::std::default::Default for DeviceInfoRequest {
  fn default() -> Self {
    DeviceInfoRequest {
    }
  }
}
lazy_static! {
  pub static ref DeviceInfoRequest_default: DeviceInfoRequest = DeviceInfoRequest::default();
}
impl ::pb_jelly::Message for DeviceInfoRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for DeviceInfoRequest {
  const NAME: &'static str = "DeviceInfoRequest";
  const FULL_NAME: &'static str = "espapi.DeviceInfoRequest";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceInfoResponse {
  pub uses_password: bool,
  /// The name of the node, given by "App.set_name()"
  pub name: ::std::string::String,
  /// The mac address of the device. For example "AC:BC:32:89:0E:A9"
  pub mac_address: ::std::string::String,
  /// A string describing the ESPHome version. For example "1.10.0"
  pub esphome_version: ::std::string::String,
  /// A string describing the date of compilation, this is generated by the compiler
  /// and therefore may not be in the same format all the time.
  /// If the user isn't using ESPHome, this will also not be set.
  pub compilation_time: ::std::string::String,
  /// The model of the board. For example NodeMCU
  pub model: ::std::string::String,
  pub has_deep_sleep: bool,
}
impl ::std::default::Default for DeviceInfoResponse {
  fn default() -> Self {
    DeviceInfoResponse {
      uses_password: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      mac_address: ::std::default::Default::default(),
      esphome_version: ::std::default::Default::default(),
      compilation_time: ::std::default::Default::default(),
      model: ::std::default::Default::default(),
      has_deep_sleep: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref DeviceInfoResponse_default: DeviceInfoResponse = DeviceInfoResponse::default();
}
impl ::pb_jelly::Message for DeviceInfoResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut uses_password_size = 0;
    if self.uses_password != <bool as ::std::default::Default>::default() {
      let val = &self.uses_password;
      let l = ::pb_jelly::Message::compute_size(val);
      uses_password_size += ::pb_jelly::wire_format::serialized_length(1);
      uses_password_size += l;
    }
    size += uses_password_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(2);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut mac_address_size = 0;
    if self.mac_address != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.mac_address;
      let l = ::pb_jelly::Message::compute_size(val);
      mac_address_size += ::pb_jelly::wire_format::serialized_length(3);
      mac_address_size += ::pb_jelly::varint::serialized_length(l as u64);
      mac_address_size += l;
    }
    size += mac_address_size;
    let mut esphome_version_size = 0;
    if self.esphome_version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.esphome_version;
      let l = ::pb_jelly::Message::compute_size(val);
      esphome_version_size += ::pb_jelly::wire_format::serialized_length(4);
      esphome_version_size += ::pb_jelly::varint::serialized_length(l as u64);
      esphome_version_size += l;
    }
    size += esphome_version_size;
    let mut compilation_time_size = 0;
    if self.compilation_time != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.compilation_time;
      let l = ::pb_jelly::Message::compute_size(val);
      compilation_time_size += ::pb_jelly::wire_format::serialized_length(5);
      compilation_time_size += ::pb_jelly::varint::serialized_length(l as u64);
      compilation_time_size += l;
    }
    size += compilation_time_size;
    let mut model_size = 0;
    if self.model != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.model;
      let l = ::pb_jelly::Message::compute_size(val);
      model_size += ::pb_jelly::wire_format::serialized_length(6);
      model_size += ::pb_jelly::varint::serialized_length(l as u64);
      model_size += l;
    }
    size += model_size;
    let mut has_deep_sleep_size = 0;
    if self.has_deep_sleep != <bool as ::std::default::Default>::default() {
      let val = &self.has_deep_sleep;
      let l = ::pb_jelly::Message::compute_size(val);
      has_deep_sleep_size += ::pb_jelly::wire_format::serialized_length(7);
      has_deep_sleep_size += l;
    }
    size += has_deep_sleep_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.uses_password != <bool as ::std::default::Default>::default() {
      let val = &self.uses_password;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.mac_address != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.mac_address;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.esphome_version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.esphome_version;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.compilation_time != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.compilation_time;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.model != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.model;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_deep_sleep != <bool as ::std::default::Default>::default() {
      let val = &self.has_deep_sleep;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.uses_password != <bool as ::std::default::Default>::default() {
      let val = &self.uses_password;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.mac_address != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.mac_address;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.esphome_version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.esphome_version;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.compilation_time != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.compilation_time;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.model != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.model;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_deep_sleep != <bool as ::std::default::Default>::default() {
      let val = &self.has_deep_sleep;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "DeviceInfoResponse", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.uses_password = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "DeviceInfoResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "DeviceInfoResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.mac_address = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "DeviceInfoResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.esphome_version = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "DeviceInfoResponse", 5)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.compilation_time = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "DeviceInfoResponse", 6)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.model = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "DeviceInfoResponse", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_deep_sleep = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for DeviceInfoResponse {
  const NAME: &'static str = "DeviceInfoResponse";
  const FULL_NAME: &'static str = "espapi.DeviceInfoResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesRequest {
}
impl ::std::default::Default for ListEntitiesRequest {
  fn default() -> Self {
    ListEntitiesRequest {
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesRequest_default: ListEntitiesRequest = ListEntitiesRequest::default();
}
impl ::pb_jelly::Message for ListEntitiesRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesRequest {
  const NAME: &'static str = "ListEntitiesRequest";
  const FULL_NAME: &'static str = "espapi.ListEntitiesRequest";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesDoneResponse {
}
impl ::std::default::Default for ListEntitiesDoneResponse {
  fn default() -> Self {
    ListEntitiesDoneResponse {
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesDoneResponse_default: ListEntitiesDoneResponse = ListEntitiesDoneResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesDoneResponse {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesDoneResponse {
  const NAME: &'static str = "ListEntitiesDoneResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesDoneResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeStatesRequest {
}
impl ::std::default::Default for SubscribeStatesRequest {
  fn default() -> Self {
    SubscribeStatesRequest {
    }
  }
}
lazy_static! {
  pub static ref SubscribeStatesRequest_default: SubscribeStatesRequest = SubscribeStatesRequest::default();
}
impl ::pb_jelly::Message for SubscribeStatesRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeStatesRequest {
  const NAME: &'static str = "SubscribeStatesRequest";
  const FULL_NAME: &'static str = "espapi.SubscribeStatesRequest";
}

/// ==================== BINARY SENSOR ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesBinarySensorResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub device_class: ::std::string::String,
  pub is_status_binary_sensor: bool,
}
impl ::std::default::Default for ListEntitiesBinarySensorResponse {
  fn default() -> Self {
    ListEntitiesBinarySensorResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      device_class: ::std::default::Default::default(),
      is_status_binary_sensor: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesBinarySensorResponse_default: ListEntitiesBinarySensorResponse = ListEntitiesBinarySensorResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesBinarySensorResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut device_class_size = 0;
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      let l = ::pb_jelly::Message::compute_size(val);
      device_class_size += ::pb_jelly::wire_format::serialized_length(5);
      device_class_size += ::pb_jelly::varint::serialized_length(l as u64);
      device_class_size += l;
    }
    size += device_class_size;
    let mut is_status_binary_sensor_size = 0;
    if self.is_status_binary_sensor != <bool as ::std::default::Default>::default() {
      let val = &self.is_status_binary_sensor;
      let l = ::pb_jelly::Message::compute_size(val);
      is_status_binary_sensor_size += ::pb_jelly::wire_format::serialized_length(6);
      is_status_binary_sensor_size += l;
    }
    size += is_status_binary_sensor_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.is_status_binary_sensor != <bool as ::std::default::Default>::default() {
      let val = &self.is_status_binary_sensor;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.is_status_binary_sensor != <bool as ::std::default::Default>::default() {
      let val = &self.is_status_binary_sensor;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesBinarySensorResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesBinarySensorResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesBinarySensorResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesBinarySensorResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesBinarySensorResponse", 5)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.device_class = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesBinarySensorResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.is_status_binary_sensor = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesBinarySensorResponse {
  const NAME: &'static str = "ListEntitiesBinarySensorResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesBinarySensorResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinarySensorStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: bool,
  /// If the binary sensor does not have a valid state yet.
  /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
  pub missing_state: bool,
}
impl ::std::default::Default for BinarySensorStateResponse {
  fn default() -> Self {
    BinarySensorStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      missing_state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref BinarySensorStateResponse_default: BinarySensorStateResponse = BinarySensorStateResponse::default();
}
impl ::pb_jelly::Message for BinarySensorStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    let mut missing_state_size = 0;
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      let l = ::pb_jelly::Message::compute_size(val);
      missing_state_size += ::pb_jelly::wire_format::serialized_length(3);
      missing_state_size += l;
    }
    size += missing_state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "BinarySensorStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "BinarySensorStateResponse", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "BinarySensorStateResponse", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.missing_state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for BinarySensorStateResponse {
  const NAME: &'static str = "BinarySensorStateResponse";
  const FULL_NAME: &'static str = "espapi.BinarySensorStateResponse";
}

/// ==================== COVER ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesCoverResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub assumed_state: bool,
  pub supports_position: bool,
  pub supports_tilt: bool,
  pub device_class: ::std::string::String,
}
impl ::std::default::Default for ListEntitiesCoverResponse {
  fn default() -> Self {
    ListEntitiesCoverResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      assumed_state: ::std::default::Default::default(),
      supports_position: ::std::default::Default::default(),
      supports_tilt: ::std::default::Default::default(),
      device_class: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesCoverResponse_default: ListEntitiesCoverResponse = ListEntitiesCoverResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesCoverResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut assumed_state_size = 0;
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      let l = ::pb_jelly::Message::compute_size(val);
      assumed_state_size += ::pb_jelly::wire_format::serialized_length(5);
      assumed_state_size += l;
    }
    size += assumed_state_size;
    let mut supports_position_size = 0;
    if self.supports_position != <bool as ::std::default::Default>::default() {
      let val = &self.supports_position;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_position_size += ::pb_jelly::wire_format::serialized_length(6);
      supports_position_size += l;
    }
    size += supports_position_size;
    let mut supports_tilt_size = 0;
    if self.supports_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.supports_tilt;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_tilt_size += ::pb_jelly::wire_format::serialized_length(7);
      supports_tilt_size += l;
    }
    size += supports_tilt_size;
    let mut device_class_size = 0;
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      let l = ::pb_jelly::Message::compute_size(val);
      device_class_size += ::pb_jelly::wire_format::serialized_length(8);
      device_class_size += ::pb_jelly::varint::serialized_length(l as u64);
      device_class_size += l;
    }
    size += device_class_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_position != <bool as ::std::default::Default>::default() {
      let val = &self.supports_position;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.supports_tilt;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_position != <bool as ::std::default::Default>::default() {
      let val = &self.supports_position;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.supports_tilt;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.device_class != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.device_class;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCoverResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesCoverResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCoverResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCoverResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesCoverResponse", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.assumed_state = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesCoverResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_position = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesCoverResponse", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_tilt = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCoverResponse", 8)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.device_class = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesCoverResponse {
  const NAME: &'static str = "ListEntitiesCoverResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesCoverResponse";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoverStateResponse {
  pub key: ::pb_jelly::Fixed32,
  /// legacy: state has been removed in 1.13
  /// clients/servers must still send/accept it until the next protocol change
  pub legacy_state: LegacyCoverState,
  pub position: f32,
  pub tilt: f32,
  pub current_operation: CoverOperation,
}
impl ::std::default::Default for CoverStateResponse {
  fn default() -> Self {
    CoverStateResponse {
      key: ::std::default::Default::default(),
      legacy_state: ::std::default::Default::default(),
      position: ::std::default::Default::default(),
      tilt: ::std::default::Default::default(),
      current_operation: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CoverStateResponse_default: CoverStateResponse = CoverStateResponse::default();
}
impl ::pb_jelly::Message for CoverStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut legacy_state_size = 0;
    if self.legacy_state != <LegacyCoverState as ::std::default::Default>::default() {
      let val = &self.legacy_state;
      let l = ::pb_jelly::Message::compute_size(val);
      legacy_state_size += ::pb_jelly::wire_format::serialized_length(2);
      legacy_state_size += l;
    }
    size += legacy_state_size;
    let mut position_size = 0;
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      let l = ::pb_jelly::Message::compute_size(val);
      position_size += ::pb_jelly::wire_format::serialized_length(3);
      position_size += l;
    }
    size += position_size;
    let mut tilt_size = 0;
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      let l = ::pb_jelly::Message::compute_size(val);
      tilt_size += ::pb_jelly::wire_format::serialized_length(4);
      tilt_size += l;
    }
    size += tilt_size;
    let mut current_operation_size = 0;
    if self.current_operation != <CoverOperation as ::std::default::Default>::default() {
      let val = &self.current_operation;
      let l = ::pb_jelly::Message::compute_size(val);
      current_operation_size += ::pb_jelly::wire_format::serialized_length(5);
      current_operation_size += l;
    }
    size += current_operation_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.legacy_state != <LegacyCoverState as ::std::default::Default>::default() {
      let val = &self.legacy_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.current_operation != <CoverOperation as ::std::default::Default>::default() {
      let val = &self.current_operation;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.legacy_state != <LegacyCoverState as ::std::default::Default>::default() {
      let val = &self.legacy_state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.current_operation != <CoverOperation as ::std::default::Default>::default() {
      let val = &self.current_operation;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverStateResponse", 2)?;
          let mut val: LegacyCoverState = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.legacy_state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverStateResponse", 3)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.position = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverStateResponse", 4)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.tilt = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverStateResponse", 5)?;
          let mut val: CoverOperation = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.current_operation = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for CoverStateResponse {
  const NAME: &'static str = "CoverStateResponse";
  const FULL_NAME: &'static str = "espapi.CoverStateResponse";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoverCommandRequest {
  pub key: ::pb_jelly::Fixed32,
  /// legacy: command has been removed in 1.13
  /// clients/servers must still send/accept it until the next protocol change
  pub has_legacy_command: bool,
  pub legacy_command: LegacyCoverCommand,
  pub has_position: bool,
  pub position: f32,
  pub has_tilt: bool,
  pub tilt: f32,
  pub stop: bool,
}
impl ::std::default::Default for CoverCommandRequest {
  fn default() -> Self {
    CoverCommandRequest {
      key: ::std::default::Default::default(),
      has_legacy_command: ::std::default::Default::default(),
      legacy_command: ::std::default::Default::default(),
      has_position: ::std::default::Default::default(),
      position: ::std::default::Default::default(),
      has_tilt: ::std::default::Default::default(),
      tilt: ::std::default::Default::default(),
      stop: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CoverCommandRequest_default: CoverCommandRequest = CoverCommandRequest::default();
}
impl ::pb_jelly::Message for CoverCommandRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut has_legacy_command_size = 0;
    if self.has_legacy_command != <bool as ::std::default::Default>::default() {
      let val = &self.has_legacy_command;
      let l = ::pb_jelly::Message::compute_size(val);
      has_legacy_command_size += ::pb_jelly::wire_format::serialized_length(2);
      has_legacy_command_size += l;
    }
    size += has_legacy_command_size;
    let mut legacy_command_size = 0;
    if self.legacy_command != <LegacyCoverCommand as ::std::default::Default>::default() {
      let val = &self.legacy_command;
      let l = ::pb_jelly::Message::compute_size(val);
      legacy_command_size += ::pb_jelly::wire_format::serialized_length(3);
      legacy_command_size += l;
    }
    size += legacy_command_size;
    let mut has_position_size = 0;
    if self.has_position != <bool as ::std::default::Default>::default() {
      let val = &self.has_position;
      let l = ::pb_jelly::Message::compute_size(val);
      has_position_size += ::pb_jelly::wire_format::serialized_length(4);
      has_position_size += l;
    }
    size += has_position_size;
    let mut position_size = 0;
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      let l = ::pb_jelly::Message::compute_size(val);
      position_size += ::pb_jelly::wire_format::serialized_length(5);
      position_size += l;
    }
    size += position_size;
    let mut has_tilt_size = 0;
    if self.has_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.has_tilt;
      let l = ::pb_jelly::Message::compute_size(val);
      has_tilt_size += ::pb_jelly::wire_format::serialized_length(6);
      has_tilt_size += l;
    }
    size += has_tilt_size;
    let mut tilt_size = 0;
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      let l = ::pb_jelly::Message::compute_size(val);
      tilt_size += ::pb_jelly::wire_format::serialized_length(7);
      tilt_size += l;
    }
    size += tilt_size;
    let mut stop_size = 0;
    if self.stop != <bool as ::std::default::Default>::default() {
      let val = &self.stop;
      let l = ::pb_jelly::Message::compute_size(val);
      stop_size += ::pb_jelly::wire_format::serialized_length(8);
      stop_size += l;
    }
    size += stop_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_legacy_command != <bool as ::std::default::Default>::default() {
      let val = &self.has_legacy_command;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.legacy_command != <LegacyCoverCommand as ::std::default::Default>::default() {
      let val = &self.legacy_command;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_position != <bool as ::std::default::Default>::default() {
      let val = &self.has_position;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.has_tilt;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.stop != <bool as ::std::default::Default>::default() {
      let val = &self.stop;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_legacy_command != <bool as ::std::default::Default>::default() {
      let val = &self.has_legacy_command;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.legacy_command != <LegacyCoverCommand as ::std::default::Default>::default() {
      let val = &self.legacy_command;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_position != <bool as ::std::default::Default>::default() {
      let val = &self.has_position;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.position != <f32 as ::std::default::Default>::default() {
      let val = &self.position;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_tilt != <bool as ::std::default::Default>::default() {
      let val = &self.has_tilt;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.tilt != <f32 as ::std::default::Default>::default() {
      let val = &self.tilt;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.stop != <bool as ::std::default::Default>::default() {
      let val = &self.stop;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverCommandRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverCommandRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_legacy_command = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverCommandRequest", 3)?;
          let mut val: LegacyCoverCommand = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.legacy_command = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverCommandRequest", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_position = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverCommandRequest", 5)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.position = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverCommandRequest", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_tilt = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CoverCommandRequest", 7)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.tilt = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CoverCommandRequest", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.stop = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for CoverCommandRequest {
  const NAME: &'static str = "CoverCommandRequest";
  const FULL_NAME: &'static str = "espapi.CoverCommandRequest";
}

/// ==================== FAN ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesFanResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub supports_oscillation: bool,
  pub supports_speed: bool,
  pub supports_direction: bool,
}
impl ::std::default::Default for ListEntitiesFanResponse {
  fn default() -> Self {
    ListEntitiesFanResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      supports_oscillation: ::std::default::Default::default(),
      supports_speed: ::std::default::Default::default(),
      supports_direction: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesFanResponse_default: ListEntitiesFanResponse = ListEntitiesFanResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesFanResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut supports_oscillation_size = 0;
    if self.supports_oscillation != <bool as ::std::default::Default>::default() {
      let val = &self.supports_oscillation;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_oscillation_size += ::pb_jelly::wire_format::serialized_length(5);
      supports_oscillation_size += l;
    }
    size += supports_oscillation_size;
    let mut supports_speed_size = 0;
    if self.supports_speed != <bool as ::std::default::Default>::default() {
      let val = &self.supports_speed;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_speed_size += ::pb_jelly::wire_format::serialized_length(6);
      supports_speed_size += l;
    }
    size += supports_speed_size;
    let mut supports_direction_size = 0;
    if self.supports_direction != <bool as ::std::default::Default>::default() {
      let val = &self.supports_direction;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_direction_size += ::pb_jelly::wire_format::serialized_length(7);
      supports_direction_size += l;
    }
    size += supports_direction_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_oscillation != <bool as ::std::default::Default>::default() {
      let val = &self.supports_oscillation;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_speed != <bool as ::std::default::Default>::default() {
      let val = &self.supports_speed;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_direction != <bool as ::std::default::Default>::default() {
      let val = &self.supports_direction;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_oscillation != <bool as ::std::default::Default>::default() {
      let val = &self.supports_oscillation;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_speed != <bool as ::std::default::Default>::default() {
      let val = &self.supports_speed;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_direction != <bool as ::std::default::Default>::default() {
      let val = &self.supports_direction;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesFanResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesFanResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesFanResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesFanResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesFanResponse", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_oscillation = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesFanResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_speed = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesFanResponse", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_direction = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesFanResponse {
  const NAME: &'static str = "ListEntitiesFanResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesFanResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FanStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: bool,
  pub oscillating: bool,
  pub speed: FanSpeed,
  pub direction: FanDirection,
}
impl ::std::default::Default for FanStateResponse {
  fn default() -> Self {
    FanStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      oscillating: ::std::default::Default::default(),
      speed: ::std::default::Default::default(),
      direction: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FanStateResponse_default: FanStateResponse = FanStateResponse::default();
}
impl ::pb_jelly::Message for FanStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    let mut oscillating_size = 0;
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      let l = ::pb_jelly::Message::compute_size(val);
      oscillating_size += ::pb_jelly::wire_format::serialized_length(3);
      oscillating_size += l;
    }
    size += oscillating_size;
    let mut speed_size = 0;
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      let l = ::pb_jelly::Message::compute_size(val);
      speed_size += ::pb_jelly::wire_format::serialized_length(4);
      speed_size += l;
    }
    size += speed_size;
    let mut direction_size = 0;
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      let l = ::pb_jelly::Message::compute_size(val);
      direction_size += ::pb_jelly::wire_format::serialized_length(5);
      direction_size += l;
    }
    size += direction_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "FanStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanStateResponse", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanStateResponse", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.oscillating = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanStateResponse", 4)?;
          let mut val: FanSpeed = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.speed = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanStateResponse", 5)?;
          let mut val: FanDirection = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.direction = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for FanStateResponse {
  const NAME: &'static str = "FanStateResponse";
  const FULL_NAME: &'static str = "espapi.FanStateResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FanCommandRequest {
  pub key: ::pb_jelly::Fixed32,
  pub has_state: bool,
  pub state: bool,
  pub has_speed: bool,
  pub speed: FanSpeed,
  pub has_oscillating: bool,
  pub oscillating: bool,
  pub has_direction: bool,
  pub direction: FanDirection,
}
impl ::std::default::Default for FanCommandRequest {
  fn default() -> Self {
    FanCommandRequest {
      key: ::std::default::Default::default(),
      has_state: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      has_speed: ::std::default::Default::default(),
      speed: ::std::default::Default::default(),
      has_oscillating: ::std::default::Default::default(),
      oscillating: ::std::default::Default::default(),
      has_direction: ::std::default::Default::default(),
      direction: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FanCommandRequest_default: FanCommandRequest = FanCommandRequest::default();
}
impl ::pb_jelly::Message for FanCommandRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut has_state_size = 0;
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      let l = ::pb_jelly::Message::compute_size(val);
      has_state_size += ::pb_jelly::wire_format::serialized_length(2);
      has_state_size += l;
    }
    size += has_state_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(3);
      state_size += l;
    }
    size += state_size;
    let mut has_speed_size = 0;
    if self.has_speed != <bool as ::std::default::Default>::default() {
      let val = &self.has_speed;
      let l = ::pb_jelly::Message::compute_size(val);
      has_speed_size += ::pb_jelly::wire_format::serialized_length(4);
      has_speed_size += l;
    }
    size += has_speed_size;
    let mut speed_size = 0;
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      let l = ::pb_jelly::Message::compute_size(val);
      speed_size += ::pb_jelly::wire_format::serialized_length(5);
      speed_size += l;
    }
    size += speed_size;
    let mut has_oscillating_size = 0;
    if self.has_oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.has_oscillating;
      let l = ::pb_jelly::Message::compute_size(val);
      has_oscillating_size += ::pb_jelly::wire_format::serialized_length(6);
      has_oscillating_size += l;
    }
    size += has_oscillating_size;
    let mut oscillating_size = 0;
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      let l = ::pb_jelly::Message::compute_size(val);
      oscillating_size += ::pb_jelly::wire_format::serialized_length(7);
      oscillating_size += l;
    }
    size += oscillating_size;
    let mut has_direction_size = 0;
    if self.has_direction != <bool as ::std::default::Default>::default() {
      let val = &self.has_direction;
      let l = ::pb_jelly::Message::compute_size(val);
      has_direction_size += ::pb_jelly::wire_format::serialized_length(8);
      has_direction_size += l;
    }
    size += has_direction_size;
    let mut direction_size = 0;
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      let l = ::pb_jelly::Message::compute_size(val);
      direction_size += ::pb_jelly::wire_format::serialized_length(9);
      direction_size += l;
    }
    size += direction_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_speed != <bool as ::std::default::Default>::default() {
      let val = &self.has_speed;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.has_oscillating;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_direction != <bool as ::std::default::Default>::default() {
      let val = &self.has_direction;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_speed != <bool as ::std::default::Default>::default() {
      let val = &self.has_speed;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.speed != <FanSpeed as ::std::default::Default>::default() {
      let val = &self.speed;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.has_oscillating;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.oscillating != <bool as ::std::default::Default>::default() {
      let val = &self.oscillating;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_direction != <bool as ::std::default::Default>::default() {
      let val = &self.has_direction;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.direction != <FanDirection as ::std::default::Default>::default() {
      let val = &self.direction;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "FanCommandRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_speed = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 5)?;
          let mut val: FanSpeed = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.speed = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_oscillating = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.oscillating = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_direction = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "FanCommandRequest", 9)?;
          let mut val: FanDirection = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.direction = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for FanCommandRequest {
  const NAME: &'static str = "FanCommandRequest";
  const FULL_NAME: &'static str = "espapi.FanCommandRequest";
}

/// ==================== LIGHT ====================
#[derive(Clone, Debug, PartialEq)]
pub struct ListEntitiesLightResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub supports_brightness: bool,
  pub supports_rgb: bool,
  pub supports_white_value: bool,
  pub supports_color_temperature: bool,
  pub min_mireds: f32,
  pub max_mireds: f32,
  pub effects: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for ListEntitiesLightResponse {
  fn default() -> Self {
    ListEntitiesLightResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      supports_brightness: ::std::default::Default::default(),
      supports_rgb: ::std::default::Default::default(),
      supports_white_value: ::std::default::Default::default(),
      supports_color_temperature: ::std::default::Default::default(),
      min_mireds: ::std::default::Default::default(),
      max_mireds: ::std::default::Default::default(),
      effects: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesLightResponse_default: ListEntitiesLightResponse = ListEntitiesLightResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesLightResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut supports_brightness_size = 0;
    if self.supports_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.supports_brightness;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_brightness_size += ::pb_jelly::wire_format::serialized_length(5);
      supports_brightness_size += l;
    }
    size += supports_brightness_size;
    let mut supports_rgb_size = 0;
    if self.supports_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.supports_rgb;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_rgb_size += ::pb_jelly::wire_format::serialized_length(6);
      supports_rgb_size += l;
    }
    size += supports_rgb_size;
    let mut supports_white_value_size = 0;
    if self.supports_white_value != <bool as ::std::default::Default>::default() {
      let val = &self.supports_white_value;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_white_value_size += ::pb_jelly::wire_format::serialized_length(7);
      supports_white_value_size += l;
    }
    size += supports_white_value_size;
    let mut supports_color_temperature_size = 0;
    if self.supports_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_color_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_color_temperature_size += ::pb_jelly::wire_format::serialized_length(8);
      supports_color_temperature_size += l;
    }
    size += supports_color_temperature_size;
    let mut min_mireds_size = 0;
    if self.min_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.min_mireds;
      let l = ::pb_jelly::Message::compute_size(val);
      min_mireds_size += ::pb_jelly::wire_format::serialized_length(9);
      min_mireds_size += l;
    }
    size += min_mireds_size;
    let mut max_mireds_size = 0;
    if self.max_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.max_mireds;
      let l = ::pb_jelly::Message::compute_size(val);
      max_mireds_size += ::pb_jelly::wire_format::serialized_length(10);
      max_mireds_size += l;
    }
    size += max_mireds_size;
    let mut effects_size = 0;
    for val in &self.effects {
      let l = ::pb_jelly::Message::compute_size(val);
      effects_size += ::pb_jelly::wire_format::serialized_length(11);
      effects_size += ::pb_jelly::varint::serialized_length(l as u64);
      effects_size += l;
    }
    size += effects_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.supports_brightness;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.supports_rgb;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_white_value != <bool as ::std::default::Default>::default() {
      let val = &self.supports_white_value;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_color_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.min_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.min_mireds;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.max_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.max_mireds;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.effects {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.supports_brightness;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.supports_rgb;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_white_value != <bool as ::std::default::Default>::default() {
      let val = &self.supports_white_value;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_color_temperature;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.min_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.min_mireds;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.max_mireds != <f32 as ::std::default::Default>::default() {
      let val = &self.max_mireds;
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.effects {
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesLightResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesLightResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesLightResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesLightResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesLightResponse", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_brightness = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesLightResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_rgb = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesLightResponse", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_white_value = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesLightResponse", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_color_temperature = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesLightResponse", 9)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.min_mireds = val;
        }
        10 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesLightResponse", 10)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.max_mireds = val;
        }
        11 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesLightResponse", 11)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.effects.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesLightResponse {
  const NAME: &'static str = "ListEntitiesLightResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesLightResponse";
}

#[derive(Clone, Debug, PartialEq)]
pub struct LightStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: bool,
  pub brightness: f32,
  pub red: f32,
  pub green: f32,
  pub blue: f32,
  pub white: f32,
  pub color_temperature: f32,
  pub effect: ::std::string::String,
}
impl ::std::default::Default for LightStateResponse {
  fn default() -> Self {
    LightStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      brightness: ::std::default::Default::default(),
      red: ::std::default::Default::default(),
      green: ::std::default::Default::default(),
      blue: ::std::default::Default::default(),
      white: ::std::default::Default::default(),
      color_temperature: ::std::default::Default::default(),
      effect: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref LightStateResponse_default: LightStateResponse = LightStateResponse::default();
}
impl ::pb_jelly::Message for LightStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    let mut brightness_size = 0;
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      let l = ::pb_jelly::Message::compute_size(val);
      brightness_size += ::pb_jelly::wire_format::serialized_length(3);
      brightness_size += l;
    }
    size += brightness_size;
    let mut red_size = 0;
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      let l = ::pb_jelly::Message::compute_size(val);
      red_size += ::pb_jelly::wire_format::serialized_length(4);
      red_size += l;
    }
    size += red_size;
    let mut green_size = 0;
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      let l = ::pb_jelly::Message::compute_size(val);
      green_size += ::pb_jelly::wire_format::serialized_length(5);
      green_size += l;
    }
    size += green_size;
    let mut blue_size = 0;
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      let l = ::pb_jelly::Message::compute_size(val);
      blue_size += ::pb_jelly::wire_format::serialized_length(6);
      blue_size += l;
    }
    size += blue_size;
    let mut white_size = 0;
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      let l = ::pb_jelly::Message::compute_size(val);
      white_size += ::pb_jelly::wire_format::serialized_length(7);
      white_size += l;
    }
    size += white_size;
    let mut color_temperature_size = 0;
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      color_temperature_size += ::pb_jelly::wire_format::serialized_length(8);
      color_temperature_size += l;
    }
    size += color_temperature_size;
    let mut effect_size = 0;
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      let l = ::pb_jelly::Message::compute_size(val);
      effect_size += ::pb_jelly::wire_format::serialized_length(9);
      effect_size += ::pb_jelly::varint::serialized_length(l as u64);
      effect_size += l;
    }
    size += effect_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightStateResponse", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 3)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.brightness = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 4)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.red = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 5)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.green = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 6)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.blue = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 7)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.white = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightStateResponse", 8)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.color_temperature = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "LightStateResponse", 9)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.effect = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for LightStateResponse {
  const NAME: &'static str = "LightStateResponse";
  const FULL_NAME: &'static str = "espapi.LightStateResponse";
}

#[derive(Clone, Debug, PartialEq)]
pub struct LightCommandRequest {
  pub key: ::pb_jelly::Fixed32,
  pub has_state: bool,
  pub state: bool,
  pub has_brightness: bool,
  pub brightness: f32,
  pub has_rgb: bool,
  pub red: f32,
  pub green: f32,
  pub blue: f32,
  pub has_white: bool,
  pub white: f32,
  pub has_color_temperature: bool,
  pub color_temperature: f32,
  pub has_transition_length: bool,
  pub transition_length: u32,
  pub has_flash_length: bool,
  pub flash_length: u32,
  pub has_effect: bool,
  pub effect: ::std::string::String,
}
impl ::std::default::Default for LightCommandRequest {
  fn default() -> Self {
    LightCommandRequest {
      key: ::std::default::Default::default(),
      has_state: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      has_brightness: ::std::default::Default::default(),
      brightness: ::std::default::Default::default(),
      has_rgb: ::std::default::Default::default(),
      red: ::std::default::Default::default(),
      green: ::std::default::Default::default(),
      blue: ::std::default::Default::default(),
      has_white: ::std::default::Default::default(),
      white: ::std::default::Default::default(),
      has_color_temperature: ::std::default::Default::default(),
      color_temperature: ::std::default::Default::default(),
      has_transition_length: ::std::default::Default::default(),
      transition_length: ::std::default::Default::default(),
      has_flash_length: ::std::default::Default::default(),
      flash_length: ::std::default::Default::default(),
      has_effect: ::std::default::Default::default(),
      effect: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref LightCommandRequest_default: LightCommandRequest = LightCommandRequest::default();
}
impl ::pb_jelly::Message for LightCommandRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut has_state_size = 0;
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      let l = ::pb_jelly::Message::compute_size(val);
      has_state_size += ::pb_jelly::wire_format::serialized_length(2);
      has_state_size += l;
    }
    size += has_state_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(3);
      state_size += l;
    }
    size += state_size;
    let mut has_brightness_size = 0;
    if self.has_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.has_brightness;
      let l = ::pb_jelly::Message::compute_size(val);
      has_brightness_size += ::pb_jelly::wire_format::serialized_length(4);
      has_brightness_size += l;
    }
    size += has_brightness_size;
    let mut brightness_size = 0;
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      let l = ::pb_jelly::Message::compute_size(val);
      brightness_size += ::pb_jelly::wire_format::serialized_length(5);
      brightness_size += l;
    }
    size += brightness_size;
    let mut has_rgb_size = 0;
    if self.has_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.has_rgb;
      let l = ::pb_jelly::Message::compute_size(val);
      has_rgb_size += ::pb_jelly::wire_format::serialized_length(6);
      has_rgb_size += l;
    }
    size += has_rgb_size;
    let mut red_size = 0;
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      let l = ::pb_jelly::Message::compute_size(val);
      red_size += ::pb_jelly::wire_format::serialized_length(7);
      red_size += l;
    }
    size += red_size;
    let mut green_size = 0;
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      let l = ::pb_jelly::Message::compute_size(val);
      green_size += ::pb_jelly::wire_format::serialized_length(8);
      green_size += l;
    }
    size += green_size;
    let mut blue_size = 0;
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      let l = ::pb_jelly::Message::compute_size(val);
      blue_size += ::pb_jelly::wire_format::serialized_length(9);
      blue_size += l;
    }
    size += blue_size;
    let mut has_white_size = 0;
    if self.has_white != <bool as ::std::default::Default>::default() {
      let val = &self.has_white;
      let l = ::pb_jelly::Message::compute_size(val);
      has_white_size += ::pb_jelly::wire_format::serialized_length(10);
      has_white_size += l;
    }
    size += has_white_size;
    let mut white_size = 0;
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      let l = ::pb_jelly::Message::compute_size(val);
      white_size += ::pb_jelly::wire_format::serialized_length(11);
      white_size += l;
    }
    size += white_size;
    let mut has_color_temperature_size = 0;
    if self.has_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_color_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      has_color_temperature_size += ::pb_jelly::wire_format::serialized_length(12);
      has_color_temperature_size += l;
    }
    size += has_color_temperature_size;
    let mut color_temperature_size = 0;
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      color_temperature_size += ::pb_jelly::wire_format::serialized_length(13);
      color_temperature_size += l;
    }
    size += color_temperature_size;
    let mut has_transition_length_size = 0;
    if self.has_transition_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_transition_length;
      let l = ::pb_jelly::Message::compute_size(val);
      has_transition_length_size += ::pb_jelly::wire_format::serialized_length(14);
      has_transition_length_size += l;
    }
    size += has_transition_length_size;
    let mut transition_length_size = 0;
    if self.transition_length != <u32 as ::std::default::Default>::default() {
      let val = &self.transition_length;
      let l = ::pb_jelly::Message::compute_size(val);
      transition_length_size += ::pb_jelly::wire_format::serialized_length(15);
      transition_length_size += l;
    }
    size += transition_length_size;
    let mut has_flash_length_size = 0;
    if self.has_flash_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_flash_length;
      let l = ::pb_jelly::Message::compute_size(val);
      has_flash_length_size += ::pb_jelly::wire_format::serialized_length(16);
      has_flash_length_size += l;
    }
    size += has_flash_length_size;
    let mut flash_length_size = 0;
    if self.flash_length != <u32 as ::std::default::Default>::default() {
      let val = &self.flash_length;
      let l = ::pb_jelly::Message::compute_size(val);
      flash_length_size += ::pb_jelly::wire_format::serialized_length(17);
      flash_length_size += l;
    }
    size += flash_length_size;
    let mut has_effect_size = 0;
    if self.has_effect != <bool as ::std::default::Default>::default() {
      let val = &self.has_effect;
      let l = ::pb_jelly::Message::compute_size(val);
      has_effect_size += ::pb_jelly::wire_format::serialized_length(18);
      has_effect_size += l;
    }
    size += has_effect_size;
    let mut effect_size = 0;
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      let l = ::pb_jelly::Message::compute_size(val);
      effect_size += ::pb_jelly::wire_format::serialized_length(19);
      effect_size += ::pb_jelly::varint::serialized_length(l as u64);
      effect_size += l;
    }
    size += effect_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.has_brightness;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.has_rgb;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_white != <bool as ::std::default::Default>::default() {
      let val = &self.has_white;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_color_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_transition_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_transition_length;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.transition_length != <u32 as ::std::default::Default>::default() {
      let val = &self.transition_length;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_flash_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_flash_length;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.flash_length != <u32 as ::std::default::Default>::default() {
      let val = &self.flash_length;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_effect != <bool as ::std::default::Default>::default() {
      let val = &self.has_effect;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_state != <bool as ::std::default::Default>::default() {
      let val = &self.has_state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_brightness != <bool as ::std::default::Default>::default() {
      let val = &self.has_brightness;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.brightness != <f32 as ::std::default::Default>::default() {
      let val = &self.brightness;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_rgb != <bool as ::std::default::Default>::default() {
      let val = &self.has_rgb;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.red != <f32 as ::std::default::Default>::default() {
      let val = &self.red;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.green != <f32 as ::std::default::Default>::default() {
      let val = &self.green;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.blue != <f32 as ::std::default::Default>::default() {
      let val = &self.blue;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_white != <bool as ::std::default::Default>::default() {
      let val = &self.has_white;
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.white != <f32 as ::std::default::Default>::default() {
      let val = &self.white;
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_color_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_color_temperature;
      ::pb_jelly::wire_format::write(12, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.color_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.color_temperature;
      ::pb_jelly::wire_format::write(13, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_transition_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_transition_length;
      ::pb_jelly::wire_format::write(14, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.transition_length != <u32 as ::std::default::Default>::default() {
      let val = &self.transition_length;
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_flash_length != <bool as ::std::default::Default>::default() {
      let val = &self.has_flash_length;
      ::pb_jelly::wire_format::write(16, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.flash_length != <u32 as ::std::default::Default>::default() {
      let val = &self.flash_length;
      ::pb_jelly::wire_format::write(17, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_effect != <bool as ::std::default::Default>::default() {
      let val = &self.has_effect;
      ::pb_jelly::wire_format::write(18, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.effect != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.effect;
      ::pb_jelly::wire_format::write(19, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_brightness = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 5)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.brightness = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_rgb = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 7)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.red = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 8)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.green = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 9)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.blue = val;
        }
        10 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 10)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_white = val;
        }
        11 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 11)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.white = val;
        }
        12 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 12)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_color_temperature = val;
        }
        13 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "LightCommandRequest", 13)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.color_temperature = val;
        }
        14 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 14)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_transition_length = val;
        }
        15 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 15)?;
          let mut val: u32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.transition_length = val;
        }
        16 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 16)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_flash_length = val;
        }
        17 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 17)?;
          let mut val: u32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.flash_length = val;
        }
        18 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "LightCommandRequest", 18)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_effect = val;
        }
        19 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "LightCommandRequest", 19)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.effect = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for LightCommandRequest {
  const NAME: &'static str = "LightCommandRequest";
  const FULL_NAME: &'static str = "espapi.LightCommandRequest";
}

/// ==================== SENSOR ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesSensorResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub icon: ::std::string::String,
  pub unit_of_measurement: ::std::string::String,
  pub accuracy_decimals: i32,
  pub force_update: bool,
}
impl ::std::default::Default for ListEntitiesSensorResponse {
  fn default() -> Self {
    ListEntitiesSensorResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      icon: ::std::default::Default::default(),
      unit_of_measurement: ::std::default::Default::default(),
      accuracy_decimals: ::std::default::Default::default(),
      force_update: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesSensorResponse_default: ListEntitiesSensorResponse = ListEntitiesSensorResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesSensorResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut icon_size = 0;
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      let l = ::pb_jelly::Message::compute_size(val);
      icon_size += ::pb_jelly::wire_format::serialized_length(5);
      icon_size += ::pb_jelly::varint::serialized_length(l as u64);
      icon_size += l;
    }
    size += icon_size;
    let mut unit_of_measurement_size = 0;
    if self.unit_of_measurement != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unit_of_measurement;
      let l = ::pb_jelly::Message::compute_size(val);
      unit_of_measurement_size += ::pb_jelly::wire_format::serialized_length(6);
      unit_of_measurement_size += ::pb_jelly::varint::serialized_length(l as u64);
      unit_of_measurement_size += l;
    }
    size += unit_of_measurement_size;
    let mut accuracy_decimals_size = 0;
    if self.accuracy_decimals != <i32 as ::std::default::Default>::default() {
      let val = &self.accuracy_decimals;
      let l = ::pb_jelly::Message::compute_size(val);
      accuracy_decimals_size += ::pb_jelly::wire_format::serialized_length(7);
      accuracy_decimals_size += l;
    }
    size += accuracy_decimals_size;
    let mut force_update_size = 0;
    if self.force_update != <bool as ::std::default::Default>::default() {
      let val = &self.force_update;
      let l = ::pb_jelly::Message::compute_size(val);
      force_update_size += ::pb_jelly::wire_format::serialized_length(8);
      force_update_size += l;
    }
    size += force_update_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unit_of_measurement != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unit_of_measurement;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.accuracy_decimals != <i32 as ::std::default::Default>::default() {
      let val = &self.accuracy_decimals;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.force_update != <bool as ::std::default::Default>::default() {
      let val = &self.force_update;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unit_of_measurement != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unit_of_measurement;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.accuracy_decimals != <i32 as ::std::default::Default>::default() {
      let val = &self.accuracy_decimals;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.force_update != <bool as ::std::default::Default>::default() {
      let val = &self.force_update;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSensorResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesSensorResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSensorResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSensorResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSensorResponse", 5)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.icon = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSensorResponse", 6)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unit_of_measurement = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesSensorResponse", 7)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.accuracy_decimals = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesSensorResponse", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.force_update = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesSensorResponse {
  const NAME: &'static str = "ListEntitiesSensorResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesSensorResponse";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensorStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: f32,
  /// If the sensor does not have a valid state yet.
  /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
  pub missing_state: bool,
}
impl ::std::default::Default for SensorStateResponse {
  fn default() -> Self {
    SensorStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      missing_state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SensorStateResponse_default: SensorStateResponse = SensorStateResponse::default();
}
impl ::pb_jelly::Message for SensorStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <f32 as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    let mut missing_state_size = 0;
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      let l = ::pb_jelly::Message::compute_size(val);
      missing_state_size += ::pb_jelly::wire_format::serialized_length(3);
      missing_state_size += l;
    }
    size += missing_state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <f32 as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <f32 as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "SensorStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "SensorStateResponse", 2)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SensorStateResponse", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.missing_state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SensorStateResponse {
  const NAME: &'static str = "SensorStateResponse";
  const FULL_NAME: &'static str = "espapi.SensorStateResponse";
}

/// ==================== SWITCH ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesSwitchResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub icon: ::std::string::String,
  pub assumed_state: bool,
}
impl ::std::default::Default for ListEntitiesSwitchResponse {
  fn default() -> Self {
    ListEntitiesSwitchResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      icon: ::std::default::Default::default(),
      assumed_state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesSwitchResponse_default: ListEntitiesSwitchResponse = ListEntitiesSwitchResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesSwitchResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut icon_size = 0;
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      let l = ::pb_jelly::Message::compute_size(val);
      icon_size += ::pb_jelly::wire_format::serialized_length(5);
      icon_size += ::pb_jelly::varint::serialized_length(l as u64);
      icon_size += l;
    }
    size += icon_size;
    let mut assumed_state_size = 0;
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      let l = ::pb_jelly::Message::compute_size(val);
      assumed_state_size += ::pb_jelly::wire_format::serialized_length(6);
      assumed_state_size += l;
    }
    size += assumed_state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.assumed_state != <bool as ::std::default::Default>::default() {
      let val = &self.assumed_state;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSwitchResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesSwitchResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSwitchResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSwitchResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesSwitchResponse", 5)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.icon = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesSwitchResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.assumed_state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesSwitchResponse {
  const NAME: &'static str = "ListEntitiesSwitchResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesSwitchResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwitchStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: bool,
}
impl ::std::default::Default for SwitchStateResponse {
  fn default() -> Self {
    SwitchStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SwitchStateResponse_default: SwitchStateResponse = SwitchStateResponse::default();
}
impl ::pb_jelly::Message for SwitchStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "SwitchStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SwitchStateResponse", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SwitchStateResponse {
  const NAME: &'static str = "SwitchStateResponse";
  const FULL_NAME: &'static str = "espapi.SwitchStateResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwitchCommandRequest {
  pub key: ::pb_jelly::Fixed32,
  pub state: bool,
}
impl ::std::default::Default for SwitchCommandRequest {
  fn default() -> Self {
    SwitchCommandRequest {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SwitchCommandRequest_default: SwitchCommandRequest = SwitchCommandRequest::default();
}
impl ::pb_jelly::Message for SwitchCommandRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += l;
    }
    size += state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <bool as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "SwitchCommandRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SwitchCommandRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SwitchCommandRequest {
  const NAME: &'static str = "SwitchCommandRequest";
  const FULL_NAME: &'static str = "espapi.SwitchCommandRequest";
}

/// ==================== TEXT SENSOR ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesTextSensorResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub icon: ::std::string::String,
}
impl ::std::default::Default for ListEntitiesTextSensorResponse {
  fn default() -> Self {
    ListEntitiesTextSensorResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      icon: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesTextSensorResponse_default: ListEntitiesTextSensorResponse = ListEntitiesTextSensorResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesTextSensorResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut icon_size = 0;
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      let l = ::pb_jelly::Message::compute_size(val);
      icon_size += ::pb_jelly::wire_format::serialized_length(5);
      icon_size += ::pb_jelly::varint::serialized_length(l as u64);
      icon_size += l;
    }
    size += icon_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.icon != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.icon;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesTextSensorResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesTextSensorResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesTextSensorResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesTextSensorResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesTextSensorResponse", 5)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.icon = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesTextSensorResponse {
  const NAME: &'static str = "ListEntitiesTextSensorResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesTextSensorResponse";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextSensorStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub state: ::std::string::String,
  /// If the text sensor does not have a valid state yet.
  /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
  pub missing_state: bool,
}
impl ::std::default::Default for TextSensorStateResponse {
  fn default() -> Self {
    TextSensorStateResponse {
      key: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
      missing_state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref TextSensorStateResponse_default: TextSensorStateResponse = TextSensorStateResponse::default();
}
impl ::pb_jelly::Message for TextSensorStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut state_size = 0;
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += ::pb_jelly::varint::serialized_length(l as u64);
      state_size += l;
    }
    size += state_size;
    let mut missing_state_size = 0;
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      let l = ::pb_jelly::Message::compute_size(val);
      missing_state_size += ::pb_jelly::wire_format::serialized_length(3);
      missing_state_size += l;
    }
    size += missing_state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.missing_state != <bool as ::std::default::Default>::default() {
      let val = &self.missing_state;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "TextSensorStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "TextSensorStateResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.state = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "TextSensorStateResponse", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.missing_state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for TextSensorStateResponse {
  const NAME: &'static str = "TextSensorStateResponse";
  const FULL_NAME: &'static str = "espapi.TextSensorStateResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeLogsRequest {
  pub level: LogLevel,
  pub dump_config: bool,
}
impl ::std::default::Default for SubscribeLogsRequest {
  fn default() -> Self {
    SubscribeLogsRequest {
      level: ::std::default::Default::default(),
      dump_config: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SubscribeLogsRequest_default: SubscribeLogsRequest = SubscribeLogsRequest::default();
}
impl ::pb_jelly::Message for SubscribeLogsRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut level_size = 0;
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      let l = ::pb_jelly::Message::compute_size(val);
      level_size += ::pb_jelly::wire_format::serialized_length(1);
      level_size += l;
    }
    size += level_size;
    let mut dump_config_size = 0;
    if self.dump_config != <bool as ::std::default::Default>::default() {
      let val = &self.dump_config;
      let l = ::pb_jelly::Message::compute_size(val);
      dump_config_size += ::pb_jelly::wire_format::serialized_length(2);
      dump_config_size += l;
    }
    size += dump_config_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.dump_config != <bool as ::std::default::Default>::default() {
      let val = &self.dump_config;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.dump_config != <bool as ::std::default::Default>::default() {
      let val = &self.dump_config;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SubscribeLogsRequest", 1)?;
          let mut val: LogLevel = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.level = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SubscribeLogsRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.dump_config = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeLogsRequest {
  const NAME: &'static str = "SubscribeLogsRequest";
  const FULL_NAME: &'static str = "espapi.SubscribeLogsRequest";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeLogsResponse {
  pub level: LogLevel,
  pub tag: ::std::string::String,
  pub message: ::std::string::String,
  pub send_failed: bool,
}
impl ::std::default::Default for SubscribeLogsResponse {
  fn default() -> Self {
    SubscribeLogsResponse {
      level: ::std::default::Default::default(),
      tag: ::std::default::Default::default(),
      message: ::std::default::Default::default(),
      send_failed: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SubscribeLogsResponse_default: SubscribeLogsResponse = SubscribeLogsResponse::default();
}
impl ::pb_jelly::Message for SubscribeLogsResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut level_size = 0;
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      let l = ::pb_jelly::Message::compute_size(val);
      level_size += ::pb_jelly::wire_format::serialized_length(1);
      level_size += l;
    }
    size += level_size;
    let mut tag_size = 0;
    if self.tag != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.tag;
      let l = ::pb_jelly::Message::compute_size(val);
      tag_size += ::pb_jelly::wire_format::serialized_length(2);
      tag_size += ::pb_jelly::varint::serialized_length(l as u64);
      tag_size += l;
    }
    size += tag_size;
    let mut message_size = 0;
    if self.message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.message;
      let l = ::pb_jelly::Message::compute_size(val);
      message_size += ::pb_jelly::wire_format::serialized_length(3);
      message_size += ::pb_jelly::varint::serialized_length(l as u64);
      message_size += l;
    }
    size += message_size;
    let mut send_failed_size = 0;
    if self.send_failed != <bool as ::std::default::Default>::default() {
      let val = &self.send_failed;
      let l = ::pb_jelly::Message::compute_size(val);
      send_failed_size += ::pb_jelly::wire_format::serialized_length(4);
      send_failed_size += l;
    }
    size += send_failed_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.tag != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.tag;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.message;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.send_failed != <bool as ::std::default::Default>::default() {
      let val = &self.send_failed;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.level != <LogLevel as ::std::default::Default>::default() {
      let val = &self.level;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.tag != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.tag;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.message;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.send_failed != <bool as ::std::default::Default>::default() {
      let val = &self.send_failed;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SubscribeLogsResponse", 1)?;
          let mut val: LogLevel = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.level = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "SubscribeLogsResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.tag = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "SubscribeLogsResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.message = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "SubscribeLogsResponse", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.send_failed = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeLogsResponse {
  const NAME: &'static str = "SubscribeLogsResponse";
  const FULL_NAME: &'static str = "espapi.SubscribeLogsResponse";
}

/// ==================== HOMEASSISTANT.SERVICE ====================
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeHomeassistantServicesRequest {
}
impl ::std::default::Default for SubscribeHomeassistantServicesRequest {
  fn default() -> Self {
    SubscribeHomeassistantServicesRequest {
    }
  }
}
lazy_static! {
  pub static ref SubscribeHomeassistantServicesRequest_default: SubscribeHomeassistantServicesRequest = SubscribeHomeassistantServicesRequest::default();
}
impl ::pb_jelly::Message for SubscribeHomeassistantServicesRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeHomeassistantServicesRequest {
  const NAME: &'static str = "SubscribeHomeassistantServicesRequest";
  const FULL_NAME: &'static str = "espapi.SubscribeHomeassistantServicesRequest";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HomeassistantServiceMap {
  pub key: ::std::string::String,
  pub value: ::std::string::String,
}
impl ::std::default::Default for HomeassistantServiceMap {
  fn default() -> Self {
    HomeassistantServiceMap {
      key: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HomeassistantServiceMap_default: HomeassistantServiceMap = HomeassistantServiceMap::default();
}
impl ::pb_jelly::Message for HomeassistantServiceMap {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += ::pb_jelly::varint::serialized_length(l as u64);
      key_size += l;
    }
    size += key_size;
    let mut value_size = 0;
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb_jelly::Message::compute_size(val);
      value_size += ::pb_jelly::wire_format::serialized_length(2);
      value_size += ::pb_jelly::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceMap", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceMap", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.value = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for HomeassistantServiceMap {
  const NAME: &'static str = "HomeassistantServiceMap";
  const FULL_NAME: &'static str = "espapi.HomeassistantServiceMap";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HomeassistantServiceResponse {
  pub service: ::std::string::String,
  pub data: ::std::vec::Vec<HomeassistantServiceMap>,
  pub data_template: ::std::vec::Vec<HomeassistantServiceMap>,
  pub variables: ::std::vec::Vec<HomeassistantServiceMap>,
  pub is_event: bool,
}
impl ::std::default::Default for HomeassistantServiceResponse {
  fn default() -> Self {
    HomeassistantServiceResponse {
      service: ::std::default::Default::default(),
      data: ::std::default::Default::default(),
      data_template: ::std::default::Default::default(),
      variables: ::std::default::Default::default(),
      is_event: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HomeassistantServiceResponse_default: HomeassistantServiceResponse = HomeassistantServiceResponse::default();
}
impl ::pb_jelly::Message for HomeassistantServiceResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut service_size = 0;
    if self.service != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.service;
      let l = ::pb_jelly::Message::compute_size(val);
      service_size += ::pb_jelly::wire_format::serialized_length(1);
      service_size += ::pb_jelly::varint::serialized_length(l as u64);
      service_size += l;
    }
    size += service_size;
    let mut data_size = 0;
    for val in &self.data {
      let l = ::pb_jelly::Message::compute_size(val);
      data_size += ::pb_jelly::wire_format::serialized_length(2);
      data_size += ::pb_jelly::varint::serialized_length(l as u64);
      data_size += l;
    }
    size += data_size;
    let mut data_template_size = 0;
    for val in &self.data_template {
      let l = ::pb_jelly::Message::compute_size(val);
      data_template_size += ::pb_jelly::wire_format::serialized_length(3);
      data_template_size += ::pb_jelly::varint::serialized_length(l as u64);
      data_template_size += l;
    }
    size += data_template_size;
    let mut variables_size = 0;
    for val in &self.variables {
      let l = ::pb_jelly::Message::compute_size(val);
      variables_size += ::pb_jelly::wire_format::serialized_length(4);
      variables_size += ::pb_jelly::varint::serialized_length(l as u64);
      variables_size += l;
    }
    size += variables_size;
    let mut is_event_size = 0;
    if self.is_event != <bool as ::std::default::Default>::default() {
      let val = &self.is_event;
      let l = ::pb_jelly::Message::compute_size(val);
      is_event_size += ::pb_jelly::wire_format::serialized_length(5);
      is_event_size += l;
    }
    size += is_event_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.service != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.service;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.data {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.data_template {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.variables {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.is_event != <bool as ::std::default::Default>::default() {
      let val = &self.is_event;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.service != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.service;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.data {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.data_template {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.variables {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.is_event != <bool as ::std::default::Default>::default() {
      let val = &self.is_event;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.service = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: HomeassistantServiceMap = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.data.push(val);
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: HomeassistantServiceMap = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.data_template.push(val);
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeassistantServiceResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: HomeassistantServiceMap = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.variables.push(val);
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "HomeassistantServiceResponse", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.is_event = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for HomeassistantServiceResponse {
  const NAME: &'static str = "HomeassistantServiceResponse";
  const FULL_NAME: &'static str = "espapi.HomeassistantServiceResponse";
}

/// ==================== IMPORT HOME ASSISTANT STATES ====================
/// 1. Client sends SubscribeHomeAssistantStatesRequest
/// 2. Server responds with zero or more SubscribeHomeAssistantStateResponse (async)
/// 3. Client sends HomeAssistantStateResponse for state changes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeHomeAssistantStatesRequest {
}
impl ::std::default::Default for SubscribeHomeAssistantStatesRequest {
  fn default() -> Self {
    SubscribeHomeAssistantStatesRequest {
    }
  }
}
lazy_static! {
  pub static ref SubscribeHomeAssistantStatesRequest_default: SubscribeHomeAssistantStatesRequest = SubscribeHomeAssistantStatesRequest::default();
}
impl ::pb_jelly::Message for SubscribeHomeAssistantStatesRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeHomeAssistantStatesRequest {
  const NAME: &'static str = "SubscribeHomeAssistantStatesRequest";
  const FULL_NAME: &'static str = "espapi.SubscribeHomeAssistantStatesRequest";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubscribeHomeAssistantStateResponse {
  pub entity_id: ::std::string::String,
}
impl ::std::default::Default for SubscribeHomeAssistantStateResponse {
  fn default() -> Self {
    SubscribeHomeAssistantStateResponse {
      entity_id: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SubscribeHomeAssistantStateResponse_default: SubscribeHomeAssistantStateResponse = SubscribeHomeAssistantStateResponse::default();
}
impl ::pb_jelly::Message for SubscribeHomeAssistantStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut entity_id_size = 0;
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      let l = ::pb_jelly::Message::compute_size(val);
      entity_id_size += ::pb_jelly::wire_format::serialized_length(1);
      entity_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      entity_id_size += l;
    }
    size += entity_id_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "SubscribeHomeAssistantStateResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.entity_id = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SubscribeHomeAssistantStateResponse {
  const NAME: &'static str = "SubscribeHomeAssistantStateResponse";
  const FULL_NAME: &'static str = "espapi.SubscribeHomeAssistantStateResponse";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HomeAssistantStateResponse {
  pub entity_id: ::std::string::String,
  pub state: ::std::string::String,
}
impl ::std::default::Default for HomeAssistantStateResponse {
  fn default() -> Self {
    HomeAssistantStateResponse {
      entity_id: ::std::default::Default::default(),
      state: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HomeAssistantStateResponse_default: HomeAssistantStateResponse = HomeAssistantStateResponse::default();
}
impl ::pb_jelly::Message for HomeAssistantStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut entity_id_size = 0;
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      let l = ::pb_jelly::Message::compute_size(val);
      entity_id_size += ::pb_jelly::wire_format::serialized_length(1);
      entity_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      entity_id_size += l;
    }
    size += entity_id_size;
    let mut state_size = 0;
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      let l = ::pb_jelly::Message::compute_size(val);
      state_size += ::pb_jelly::wire_format::serialized_length(2);
      state_size += ::pb_jelly::varint::serialized_length(l as u64);
      state_size += l;
    }
    size += state_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.entity_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.entity_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.state != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.state;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeAssistantStateResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.entity_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "HomeAssistantStateResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.state = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for HomeAssistantStateResponse {
  const NAME: &'static str = "HomeAssistantStateResponse";
  const FULL_NAME: &'static str = "espapi.HomeAssistantStateResponse";
}

/// ==================== IMPORT TIME ====================
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetTimeRequest {
}
impl ::std::default::Default for GetTimeRequest {
  fn default() -> Self {
    GetTimeRequest {
    }
  }
}
lazy_static! {
  pub static ref GetTimeRequest_default: GetTimeRequest = GetTimeRequest::default();
}
impl ::pb_jelly::Message for GetTimeRequest {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for GetTimeRequest {
  const NAME: &'static str = "GetTimeRequest";
  const FULL_NAME: &'static str = "espapi.GetTimeRequest";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetTimeResponse {
  pub epoch_seconds: ::pb_jelly::Fixed32,
}
impl ::std::default::Default for GetTimeResponse {
  fn default() -> Self {
    GetTimeResponse {
      epoch_seconds: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref GetTimeResponse_default: GetTimeResponse = GetTimeResponse::default();
}
impl ::pb_jelly::Message for GetTimeResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut epoch_seconds_size = 0;
    if self.epoch_seconds != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.epoch_seconds;
      let l = ::pb_jelly::Message::compute_size(val);
      epoch_seconds_size += ::pb_jelly::wire_format::serialized_length(1);
      epoch_seconds_size += l;
    }
    size += epoch_seconds_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.epoch_seconds != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.epoch_seconds;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.epoch_seconds != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.epoch_seconds;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "GetTimeResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.epoch_seconds = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for GetTimeResponse {
  const NAME: &'static str = "GetTimeResponse";
  const FULL_NAME: &'static str = "espapi.GetTimeResponse";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesServicesArgument {
  pub name: ::std::string::String,
  pub type_: ServiceArgType,
}
impl ::std::default::Default for ListEntitiesServicesArgument {
  fn default() -> Self {
    ListEntitiesServicesArgument {
      name: ::std::default::Default::default(),
      type_: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesServicesArgument_default: ListEntitiesServicesArgument = ListEntitiesServicesArgument::default();
}
impl ::pb_jelly::Message for ListEntitiesServicesArgument {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut type__size = 0;
    if self.type_ != <ServiceArgType as ::std::default::Default>::default() {
      let val = &self.type_;
      let l = ::pb_jelly::Message::compute_size(val);
      type__size += ::pb_jelly::wire_format::serialized_length(2);
      type__size += l;
    }
    size += type__size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.type_ != <ServiceArgType as ::std::default::Default>::default() {
      let val = &self.type_;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.type_ != <ServiceArgType as ::std::default::Default>::default() {
      let val = &self.type_;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesServicesArgument", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesServicesArgument", 2)?;
          let mut val: ServiceArgType = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.type_ = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesServicesArgument {
  const NAME: &'static str = "ListEntitiesServicesArgument";
  const FULL_NAME: &'static str = "espapi.ListEntitiesServicesArgument";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesServicesResponse {
  pub name: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub args: ::std::vec::Vec<ListEntitiesServicesArgument>,
}
impl ::std::default::Default for ListEntitiesServicesResponse {
  fn default() -> Self {
    ListEntitiesServicesResponse {
      name: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      args: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesServicesResponse_default: ListEntitiesServicesResponse = ListEntitiesServicesResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesServicesResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut args_size = 0;
    for val in &self.args {
      let l = ::pb_jelly::Message::compute_size(val);
      args_size += ::pb_jelly::wire_format::serialized_length(3);
      args_size += ::pb_jelly::varint::serialized_length(l as u64);
      args_size += l;
    }
    size += args_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.args {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.args {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesServicesResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesServicesResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesServicesResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ListEntitiesServicesArgument = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.args.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesServicesResponse {
  const NAME: &'static str = "ListEntitiesServicesResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesServicesResponse";
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteServiceArgument {
  pub bool_: bool,
  pub legacy_int: i32,
  pub float_: f32,
  pub string_: ::std::string::String,
  /// ESPHome 1.14 (api v1.3) make int a signed value
  pub int_: ::pb_jelly::Signed32,
  pub bool_array: ::std::vec::Vec<bool>,
  pub int_array: ::std::vec::Vec<::pb_jelly::Signed32>,
  pub float_array: ::std::vec::Vec<f32>,
  pub string_array: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for ExecuteServiceArgument {
  fn default() -> Self {
    ExecuteServiceArgument {
      bool_: ::std::default::Default::default(),
      legacy_int: ::std::default::Default::default(),
      float_: ::std::default::Default::default(),
      string_: ::std::default::Default::default(),
      int_: ::std::default::Default::default(),
      bool_array: ::std::default::Default::default(),
      int_array: ::std::default::Default::default(),
      float_array: ::std::default::Default::default(),
      string_array: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ExecuteServiceArgument_default: ExecuteServiceArgument = ExecuteServiceArgument::default();
}
impl ::pb_jelly::Message for ExecuteServiceArgument {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut bool__size = 0;
    if self.bool_ != <bool as ::std::default::Default>::default() {
      let val = &self.bool_;
      let l = ::pb_jelly::Message::compute_size(val);
      bool__size += ::pb_jelly::wire_format::serialized_length(1);
      bool__size += l;
    }
    size += bool__size;
    let mut legacy_int_size = 0;
    if self.legacy_int != <i32 as ::std::default::Default>::default() {
      let val = &self.legacy_int;
      let l = ::pb_jelly::Message::compute_size(val);
      legacy_int_size += ::pb_jelly::wire_format::serialized_length(2);
      legacy_int_size += l;
    }
    size += legacy_int_size;
    let mut float__size = 0;
    if self.float_ != <f32 as ::std::default::Default>::default() {
      let val = &self.float_;
      let l = ::pb_jelly::Message::compute_size(val);
      float__size += ::pb_jelly::wire_format::serialized_length(3);
      float__size += l;
    }
    size += float__size;
    let mut string__size = 0;
    if self.string_ != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.string_;
      let l = ::pb_jelly::Message::compute_size(val);
      string__size += ::pb_jelly::wire_format::serialized_length(4);
      string__size += ::pb_jelly::varint::serialized_length(l as u64);
      string__size += l;
    }
    size += string__size;
    let mut int__size = 0;
    if self.int_ != <::pb_jelly::Signed32 as ::std::default::Default>::default() {
      let val = &self.int_;
      let l = ::pb_jelly::Message::compute_size(val);
      int__size += ::pb_jelly::wire_format::serialized_length(5);
      int__size += l;
    }
    size += int__size;
    let mut bool_array_size = 0;
    for val in &self.bool_array {
      let l = ::pb_jelly::Message::compute_size(val);
      bool_array_size += l;
    }
    if !self.bool_array.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(6);
      size += ::pb_jelly::varint::serialized_length(bool_array_size as u64);
    }
    size += bool_array_size;
    let mut int_array_size = 0;
    for val in &self.int_array {
      let l = ::pb_jelly::Message::compute_size(val);
      int_array_size += l;
    }
    if !self.int_array.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(7);
      size += ::pb_jelly::varint::serialized_length(int_array_size as u64);
    }
    size += int_array_size;
    let mut float_array_size = 0;
    for val in &self.float_array {
      let l = ::pb_jelly::Message::compute_size(val);
      float_array_size += l;
    }
    if !self.float_array.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(8);
      size += ::pb_jelly::varint::serialized_length(float_array_size as u64);
    }
    size += float_array_size;
    let mut string_array_size = 0;
    for val in &self.string_array {
      let l = ::pb_jelly::Message::compute_size(val);
      string_array_size += ::pb_jelly::wire_format::serialized_length(9);
      string_array_size += ::pb_jelly::varint::serialized_length(l as u64);
      string_array_size += l;
    }
    size += string_array_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.bool_ != <bool as ::std::default::Default>::default() {
      let val = &self.bool_;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.legacy_int != <i32 as ::std::default::Default>::default() {
      let val = &self.legacy_int;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.float_ != <f32 as ::std::default::Default>::default() {
      let val = &self.float_;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.string_ != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.string_;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.int_ != <::pb_jelly::Signed32 as ::std::default::Default>::default() {
      let val = &self.int_;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.bool_array {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.int_array {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.float_array {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.string_array {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.bool_ != <bool as ::std::default::Default>::default() {
      let val = &self.bool_;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.legacy_int != <i32 as ::std::default::Default>::default() {
      let val = &self.legacy_int;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.float_ != <f32 as ::std::default::Default>::default() {
      let val = &self.float_;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.string_ != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.string_;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.int_ != <::pb_jelly::Signed32 as ::std::default::Default>::default() {
      let val = &self.int_;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.bool_array.is_empty() {
      let mut size = 0;
      for val in &self.bool_array {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.bool_array {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.int_array.is_empty() {
      let mut size = 0;
      for val in &self.int_array {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.int_array {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.float_array.is_empty() {
      let mut size = 0;
      for val in &self.float_array {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.float_array {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.string_array {
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ExecuteServiceArgument", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.bool_ = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ExecuteServiceArgument", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.legacy_int = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ExecuteServiceArgument", 3)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.float_ = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ExecuteServiceArgument", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.string_ = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ExecuteServiceArgument", 5)?;
          let mut val: ::pb_jelly::Signed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.int_ = val;
        }
        6 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: bool = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.bool_array.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ExecuteServiceArgument", 6)?;
              let mut val: bool = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.bool_array.push(val);
            }
          }
        }
        7 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: ::pb_jelly::Signed32 = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.int_array.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ExecuteServiceArgument", 7)?;
              let mut val: ::pb_jelly::Signed32 = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.int_array.push(val);
            }
          }
        }
        8 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: f32 = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.float_array.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ExecuteServiceArgument", 8)?;
              let mut val: f32 = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.float_array.push(val);
            }
          }
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ExecuteServiceArgument", 9)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.string_array.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ExecuteServiceArgument {
  const NAME: &'static str = "ExecuteServiceArgument";
  const FULL_NAME: &'static str = "espapi.ExecuteServiceArgument";
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteServiceRequest {
  pub key: ::pb_jelly::Fixed32,
  pub args: ::std::vec::Vec<ExecuteServiceArgument>,
}
impl ::std::default::Default for ExecuteServiceRequest {
  fn default() -> Self {
    ExecuteServiceRequest {
      key: ::std::default::Default::default(),
      args: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ExecuteServiceRequest_default: ExecuteServiceRequest = ExecuteServiceRequest::default();
}
impl ::pb_jelly::Message for ExecuteServiceRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut args_size = 0;
    for val in &self.args {
      let l = ::pb_jelly::Message::compute_size(val);
      args_size += ::pb_jelly::wire_format::serialized_length(2);
      args_size += ::pb_jelly::varint::serialized_length(l as u64);
      args_size += l;
    }
    size += args_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.args {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.args {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ExecuteServiceRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ExecuteServiceRequest", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ExecuteServiceArgument = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.args.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ExecuteServiceRequest {
  const NAME: &'static str = "ExecuteServiceRequest";
  const FULL_NAME: &'static str = "espapi.ExecuteServiceRequest";
}

/// ==================== CAMERA ====================
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListEntitiesCameraResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
}
impl ::std::default::Default for ListEntitiesCameraResponse {
  fn default() -> Self {
    ListEntitiesCameraResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesCameraResponse_default: ListEntitiesCameraResponse = ListEntitiesCameraResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesCameraResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCameraResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesCameraResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCameraResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesCameraResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesCameraResponse {
  const NAME: &'static str = "ListEntitiesCameraResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesCameraResponse";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CameraImageResponse {
  pub key: ::pb_jelly::Fixed32,
  pub data: ::std::vec::Vec<u8>,
  pub done: bool,
}
impl ::std::default::Default for CameraImageResponse {
  fn default() -> Self {
    CameraImageResponse {
      key: ::std::default::Default::default(),
      data: ::std::default::Default::default(),
      done: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CameraImageResponse_default: CameraImageResponse = CameraImageResponse::default();
}
impl ::pb_jelly::Message for CameraImageResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut data_size = 0;
    if self.data != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.data;
      let l = ::pb_jelly::Message::compute_size(val);
      data_size += ::pb_jelly::wire_format::serialized_length(2);
      data_size += ::pb_jelly::varint::serialized_length(l as u64);
      data_size += l;
    }
    size += data_size;
    let mut done_size = 0;
    if self.done != <bool as ::std::default::Default>::default() {
      let val = &self.done;
      let l = ::pb_jelly::Message::compute_size(val);
      done_size += ::pb_jelly::wire_format::serialized_length(3);
      done_size += l;
    }
    size += done_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.data != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.data;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.done != <bool as ::std::default::Default>::default() {
      let val = &self.done;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.data != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.data;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.done != <bool as ::std::default::Default>::default() {
      let val = &self.done;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "CameraImageResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "CameraImageResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::vec::Vec<u8> = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.data = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CameraImageResponse", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.done = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for CameraImageResponse {
  const NAME: &'static str = "CameraImageResponse";
  const FULL_NAME: &'static str = "espapi.CameraImageResponse";
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CameraImageRequest {
  pub single: bool,
  pub stream: bool,
}
impl ::std::default::Default for CameraImageRequest {
  fn default() -> Self {
    CameraImageRequest {
      single: ::std::default::Default::default(),
      stream: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CameraImageRequest_default: CameraImageRequest = CameraImageRequest::default();
}
impl ::pb_jelly::Message for CameraImageRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut single_size = 0;
    if self.single != <bool as ::std::default::Default>::default() {
      let val = &self.single;
      let l = ::pb_jelly::Message::compute_size(val);
      single_size += ::pb_jelly::wire_format::serialized_length(1);
      single_size += l;
    }
    size += single_size;
    let mut stream_size = 0;
    if self.stream != <bool as ::std::default::Default>::default() {
      let val = &self.stream;
      let l = ::pb_jelly::Message::compute_size(val);
      stream_size += ::pb_jelly::wire_format::serialized_length(2);
      stream_size += l;
    }
    size += stream_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.single != <bool as ::std::default::Default>::default() {
      let val = &self.single;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.stream != <bool as ::std::default::Default>::default() {
      let val = &self.stream;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.single != <bool as ::std::default::Default>::default() {
      let val = &self.single;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.stream != <bool as ::std::default::Default>::default() {
      let val = &self.stream;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CameraImageRequest", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.single = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "CameraImageRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.stream = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for CameraImageRequest {
  const NAME: &'static str = "CameraImageRequest";
  const FULL_NAME: &'static str = "espapi.CameraImageRequest";
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListEntitiesClimateResponse {
  pub object_id: ::std::string::String,
  pub key: ::pb_jelly::Fixed32,
  pub name: ::std::string::String,
  pub unique_id: ::std::string::String,
  pub supports_current_temperature: bool,
  pub supports_two_point_target_temperature: bool,
  pub supported_modes: ::std::vec::Vec<ClimateMode>,
  pub visual_min_temperature: f32,
  pub visual_max_temperature: f32,
  pub visual_temperature_step: f32,
  pub supports_away: bool,
  pub supports_action: bool,
  pub supported_fan_modes: ::std::vec::Vec<ClimateFanMode>,
  pub supported_swing_modes: ::std::vec::Vec<ClimateSwingMode>,
}
impl ::std::default::Default for ListEntitiesClimateResponse {
  fn default() -> Self {
    ListEntitiesClimateResponse {
      object_id: ::std::default::Default::default(),
      key: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      unique_id: ::std::default::Default::default(),
      supports_current_temperature: ::std::default::Default::default(),
      supports_two_point_target_temperature: ::std::default::Default::default(),
      supported_modes: ::std::default::Default::default(),
      visual_min_temperature: ::std::default::Default::default(),
      visual_max_temperature: ::std::default::Default::default(),
      visual_temperature_step: ::std::default::Default::default(),
      supports_away: ::std::default::Default::default(),
      supports_action: ::std::default::Default::default(),
      supported_fan_modes: ::std::default::Default::default(),
      supported_swing_modes: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListEntitiesClimateResponse_default: ListEntitiesClimateResponse = ListEntitiesClimateResponse::default();
}
impl ::pb_jelly::Message for ListEntitiesClimateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut object_id_size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      let l = ::pb_jelly::Message::compute_size(val);
      object_id_size += ::pb_jelly::wire_format::serialized_length(1);
      object_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      object_id_size += l;
    }
    size += object_id_size;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(2);
      key_size += l;
    }
    size += key_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(3);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut unique_id_size = 0;
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      let l = ::pb_jelly::Message::compute_size(val);
      unique_id_size += ::pb_jelly::wire_format::serialized_length(4);
      unique_id_size += ::pb_jelly::varint::serialized_length(l as u64);
      unique_id_size += l;
    }
    size += unique_id_size;
    let mut supports_current_temperature_size = 0;
    if self.supports_current_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_current_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_current_temperature_size += ::pb_jelly::wire_format::serialized_length(5);
      supports_current_temperature_size += l;
    }
    size += supports_current_temperature_size;
    let mut supports_two_point_target_temperature_size = 0;
    if self.supports_two_point_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_two_point_target_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_two_point_target_temperature_size += ::pb_jelly::wire_format::serialized_length(6);
      supports_two_point_target_temperature_size += l;
    }
    size += supports_two_point_target_temperature_size;
    let mut supported_modes_size = 0;
    for val in &self.supported_modes {
      let l = ::pb_jelly::Message::compute_size(val);
      supported_modes_size += l;
    }
    if !self.supported_modes.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(7);
      size += ::pb_jelly::varint::serialized_length(supported_modes_size as u64);
    }
    size += supported_modes_size;
    let mut visual_min_temperature_size = 0;
    if self.visual_min_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_min_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      visual_min_temperature_size += ::pb_jelly::wire_format::serialized_length(8);
      visual_min_temperature_size += l;
    }
    size += visual_min_temperature_size;
    let mut visual_max_temperature_size = 0;
    if self.visual_max_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_max_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      visual_max_temperature_size += ::pb_jelly::wire_format::serialized_length(9);
      visual_max_temperature_size += l;
    }
    size += visual_max_temperature_size;
    let mut visual_temperature_step_size = 0;
    if self.visual_temperature_step != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_temperature_step;
      let l = ::pb_jelly::Message::compute_size(val);
      visual_temperature_step_size += ::pb_jelly::wire_format::serialized_length(10);
      visual_temperature_step_size += l;
    }
    size += visual_temperature_step_size;
    let mut supports_away_size = 0;
    if self.supports_away != <bool as ::std::default::Default>::default() {
      let val = &self.supports_away;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_away_size += ::pb_jelly::wire_format::serialized_length(11);
      supports_away_size += l;
    }
    size += supports_away_size;
    let mut supports_action_size = 0;
    if self.supports_action != <bool as ::std::default::Default>::default() {
      let val = &self.supports_action;
      let l = ::pb_jelly::Message::compute_size(val);
      supports_action_size += ::pb_jelly::wire_format::serialized_length(12);
      supports_action_size += l;
    }
    size += supports_action_size;
    let mut supported_fan_modes_size = 0;
    for val in &self.supported_fan_modes {
      let l = ::pb_jelly::Message::compute_size(val);
      supported_fan_modes_size += l;
    }
    if !self.supported_fan_modes.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(13);
      size += ::pb_jelly::varint::serialized_length(supported_fan_modes_size as u64);
    }
    size += supported_fan_modes_size;
    let mut supported_swing_modes_size = 0;
    for val in &self.supported_swing_modes {
      let l = ::pb_jelly::Message::compute_size(val);
      supported_swing_modes_size += l;
    }
    if !self.supported_swing_modes.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(14);
      size += ::pb_jelly::varint::serialized_length(supported_swing_modes_size as u64);
    }
    size += supported_swing_modes_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_current_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_current_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_two_point_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_two_point_target_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.supported_modes {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.visual_min_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_min_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.visual_max_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_max_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.visual_temperature_step != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_temperature_step;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_away != <bool as ::std::default::Default>::default() {
      let val = &self.supports_away;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.supports_action != <bool as ::std::default::Default>::default() {
      let val = &self.supports_action;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.supported_fan_modes {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    for val in &self.supported_swing_modes {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.object_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.object_id;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.unique_id != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.unique_id;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_current_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_current_temperature;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_two_point_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.supports_two_point_target_temperature;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.supported_modes.is_empty() {
      let mut size = 0;
      for val in &self.supported_modes {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.supported_modes {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.visual_min_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_min_temperature;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.visual_max_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_max_temperature;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.visual_temperature_step != <f32 as ::std::default::Default>::default() {
      let val = &self.visual_temperature_step;
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_away != <bool as ::std::default::Default>::default() {
      let val = &self.supports_away;
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.supports_action != <bool as ::std::default::Default>::default() {
      let val = &self.supports_action;
      ::pb_jelly::wire_format::write(12, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.supported_fan_modes.is_empty() {
      let mut size = 0;
      for val in &self.supported_fan_modes {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(13, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.supported_fan_modes {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.supported_swing_modes.is_empty() {
      let mut size = 0;
      for val in &self.supported_swing_modes {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(14, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.supported_swing_modes {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesClimateResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.object_id = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesClimateResponse", 2)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesClimateResponse", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ListEntitiesClimateResponse", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.unique_id = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_current_temperature = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_two_point_target_temperature = val;
        }
        7 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: ClimateMode = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.supported_modes.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 7)?;
              let mut val: ClimateMode = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.supported_modes.push(val);
            }
          }
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesClimateResponse", 8)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.visual_min_temperature = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesClimateResponse", 9)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.visual_max_temperature = val;
        }
        10 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ListEntitiesClimateResponse", 10)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.visual_temperature_step = val;
        }
        11 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 11)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_away = val;
        }
        12 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 12)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.supports_action = val;
        }
        13 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: ClimateFanMode = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.supported_fan_modes.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 13)?;
              let mut val: ClimateFanMode = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.supported_fan_modes.push(val);
            }
          }
        }
        14 => {
          match typ {
            ::pb_jelly::wire_format::Type::LengthDelimited => {
              let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: ClimateSwingMode = ::std::default::Default::default();
                ::pb_jelly::Message::deserialize(&mut val, &mut vals)?;
                self.supported_swing_modes.push(val);
              }
            }
            _ => {
              ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ListEntitiesClimateResponse", 14)?;
              let mut val: ClimateSwingMode = ::std::default::Default::default();
              ::pb_jelly::Message::deserialize(&mut val, buf)?;
              self.supported_swing_modes.push(val);
            }
          }
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ListEntitiesClimateResponse {
  const NAME: &'static str = "ListEntitiesClimateResponse";
  const FULL_NAME: &'static str = "espapi.ListEntitiesClimateResponse";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClimateStateResponse {
  pub key: ::pb_jelly::Fixed32,
  pub mode: ClimateMode,
  pub current_temperature: f32,
  pub target_temperature: f32,
  pub target_temperature_low: f32,
  pub target_temperature_high: f32,
  pub away: bool,
  pub action: ClimateAction,
  pub fan_mode: ClimateFanMode,
  pub swing_mode: ClimateSwingMode,
}
impl ::std::default::Default for ClimateStateResponse {
  fn default() -> Self {
    ClimateStateResponse {
      key: ::std::default::Default::default(),
      mode: ::std::default::Default::default(),
      current_temperature: ::std::default::Default::default(),
      target_temperature: ::std::default::Default::default(),
      target_temperature_low: ::std::default::Default::default(),
      target_temperature_high: ::std::default::Default::default(),
      away: ::std::default::Default::default(),
      action: ::std::default::Default::default(),
      fan_mode: ::std::default::Default::default(),
      swing_mode: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ClimateStateResponse_default: ClimateStateResponse = ClimateStateResponse::default();
}
impl ::pb_jelly::Message for ClimateStateResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut mode_size = 0;
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      let l = ::pb_jelly::Message::compute_size(val);
      mode_size += ::pb_jelly::wire_format::serialized_length(2);
      mode_size += l;
    }
    size += mode_size;
    let mut current_temperature_size = 0;
    if self.current_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.current_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      current_temperature_size += ::pb_jelly::wire_format::serialized_length(3);
      current_temperature_size += l;
    }
    size += current_temperature_size;
    let mut target_temperature_size = 0;
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_size += ::pb_jelly::wire_format::serialized_length(4);
      target_temperature_size += l;
    }
    size += target_temperature_size;
    let mut target_temperature_low_size = 0;
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_low_size += ::pb_jelly::wire_format::serialized_length(5);
      target_temperature_low_size += l;
    }
    size += target_temperature_low_size;
    let mut target_temperature_high_size = 0;
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_high_size += ::pb_jelly::wire_format::serialized_length(6);
      target_temperature_high_size += l;
    }
    size += target_temperature_high_size;
    let mut away_size = 0;
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      let l = ::pb_jelly::Message::compute_size(val);
      away_size += ::pb_jelly::wire_format::serialized_length(7);
      away_size += l;
    }
    size += away_size;
    let mut action_size = 0;
    if self.action != <ClimateAction as ::std::default::Default>::default() {
      let val = &self.action;
      let l = ::pb_jelly::Message::compute_size(val);
      action_size += ::pb_jelly::wire_format::serialized_length(8);
      action_size += l;
    }
    size += action_size;
    let mut fan_mode_size = 0;
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      fan_mode_size += ::pb_jelly::wire_format::serialized_length(9);
      fan_mode_size += l;
    }
    size += fan_mode_size;
    let mut swing_mode_size = 0;
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      swing_mode_size += ::pb_jelly::wire_format::serialized_length(10);
      swing_mode_size += l;
    }
    size += swing_mode_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.current_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.current_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.action != <ClimateAction as ::std::default::Default>::default() {
      let val = &self.action;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.current_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.current_temperature;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.action != <ClimateAction as ::std::default::Default>::default() {
      let val = &self.action;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateStateResponse", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateStateResponse", 2)?;
          let mut val: ClimateMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.mode = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateStateResponse", 3)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.current_temperature = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateStateResponse", 4)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateStateResponse", 5)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature_low = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateStateResponse", 6)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature_high = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateStateResponse", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.away = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateStateResponse", 8)?;
          let mut val: ClimateAction = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.action = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateStateResponse", 9)?;
          let mut val: ClimateFanMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.fan_mode = val;
        }
        10 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateStateResponse", 10)?;
          let mut val: ClimateSwingMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.swing_mode = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ClimateStateResponse {
  const NAME: &'static str = "ClimateStateResponse";
  const FULL_NAME: &'static str = "espapi.ClimateStateResponse";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClimateCommandRequest {
  pub key: ::pb_jelly::Fixed32,
  pub has_mode: bool,
  pub mode: ClimateMode,
  pub has_target_temperature: bool,
  pub target_temperature: f32,
  pub has_target_temperature_low: bool,
  pub target_temperature_low: f32,
  pub has_target_temperature_high: bool,
  pub target_temperature_high: f32,
  pub has_away: bool,
  pub away: bool,
  pub has_fan_mode: bool,
  pub fan_mode: ClimateFanMode,
  pub has_swing_mode: bool,
  pub swing_mode: ClimateSwingMode,
}
impl ::std::default::Default for ClimateCommandRequest {
  fn default() -> Self {
    ClimateCommandRequest {
      key: ::std::default::Default::default(),
      has_mode: ::std::default::Default::default(),
      mode: ::std::default::Default::default(),
      has_target_temperature: ::std::default::Default::default(),
      target_temperature: ::std::default::Default::default(),
      has_target_temperature_low: ::std::default::Default::default(),
      target_temperature_low: ::std::default::Default::default(),
      has_target_temperature_high: ::std::default::Default::default(),
      target_temperature_high: ::std::default::Default::default(),
      has_away: ::std::default::Default::default(),
      away: ::std::default::Default::default(),
      has_fan_mode: ::std::default::Default::default(),
      fan_mode: ::std::default::Default::default(),
      has_swing_mode: ::std::default::Default::default(),
      swing_mode: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ClimateCommandRequest_default: ClimateCommandRequest = ClimateCommandRequest::default();
}
impl ::pb_jelly::Message for ClimateCommandRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb_jelly::Message::compute_size(val);
      key_size += ::pb_jelly::wire_format::serialized_length(1);
      key_size += l;
    }
    size += key_size;
    let mut has_mode_size = 0;
    if self.has_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      has_mode_size += ::pb_jelly::wire_format::serialized_length(2);
      has_mode_size += l;
    }
    size += has_mode_size;
    let mut mode_size = 0;
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      let l = ::pb_jelly::Message::compute_size(val);
      mode_size += ::pb_jelly::wire_format::serialized_length(3);
      mode_size += l;
    }
    size += mode_size;
    let mut has_target_temperature_size = 0;
    if self.has_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      has_target_temperature_size += ::pb_jelly::wire_format::serialized_length(4);
      has_target_temperature_size += l;
    }
    size += has_target_temperature_size;
    let mut target_temperature_size = 0;
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_size += ::pb_jelly::wire_format::serialized_length(5);
      target_temperature_size += l;
    }
    size += target_temperature_size;
    let mut has_target_temperature_low_size = 0;
    if self.has_target_temperature_low != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_low;
      let l = ::pb_jelly::Message::compute_size(val);
      has_target_temperature_low_size += ::pb_jelly::wire_format::serialized_length(6);
      has_target_temperature_low_size += l;
    }
    size += has_target_temperature_low_size;
    let mut target_temperature_low_size = 0;
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_low_size += ::pb_jelly::wire_format::serialized_length(7);
      target_temperature_low_size += l;
    }
    size += target_temperature_low_size;
    let mut has_target_temperature_high_size = 0;
    if self.has_target_temperature_high != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_high;
      let l = ::pb_jelly::Message::compute_size(val);
      has_target_temperature_high_size += ::pb_jelly::wire_format::serialized_length(8);
      has_target_temperature_high_size += l;
    }
    size += has_target_temperature_high_size;
    let mut target_temperature_high_size = 0;
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      let l = ::pb_jelly::Message::compute_size(val);
      target_temperature_high_size += ::pb_jelly::wire_format::serialized_length(9);
      target_temperature_high_size += l;
    }
    size += target_temperature_high_size;
    let mut has_away_size = 0;
    if self.has_away != <bool as ::std::default::Default>::default() {
      let val = &self.has_away;
      let l = ::pb_jelly::Message::compute_size(val);
      has_away_size += ::pb_jelly::wire_format::serialized_length(10);
      has_away_size += l;
    }
    size += has_away_size;
    let mut away_size = 0;
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      let l = ::pb_jelly::Message::compute_size(val);
      away_size += ::pb_jelly::wire_format::serialized_length(11);
      away_size += l;
    }
    size += away_size;
    let mut has_fan_mode_size = 0;
    if self.has_fan_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_fan_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      has_fan_mode_size += ::pb_jelly::wire_format::serialized_length(12);
      has_fan_mode_size += l;
    }
    size += has_fan_mode_size;
    let mut fan_mode_size = 0;
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      fan_mode_size += ::pb_jelly::wire_format::serialized_length(13);
      fan_mode_size += l;
    }
    size += fan_mode_size;
    let mut has_swing_mode_size = 0;
    if self.has_swing_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_swing_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      has_swing_mode_size += ::pb_jelly::wire_format::serialized_length(14);
      has_swing_mode_size += l;
    }
    size += has_swing_mode_size;
    let mut swing_mode_size = 0;
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      let l = ::pb_jelly::Message::compute_size(val);
      swing_mode_size += ::pb_jelly::wire_format::serialized_length(15);
      swing_mode_size += l;
    }
    size += swing_mode_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_target_temperature_low != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_low;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_target_temperature_high != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_high;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_away != <bool as ::std::default::Default>::default() {
      let val = &self.has_away;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_fan_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_fan_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.has_swing_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_swing_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::pb_jelly::Fixed32 as ::std::default::Default>::default() {
      let val = &self.key;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_mode;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.mode != <ClimateMode as ::std::default::Default>::default() {
      let val = &self.mode;
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_target_temperature != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature;
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature;
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_target_temperature_low != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_low;
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature_low != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_low;
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_target_temperature_high != <bool as ::std::default::Default>::default() {
      let val = &self.has_target_temperature_high;
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.target_temperature_high != <f32 as ::std::default::Default>::default() {
      let val = &self.target_temperature_high;
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Fixed32, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_away != <bool as ::std::default::Default>::default() {
      let val = &self.has_away;
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.away != <bool as ::std::default::Default>::default() {
      let val = &self.away;
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_fan_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_fan_mode;
      ::pb_jelly::wire_format::write(12, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.fan_mode != <ClimateFanMode as ::std::default::Default>::default() {
      let val = &self.fan_mode;
      ::pb_jelly::wire_format::write(13, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.has_swing_mode != <bool as ::std::default::Default>::default() {
      let val = &self.has_swing_mode;
      ::pb_jelly::wire_format::write(14, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.swing_mode != <ClimateSwingMode as ::std::default::Default>::default() {
      let val = &self.swing_mode;
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateCommandRequest", 1)?;
          let mut val: ::pb_jelly::Fixed32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.key = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_mode = val;
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 3)?;
          let mut val: ClimateMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.mode = val;
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_target_temperature = val;
        }
        5 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateCommandRequest", 5)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature = val;
        }
        6 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_target_temperature_low = val;
        }
        7 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateCommandRequest", 7)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature_low = val;
        }
        8 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_target_temperature_high = val;
        }
        9 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Fixed32, "ClimateCommandRequest", 9)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.target_temperature_high = val;
        }
        10 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 10)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_away = val;
        }
        11 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 11)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.away = val;
        }
        12 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 12)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_fan_mode = val;
        }
        13 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 13)?;
          let mut val: ClimateFanMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.fan_mode = val;
        }
        14 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 14)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.has_swing_mode = val;
        }
        15 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::Varint, "ClimateCommandRequest", 15)?;
          let mut val: ClimateSwingMode = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, buf)?;
          self.swing_mode = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ClimateCommandRequest {
  const NAME: &'static str = "ClimateCommandRequest";
  const FULL_NAME: &'static str = "espapi.ClimateCommandRequest";
}

