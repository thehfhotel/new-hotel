//! Server-side document rendering (compositing).
//!
//! Pure, self-contained image builders — the only I/O is reading the on-disk
//! template + font files whose paths the caller passes in. No DB access, no
//! network, no legacy MSSQL: these live below the route layer purely so the
//! request path can composite a document in-process.
//!
//! - [`thai_id_card`] — reconstruct the iHOTEL Thai national-ID card face
//!   (template background + chip face photo + shaped Thai/English text) as a
//!   PNG. Clean reimplement from the extracted iHOTEL ReportReg layout recipe;
//!   NO vendor library.

pub mod thai_id_card;
