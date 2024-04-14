// SPDX-License-Identifier: MIT OR Apache-2.0
// This file is part of Static Web Server.
// See https://static-web-server.net/ for more information
// Copyright (C) 2019-present Jose Quintana <joseluisq.net>

//! Development utilities for testing of SWS.
//!

/// SWS fixtures module.
#[doc(hidden)]
pub mod fixtures {
    use std::{path::PathBuf, sync::Arc};

    use http::StatusCode;

    use crate::{
        cors,
        directory_listing::DirListFmt,
        handler::{RequestHandler, RequestHandlerOpts},
        settings::{file::Advanced as FileAdvanced, read_file_settings, Advanced, Settings},
    };

    /// Testing Remote address
    pub const REMOTE_ADDR: &str = "127.0.0.1:1234";

    /// Create a `RequestHandler` from a custom TOML config file (fixture).
    pub fn fixture_req_handler(fixture_toml: &str) -> RequestHandler {
        // load the fixture TOML settings
        let f = PathBuf::from("tests/fixtures").join(fixture_toml);

        let general: crate::settings::file::General;
        let advanced: FileAdvanced;

        if let Some((settings, _)) = read_file_settings(f.as_path()).unwrap() {
            general = settings.general.unwrap();
            advanced = settings.advanced.unwrap();
        } else {
            general = Default::default();
            advanced = Default::default();
        }

        let req_handler_opts = RequestHandlerOpts {
            root_dir: general.root.unwrap_or(PathBuf::new()),
            compression: general.compression.unwrap_or(true),
            compression_static: general.compression_static.unwrap_or(true),
            #[cfg(feature = "directory-listing")]
            dir_listing: general.directory_listing.unwrap_or(false),
            #[cfg(feature = "directory-listing")]
            dir_listing_order: general.directory_listing_order.unwrap_or(6),
            #[cfg(feature = "directory-listing")]
            dir_listing_format: general.directory_listing_format.unwrap_or(DirListFmt::Html),
            cors: cors::new(
                &general.cors_allow_origins.unwrap_or("".to_string()),
                &general.cors_allow_headers.unwrap_or("".to_string()),
                &general.cors_expose_headers.unwrap_or("".to_string()),
            ),
            security_headers: general.security_headers.unwrap_or(false),
            cache_control_headers: general.cache_control_headers.unwrap_or(true),
            page404: general.page404.unwrap_or(PathBuf::new()),
            page50x: general.page50x.unwrap_or(PathBuf::new()),
            // TODO: add support or `page_fallback` when required
            #[cfg(feature = "fallback-page")]
            page_fallback: vec![],
            #[cfg(feature = "basic-auth")]
            basic_auth: general.basic_auth.unwrap_or("".to_string()),
            log_remote_address: general.log_remote_address.unwrap_or(false),
            redirect_trailing_slash: general.redirect_trailing_slash.unwrap_or(false),
            ignore_hidden_files: general.ignore_hidden_files.unwrap_or(false),
            index_files: vec![general.index_files.unwrap_or("".to_string())],
            health: general.health.unwrap_or(false),
            #[cfg(all(unix, feature = "experimental"))]
            experimental_metrics: opts.general.experimental_metrics,
            maintenance_mode: general.maintenance_mode.unwrap_or(false),
            maintenance_mode_status: StatusCode::from_u16(
                general.maintenance_mode_status.unwrap_or(503 as u16),
            )
            .unwrap_or(StatusCode::SERVICE_UNAVAILABLE),
            maintenance_mode_file: general.maintenance_mode_file.unwrap_or(PathBuf::new()),
            advanced_opts: Some(Advanced {
                headers: Settings::decode_headers(advanced.headers),
                rewrites: Settings::decode_rewrites(advanced.rewrites),
                redirects: Settings::decode_redirects(advanced.redirects),
                virtual_hosts: Settings::decode_vhosts(advanced.virtual_hosts),
            }),
        };

        RequestHandler {
            opts: Arc::from(req_handler_opts),
        }
    }
}
