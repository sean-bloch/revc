use std::rc::Rc;
use std::vec::Vec;
use std::{cmp, fmt, io};

use arg_enum_proc_macro::ArgEnum;
use num_derive::*;

pub mod frame;
pub mod util;

use frame::Frame;
use util::Pixel;

use crate::dec::*;

/*****************************************************************************
 * return values and error code
 *****************************************************************************/
/* no more frames, but it is OK */
const EVC_OK_NO_MORE_FRM: usize = 205;
/* progress success, but output is not available temporarily */
const EVC_OK_OUT_NOT_AVAILABLE: usize = 204;
/* frame dimension (width or height) has been changed */
const EVC_OK_DIM_CHANGED: usize = (203);
/* decoding success, but output frame has been delayed */
const EVC_OK_FRM_DELAYED: usize = (202);
/* not matched CRC value */
pub const EVC_ERR_BAD_CRC: usize = (201);
/* CRC value presented but ignored at decoder*/
pub const EVC_WARN_CRC_IGNORED: usize = (200);
pub const EVC_OK: usize = 0;

#[derive(Debug, FromPrimitive, ToPrimitive, PartialOrd, Ord, PartialEq, Eq)]
pub enum EvcError {
    EVC_ERR = (-1), /* generic error */
    EVC_ERR_INVALID_ARGUMENT = (-101),
    EVC_ERR_OUT_OF_MEMORY = (-102),
    EVC_ERR_REACHED_MAX = (-103),
    EVC_ERR_UNSUPPORTED = (-104),
    EVC_ERR_UNEXPECTED = (-105),
    EVC_ERR_UNSUPPORTED_COLORSPACE = (-201),
    EVC_ERR_MALFORMED_BITSTREAM = (-202),

    EVC_ERR_UNKNOWN = (-32767), /* unknown error */
}

impl Default for EvcError {
    fn default() -> Self {
        EvcError::EVC_ERR
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(C)]
pub enum NaluType {
    EVC_NONIDR_NUT = 0,
    EVC_IDR_NUT = 1,
    EVC_SPS_NUT = 24,
    EVC_PPS_NUT = 25,
    EVC_APS_NUT = 26,
    EVC_SEI_NUT = 27,
}

impl fmt::Display for NaluType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::NaluType::*;
        match self {
            EVC_NONIDR_NUT => write!(f, "Non-IDR"),
            EVC_IDR_NUT => write!(f, "Instantaneous Decoder Refresh"),
            EVC_SPS_NUT => write!(f, "Sequence Parameter Se"),
            EVC_PPS_NUT => write!(f, "Picture Parameter Set"),
            EVC_APS_NUT => write!(f, "Adaptation Parameter Set"),
            EVC_SEI_NUT => write!(f, "Supplemental Enhancement Information"),
        }
    }
}

impl Default for NaluType {
    fn default() -> Self {
        NaluType::EVC_NONIDR_NUT
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(C)]
pub enum SliceType {
    EVC_ST_UNKNOWN = 0,
    EVC_ST_I = 1,
    EVC_ST_P = 2,
    EVC_ST_B = 3,
}

impl fmt::Display for SliceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::SliceType::*;
        match self {
            EVC_ST_UNKNOWN => write!(f, "Unknown"),
            EVC_ST_I => write!(f, "I"),
            EVC_ST_P => write!(f, "P"),
            EVC_ST_B => write!(f, "B"),
        }
    }
}

impl Default for SliceType {
    fn default() -> Self {
        SliceType::EVC_ST_UNKNOWN
    }
}

/*****************************************************************************
 * status after decoder operation
 *****************************************************************************/
#[derive(Debug, Default)]
pub struct EvcdStat {
    /* byte size of decoded bitstream (read size of bitstream) */
    pub read: usize,
    /* nalu type */
    pub nalu_type: NaluType,
    /* slice type */
    pub stype: SliceType,
    /* frame number monotonically increased whenever decoding a frame.
    note that it has negative value if the decoded data is not frame */
    pub fnum: isize,
    /* picture order count */
    pub poc: isize,
    /* layer id */
    pub tid: isize,

    /* number of reference pictures */
    pub refpic_num: [u8; 2],
    /* list of reference pictures */
    pub refpic: [[isize; 16]; 2], //[2][16]

    pub ret: usize,
}

pub const MAX_NUM_REF_PICS: usize = 21;
pub const MAX_NUM_ACTIVE_REF_FRAME: usize = 5;
pub const MAX_NUM_RPLS: usize = 32;

/* rpl structure */
#[derive(Default)]
pub struct EvcRpl {
    pub poc: usize,
    pub tid: usize,
    pub ref_pic_num: u8,
    pub ref_pic_active_num: u8,
    pub ref_pics: [u8; MAX_NUM_REF_PICS],
    pub pic_type: u8,
}

pub const MAX_QP_TABLE_SIZE: usize = 58;
pub const MAX_QP_TABLE_SIZE_EXT: usize = 70;

/* chromaQP table structure to be signalled in SPS*/
pub struct EvcChromaTable {
    pub chroma_qp_table_present_flag: bool,
    pub same_qp_table_for_chroma: bool,
    pub global_offset_flag: bool,
    pub num_points_in_qp_table_minus1: [usize; 2],
    pub delta_qp_in_val_minus1: [[i8; MAX_QP_TABLE_SIZE]; 2],
    pub delta_qp_out_val: [[i8; MAX_QP_TABLE_SIZE]; 2],
}

static default_qp_talbe: [[i8; MAX_QP_TABLE_SIZE]; 2] = [[0; MAX_QP_TABLE_SIZE]; 2];
impl Default for EvcChromaTable {
    fn default() -> Self {
        EvcChromaTable {
            chroma_qp_table_present_flag: false,
            same_qp_table_for_chroma: false,
            global_offset_flag: false,
            num_points_in_qp_table_minus1: [0; 2],
            delta_qp_in_val_minus1: default_qp_talbe,
            delta_qp_out_val: default_qp_talbe,
        }
    }
}

pub struct Packet {
    pub data: Vec<u8>,
    pub offset: usize,
    pub pts: u64,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Packet {} - {} bytes", self.pts, self.data.len())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ChromaSampling {
    Cs420,
    Cs422,
    Cs444,
    Cs400,
}

impl Default for ChromaSampling {
    fn default() -> Self {
        ChromaSampling::Cs420
    }
}

impl ChromaSampling {
    // Provides the sampling period in the horizontal and vertical axes.
    pub fn sampling_period(self) -> (usize, usize) {
        use self::ChromaSampling::*;
        match self {
            Cs420 => (2, 2),
            Cs422 => (2, 1),
            Cs444 => (1, 1),
            Cs400 => (2, 2),
        }
    }
}

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum PixelRange {
    Unspecified = 0,
    Limited,
    Full,
}

impl Default for PixelRange {
    fn default() -> Self {
        PixelRange::Unspecified
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Rational {
    pub num: u64,
    pub den: u64,
}

impl Rational {
    pub fn new(num: u64, den: u64) -> Self {
        Rational { num, den }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config { threads: 0 }
    }
}

pub struct Context<T: Pixel> {
    pub(crate) frame: Option<Frame<T>>,
    pub(crate) packet: Option<Packet>,

    evcd_ctx: EvcdCtx,
}

impl<T: Pixel> Context<T> {
    pub fn new(cfg: &Config) -> Self {
        let mut evcd_ctx = EvcdCtx::default();
        evcd_ctx.magic = EVCD_MAGIC_CODE;

        //TODO:
        //evc_scan_tbl_init
        //evc_init_multi_tbl();
        //evc_init_multi_inv_tbl();

        Context {
            frame: None,
            packet: None,
            evcd_ctx,
        }
    }

    pub fn decode(&mut self, pkt: &mut Option<Packet>) -> Result<EvcdStat, EvcError> {
        Ok(EvcdStat::default())
    }

    pub fn pull(&mut self) -> Result<Frame<T>, EvcError> {
        Err(EvcError::default())
    }
}