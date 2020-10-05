// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

use sgx_libc::{c_float, c_double};

#[link_name = "sgx_tstdc"]
extern "C" {
    pub fn acos(n: c_double) -> c_double;
    pub fn acosf(n: c_float) -> c_float;
    pub fn asin(n: c_double) -> c_double;
    pub fn asinf(n: c_float) -> c_float;
    pub fn atan(n: c_double) -> c_double;
    pub fn atan2(a: c_double, b: c_double) -> c_double;
    pub fn atan2f(a: c_float, b: c_float) -> c_float;
    pub fn atanf(n: c_float) -> c_float;
    pub fn cbrt(n: c_double) -> c_double;
    pub fn cbrtf(n: c_float) -> c_float;
    pub fn cosh(n: c_double) -> c_double;
    pub fn coshf(n: c_float) -> c_float;
    pub fn expm1(n: c_double) -> c_double;
    pub fn expm1f(n: c_float) -> c_float;
    pub fn fdim(a: c_double, b: c_double) -> c_double;
    pub fn fdimf(a: c_float, b: c_float) -> c_float;
    pub fn hypot(x: c_double, y: c_double) -> c_double;
    pub fn hypotf(x: c_float, y: c_float) -> c_float;
    pub fn log1p(n: c_double) -> c_double;
    pub fn log1pf(n: c_float) -> c_float;
    pub fn sinh(n: c_double) -> c_double;
    pub fn sinhf(n: c_float) -> c_float;
    pub fn tan(n: c_double) -> c_double;
    pub fn tanf(n: c_float) -> c_float;
    pub fn tanh(n: c_double) -> c_double;
    pub fn tanhf(n: c_float) -> c_float;
}