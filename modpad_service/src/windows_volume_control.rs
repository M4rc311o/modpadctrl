use std::collections::HashMap;

use windows::core::Interface;
use windows::Win32::Media::Audio::{eConsole, eRender, ISimpleAudioVolume};
use windows::Win32::Media::{Audio, KernelStreaming::GUID_NULL};
use windows::Win32::System::Com::{self, CoInitializeEx, CoUninitialize, CLSCTX_ALL};

pub struct Application {
    name: String,
    sessions: Vec<Audio::ISimpleAudioVolume>
}

impl Application {
    pub fn set_volume(&self, volume: f32) -> Result<(), windows::core::Error> {
        for session in self.sessions.iter() {
            unsafe {session.SetMasterVolume(volume, &GUID_NULL)?;}
        }
        Ok(())
    }

    pub fn set_session_volume(&self, volume: f32, session: usize) -> Result<(), windows::core::Error> {
        if let Some(session) = self.sessions.get(session) {
            unsafe {session.SetMasterVolume(volume, &GUID_NULL)?;}
        } else {
            if let Some(session) = self.sessions.last() {
                unsafe {session.SetMasterVolume(volume, &GUID_NULL)?;}
            }
        }
        Ok(())
    }

    pub fn get_volume(&self) -> Result<f32, windows::core::Error> {
        if let Some(session) = self.sessions.first() {
            let volume = unsafe {session.GetMasterVolume()?};
            Ok(volume)
        } else {
            Ok(0f32)
        }
    }
}

pub struct ApplicationManager {
    applications: HashMap<String, Application>
}

impl Drop for ApplicationManager {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

impl ApplicationManager {
    pub fn new() -> Result<Self, windows::core::Error> {
        unsafe {CoInitializeEx(None, Com::COINIT_MULTITHREADED).ok()?;}

        let device_enumerator = unsafe {Com::CoCreateInstance::<_, Audio::IMMDeviceEnumerator>(
            &Audio::MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        )?};
        let device = unsafe {device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?};
        let session_manager2 = unsafe {device.Activate::<Audio::IAudioSessionManager2>(CLSCTX_ALL, None)?};
        let session_enumerator = unsafe {session_manager2.GetSessionEnumerator()?};
        let session_count = unsafe {session_enumerator.GetCount()?};

        let mut applications = HashMap::new();
        
        for s in 0..session_count {
            let session_control2 = unsafe {session_enumerator.GetSession(s)?}.cast::<Audio::IAudioSessionControl2>()?;
            let session_identifier = unsafe {session_control2.GetSessionInstanceIdentifier()?.to_string()?};
            let simple_volume = session_control2.cast::<ISimpleAudioVolume>()?;
            if let Some(name) = session_identifier
                .rsplit_once("\\")
                .and_then(|(_, p)| p.split_once("%").map(|(name, _)| name))
            {
                let application = applications.entry(name.to_string().to_lowercase()).or_insert(Application {name: name.to_string(), sessions: Vec::new()});
                application.sessions.push(simple_volume);
            }
        }
    
        Ok(Self {applications})
    }

    pub fn find(&self, name: &str) -> Option<&Application> {
        self.applications.get(name)
    }
}
