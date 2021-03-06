/// Defines the `ISAExt` enum
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ISAExt {
    INVALID,
    ADOX_ADCX,
    AES,
    AMD3DNOW,
    AVX,
    AVX2,
    AVX2GATHER,
    AVX512EVEX,
    AVX512VEX,
    AVXAES,
    BASE,
    BMI1,
    BMI2,
    CET,
    CLDEMOTE,
    CLFLUSHOPT,
    CLFSH,
    CLWB,
    CLZERO,
    F16C,
    FMA,
    FMA4,
    GFNI,
    INVPCID,
    KNC,
    KNCE,
    KNCV,
    LONGMODE,
    LZCNT,
    MMX,
    MONITOR,
    MONITORX,
    MOVBE,
    MOVDIR,
    MPX,
    PADLOCK,
    PAUSE,
    PCLMULQDQ,
    PCONFIG,
    PKU,
    PREFETCHWT1,
    PT,
    RDPID,
    RDRAND,
    RDSEED,
    RDTSCP,
    RDWRFSGS,
    RTM,
    SGX,
    SGX_ENCLV,
    SHA,
    SMAP,
    SMX,
    SSE,
    SSE2,
    SSE3,
    SSE4,
    SSE4A,
    SSSE3,
    SVM,
    TBM,
    VAES,
    VMFUNC,
    VPCLMULQDQ,
    VTX,
    WAITPKG,
    X87,
    XOP,
    XSAVE,
    XSAVEC,
    XSAVEOPT,
    XSAVES
}

/// The last value of the `ISAExt` enum
pub const ISA_EXT_MAX_VALUE: ISAExt = ISAExt::XSAVES;

/// The number of values in the `ISAExt` enum
pub const ISA_EXT_COUNT: usize = 72;
