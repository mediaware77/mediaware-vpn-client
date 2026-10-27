//! Per-user Windows startup registration. The registry contains no credentials.
use crate::error::Error;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::Path};
use windows_sys::Win32::{
    Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS},
    System::Registry::{RegDeleteKeyValueW, RegSetKeyValueW, HKEY_CURRENT_USER, REG_SZ},
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "PMCGVPN";
pub const STARTUP_ARGUMENT: &str = "--auto-connect";

fn wide(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain(Some(0)).collect()
}

fn write_entry(value_name: &str, executable: Option<&Path>) -> std::io::Result<()> {
    let key = wide(OsStr::new(RUN_KEY));
    let name = wide(OsStr::new(value_name));
    // Registry data, never a shell command. Quoting supports paths with spaces and Unicode.
    let status = if let Some(path) = executable {
        let mut command = std::ffi::OsString::from("\"");
        command.push(path);
        command.push(format!("\" {STARTUP_ARGUMENT}"));
        let data = wide(&command);
        unsafe {
            RegSetKeyValueW(
                HKEY_CURRENT_USER,
                key.as_ptr(),
                name.as_ptr(),
                REG_SZ,
                data.as_ptr().cast(),
                (data.len() * 2) as u32,
            )
        }
    } else {
        unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr()) }
    };
    if status == ERROR_SUCCESS || (executable.is_none() && status == ERROR_FILE_NOT_FOUND) {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(status as i32))
    }
}

pub fn set_enabled(enabled: bool) -> Result<(), Error> {
    let executable = std::env::current_exe()?;
    write_entry(VALUE_NAME, enabled.then_some(executable.as_path()))?;
    Ok(())
}

pub fn launched_at_logon() -> bool {
    std::env::args().any(|argument| argument == STARTUP_ARGUMENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::System::Registry::{RegGetValueW, RRF_RT_REG_SZ};

    #[test]
    fn registry_entry_quotes_unicode_path_and_can_be_removed() {
        let name = format!("PMCGVPN-Test-{}", std::process::id());
        let path = Path::new(r"C:\Program Files\Prefeitura São José\defguard-client.exe");
        write_entry(&name, Some(path)).unwrap();
        let key = wide(OsStr::new(RUN_KEY));
        let value_name = wide(OsStr::new(&name));
        let mut buffer = [0u16; 1024];
        let mut size = (buffer.len() * 2) as u32;
        let status = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                key.as_ptr(),
                value_name.as_ptr(),
                RRF_RT_REG_SZ,
                std::ptr::null_mut(),
                buffer.as_mut_ptr().cast(),
                &mut size,
            )
        };
        write_entry(&name, None).unwrap();
        assert_eq!(status, ERROR_SUCCESS);
        let value = String::from_utf16(&buffer[..size as usize / 2 - 1]).unwrap();
        assert_eq!(value, format!("\"{}\" --auto-connect", path.display()));
        write_entry(&name, None).unwrap(); // disabling twice is harmless
    }
}
