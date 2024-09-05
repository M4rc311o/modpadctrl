use std::fmt;
use std::process;
use log::LevelFilter;
use modpadctrl::ModpadApi;
use windows::core::Interface;
use windows::Win32::Media::Audio::{
    self, eConsole, eRender, ISimpleAudioVolume,
};
use windows::Win32::Media::KernelStreaming::GUID_NULL;
use windows::Win32::System::Com::{
    self, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

struct Session {
    name: String,
    control: ISimpleAudioVolume,
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {} Volume: {}", self.name, self.get_volume())
        }
    }
struct Sessions {
    list: Vec<Session>,
}

impl Session {
    fn set_volume(&self,volume: f32) {
        unsafe {
            self.control
                .SetMasterVolume(volume, &GUID_NULL).unwrap()
        }
    }
    fn get_volume(&self) -> f32 {
        unsafe {
            self.control.GetMasterVolume().unwrap()
        }
    }
}


impl Sessions {
    fn find(&self, name: &str) -> Option<&Session> {
        self.list.iter().position(|session| session.name == name).map(|index| &self.list[index])
    }
    fn de_init(&self){
        unsafe {
            CoUninitialize();
        }
    }
}

fn list_all_sessions() -> Sessions {
    unsafe {
        //Init of COM library
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        //Creating IMMDevice
        let device_enumerator: Audio::IMMDeviceEnumerator =
            Com::CoCreateInstance::<_, Audio::IMMDeviceEnumerator>(
                &Audio::MMDeviceEnumerator,
                None,
                CLSCTX_ALL,
            )
            .unwrap();
        let device = device_enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .unwrap();
        //Activating different volume control interfaces
        let sessions_manager2 = device
            .Activate::<Audio::IAudioSessionManager2>(CLSCTX_ALL, None)
            .unwrap();
        //SimpleAudio interface (using AudioSessionManager1/2)
        let session_enumerator = sessions_manager2.GetSessionEnumerator().unwrap();
        let count = session_enumerator.GetCount().unwrap();

        let mut list = vec![];
        for num in 0..count {
            //Creating a control interface for the session that is currently being controlled
            let session_control = session_enumerator.GetSession(num).unwrap();
            //Getting the path of the program that is controlled by the session
            let session_control2 = session_control
                .cast::<Audio::IAudioSessionControl2>()
                .expect("Cast session2 not worky");
            let identifier = session_control2
                .GetSessionInstanceIdentifier()
                .unwrap()
                .to_string()
                .unwrap();
            //Extracting the name of the program from the path
            let name = identifier
                .rsplit_once("\\")
                .and_then(|(_, part)| part.split_once("%").map(|(application, _)| application)) 
                .unwrap_or("Unknown");
            //println!("{name}");
            let control = session_control
                .cast::<Audio::ISimpleAudioVolume>()
                .expect("Cast not worky");
            list.push(Session {name: name.to_string(),control,})
        }
        Sessions { list }
    }
}

fn list_session(name: &str) -> Sessions {
    unsafe {
        //Init of COM library
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        //Creating IMMDevice
        let device_enumerator: Audio::IMMDeviceEnumerator =
            Com::CoCreateInstance::<_, Audio::IMMDeviceEnumerator>(
                &Audio::MMDeviceEnumerator,
                None,
                CLSCTX_ALL,
            )
            .unwrap();
        let device = device_enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .unwrap();
        //Activating different volume control interfaces
        let sessions_manager2 = device
            .Activate::<Audio::IAudioSessionManager2>(CLSCTX_ALL, None)
            .unwrap();
        //SimpleAudio interface (using AudioSessionManager1/2)
        let session_enumerator = sessions_manager2.GetSessionEnumerator().unwrap();
        let count = session_enumerator.GetCount().unwrap();

        let mut list = vec![];
        for num in 0..count {
            //Creating a control interface for the session that is currently being controlled
            let session_control = session_enumerator.GetSession(num).unwrap();
            //Getting the path of the program that is controlled by the session
            let session_control2 = session_control
                .cast::<Audio::IAudioSessionControl2>()
                .expect("Cast session2 not worky");
            let identifier = session_control2
                .GetSessionInstanceIdentifier()
                .unwrap()
                .to_string()
                .unwrap();
            //Extracting the name of the program from the path
            let application = match identifier.rsplit_once("\\").and_then(|(_, part)| part.split_once("%").map(|(application, _)| application)){
                Some(application) => if application == name {
                    application
                } else {
                    continue
                },
                None => {
                    //println!("Application {name} not found");
                    continue
                }
            };
            let control = session_control
                .cast::<Audio::ISimpleAudioVolume>()
                .expect("Cast not worky");
            list.push(Session {name: application.to_string(),control,})
        }
        Sessions { list }
    }
}



fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let modpad_api = ModpadApi::new().unwrap_or_else(|err| {
        log::error!("Creating ModpadApi failed: {err:?}");
        process::exit(1)
    });
    log::info!("ModpadApi created");
    let all_sessions = list_all_sessions();
    let apps = ["steam.exe", "chrome.exe", "Spotify.exe"];

    //report.set_blocking_mode(true).unwrap();
    let mut prev_data: Vec<u8> = vec![0u8;3];
    loop {
        let data = modpad_api.read_report().unwrap();
        if !data.iter().eq(&prev_data){
            let change = data.iter().zip(&prev_data).position(|(new, prev)| new != prev).expect("Failed to get what index changed");
            prev_data[change] = data [change].clone();
            match all_sessions.find(apps[change]) {
                Some(session) => {
                    session.set_volume((data[change] as f32)/100.0);
                    log::info!("{session}");
                },
                None => log::warn!("Session {} not found",apps[change])
            }
        }
    }
    all_sessions.de_init();

}