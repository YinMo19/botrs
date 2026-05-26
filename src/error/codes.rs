pub const CodeNeedReConnect: i32 = 9000;
pub const CodeInvalidSession: i32 = 9001;
pub const CodeURLInvalid: i32 = 9002;
pub const CodeNotFoundOpenAPI: i32 = 9003;
pub const CodeSessionLimit: i32 = 9004;
pub const CodeConnCloseCantResume: i32 = 9005;
pub const CodeConnCloseCantIdentify: i32 = 9006;
pub const CodePagerIsNil: i32 = 9007;

pub const WSCodeBackendUnknownError: u16 = 4000;
pub const WSCodeBackendUnknownOpCode: u16 = 4001;
pub const WSCodeBackendDecodeError: u16 = 4002;
pub const WSCodeBackendNotAuthenticate: u16 = 4003;
pub const WSCodeBackendAuthenticationFail: u16 = 4004;
pub const WSCodeBackendAlreadyAuthenticate: u16 = 4005;
pub const WSCodeBackendSessionNoLongerValid: u16 = 4006;
pub const WSCodeBackendInvalidSeq: u16 = 4007;
pub const WSCodeBackendRateLimit: u16 = 4008;
pub const WSCodeBackendSessionTimeOut: u16 = 4009;
pub const WSCodeBackendInvalidShard: u16 = 4010;
pub const WSCodeBackendShardingRequired: u16 = 4011;
pub const WSCodeBackendInvalidAPIVersion: u16 = 4012;
pub const WSCodeBackendInvalidIntents: u16 = 4013;
pub const WSCodeBackendDisallowdIntents: u16 = 4014;
pub const WSCodeBackendBotOffline: u16 = 4914;
pub const WSCodeBackendBotBanned: u16 = 4915;

pub const APICodeTokenExpireOrNotExist: u32 = 11244;
