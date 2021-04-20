/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor
 *  
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use sandbox_client::sandboxconfig;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let _config = sandboxconfig::SandboxConfig::new();

        assert_eq!(_config.source_file, String::from(""));
        assert_eq!(_config.input_file, String::from(""));
        assert_eq!(_config.output_file, String::from(""));
        assert_eq!(_config.compiler_output_file, String::from(""));
        assert_eq!(_config.compile_cmd, String::from(""));
        assert_eq!(_config.executable, String::from(""));
        assert_eq!(_config.executable_args.len(), 0);
        assert_eq!(_config.uid, 0);
        assert_eq!(_config.gid, 0);
        assert_eq!(_config.time_limit, 0);
        assert_eq!(_config.memory_limit, 0);
        assert_eq!(_config.uninitialized, true);
    }
}