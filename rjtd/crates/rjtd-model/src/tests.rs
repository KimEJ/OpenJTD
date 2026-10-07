#[cfg(feature = "rendering")]
mod character_style;
mod compressed_container;
#[cfg(feature = "rendering")]
mod document_core;
mod document_text;
#[cfg(feature = "rendering")]
mod fdm_and_embedded_press;
#[cfg(feature = "rendering")]
mod fields;
#[cfg(feature = "rendering")]
mod footnote;
#[cfg(feature = "rendering")]
mod local_samples;
#[cfg(feature = "rendering")]
mod native_equation;
#[cfg(feature = "rendering")]
mod native_figures;
#[cfg(feature = "bitmap-images")]
mod native_image;
#[cfg(feature = "rendering")]
mod native_plain_pitch;
#[cfg(feature = "rendering")]
mod native_running;
#[cfg(feature = "rendering")]
mod native_toc;
#[cfg(feature = "rendering")]
mod native_tracking;
#[cfg(feature = "rendering")]
mod native_vertical;
#[cfg(feature = "rendering")]
mod page_grid_y_anchor_and_record_flags;
#[cfg(feature = "rendering")]
mod shanai_lan;
mod support;
#[cfg(feature = "rendering")]
mod table_grid_and_table_candidate;

#[cfg(feature = "rendering")]
use document_text::*;
#[cfg(feature = "rendering")]
use fdm_and_embedded_press::*;
#[cfg(feature = "rendering")]
use local_samples::*;
#[cfg(feature = "rendering")]
use shanai_lan::*;
use support::*;
#[cfg(feature = "rendering")]
use table_grid_and_table_candidate::*;

#[cfg(feature = "rendering")]
use rjtd_core::style_stream::DOCUMENT_VIEW_STYLES_PATH;
