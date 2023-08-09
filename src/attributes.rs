use std::fmt;

use bitflags::bitflags;
use neli::{
    attr::AttrHandle,
    consts::genl::NlAttrType,
    genl::Nlattr,
    types::{Buffer, GenlBuffer},
};
use neli_proc_macros::neli_enum;

pub type Attrs<'a, T> = AttrHandle<'a, GenlBuffer<T, Buffer>, Nlattr<T, Buffer>>;

/// Nl80211 netlink attributes.
///
/// nl80211_attrs enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum Attribute {
    Unspec = 0,
    /// Index of wiphy to operate on, cf. /sys/class/ieee80211/<phyname>/index.
    Wiphy = 1,
    /// Wiphy name (used for renaming).
    WiphyName = 2,
    /// Network interface index of the device to operate on.
    Ifindex = 3,
    /// Network interface name.
    Ifname = 4,
    /// Type of virtual interface (nested attribute, see enum `InterfaceType`).
    Iftype = 5,
    /// MAC address (various uses).
    Mac = 6,
    KeyData = 7,
    KeyIdx = 8,
    KeyCipher = 9,
    KeySeq = 10,
    KeyDefault = 11,
    BeaconInterval = 12,
    DtimPeriod = 13,
    BeaconHead = 14,
    BeaconTail = 15,
    StaAid = 16,
    StaFlags = 17,
    StaListenInterval = 18,
    StaSupportedRates = 19,
    StaVlan = 20,
    /// Information about a station, part of station info given for `Command.GetStation`.
    /// Contains a nested attribute. See `StationInfo` enum.
    StaInfo = 21,
    WiphyBands = 22,
    MntrFlags = 23,
    MeshId = 24,
    StaPlinkAction = 25,
    MpathNextHop = 26,
    MpathInfo = 27,
    BssCtsProt = 28,
    BssShortPreamble = 29,
    BssShortSlotTime = 30,
    HtCapability = 31,
    SupportedIftypes = 32,
    /// ISO-3166-alpha2 country code for which the current regulatory domain
    /// should be set to or is already set to.
    RegAlpha2 = 33,
    /// A nested array of regulatory domain regulatory rules (`RegRuleAttr`).
    RegRules = 34,
    MeshConfig = 35,
    BssBasicRates = 36,
    WiphyTxqParams = 37,
    /// Frequency of the selected channel in MHz. Defines the channel together with the (deprecated)
    /// `Attribute::WiphyChannelType` attribute or the attributes `Attribute::ChannelWidth` and if
    /// needed `Attribute::CenterFreq1` and `Attribute::CenterFreq2`.
    WiphyFreq = 38,
    /// Included with `WiphyFreq` if HT20 or HT40 are to be used (i.e., HT disabled if not included).
    /// This attribute is deprecated.
    WiphyChannelType = 39,
    KeyDefaultMgmt = 40,
    MgmtSubtype = 41,
    Ie = 42,
    MaxNumScanSsids = 43,
    ScanFrequencies = 44,
    ScanSsids = 45,
    /// Used to indicate consistent snapshots for dumps. This number increases whenever the object
    /// list being dumped changes, and as such userspace can verify that it has obtained a complete
    /// and consistent snapshot by verifying that all dump messages contain the same generation number.
    /// If it changed then the list changed and the dump should be repeated completely from scratch.
    Generation = 46,
    Bss = 47,
    RegInitiator = 48,
    RegType = 49,
    SupportedCommands = 50,
    Frame = 51,
    /// SSID (binary attribute, 0..32 octets).
    Ssid = 52,
    AuthType = 53,
    ReasonCode = 54,
    KeyType = 55,
    MaxScanIeLen = 56,
    CipherSuites = 57,
    FreqBefore = 58,
    FreqAfter = 59,
    FreqFixed = 60,
    WiphyRetryShort = 61,
    WiphyRetryLong = 62,
    WiphyFragThreshold = 63,
    WiphyRtsThreshold = 64,
    TimedOut = 65,
    UseMfp = 66,
    StaFlags2 = 67,
    ControlPort = 68,
    Testdata = 69,
    Privacy = 70,
    DisconnectedByAp = 71,
    StatusCode = 72,
    CipherSuitesPairwise = 73,
    CipherSuiteGroup = 74,
    WpaVersions = 75,
    AkmSuites = 76,
    ReqIe = 77,
    RespIe = 78,
    PrevBssid = 79,
    Key = 80,
    Keys = 81,
    Pid = 82,
    /// Use 4-address frames on a virtual interface.
    Use4addrFrames = 83,
    SurveyInfo = 84,
    Pmkid = 85,
    MaxNumPmkids = 86,
    Duration = 87,
    Cookie = 88,
    WiphyCoverageClass = 89,
    TxRates = 90,
    FrameMatch = 91,
    Ack = 92,
    PsState = 93,
    Cqm = 94,
    LocalStateChange = 95,
    ApIsolate = 96,
    WiphyTxPowerSetting = 97,
    /// Transmit power level in signed mBm units.
    WiphyTxPowerLevel = 98,
    TxFrameTypes = 99,
    RxFrameTypes = 100,
    FrameType = 101,
    ControlPortEthertype = 102,
    ControlPortNoEncrypt = 103,
    SupportIbssRsn = 104,
    WiphyAntennaTx = 105,
    WiphyAntennaRx = 106,
    McastRate = 107,
    OffchannelTxOk = 108,
    BssHtOpmode = 109,
    KeyDefaultTypes = 110,
    MaxRemainOnChannelDuration = 111,
    MeshSetup = 112,
    WiphyAntennaAvailTx = 113,
    WiphyAntennaAvailRx = 114,
    SupportMeshAuth = 115,
    StaPlinkState = 116,
    WowlanTriggers = 117,
    WowlanTriggersSupported = 118,
    SchedScanInterval = 119,
    InterfaceCombinations = 120,
    SoftwareIftypes = 121,
    RekeyData = 122,
    MaxNumSchedScanSsids = 123,
    MaxSchedScanIeLen = 124,
    ScanSuppRates = 125,
    HiddenSsid = 126,
    IeProbeResp = 127,
    IeAssocResp = 128,
    StaWme = 129,
    SupportApUapsd = 130,
    RoamSupport = 131,
    SchedScanMatch = 132,
    MaxMatchSets = 133,
    PmksaCandidate = 134,
    TxNoCckRate = 135,
    TdlsAction = 136,
    TdlsDialogToken = 137,
    TdlsOperation = 138,
    TdlsSupport = 139,
    TdlsExternalSetup = 140,
    DeviceApSme = 141,
    DontWaitForAck = 142,
    FeatureFlags = 143,
    ProbeRespOffload = 144,
    ProbeResp = 145,
    /// Region for regulatory rules which this country abides to when initiating
    /// radiation on DFS channels. A country maps to one DFS region.
    DfsRegion = 146,
    DisableHt = 147,
    HtCapabilityMask = 148,
    NoackMap = 149,
    InactivityTimeout = 150,
    RxSignalDbm = 151,
    BgScanPeriod = 152,
    /// Wireless device identifier, used for pseudo-devices that don't have a netdev (u64).
    Wdev = 153,
    UserRegHintType = 154,
    ConnFailedReason = 155,
    SaeData = 156,
    VhtCapability = 157,
    ScanFlags = 158,
    /// Channel width (u32) (nested attribute).
    ChannelWidth = 159,
    /// Center frequency of the first part of the channel, used for anything but 20 MHz bandwidth.
    /// In S1G this is the operating channel center frequency.
    CenterFreq1 = 160,
    /// Center frequency of the second part of the channel, used only for 80+80 MHz bandwidth.
    CenterFreq2 = 161,
    P2pCtwindow = 162,
    P2pOppps = 163,
    LocalMeshPowerMode = 164,
    AclPolicy = 165,
    MacAddrs = 166,
    MacAclMax = 167,
    RadarEvent = 168,
    ExtCapa = 169,
    ExtCapaMask = 170,
    StaCapability = 171,
    StaExtCapability = 172,
    ProtocolFeatures = 173,
    SplitWiphyDump = 174,
    DisableVht = 175,
    VhtCapabilityMask = 176,
    Mdid = 177,
    IeRic = 178,
    CritProtId = 179,
    MaxCritProtDuration = 180,
    PeerAid = 181,
    CoalesceRule = 182,
    ChSwitchCount = 183,
    ChSwitchBlockTx = 184,
    CsaIes = 185,
    CsaCOffBeacon = 186,
    CsaCOffPresp = 187,
    RxmgmtFlags = 188,
    StaSupportedChannels = 189,
    StaSupportedOperClasses = 190,
    HandleDfs = 191,
    Support5Mhz = 192,
    Support10Mhz = 193,
    OpmodeNotif = 194,
    VendorId = 195,
    VendorSubcmd = 196,
    VendorData = 197,
    VendorEvents = 198,
    QosMap = 199,
    MacHint = 200,
    WiphyFreqHint = 201,
    MaxApAssocSta = 202,
    TdlsPeerCapability = 203,
    SocketOwner = 204,
    CsaCOffsetsTx = 205,
    MaxCsaCounters = 206,
    TdlsInitiator = 207,
    UseRrm = 208,
    WiphyDynAck = 209,
    Tsid = 210,
    UserPrio = 211,
    AdmittedTime = 212,
    SmpsMode = 213,
    OperClass = 214,
    MacMask = 215,
    /// Flag attribute indicating this device is self-managing its regulatory
    /// information.
    WiphySelfManagedReg = 216,
    ExtFeatures = 217,
    SurveyRadioStats = 218,
    NetnsFd = 219,
    SchedScanDelay = 220,
    RegIndoor = 221,
    MaxNumSchedScanPlans = 222,
    MaxScanPlanInterval = 223,
    MaxScanPlanIterations = 224,
    SchedScanPlans = 225,
    Pbss = 226,
    BssSelect = 227,
    StaSupportP2pPs = 228,
    Pad = 229,
    IftypeExtCapa = 230,
    MuMimoGroupData = 231,
    MuMimoFollowMacAddr = 232,
    ScanStartTimeTsf = 233,
    ScanStartTimeTsfBssid = 234,
    MeasurementDuration = 235,
    MeasurementDurationMandatory = 236,
    MeshPeerAid = 237,
    NanMasterPref = 238,
    NanDual = 239,
    NanFunc = 240,
    NanMatch = 241,
    FilsKek = 242,
    FilsNonces = 243,
    MulticastToUnicastEnabled = 244,
    Bssid = 245,
    SchedScanRelativeRssi = 246,
    SchedScanRssiAdjust = 247,
    AttrTimeoutReason = 248,
    FilsErpUsername = 249,
    FilsErpRealm = 250,
    FilsErpNextSeqNum = 251,
    FilsErpRrk = 252,
    FilsCacheId = 253,
    Pmk = 254,
    SchedScanMulti = 255,
    SchedScanMaxReqs = 256,
    Want1x4wayHs = 257,
    Pmkr0Name = 258,
    PortAuthorized = 259,
    ExternalAuthAction = 260,
    ExternalAuthSupport = 261,
    Nss = 262,
    AckSignal = 263,
    ControlPortOverNl80211 = 264,
    /// TXQ statistics (nested attribute, see enum `TxqStats`).
    TxqStats = 265,
    TxqLimit = 266,
    TxqMemoryLimit = 267,
    TxqQuantum = 268,
    HeCapability = 269,
    FtmResponder = 270,
    FtmResponderStats = 271,
    Timeout = 272,
    PeerMeasurements = 273,
    AirtimeWeight = 274,
    StaTxPowerSetting = 275,
    StaTxPower = 276,
    SaePassword = 277,
    TwtResponder = 278,
    HeObssPd = 279,
    WiphyEdmgChannels = 280,
    WiphyEdmgBwConfig = 281,
    VlanId = 282,
    HeBssColor = 283,
    IfTypeAkmSuites = 284,
    TidConfig = 285,
    ControlPortNoPreauth = 286,
    PmkLifetime = 287,
    PmkReauthThreshold = 288,
    ReceiveMulticast = 289,
    /// Offset of the associated `Attribute::WiphyFreq` in positive KHz.
    WiphyFreqOffset = 290,
    CenterFreq1Offset = 291,
    ScanFreqKhz = 292,
    He6ghzCapability = 293,
    FilsDiscovery = 294,
    UnsolBcastProbeResp = 295,
    S1gCapability = 296,
    S1gCapabilityMask = 297,
}

impl NlAttrType for Attribute {}

/// Nl80211 (virtual) interface types.
///
/// These attribute types are used with `Attribute.IfType`
/// to set the type of an interface.
///
/// nl80211_iftype enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u32")]
pub(crate) enum InterfaceType {
    /// Unspecified type, driver decides.
    Unspecified = 0,
    /// Independent BSS member.
    Adhoc = 1,
    /// Managed BSS member.
    Station = 2,
    /// Access point.
    Ap = 3,
    /// VLAN interface for access points; VLAN interfaces are a bit special in
    /// that they must always be tied to a pre-existing AP type interface.
    ApVlan = 4,
    /// Wireless distribution interface.
    Wds = 5,
    /// Monitor interface receiving all frames.
    Monitor = 6,
    /// Mesh point.
    MeshPoint = 7,
    /// P2P client.
    P2pClient = 8,
    /// P2P group owner.
    P2pGo = 9,
    /// P2P device interface type, this is not a netdev and therefore can't be
    /// created in the normal ways, use the use the `Command.StartP2pDevice` and
    /// `Command.StopP2pDevice` commands to create and destroy one.
    P2pDevice = 10,
    /// Outside Context of a BSS.
    Ocb = 11,
    /// NAN device interface type (not a netdev).
    Nan = 12,
}

/// Nl80211 per TXQ (transmit queue) statistics.
///
/// These attribute types are used with `Attribute.TxqStats` to get
/// transmit queue statistics.
///
/// nl80211_txq_stats enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum TxqStats {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Number of bytes currently backlogged.
    BacklogBytes = 1,
    /// Number of packets currently backlogged.
    BacklogPackets = 2,
    /// Total number of new flows seen.
    Flows = 3,
    /// Total number of packet drops.
    Drops = 4,
    /// Total number of packet ECN marks.
    EcnMarks = 5,
    /// Number of drops due to queue space overflow.
    Overlimit = 6,
    /// Number of drops due to memory limit overflow (only for per-phy stats).
    Overmemory = 7,
    /// Number of hash collisions.
    Collisions = 8,
    /// Total number of bytes dequeued from TXQ.
    TxBytes = 9,
    /// Total number of packets dequeued from TXQ.
    TxPackets = 10,
    /// Number of flow buckets for PHY.
    MaxFlows = 11,
}

impl NlAttrType for TxqStats {}

/// Nl80211 netlink station information attributes.
///
/// These attribute types are used with `Attribute.StaInfo`
/// when getting information about a station.
///
/// nl80211_sta_info enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum StationInfo {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Time since last activity.
    InactiveTime = 1,
    /// Total received bytes (MPDU length) (u32, from this station).
    RxBytes = 2,
    /// Total transmitted bytes (MPDU length) (u32, to this station).
    TxBytes = 3,
    /// The station's mesh LLID.
    Llid = 4,
    /// The station's mesh PLID.
    Plid = 5,
    /// Peer link state for the station (nested attribute).
    PlinkState = 6,
    /// Signal strength of last received PPDU (u8, dBm).
    Signal = 7,
    /// Current unicast tx rate (nested attribute, see enum `RateInfo`).
    TxBitrate = 8,
    /// Total received packet (MSDUs and MMPDUs) (u32, from this station)
    RxPackets = 9,
    /// Total transmitted packets (MSDUs and MMPDUs) (u32, to this station)
    TxPackets = 10,
    /// Total retries (MPDUs) (u32, to this station).
    TxRetries = 11,
    /// Total failed packets (MPDUs) (u32, to this station).
    TxFailed = 12,
    /// Signal strength average (u8, dBm).
    SignalAvg = 13,
    /// Last unicast data frame rx rate (nested attribute, see enum `RateInfo`).
    RxBitrate = 14,
    /// Current station's view of BSS (nested attributem see enum `BssParam`).
    BssParam = 15,
    /// Time since the station is last connected.
    ConnectedTime = 16,
    /// Contains a struct nl80211_sta_flag_update.
    StaFlags = 17,
    /// Count of times beacon loss was detected (u32).
    BeaconLoss = 18,
    /// Timing offset with respect to this STA (s64).
    TOffset = 19,
    /// Local mesh STA link-specific power mode.
    LocalPm = 20,
    /// Peer mesh STA link-specific power mode.
    PeerPm = 21,
    /// Neighbor mesh STA power save mode towards non-peer STA.
    NonpeerPm = 22,
    /// Total received bytes (MPDU length) (u64, from this station).
    RxBytes64 = 23,
    /// Total transmitted bytes (MPDU length) (u64, to this station).
    TxBytes64 = 24,
    /// Per-chain signal strength of last PPDU. Contains a nested array of signal
    /// strength attributes (u8, dBm).
    ChainSignal = 25,
    /// Per-chain signal strength average. Same format as `ChainSignal`.
    ChainSignalAvg = 26,
    /// Expected throughput considering also the 802.11 header (u32, kbps).
    ExpectedThroughput = 27,
    /// RX packets dropped for unspecified reasons (u64).
    RxDropMisc = 28,
    /// Number of beacons received from this peer (u64).
    BeaconRx = 29,
    /// Signal strength average for beacons only (u8, dBm).
    BeaconSignalAvg = 30,
    /// Per-TID  statistics (nested attribute, see enum `TidStats`).
    /// This is a nested attribute where each the inner attribute number is the
    /// TID+1 and the special TID 16 (i.e. value 17) is used for non-QoS frames;
    /// each one of those is again nested with `TidStats` attributes carrying the
    /// actual values.
    TidStats = 31,
    /// Aggregate PPDU duration for all frames received from the station (u64, usec).
    RxDuration = 32,
    /// Attribute used for padding for 64-bit alignment.
    Pad = 33,
    /// Signal strength of the last ACK frame (u8, dBm).
    AckSignal = 34,
    /// Average signal strength of ACK frames (s8, dBm).
    AckSignalAvg = 35,
    /// Total number of received packets (MPDUs) (u32, from this station).
    RxMpdus = 36,
    /// Total number of packets (MPDUs) received with an FCS error (u32, from this station).
    /// This count may not include some packets with an FCS error due to TA corruption.
    /// Hence this counter might not be fully accurate.
    FcsErrorCount = 37,
    /// Set to true if STA has a path to a mesh gate (u8, 0 or 1).
    ConnectedToGate = 38,
    /// Aggregate PPDU duration for all frames sent to the station (u64, usec).
    TxDuration = 39,
    /// Current airtime weight for station (u16).
    AirtimeWeight = 40,
    /// Airtime link metric for mesh station.
    AirtimeLinkMetric = 41,
    /// Timestamp (CLOCK_BOOTTIME, nanoseconds) of STA's association.
    AssocAtBootTime = 42,
    /// Set to true if STA has a path to an authentication server (u8, 0 or 1).
    ConnectedToAs = 43,
}

impl NlAttrType for StationInfo {}

/// Nl80211 bitrate information.
///
/// These attribute types are used with `Attribute.StaInfo.RxBitrate` or
/// `Attribute.StaInfo.TxBitrate` when getting information about a station.
///
/// nl80211_rate_info enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum RateInfo {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Total bitrate (u16, 100kbit/s).
    Bitrate = 1,
    /// MCS index for 802.11n (u8).
    Mcs = 2,
    /// 40 MHz dualchannel bitrate (flag).
    MhzWidth40 = 3,
    /// 400ns guard interval (flag).
    ShortGuardInterval = 4,
    /// Total bitrate (u32, 100kbit/s).
    Bitrate32 = 5,
    /// MCS index for VHT (u8).
    VhtMcs = 6,
    /// Number of streams in VHT (u8)
    VhtNss = 7,
    /// 80 MHz VHT rate (flag).
    MhzWidth80 = 8,
    /// Unused. 80+80 is treated the same as 160 for purposes of the bitrates.
    MhzWidth80p80 = 9,
    /// 160 MHz VHT rate (flag).
    MhzWidth160 = 10,
    /// 10 MHz width. Note that this is a legacy rate and will be reported as the
    /// actual bitrate, i.e. half the base (20 MHz) rate (flag).
    MhzWidth10 = 11,
    /// 5 MHz width. Note that this is a legacy rate and will be reported as the
    /// actual bitrate, i.e. half the base (20 MHz) rate (flag).
    MhzWidth5 = 12,
    /// HE MCS index (u8, 0-11).
    HeMcs = 13,
    /// HE NSS value (u8, 1-8).
    HeNss = 14,
    /// HE guard interval identifier (u8, see enum `HeGuardInterval`).
    HeGuardInterval = 15,
    /// HE DCM value (u8, 0/1).
    HeDcm = 16,
    /// HE RU allocation, if not present then non-OFDMA was used
    /// (u8, see enum `HeRuAlloc`).
    HeRuAlloc = 17,
}

impl NlAttrType for RateInfo {}

/// Nl80211 BSS information collected by STA.
///
/// These attribute types are used with `StaInfo.BssParam`.
///
/// nl80211_sta_bss_param enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum BssParam {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Whether CTS protection is enabled (flag).
    CtsProt = 1,
    /// Whether short preamble is enabled (flag).
    ShortPreamble = 2,
    /// Whether short slot time is enabled (flag).
    ShortSlotTime = 3,
    /// DTIM period for beaconing (u8).
    DtimPeriod = 4,
    /// Beacon interval (u16).
    BeaconInterval = 5,
}

impl NlAttrType for BssParam {}

/// Nl80211 HE guard interval.
///
/// These attribute types are used with `RateInfo.HeGuardInterval`
///
/// nl80211_he_gi enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u8")]
pub(crate) enum HeGuardInterval {
    /// 0.8 usec
    Usec0_8 = 0,
    /// 1.6 usec
    Usec1_6 = 1,
    /// 3.2 usec
    Usec3_2 = 2,
}

/// Nl80211 HE RU allocation values.
///
/// These attribute types are used with `RateInfo.HeRuAlloc`.
///
/// nl80211_he_ru_alloc enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum HeRuAlloc {
    /// 26-tone RU allocation.
    Alloc26 = 0,
    /// 52-tone RU allocation.
    Alloc52 = 1,
    /// 106-tone RU allocation.
    Alloc106 = 2,
    /// 242-tone RU allocation.
    Alloc242 = 3,
    /// 484-tone RU allocation.
    Alloc484 = 4,
    /// 996-tone RU allocation.
    Alloc996 = 5,
    /// 2x996-tone RU allocation.
    Alloc2x996 = 6,
}

impl NlAttrType for HeRuAlloc {}

/// Nl80211 per TID (traffic identifier) statistics attributes.
///
/// These attributes are used with `StationInfo.TidStats`.
///
/// nl80211_tid_stats enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum TidStats {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Number of MSDUs received (u64).
    RxMsdu = 1,
    /// Number of MSDUs transmitted or attempted to transmit (u64).
    TxMsdu = 2,
    /// Number of retries for transmitted MSDUs (not counting the first attempt; u64).
    TxMsduRetries = 3,
    /// Number of failed transmitted MSDUs (u64).
    TxMsduFailed = 4,
    /// Attribute used for padding for 64-bit alignment.
    Pad = 5,
    /// TXQ stats (nested attribute, see enum `TxqStats`).
    TxqStats = 6,
}

impl NlAttrType for TidStats {}

/// Nl80211 band attributes.
///
/// nl80211_band_attr enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum BandAttr {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Supported frequencies in this band, an array of nested frequency attributes.
    Frequencies = 1,
    /// Supported bitrates in this band, an array of nested bitrate attributes.
    Bitrates = 2,
    /// 16-byte attribute containing the MCS set as defined in 802.11n.
    HtMcsSet = 3,
    /// HT capabilities, as in the HT information IE.
    HtCapabilities = 4,
    /// A-MPDU factor, as in 11n.
    HtAmpduFactor = 5,
    /// A-MPDU density, as in 11n.
    HtAmpduDensity = 6,
    /// 32-byte attribute containing the MCS set as defined in 802.11ac.
    VhtMcsSet = 7,
    /// VHT capabilities, as in the HT information IE.
    VhtCapabilities = 8,
    /// Nested array attribute, with each entry using attributes from enum
    /// nl80211_band_iftype_attr.
    IftypeData = 9,
    /// Bitmap that indicates the 2.16 GHz channel(s) that are allowed to be used
    /// for EDMG transmissions. Defined by IEEE P802.11ay/D4.0 section 9.4.2.251.
    EdmgChannels = 10,
    /// Channel BW Configuration subfield encodes the allowed channel bandwidth
    /// configurations. Defined by IEEE P802.11ay/D4.0 section 9.4.2.251, Table 13.
    EdmgBwConfig = 11,
}

impl NlAttrType for BandAttr {}

/// Frequency band.
///
/// nl80211_band enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[allow(clippy::enum_variant_names)]
#[neli_enum(serialized_type = "u16")]
pub(crate) enum Band {
    /// 2.4 GHz ISM band.
    Band2ghz = 0,
    /// 5 GHz band (4.9 - 5.7 GHz).
    Band5ghz = 1,
    /// 60 GHz band (58.32 - 69.12 GHz).
    Band60ghz = 2,
    /// 6 GHz band (5.9 - 7.2 GHz).
    Band6ghz = 3,
    /// 900MHz, supported by S1G PHYs.
    BandS1ghz = 4,
    /// Light communication band (placeholder).
    BandLc = 5,
}

impl NlAttrType for Band {}

/// Frequency attributes.
///
/// nl80211_frequency_attr enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum FrequencyAttr {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Frequency in MHz
    Frequency = 1,
    /// Channel is disabled in current regulatory domain.
    Disabled = 2,
    /// No mechanisms that initiate radiation are permitted on this channel,
    /// this includes sending probe requests, or modes of operation that require
    /// beaconing.
    NoIr = 3,
    /// Not in use (NoIbss == NoIr).
    NoIbss = 4,
    /// Radar detection is mandatory on this channel in current regulatory domain.
    Radar = 5,
    /// Maximum transmission power in mBm (100 * dBm).
    MaxTxPower = 6,
    /// Current state for DFS (enum nl80211_dfs_state).
    DfsState = 7,
    /// Time in miliseconds for how long this channel is in this DFS state.
    DfdTime = 8,
    /// HT40- isn't possible with this channel as the control channel.
    NoHt40Minus = 9,
    /// HT40+ isn't possible with this channel as the control channel.
    NoHt40Plus = 10,
    /// Any 80 MHz channel using this channel as the primary or any of the
    /// secondary channels isn't possible, this includes 80+80 channels.
    No80Mhz = 11,
    /// Any 160 MHz (but not 80+80) channel using this channel as the primary or
    /// any of the secondary channels isn't possible.
    No160Mhz = 12,
    /// DFS CAC time in milliseconds.
    DfsCacTime = 13,
    /// Only indoor use is permitted on this channel.
    IndoorOnly = 14,
    /// IR operation is allowed on this channel if it's connected concurrently
    /// to a BSS on the same channel on the 2 GHz band or to a channel in the
    /// same UNII band (on the 5 GHz band), and IEEE80211_CHAN_RADAR is not set.
    IrConcurrent = 15,
    /// 20 MHz operation is not allowed on this channel in current regulatory domain.
    No20Mhz = 16,
    /// 10 MHz operation is not allowed on this channel in current regulatory domain.
    No10Mhz = 17,
    /// This channel has wmm limitations. This is a nested attribute that
    /// contains the wmm limitation per AC. (see &enum nl80211_wmm_rule).
    Wmm = 18,
    /// HE operation is not allowed on this channel in current regulatory domain.
    NoHe = 19,
    /// Frequency offset in KHz.
    Offset = 20,
    /// 1 MHz operation is allowed on this channel in current regulatory domain.
    Allow1Mhz = 21,
    /// 2 MHz operation is allowed on this channel in current regulatory domain.
    Allow2Mhz = 22,
    /// 4 MHz operation is allowed on this channel in current regulatory domain.
    Allow4Mhz = 23,
    /// 8 MHz operation is allowed on this channel in current regulatory domain.
    Allow8Mhz = 24,
    /// 16 MHz operation is allowed on this channel in current regulatory domain.
    Allow16Mhz = 25,
    /// Any 320 MHz channel using this channel as the primary or any of the
    /// secondary channels isn't possible.
    No320Mhz = 26,
    /// EHT operation is not allowed on this channel in current regulatory domain.
    NoEht = 27,
}

impl NlAttrType for FrequencyAttr {}

/// Frequency attributes.
///
/// nl80211_reg_rule_attr enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub(crate) enum RegRuleAttr {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// A set of flags which specify additional considerations for a given
    /// frequency range. These are the enum RegRuleFlags.
    RegRuleFlags = 1,
    /// Starting frequencry for the regulatory rule in KHz.
    /// This is not a center of frequency but an actual regulatory band edge.
    FreqRangeStart = 2,
    /// Ending frequency for the regulatory rule in KHz.
    /// This is not a center a frequency but an actual regulatory band edge.
    FreqRangeEnd = 3,
    /// Maximum allowed bandwidth for this frequency range, in KHz.
    FreqRangeMaxBw = 4,
    /// The maximum allowed antenna gain for a given frequency range.
    /// The value is in mBi (100 * dBi). If you don't have one then don't
    /// send this.
    PowerRuleMaxAntGain = 5,
    /// The maximum allowed EIRP for a given frequency range.
    /// The value is in mBm (100 * dBm).
    PowerRuleMaxEirp = 6,
    /// DFS CAC time in milliseconds.
    /// If not present or 0 default CAC time will be used.
    DfsCacTime = 7,
}

impl NlAttrType for RegRuleAttr {}

bitflags! {
    /// Regulatory rule flags.
    ///
    /// nl80211_reg_rule_flags enum from:
    /// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
    pub struct RegRuleFlags: u32 {
        const NO_OFDM       = 1<<0;
        const NO_CCK        = 1<<1;
        const NO_INDOOR     = 1<<2;
        const NO_OUTDOOR    = 1<<3;
        const DFS           = 1<<4;
        const PTP_ONLY      = 1<<5;
        const PTMP_ONLY     = 1<<6;
        const NO_IR         = 1<<7;
        const __NO_IBSS     = 1<<8;
        const AUTO_BW       = 1<<11;
        const IR_CONCURRENT = 1<<12;
        const NO_HT40MINUS  = 1<<13;
        const NO_HT40PLUS   = 1<<14;
        const NO_80MHZ      = 1<<15;
        const NO_160MHZ     = 1<<16;
        const NO_HE         = 1<<17;
        const NO_320MHZ     = 1<<18;
        const NO_EHT        = 1<<19;
    }
}

impl fmt::Debug for RegRuleFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Monitor mode configuration flags.
///
/// nl80211_mntr_flags enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u16")]
pub enum MonitorFlags {
    /// Attribute number 0 is reserved.
    Invalid = 0,
    /// Pass frames with bad FCS.
    FcsFail = 1,
    /// Pass frames with bad PLCP.
    PlcpFail = 2,
    /// Pass control frames.
    Control = 3,
    /// Disable BSSID filtering.
    OtherBss = 4,
    /// Report frames after processing. Overrides all other flags.
    CookFrames = 5,
    /// Use the configured MAC address and ACK incoming unicast packets.
    Active = 6,
}

impl NlAttrType for MonitorFlags {}

/// Channel width definitions.
///
/// These values are used with the `Attribute.ChannelWidth` attribute.
///
/// nl80211_chan_width enum from:
/// https://github.com/torvalds/linux/blob/master/include/uapi/linux/nl80211.h
#[neli_enum(serialized_type = "u32")]
pub(crate) enum ChannelWidth {
    Width20NoHT = 0,
    Width20 = 1,
    Width40 = 2,
    Width80 = 3,
    Width80P80 = 4,
    Width160 = 5,
    Width5 = 6,
    Width10 = 7,
    Width1 = 8,
    Width2 = 9,
    Width4 = 10,
    Width8 = 11,
    Width16 = 12,
    Width320 = 13,
}
