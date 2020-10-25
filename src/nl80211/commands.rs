use neli::consts::Cmd;
use neli::{impl_var, impl_var_base, impl_var_trait};

impl_var_trait!(
    /// Supported nl80211 commands.
    ///
    /// nl80211_commands enum from https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h.
    Command, u8, Cmd,

    Unspec => 0,

    GetWiphy => 1,
    SetWiphy => 2,
    NewWiphy => 3,
    DelWiphy => 4,

    GetInterface => 5,
    SetInterface => 6,
    NewInterface => 7,
    DelInterface => 8,

    GetKey => 9,
    SetKey => 10,
    NewKey => 11,
    DelKey => 12,

    GetBeacon => 13,
    SetBeacon => 14,
    StartAp => 15,
    NewBeacon => 15, // => StartAp
    StopAp => 16,
    DelBeacon => 16,  // => StopAp

    GetStation => 17,
    SetStation => 18,
    NewStation => 19,
    DelStation => 20,

    GetMpath => 21,
    SetMpath => 22,
    NewMpath => 23,
    DelMpath => 24,

    SetBss => 25,

    SetReg => 26,
    ReqSetReg => 27,

    GetMeshConfig => 28,
    SetMeshConfig => 29,

    SetMgmtExtraIe => 30, // reserved; not used

    GetReg => 31,

    GetScan => 32,
    TriggerScan => 33,
    NewScanResults => 34,
    ScanAborted => 35,

    RegChange => 36,

    Authenticate => 37,
    Associate => 38,
    Deauthenticate => 39,
    Disassociate => 40,

    MichaelMicFailure => 41,

    RegBeaconHint => 42,

    JoinIbss => 43,
    LeaveIbss => 44,

    Testmode => 45,

    Connect => 46,
    Roam => 47,
    Disconnect => 48,

    SetWiphyNetns => 49,

    GetSurvey => 50,
    NewSurveyResults => 51,

    SetPmksa => 52,
    DelPmksa => 53,
    FlushPmksa => 54,

    RemainOnChannel => 55,
    CancelRemainOnChannel => 56,

    SetTxBitrateMask => 57,

    RegisterFrame => 58,
    RegisterAction => 58, // => RegisterFrame,
    Frame => 59,
    Action => 59, // => Frame,
    FrameTxStatus => 60,
    ActionTxStatus => 60, // => FrameTxStatus,

    SetPowerSave => 61,
    GetPowerSave => 62,

    SetCqm => 63,
    NotifyCqm => 64,

    SetChannel => 65,
    SetWdsPeer => 66,

    FrameWaitCancel => 67,

    JoinMesh => 68,
    LeaveMesh => 69,

    UnprotDeauthenticate => 70,
    UnprotDisassociate => 71,

    NewPeerCandidate => 72,

    GetWowlan => 73,
    SetWowlan => 74,

    StartSchedScan => 75,
    StopSchedScan => 76,
    SchedScanResults => 77,
    SchedScanStopped => 78,

    SetRekeyOffload => 79,

    PmksaCandidate => 80,

    TdlsOper => 81,
    TdlsMgmt => 82,

    UnexpectedFrame => 83,

    ProbeClient => 84,

    RegisterBeacons => 85,

    Unexpected4addrFrame => 86,

    SetNoackMap => 87,

    ChSwitchNotify => 88,

    StartP2pDevice => 89,
    StopP2pDevice => 90,

    ConnFailed => 91,

    SetMcastRate => 92,

    SetMacAcl => 93,

    RadarDetect => 94,

    GetProtocolFeatures => 95,

    UpdateFtIes => 96,
    FtEvent => 97,

    CritProtocolStart => 98,
    CritProtocolStop => 99,

    GetCoalesce => 100,
    SetCoalesce => 101,

    ChannelSwitch => 102,

    Vendor => 103,

    SetQosMap => 104,

    AddTxTs => 105,
    DelTxTs => 106,

    GetMpp => 107,

    JoinOcb => 108,
    LeaveOcb => 109,

    ChSwitchStartedNotify => 110,

    TdlsChannelSwitch => 111,
    TdlsCancelChannelSwitch => 112,

    WiphyRegChange => 113,

    AbortScan => 114,

    StartNan => 115,
    StopNan => 116,
    AddNanFunction => 117,
    DelNanFunction => 118,
    ChangeNanConfig => 119,
    NanMatch => 120,

    SetMulticastToUnicast => 121,

    UpdateConnectParams => 122,

    SetPmk => 123,
    DelPmk => 124,

    PortAuthorized => 125,

    ReloadRegdb => 126,

    ExternalAuth => 127,

    StaOpmodeChanged => 128,

    ControlPortFrame => 129,

    GetFtmResponderStats => 130,

    PeerMeasurementStart => 131,
    PeerMeasurementResult => 132,
    PeerMeasurementComplete => 133,

    NotifyRadar => 134,

    UpdateOweInfo => 135,

    ProbeMeshLink => 136,

    SetTidConfig => 137,

    UnprotBeacon => 138,

    ControlPortFrameTxStatus => 139
);
