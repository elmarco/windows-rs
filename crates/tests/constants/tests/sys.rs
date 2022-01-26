use windows_sys::{Win32::Media::Audio::*};

#[test]
fn backslash() {
    assert!(VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK == "VAD\\Process_Loopback");
}
