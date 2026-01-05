// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

#[inline]
pub fn psyq_path(suffix: &str) -> String {
    format!("../tests/data/psy-q/{suffix}")
}
