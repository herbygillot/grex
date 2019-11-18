/*
 * Copyright © 2019 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster {
    pub(crate) value: String,
    pub(crate) min_occurrence: u32,
    pub(crate) max_occurrence: u32,
}

impl GraphemeCluster {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
            min_occurrence: 1,
            max_occurrence: 1,
        }
    }
}
