#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UErr {
    NoConError = 0,
    HostErr = 1,
    ConSockErr = 2,
    ConError = 3,
    ConSslErr = 4,
    ConImpossible = 5,
    NewLocation = 6,
    FtpOk = 7,
    FtpLoginC = 8,
    FtpLogRefused = 9,
    FtpPortErr = 10,
    FtpSysErr = 11,
    FtpNsfod = 12,
    FtpUnknownType = 13,
    FtpRerr = 14,
    FtpSrvErr = 15,
    FtpRetrInt = 16,
    FtpRestFail = 17,
    UrlError = 18,
    FopenErr = 19,
    FopenExclErr = 20,
    FwriteErr = 21,
    Heof = 22,
    GatewayTimeout = 23,
    Herr = 24,
    RetrOk = 25,
    RecLevelExc = 26,
    WrongCode = 27,
    FtpInvPasv = 28,
    FtpNoPasv = 29,
    FtpNoPbsz = 30,
    FtpNoProt = 31,
    FtpNoAuth = 32,
    ContNotSupported = 33,
    RetrUnneeded = 34,
    RetrFinished = 35,
    ReadErr = 36,
    TryLimExc = 37,
    FileBadFile = 38,
    RangeErr = 39,
    RetrBadPattern = 40,
    ProxErr = 41,
    AuthFailed = 42,
    QuoteExc = 43,
    WriteFailed = 44,
    SslInitFailed = 45,
    VerifCertErr = 46,
    UnlinkErr = 47,
    NewLocationKeepPost = 48,
    CloseFailed = 49,
    AttrMissing = 50,
    UnknownAttr = 51,
    WarcErr = 52,
    WarcTmpFopenErr = 53,
    WarcTmpFwriteErr = 54,
    TimeconvErr = 55,
    MetalinkParseError = 56,
    MetalinkRetrError = 57,
    MetalinkChksumError = 58,
    MetalinkSigError = 59,
    MetalinkMissingResource = 60,
    RetrWithMetalink = 61,
    MetalinkSizeError = 62,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum WgetExitStatus {
    Success = 0,
    GenericError = 1,
    ParseError = 2,
    IoFail = 3,
    NetworkFail = 4,
    SslAuthFail = 5,
    ServerAuthFail = 6,
    ProtocolError = 7,
    ServerError = 8,
    Unknown = 9,
}

static FINAL_EXIT_STATUS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(WgetExitStatus::Success as i32);

fn get_status_for_err(err: UErr) -> WgetExitStatus {
    match err {
        UErr::RetrOk => WgetExitStatus::Success,
        UErr::FopenErr
        | UErr::FopenExclErr
        | UErr::FwriteErr
        | UErr::WriteFailed
        | UErr::UnlinkErr
        | UErr::CloseFailed
        | UErr::FileBadFile => WgetExitStatus::IoFail,
        UErr::NoConError
        | UErr::HostErr
        | UErr::ConSockErr
        | UErr::ConError
        | UErr::ConSslErr
        | UErr::ConImpossible
        | UErr::FtpRerr
        | UErr::FtpInvPasv
        | UErr::ReadErr
        | UErr::TryLimExc => WgetExitStatus::NetworkFail,
        UErr::VerifCertErr => WgetExitStatus::SslAuthFail,
        UErr::FtpLoginC | UErr::FtpLogRefused | UErr::AuthFailed => WgetExitStatus::ServerAuthFail,
        UErr::Heof | UErr::Herr | UErr::AttrMissing => WgetExitStatus::ProtocolError,
        UErr::RecLevelExc
        | UErr::FtpPortErr
        | UErr::FtpSysErr
        | UErr::FtpNsfod
        | UErr::FtpUnknownType
        | UErr::FtpSrvErr
        | UErr::FtpRetrInt
        | UErr::FtpRestFail
        | UErr::FtpNoPasv
        | UErr::ContNotSupported
        | UErr::RangeErr
        | UErr::RetrBadPattern
        | UErr::ProxErr
        | UErr::GatewayTimeout => WgetExitStatus::ServerError,
        _ => WgetExitStatus::Unknown,
    }
}

pub fn inform_exit_status(err: UErr) {
    let new_status = get_status_for_err(err) as i32;
    FINAL_EXIT_STATUS.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| {
        if new_status != WgetExitStatus::Success as i32 && (current == WgetExitStatus::Success as i32 || new_status < current) {
            Some(new_status)
        } else {
            None
        }
    }).ok();
}

pub fn get_exit_status() -> i32 {
    let status = FINAL_EXIT_STATUS.load(std::sync::atomic::Ordering::SeqCst);
    if status == WgetExitStatus::Unknown as i32 {
        1
    } else {
        status
    }
}