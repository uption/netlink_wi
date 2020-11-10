use neli::consts::NlAttrType;
use neli::{impl_var, impl_var_base, impl_var_trait};

impl_var_trait!(
    /// Nl80211 netlink attributes.
    ///
    /// nl80211_attrs enum from https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h.
    Attribute, u16, NlAttrType,
    Unspec                       => 0,

    Wiphy                        => 1,
    WiphyName                    => 2,

    Ifindex                      => 3,
    Ifname                       => 4,
    Iftype                       => 5,

    Mac                          => 6,

    KeyData                      => 7,
    KeyIdx                       => 8,
    KeyCipher                    => 9,
    KeySeq                       => 10,
    KeyDefault                   => 11,

    BeaconInterval               => 12,
    DtimPeriod                   => 13,
    BeaconHead                   => 14,
    BeaconTail                   => 15,

    StaAid                       => 16,
    StaFlags                     => 17,
    StaListenInterval            => 18,
    StaSupportedRates            => 19,
    StaVlan                      => 20,
    StaInfo                      => 21,

    WiphyBands                   => 22,

    MntrFlags                    => 23,

    MeshId                       => 24,
    StaPlinkAction               => 25,
    MpathNextHop                 => 26,
    MpathInfo                    => 27,

    BssCtsProt                   => 28,
    BssShortPreamble             => 29,
    BssShortSlotTime             => 30,

    HtCapability                 => 31,

    SupportedIftypes             => 32,

    RegAlpha2                    => 33,
    RegRules                     => 34,

    MeshConfig                   => 35,

    BssBasicRates                => 36,

    WiphyTxqParams               => 37,
    WiphyFreq                    => 38,
    WiphyChannelType             => 39,

    KeyDefaultMgmt               => 40,

    MgmtSubtype                  => 41,
    Ie                           => 42,

    MaxNumScanSsids              => 43,

    ScanFrequencies              => 44,
    ScanSsids                    => 45,
    Generation                   => 46,
    Bss                          => 47,

    RegInitiator                 => 48,
    RegType                      => 49,

    SupportedCommands            => 50,

    Frame                        => 51,
    Ssid                         => 52,
    AuthType                     => 53,
    ReasonCode                   => 54,

    KeyType                      => 55,

    MaxScanIeLen                 => 56,
    CipherSuites                 => 57,

    FreqBefore                   => 58,
    FreqAfter                    => 59,

    FreqFixed                    => 60,

    WiphyRetryShort              => 61,
    WiphyRetryLong               => 62,
    WiphyFragThreshold           => 63,
    WiphyRtsThreshold            => 64,

    TimedOut                     => 65,

    UseMfp                       => 66,

    StaFlags2                    => 67,

    ControlPort                  => 68,

    Testdata                     => 69,

    Privacy                      => 70,

    DisconnectedByAp             => 71,
    StatusCode                   => 72,

    CipherSuitesPairwise         => 73,
    CipherSuiteGroup             => 74,
    WpaVersions                  => 75,
    AkmSuites                    => 76,

    ReqIe                        => 77,
    RespIe                       => 78,

    PrevBssid                    => 79,

    Key                          => 80,
    Keys                         => 81,

    Pid                          => 82,

    Use4addrFrames               => 83,

    SurveyInfo                   => 84,

    Pmkid                        => 85,
    MaxNumPmkids                 => 86,

    Duration                     => 87,

    Cookie                       => 88,

    WiphyCoverageClass           => 89,

    TxRates                      => 90,

    FrameMatch                   => 91,

    Ack                          => 92,

    PsState                      => 93,

    Cqm                          => 94,

    LocalStateChange             => 95,

    ApIsolate                    => 96,

    WiphyTxPowerSetting          => 97,
    WiphyTxPowerLevel            => 98,

    TxFrameTypes                 => 99,
    RxFrameTypes                 => 100,
    FrameType                    => 101,

    ControlPortEthertype         => 102,
    ControlPortNoEncrypt         => 103,

    SupportIbssRsn               => 104,

    WiphyAntennaTx               => 105,
    WiphyAntennaRx               => 106,

    McastRate                    => 107,

    OffchannelTxOk               => 108,

    BssHtOpmode                  => 109,

    KeyDefaultTypes              => 110,

    MaxRemainOnChannelDuration   => 111,

    MeshSetup                    => 112,

    WiphyAntennaAvailTx          => 113,
    WiphyAntennaAvailRx          => 114,

    SupportMeshAuth              => 115,
    StaPlinkState                => 116,

    WowlanTriggers               => 117,
    WowlanTriggersSupported      => 118,

    SchedScanInterval            => 119,

    InterfaceCombinations        => 120,
    SoftwareIftypes              => 121,

    RekeyData                    => 122,

    MaxNumSchedScanSsids         => 123,
    MaxSchedScanIeLen            => 124,

    ScanSuppRates                => 125,

    HiddenSsid                   => 126,

    IeProbeResp                  => 127,
    IeAssocResp                  => 128,

    StaWme                       => 129,
    SupportApUapsd               => 130,

    RoamSupport                  => 131,

    SchedScanMatch               => 132,
    MaxMatchSets                 => 133,

    PmksaCandidate               => 134,

    TxNoCckRate                  => 135,

    TdlsAction                   => 136,
    TdlsDialogToken              => 137,
    TdlsOperation                => 138,
    TdlsSupport                  => 139,
    TdlsExternalSetup            => 140,

    DeviceApSme                  => 141,

    DontWaitForAck               => 142,

    FeatureFlags                 => 143,

    ProbeRespOffload             => 144,
    ProbeResp                    => 145,

    DfsRegion                    => 146,

    DisableHt                    => 147,
    HtCapabilityMask             => 148,

    NoackMap                     => 149,

    InactivityTimeout            => 150,

    RxSignalDbm                  => 151,

    BgScanPeriod                 => 152,

    Wdev                         => 153,

    UserRegHintType              => 154,

    ConnFailedReason             => 155,

    SaeData                      => 156,

    VhtCapability                => 157,

    ScanFlags                    => 158,

    ChannelWidth                 => 159,
    CenterFreq1                  => 160,
    CenterFreq2                  => 161,

    P2pCtwindow                  => 162,
    P2pOppps                     => 163,

    LocalMeshPowerMode           => 164,

    AclPolicy                    => 165,

    MacAddrs                     => 166,

    MacAclMax                    => 167,

    RadarEvent                   => 168,

    ExtCapa                      => 169,
    ExtCapaMask                  => 170,

    StaCapability                => 171,
    StaExtCapability             => 172,

    ProtocolFeatures             => 173,
    SplitWiphyDump               => 174,

    DisableVht                   => 175,
    VhtCapabilityMask            => 176,

    Mdid                         => 177,
    IeRic                        => 178,

    CritProtId                   => 179,
    MaxCritProtDuration          => 180,

    PeerAid                      => 181,

    CoalesceRule                 => 182,

    ChSwitchCount                => 183,
    ChSwitchBlockTx              => 184,
    CsaIes                       => 185,
    CsaCOffBeacon                => 186,
    CsaCOffPresp                 => 187,

    RxmgmtFlags                  => 188,

    StaSupportedChannels         => 189,
    StaSupportedOperClasses      => 190,

    HandleDfs                    => 191,

    Support5Mhz                  => 192,
    Support10Mhz                 => 193,

    OpmodeNotif                  => 194,

    VendorId                     => 195,
    VendorSubcmd                 => 196,
    VendorData                   => 197,
    VendorEvents                 => 198,

    QosMap                       => 199,

    MacHint                      => 200,
    WiphyFreqHint                => 201,

    MaxApAssocSta                => 202,

    TdlsPeerCapability           => 203,

    SocketOwner                  => 204,

    CsaCOffsetsTx                => 205,
    MaxCsaCounters               => 206,

    TdlsInitiator                => 207,

    UseRrm                       => 208,

    WiphyDynAck                  => 209,

    Tsid                         => 210,
    UserPrio                     => 211,
    AdmittedTime                 => 212,

    SmpsMode                     => 213,

    OperClass                    => 214,

    MacMask                      => 215,

    WiphySelfManagedReg          => 216,

    ExtFeatures                  => 217,

    SurveyRadioStats             => 218,

    NetnsFd                      => 219,

    SchedScanDelay               => 220,

    RegIndoor                    => 221,

    MaxNumSchedScanPlans         => 222,
    MaxScanPlanInterval          => 223,
    MaxScanPlanIterations        => 224,
    SchedScanPlans               => 225,

    Pbss                         => 226,

    BssSelect                    => 227,

    StaSupportP2pPs              => 228,

    Pad                          => 229,

    IftypeExtCapa                => 230,

    MuMimoGroupData              => 231,
    MuMimoFollowMacAddr          => 232,

    ScanStartTimeTsf             => 233,
    ScanStartTimeTsfBssid        => 234,
    MeasurementDuration          => 235,
    MeasurementDurationMandatory => 236,

    MeshPeerAid                  => 237,

    NanMasterPref                => 238,
    NanDual                      => 239,
    NanFunc                      => 240,
    NanMatch                     => 241,

    FilsKek                      => 242,
    FilsNonces                   => 243,

    MulticastToUnicastEnabled    => 244,

    Bssid                        => 245,

    SchedScanRelativeRssi        => 246,
    SchedScanRssiAdjust          => 247,

    AttrTimeoutReason            => 248,

    FilsErpUsername              => 249,
    FilsErpRealm                 => 250,
    FilsErpNextSeqNum            => 251,
    FilsErpRrk                   => 252,
    FilsCacheId                  => 253,

    Pmk                          => 254,

    SchedScanMulti               => 255,
    SchedScanMaxReqs             => 256,

    Want1x4wayHs                 => 257,
    Pmkr0Name                    => 258,
    PortAuthorized               => 259,

    ExternalAuthAction           => 260,
    ExternalAuthSupport          => 261,

    Nss                          => 262,
    AckSignal                    => 263,

    ControlPortOverNl80211       => 264,

    TxqStats                     => 265,
    TxqLimit                     => 266,
    TxqMemoryLimit               => 267,
    TxqQuantum                   => 268,

    HeCapability                 => 269,

    FtmResponder                 => 270,
    FtmResponderStats            => 271,

    Timeout                      => 272,

    PeerMeasurements             => 273,

    AirtimeWeight                => 274,
    StaTxPowerSetting            => 275,
    StaTxPower                   => 276,

    SaePassword                  => 277,

    TwtResponder                 => 278,

    HeObssPd                     => 279,

    WiphyEdmgChannels            => 280,
    WiphyEdmgBwConfig            => 281,

    VlanId                       => 282,

    HeBssColor                   => 283,

    IfTypeAkmSuites              => 284,

    TidConfig                    => 285,

    ControlPortNoPreauth         => 286,

    PmkLifetime                  => 287,
    PmkReauthThreshold           => 288,

    ReceiveMulticast             => 289,
    WiphyFreqOffset              => 290,
    CenterFreq1Offset            => 291,
    ScanFreqKhz                  => 292,

    He6ghzCapability             => 293,

    FilsDiscovery                => 294,

    UnsolBcastProbeResp          => 295,

    S1gCapability                => 296,
    S1gCapabilityMask            => 297
);
