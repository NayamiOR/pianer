/*
 * Copyright 2025. NayamiOR
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use crate::SearchResultEntry;

fn print_schema() {
    let schema = serde_json::to_string(&SearchResultEntry {
        keyword: "example".to_string(),
        file_name: "example.txt".to_string(),
        file_path: "/path/to/example.txt".to_string(),
    })
        .unwrap();
    println!("{}", schema); // 输出: {"percentage":0.5}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_schema() {
        print_schema();
    }
}
