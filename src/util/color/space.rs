use std::{ffi::CStr, str::from_utf8_unchecked};

use crate::ffi::{AVColorSpace::*, *};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Space {
	RGB,
	BT709,
	Unspecified,
	Reserved,
	FCC,
	BT470BG,
	SMPTE170M,
	SMPTE240M,
	YCGCO,
	BT2020NCL,
	BT2020CL,
	SMPTE2085,

	ChromaDerivedNCL,
	ChromaDerivedCL,
	ICTCP,
}

impl Space {
	pub fn name(&self) -> &'static str {
		unsafe { from_utf8_unchecked(CStr::from_ptr(av_get_colorspace_name((*self).into())).to_bytes()) }
	}
}

impl From<AVColorSpace> for Space {
	fn from(value: AVColorSpace) -> Self {
		#[allow(unreachable_patterns)]
		match value {
			AVCOL_SPC_RGB => Space::RGB,
			AVCOL_SPC_BT709 => Space::BT709,
			AVCOL_SPC_UNSPECIFIED => Space::Unspecified,
			AVCOL_SPC_RESERVED => Space::Reserved,
			AVCOL_SPC_FCC => Space::FCC,
			AVCOL_SPC_BT470BG => Space::BT470BG,
			AVCOL_SPC_SMPTE170M => Space::SMPTE170M,
			AVCOL_SPC_SMPTE240M => Space::SMPTE240M,
			AVCOL_SPC_YCGCO => Space::YCGCO,
			AVCOL_SPC_BT2020_NCL => Space::BT2020NCL,
			AVCOL_SPC_BT2020_CL => Space::BT2020CL,
			AVCOL_SPC_SMPTE2085 => Space::SMPTE2085,
			AVCOL_SPC_NB => Space::Unspecified,

			AVCOL_SPC_CHROMA_DERIVED_NCL => Space::ChromaDerivedNCL,
			AVCOL_SPC_CHROMA_DERIVED_CL => Space::ChromaDerivedCL,
			AVCOL_SPC_ICTCP => Space::ICTCP,

			_ => unimplemented!(),
		}
	}
}

impl Into<AVColorSpace> for Space {
	fn into(self) -> AVColorSpace {
		match self {
			Space::RGB => AVCOL_SPC_RGB,
			Space::BT709 => AVCOL_SPC_BT709,
			Space::Unspecified => AVCOL_SPC_UNSPECIFIED,
			Space::Reserved => AVCOL_SPC_RESERVED,
			Space::FCC => AVCOL_SPC_FCC,
			Space::BT470BG => AVCOL_SPC_BT470BG,
			Space::SMPTE170M => AVCOL_SPC_SMPTE170M,
			Space::SMPTE240M => AVCOL_SPC_SMPTE240M,
			Space::YCGCO => AVCOL_SPC_YCGCO,
			Space::BT2020NCL => AVCOL_SPC_BT2020_NCL,
			Space::BT2020CL => AVCOL_SPC_BT2020_CL,
			Space::SMPTE2085 => AVCOL_SPC_SMPTE2085,

			Space::ChromaDerivedNCL => AVCOL_SPC_CHROMA_DERIVED_NCL,
			Space::ChromaDerivedCL => AVCOL_SPC_CHROMA_DERIVED_CL,
			Space::ICTCP => AVCOL_SPC_ICTCP,
		}
	}
}
