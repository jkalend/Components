//! Pinned external backend descriptors.
//!
//! The component defines and validates the pin schema. The consuming product owns
//! the pin document itself and passes its bytes to [`load_from`], so evidence
//! records the document that product actually shipped.

use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub url: String,
    pub sha256: String,
    pub executable_sha256: String,
    pub member: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendPin {
    pub version: String,
    pub license: String,
    pub source: String,
    pub windows: Asset,
    pub linux: Asset,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pins {
    pub schema_version: u32,
    pub restic: BackendPin,
    pub kopia: BackendPin,
}

pub fn load_from(bytes: &[u8]) -> Result<Pins> {
    let pins: Pins = serde_json::from_slice(bytes).context("invalid backend pins")?;
    ensure!(pins.schema_version == 2, "unsupported backend pin schema");
    for backend in [&pins.restic, &pins.kopia] {
        ensure!(
            !backend.version.is_empty()
                && !backend.license.is_empty()
                && backend.source.starts_with("https://github.com/"),
            "invalid backend metadata"
        );
        for asset in [&backend.windows, &backend.linux] {
            ensure!(
                asset.url.starts_with("https://github.com/")
                    && asset.url.contains(&format!("/v{}/", backend.version)),
                "backend pin version/URL drift"
            );
            for hash in [&asset.sha256, &asset.executable_sha256] {
                ensure!(
                    hash.len() == 64
                        && hash
                            .bytes()
                            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
                    "invalid backend digest pin"
                );
            }
            if let Some(member) = &asset.member {
                ensure!(
                    !member.is_empty() && !member.starts_with('/') && !member.contains(".."),
                    "unsafe archive member pin"
                );
            }
        }
    }
    Ok(pins)
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
