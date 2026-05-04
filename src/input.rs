use std::collections::VecDeque;
use std::default;
use std::ops::Index;

use crate::vec3::Vec3;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey, SmolStr};

pub struct Input {
    pub keyqueue: VecDeque<(KeyID, ElementState)>,
    pub keymaps: Keymaps,
    pub heldkeys: ActiveKeyLogger,
    pub mouse_sensitivity: f64,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyqueue: VecDeque::new(),
            keymaps: Default::default(),
            heldkeys: Default::default(),
            mouse_sensitivity: 2.0,
        }
    }
}

/*




















*/

#[derive(Default)]
pub struct ActiveKeyLogger {
    pub buffer: [KeyLog; KeyID::MaxControlCount as usize],
}

impl ActiveKeyLogger {
    pub fn add(&mut self, key: KeyID) {
        self.buffer[key as usize] = KeyLog::from(-1);
    }

    pub fn remove(&mut self, slice: &[KeyID]) {
        for i in 0..slice.len() {
            let index = slice[i] as usize;
            if self.buffer[index].is_valid() {
                self.buffer[index] = KeyLog::from(-1);
            }
        }
    }

    pub fn tick(&mut self, time: i32) {
        self.buffer.iter_mut().for_each(|key| {
            if key.is_valid() {
                key.duration += time;
            }
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct KeyLog {
    pub duration: i32,
}

impl KeyLog {
    #[inline(always)]
    pub fn is_valid(self) -> bool {
        !self.duration.is_negative()
    }
}

impl From<i32> for KeyLog {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self { duration: value }
    }
}
/*




















    KEYMAP SETTINGS

    each logical key is converted to a ControlOption, which is an integer denoting the simplified action of the key

    instead of hardcoding KeyCode::W as forward, the name of the
*/
pub struct Keymaps {
    pub keys: [KeyCode; KeyID::MaxControlCount as usize],
}

impl Keymaps {
    pub fn key_to_control(self, key: &KeyCode) -> KeyID {
        for i in 0..self.keys.len() {
            if *key == self.keys[i] {
                return KeyID::from(i);
            }
        }

        return KeyID::Invalid;
    }

    pub fn remap(&mut self, index: KeyID, new_key: KeyCode) {
        self.keys[index as usize] = new_key;
    }

    pub fn key_for(&mut self, index: KeyID) -> Option<&KeyCode> {
        if index as usize >= KeyID::MaxControlCount as usize {
            return None;
        }

        Some(&self.keys[index as usize])
    }
}

impl Default for Keymaps {
    fn default() -> Self {
        let mut arr: [KeyCode; KeyID::MaxControlCount as usize] =
            std::array::from_fn(|_| KeyCode::Named(SmolStr::new_inline("blank")));

        arr[KeyID::Forward as usize] = KeyCode::Char(SmolStr::new_inline("w"));

        arr[KeyID::Backward as usize] = KeyCode::Char(SmolStr::new_inline("s"));

        arr[KeyID::Left as usize] = KeyCode::Char(SmolStr::new_inline("a"));

        arr[KeyID::Right as usize] = KeyCode::Char(SmolStr::new_inline("d"));

        arr[KeyID::Jump as usize] = KeyCode::Char(SmolStr::new_inline("w"));

        arr[KeyID::Sprint as usize] = KeyCode::Named(SmolStr::new_inline("shift"));

        arr[KeyID::Sneak as usize] = KeyCode::Named(SmolStr::new_inline("control"));

        Self { keys: arr }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum KeyCode {
    Char(SmolStr),
    Named(SmolStr),
}

impl From<Key> for KeyCode {
    fn from(value: Key) -> Self {
        match value {
            Key::Named(key) => KeyCode::Named(key_to_str(key).into()),
            Key::Character(str) => KeyCode::Char(str),
            _ => unreachable!("only named and character variants will ever be converted"),
        }
    }
}

#[repr(usize)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Default)]
pub enum KeyID {
    Forward = 0,
    Backward,
    Left,
    Right,
    Jump,
    Sprint,
    Sneak,
    MaxControlCount,
    #[default]
    Invalid,
}

impl From<usize> for KeyID {
    fn from(i: usize) -> Self {
        unsafe { std::mem::transmute(i as usize) }
    }
}

const fn key_to_str(key: NamedKey) -> SmolStr {
    match key {
        NamedKey::Alt => SmolStr::new_inline("alt"),
        NamedKey::AltGraph => SmolStr::new_inline("altgraph"),
        NamedKey::CapsLock => SmolStr::new_inline("capslock"),
        NamedKey::Control => SmolStr::new_inline("control"),
        NamedKey::Fn => SmolStr::new_inline("fn"),
        NamedKey::FnLock => SmolStr::new_inline("fnlock"),
        NamedKey::NumLock => SmolStr::new_inline("numlock"),
        NamedKey::ScrollLock => SmolStr::new_inline("scrolllock"),
        NamedKey::Shift => SmolStr::new_inline("shift"),
        NamedKey::Symbol => SmolStr::new_inline("symbol"),
        NamedKey::SymbolLock => SmolStr::new_inline("symbollock"),
        NamedKey::Meta => SmolStr::new_inline("meta"),
        NamedKey::Hyper => SmolStr::new_inline("hyper"),
        NamedKey::Super => SmolStr::new_inline("super"),
        NamedKey::Enter => SmolStr::new_inline("enter"),
        NamedKey::Tab => SmolStr::new_inline("tab"),
        NamedKey::Space => SmolStr::new_inline("space"),
        NamedKey::ArrowDown => SmolStr::new_inline("arrowdown"),
        NamedKey::ArrowLeft => SmolStr::new_inline("arrowleft"),
        NamedKey::ArrowRight => SmolStr::new_inline("arrowright"),
        NamedKey::ArrowUp => SmolStr::new_inline("arrowup"),
        NamedKey::End => SmolStr::new_inline("end"),
        NamedKey::Home => SmolStr::new_inline("home"),
        NamedKey::PageDown => SmolStr::new_inline("pagedown"),
        NamedKey::PageUp => SmolStr::new_inline("pageUp"),
        NamedKey::Backspace => SmolStr::new_inline("backspace"),
        NamedKey::Clear => SmolStr::new_inline("clear"),
        NamedKey::Copy => SmolStr::new_inline("copy"),
        NamedKey::CrSel => SmolStr::new_inline("crsel"),
        NamedKey::Cut => SmolStr::new_inline("cut"),
        NamedKey::Delete => SmolStr::new_inline("delete"),
        NamedKey::EraseEof => SmolStr::new_inline("eraseeof"),
        NamedKey::ExSel => SmolStr::new_inline("exsel"),
        NamedKey::Insert => SmolStr::new_inline("insert"),
        NamedKey::Paste => SmolStr::new_inline("paste"),
        NamedKey::Redo => SmolStr::new_inline("redo"),
        NamedKey::Undo => SmolStr::new_inline("undo"),
        NamedKey::Accept => SmolStr::new_inline("accept"),
        NamedKey::Again => SmolStr::new_inline("again"),
        NamedKey::Attn => SmolStr::new_inline("attn"),
        NamedKey::Cancel => SmolStr::new_inline("cancel"),
        NamedKey::ContextMenu => SmolStr::new_inline("contextmenu"),
        NamedKey::Escape => SmolStr::new_inline("escape"),
        NamedKey::Execute => SmolStr::new_inline("execute"),
        NamedKey::Find => SmolStr::new_inline("find"),
        NamedKey::Help => SmolStr::new_inline("help"),
        NamedKey::Pause => SmolStr::new_inline("pause"),
        NamedKey::Play => SmolStr::new_inline("play"),
        NamedKey::Props => SmolStr::new_inline("props"),
        NamedKey::Select => SmolStr::new_inline("select"),
        NamedKey::ZoomIn => SmolStr::new_inline("zoomIn"),
        NamedKey::ZoomOut => SmolStr::new_inline("zoomout"),
        NamedKey::BrightnessDown => SmolStr::new_inline("brightnessdown"),
        NamedKey::BrightnessUp => SmolStr::new_inline("brightnessup"),
        NamedKey::Eject => SmolStr::new_inline("eject"),
        NamedKey::LogOff => SmolStr::new_inline("logOff"),
        NamedKey::Power => SmolStr::new_inline("power"),
        NamedKey::PowerOff => SmolStr::new_inline("poweroff"),
        NamedKey::PrintScreen => SmolStr::new_inline("printscreen"),
        NamedKey::Hibernate => SmolStr::new_inline("hibernate"),
        NamedKey::Standby => SmolStr::new_inline("standby"),
        NamedKey::WakeUp => SmolStr::new_inline("wakeUp"),
        NamedKey::AllCandidates => SmolStr::new_inline("allcandidates"),
        NamedKey::Alphanumeric => SmolStr::new_inline("alphanumeric"),
        NamedKey::CodeInput => SmolStr::new_inline("codeinput"),
        NamedKey::Compose => SmolStr::new_inline("compose"),
        NamedKey::Convert => SmolStr::new_inline("convert"),
        NamedKey::FinalMode => SmolStr::new_inline("finalmode"),
        NamedKey::GroupFirst => SmolStr::new_inline("groupfirst"),
        NamedKey::GroupLast => SmolStr::new_inline("grouplast"),
        NamedKey::GroupNext => SmolStr::new_inline("groupnext"),
        NamedKey::GroupPrevious => SmolStr::new_inline("groupprevious"),
        NamedKey::ModeChange => SmolStr::new_inline("modechange"),
        NamedKey::NextCandidate => SmolStr::new_inline("nextcandidate"),
        NamedKey::NonConvert => SmolStr::new_inline("nonconvert"),
        NamedKey::PreviousCandidate => SmolStr::new_inline("previouscandidate"),
        NamedKey::Process => SmolStr::new_inline("process"),
        NamedKey::SingleCandidate => SmolStr::new_inline("singlecandidate"),
        NamedKey::HangulMode => SmolStr::new_inline("hangulmode"),
        NamedKey::HanjaMode => SmolStr::new_inline("hanjamode"),
        NamedKey::JunjaMode => SmolStr::new_inline("junjamode"),
        NamedKey::Eisu => SmolStr::new_inline("eisu"),
        NamedKey::Hankaku => SmolStr::new_inline("hankaku"),
        NamedKey::Hiragana => SmolStr::new_inline("hiragana"),
        NamedKey::HiraganaKatakana => SmolStr::new_inline("hiraganakatakana"),
        NamedKey::KanaMode => SmolStr::new_inline("kanamode"),
        NamedKey::KanjiMode => SmolStr::new_inline("kanjimode"),
        NamedKey::Katakana => SmolStr::new_inline("katakana"),
        NamedKey::Romaji => SmolStr::new_inline("romaji"),
        NamedKey::Zenkaku => SmolStr::new_inline("zenkaku"),
        NamedKey::ZenkakuHankaku => SmolStr::new_inline("zenkakuhankaku"),
        NamedKey::Soft1 => SmolStr::new_inline("soft1"),
        NamedKey::Soft2 => SmolStr::new_inline("soft2"),
        NamedKey::Soft3 => SmolStr::new_inline("soft3"),
        NamedKey::Soft4 => SmolStr::new_inline("soft4"),
        NamedKey::ChannelDown => SmolStr::new_inline("channeldown"),
        NamedKey::ChannelUp => SmolStr::new_inline("channelup"),
        NamedKey::Close => SmolStr::new_inline("close"),
        NamedKey::MailForward => SmolStr::new_inline("mailforward"),
        NamedKey::MailReply => SmolStr::new_inline("mailreply"),
        NamedKey::MailSend => SmolStr::new_inline("mailsend"),
        NamedKey::MediaClose => SmolStr::new_inline("mediaclose"),
        NamedKey::MediaFastForward => SmolStr::new_inline("mediafastforward"),
        NamedKey::MediaPause => SmolStr::new_inline("mediapause"),
        NamedKey::MediaPlay => SmolStr::new_inline("mediaplay"),
        NamedKey::MediaPlayPause => SmolStr::new_inline("mediaplaypause"),
        NamedKey::MediaRecord => SmolStr::new_inline("mediarecord"),
        NamedKey::MediaRewind => SmolStr::new_inline("mediarewind"),
        NamedKey::MediaStop => SmolStr::new_inline("mediastop"),
        NamedKey::MediaTrackNext => SmolStr::new_inline("mediatracknext"),
        NamedKey::MediaTrackPrevious => SmolStr::new_inline("mediatrackprevious"),
        NamedKey::New => SmolStr::new_inline("new"),
        NamedKey::Open => SmolStr::new_inline("open"),
        NamedKey::Print => SmolStr::new_inline("print"),
        NamedKey::Save => SmolStr::new_inline("save"),
        NamedKey::SpellCheck => SmolStr::new_inline("spellcheck"),
        NamedKey::Key11 => SmolStr::new_inline("key11"),
        NamedKey::Key12 => SmolStr::new_inline("key12"),
        NamedKey::AudioBalanceLeft => SmolStr::new_inline("audioblnceleft"),
        NamedKey::AudioBalanceRight => SmolStr::new_inline("audioblnceright"),
        NamedKey::AudioBassBoostDown => SmolStr::new_inline("audiobassbstdown"),
        NamedKey::AudioBassBoostToggle => SmolStr::new_inline("audiobassbsttoggle"),
        NamedKey::AudioBassBoostUp => SmolStr::new_inline("audiobassboostup"),
        NamedKey::AudioFaderFront => SmolStr::new_inline("audiofaderfront"),
        NamedKey::AudioFaderRear => SmolStr::new_inline("audiofaderrear"),
        NamedKey::AudioSurroundModeNext => SmolStr::new_inline("audiosrrndmodenext"),
        NamedKey::AudioTrebleDown => SmolStr::new_inline("audiotrebledown"),
        NamedKey::AudioTrebleUp => SmolStr::new_inline("audiotrebleup"),
        NamedKey::AudioVolumeDown => SmolStr::new_inline("audiovolumedown"),
        NamedKey::AudioVolumeUp => SmolStr::new_inline("audiovolumeup"),
        NamedKey::AudioVolumeMute => SmolStr::new_inline("audiovolumemute"),
        NamedKey::MicrophoneToggle => SmolStr::new_inline("mictoggle"),
        NamedKey::MicrophoneVolumeDown => SmolStr::new_inline("micvolumedown"),
        NamedKey::MicrophoneVolumeUp => SmolStr::new_inline("micvolumeup"),
        NamedKey::MicrophoneVolumeMute => SmolStr::new_inline("micvolumemute"),
        NamedKey::SpeechCorrectionList => SmolStr::new_inline("speechcorrectionlist"),
        NamedKey::SpeechInputToggle => SmolStr::new_inline("speechinputtoggle"),
        NamedKey::LaunchApplication1 => SmolStr::new_inline("launchapplication1"),
        NamedKey::LaunchApplication2 => SmolStr::new_inline("launchapplication2"),
        NamedKey::LaunchCalendar => SmolStr::new_inline("launchcalendar"),
        NamedKey::LaunchContacts => SmolStr::new_inline("launchcontacts"),
        NamedKey::LaunchMail => SmolStr::new_inline("launchmail"),
        NamedKey::LaunchMediaPlayer => SmolStr::new_inline("launchmediaplayer"),
        NamedKey::LaunchMusicPlayer => SmolStr::new_inline("launchmusicplayer"),
        NamedKey::LaunchPhone => SmolStr::new_inline("launchphone"),
        NamedKey::LaunchScreenSaver => SmolStr::new_inline("launchscreensaver"),
        NamedKey::LaunchSpreadsheet => SmolStr::new_inline("launchspreadsheet"),
        NamedKey::LaunchWebBrowser => SmolStr::new_inline("launchwebbrowser"),
        NamedKey::LaunchWebCam => SmolStr::new_inline("launchwebcam"),
        NamedKey::LaunchWordProcessor => SmolStr::new_inline("launchwordprocessor"),
        NamedKey::BrowserBack => SmolStr::new_inline("browserback"),
        NamedKey::BrowserFavorites => SmolStr::new_inline("browserfavorites"),
        NamedKey::BrowserForward => SmolStr::new_inline("browserforward"),
        NamedKey::BrowserHome => SmolStr::new_inline("browserhome"),
        NamedKey::BrowserRefresh => SmolStr::new_inline("browserrefresh"),
        NamedKey::BrowserSearch => SmolStr::new_inline("browsersearch"),
        NamedKey::BrowserStop => SmolStr::new_inline("browserstop"),
        NamedKey::AppSwitch => SmolStr::new_inline("appswitch"),
        NamedKey::Call => SmolStr::new_inline("call"),
        NamedKey::Camera => SmolStr::new_inline("camera"),
        NamedKey::CameraFocus => SmolStr::new_inline("camerafocus"),
        NamedKey::EndCall => SmolStr::new_inline("endcall"),
        NamedKey::GoBack => SmolStr::new_inline("goBack"),
        NamedKey::GoHome => SmolStr::new_inline("goHome"),
        NamedKey::HeadsetHook => SmolStr::new_inline("headsethook"),
        NamedKey::LastNumberRedial => SmolStr::new_inline("lastnumberredial"),
        NamedKey::Notification => SmolStr::new_inline("notification"),
        NamedKey::MannerMode => SmolStr::new_inline("mannermode"),
        NamedKey::VoiceDial => SmolStr::new_inline("voicedial"),
        NamedKey::TV => SmolStr::new_inline("tV"),
        NamedKey::TV3DMode => SmolStr::new_inline("tv3dmode"),
        NamedKey::TVAntennaCable => SmolStr::new_inline("tvantennacable"),
        NamedKey::TVAudioDescription => SmolStr::new_inline("tvaudiodescription"),
        NamedKey::TVAudioDescriptionMixDown => SmolStr::new_inline("tvaudiodescmixdown"),
        NamedKey::TVAudioDescriptionMixUp => SmolStr::new_inline("tvaudiodescmixup"),
        NamedKey::TVContentsMenu => SmolStr::new_inline("tvcontentsmenu"),
        NamedKey::TVDataService => SmolStr::new_inline("tvdataservice"),
        NamedKey::TVInput => SmolStr::new_inline("tvinput"),
        NamedKey::TVInputComponent1 => SmolStr::new_inline("tvinputcomponent1"),
        NamedKey::TVInputComponent2 => SmolStr::new_inline("tvinputcomponent2"),
        NamedKey::TVInputComposite1 => SmolStr::new_inline("tvinputcomposite1"),
        NamedKey::TVInputComposite2 => SmolStr::new_inline("tvinputcomposite2"),
        NamedKey::TVInputHDMI1 => SmolStr::new_inline("tvinputhdmi1"),
        NamedKey::TVInputHDMI2 => SmolStr::new_inline("tvinputhdmi2"),
        NamedKey::TVInputHDMI3 => SmolStr::new_inline("tvinputhdmi3"),
        NamedKey::TVInputHDMI4 => SmolStr::new_inline("tvinputhdmi4"),
        NamedKey::TVInputVGA1 => SmolStr::new_inline("tvinputvga1"),
        NamedKey::TVMediaContext => SmolStr::new_inline("tvmediacontext"),
        NamedKey::TVNetwork => SmolStr::new_inline("tvnetwork"),
        NamedKey::TVNumberEntry => SmolStr::new_inline("tvnumberentry"),
        NamedKey::TVPower => SmolStr::new_inline("tvpower"),
        NamedKey::TVRadioService => SmolStr::new_inline("tvradioservice"),
        NamedKey::TVSatellite => SmolStr::new_inline("tvsatellite"),
        NamedKey::TVSatelliteBS => SmolStr::new_inline("tvsatellitebs"),
        NamedKey::TVSatelliteCS => SmolStr::new_inline("tvsatellitecs"),
        NamedKey::TVSatelliteToggle => SmolStr::new_inline("tvsatellitetoggle"),
        NamedKey::TVTerrestrialAnalog => SmolStr::new_inline("tvterrestrialanalog"),
        NamedKey::TVTerrestrialDigital => SmolStr::new_inline("tvterrestrialdigital"),
        NamedKey::TVTimer => SmolStr::new_inline("tvtimer"),
        NamedKey::AVRInput => SmolStr::new_inline("avrinput"),
        NamedKey::AVRPower => SmolStr::new_inline("avrpower"),
        NamedKey::ColorF0Red => SmolStr::new_inline("colorf0red"),
        NamedKey::ColorF1Green => SmolStr::new_inline("colorf1green"),
        NamedKey::ColorF2Yellow => SmolStr::new_inline("colorf2yellow"),
        NamedKey::ColorF3Blue => SmolStr::new_inline("colorf3blue"),
        NamedKey::ColorF4Grey => SmolStr::new_inline("colorf4grey"),
        NamedKey::ColorF5Brown => SmolStr::new_inline("colorf5brown"),
        NamedKey::ClosedCaptionToggle => SmolStr::new_inline("closedcaptiontoggle"),
        NamedKey::Dimmer => SmolStr::new_inline("dimmer"),
        NamedKey::DisplaySwap => SmolStr::new_inline("displayswap"),
        NamedKey::DVR => SmolStr::new_inline("dVR"),
        NamedKey::Exit => SmolStr::new_inline("exit"),
        NamedKey::FavoriteClear0 => SmolStr::new_inline("favclear0"),
        NamedKey::FavoriteClear1 => SmolStr::new_inline("favclear1"),
        NamedKey::FavoriteClear2 => SmolStr::new_inline("favclear2"),
        NamedKey::FavoriteClear3 => SmolStr::new_inline("favclear3"),
        NamedKey::FavoriteRecall0 => SmolStr::new_inline("favrecall0"),
        NamedKey::FavoriteRecall1 => SmolStr::new_inline("favrecall1"),
        NamedKey::FavoriteRecall2 => SmolStr::new_inline("favrecall2"),
        NamedKey::FavoriteRecall3 => SmolStr::new_inline("favrecall3"),
        NamedKey::FavoriteStore0 => SmolStr::new_inline("favstore0"),
        NamedKey::FavoriteStore1 => SmolStr::new_inline("favstore1"),
        NamedKey::FavoriteStore2 => SmolStr::new_inline("favstore2"),
        NamedKey::FavoriteStore3 => SmolStr::new_inline("favstore3"),
        NamedKey::Guide => SmolStr::new_inline("guide"),
        NamedKey::GuideNextDay => SmolStr::new_inline("guidenextday"),
        NamedKey::GuidePreviousDay => SmolStr::new_inline("guidepreviousday"),
        NamedKey::Info => SmolStr::new_inline("info"),
        NamedKey::InstantReplay => SmolStr::new_inline("instantreplay"),
        NamedKey::Link => SmolStr::new_inline("link"),
        NamedKey::ListProgram => SmolStr::new_inline("listprogram"),
        NamedKey::LiveContent => SmolStr::new_inline("livecontent"),
        NamedKey::Lock => SmolStr::new_inline("lock"),
        NamedKey::MediaApps => SmolStr::new_inline("mediaapps"),
        NamedKey::MediaAudioTrack => SmolStr::new_inline("mediaaudiotrack"),
        NamedKey::MediaLast => SmolStr::new_inline("medialast"),
        NamedKey::MediaSkipBackward => SmolStr::new_inline("mediaskipbackward"),
        NamedKey::MediaSkipForward => SmolStr::new_inline("mediaskipforward"),
        NamedKey::MediaStepBackward => SmolStr::new_inline("mediastepbackward"),
        NamedKey::MediaStepForward => SmolStr::new_inline("mediastepforward"),
        NamedKey::MediaTopMenu => SmolStr::new_inline("mediatopmenu"),
        NamedKey::NavigateIn => SmolStr::new_inline("navigatein"),
        NamedKey::NavigateNext => SmolStr::new_inline("navigatenext"),
        NamedKey::NavigateOut => SmolStr::new_inline("navigateout"),
        NamedKey::NavigatePrevious => SmolStr::new_inline("navigateprevious"),
        NamedKey::NextFavoriteChannel => SmolStr::new_inline("nextfavoritechannel"),
        NamedKey::NextUserProfile => SmolStr::new_inline("nextuserprofile"),
        NamedKey::OnDemand => SmolStr::new_inline("ondemand"),
        NamedKey::Pairing => SmolStr::new_inline("pairing"),
        NamedKey::PinPDown => SmolStr::new_inline("pinpdown"),
        NamedKey::PinPMove => SmolStr::new_inline("pinpmove"),
        NamedKey::PinPToggle => SmolStr::new_inline("pinptoggle"),
        NamedKey::PinPUp => SmolStr::new_inline("pinPUp"),
        NamedKey::PlaySpeedDown => SmolStr::new_inline("playspeeddown"),
        NamedKey::PlaySpeedReset => SmolStr::new_inline("playspeedreset"),
        NamedKey::PlaySpeedUp => SmolStr::new_inline("playspeedup"),
        NamedKey::RandomToggle => SmolStr::new_inline("randomtoggle"),
        NamedKey::RcLowBattery => SmolStr::new_inline("rclowbattery"),
        NamedKey::RecordSpeedNext => SmolStr::new_inline("recordspeednext"),
        NamedKey::RfBypass => SmolStr::new_inline("rfbypass"),
        NamedKey::ScanChannelsToggle => SmolStr::new_inline("scanchannelstoggle"),
        NamedKey::ScreenModeNext => SmolStr::new_inline("screenmodenext"),
        NamedKey::Settings => SmolStr::new_inline("settings"),
        NamedKey::SplitScreenToggle => SmolStr::new_inline("splitscreentoggle"),
        NamedKey::STBInput => SmolStr::new_inline("stbinput"),
        NamedKey::STBPower => SmolStr::new_inline("stbpower"),
        NamedKey::Subtitle => SmolStr::new_inline("subtitle"),
        NamedKey::Teletext => SmolStr::new_inline("teletext"),
        NamedKey::VideoModeNext => SmolStr::new_inline("videomodenext"),
        NamedKey::Wink => SmolStr::new_inline("wink"),
        NamedKey::ZoomToggle => SmolStr::new_inline("zoomtoggle"),
        NamedKey::F1 => SmolStr::new_inline("f1"),
        NamedKey::F2 => SmolStr::new_inline("f2"),
        NamedKey::F3 => SmolStr::new_inline("f3"),
        NamedKey::F4 => SmolStr::new_inline("f4"),
        NamedKey::F5 => SmolStr::new_inline("f5"),
        NamedKey::F6 => SmolStr::new_inline("f6"),
        NamedKey::F7 => SmolStr::new_inline("f7"),
        NamedKey::F8 => SmolStr::new_inline("f8"),
        NamedKey::F9 => SmolStr::new_inline("f9"),
        NamedKey::F10 => SmolStr::new_inline("f10"),
        NamedKey::F11 => SmolStr::new_inline("f11"),
        NamedKey::F12 => SmolStr::new_inline("f12"),
        NamedKey::F13 => SmolStr::new_inline("f13"),
        NamedKey::F14 => SmolStr::new_inline("f14"),
        NamedKey::F15 => SmolStr::new_inline("f15"),
        NamedKey::F16 => SmolStr::new_inline("f16"),
        NamedKey::F17 => SmolStr::new_inline("f17"),
        NamedKey::F18 => SmolStr::new_inline("f18"),
        NamedKey::F19 => SmolStr::new_inline("f19"),
        NamedKey::F20 => SmolStr::new_inline("f20"),
        NamedKey::F21 => SmolStr::new_inline("f21"),
        NamedKey::F22 => SmolStr::new_inline("f22"),
        NamedKey::F23 => SmolStr::new_inline("f23"),
        NamedKey::F24 => SmolStr::new_inline("f24"),
        NamedKey::F25 => SmolStr::new_inline("f25"),
        NamedKey::F26 => SmolStr::new_inline("f26"),
        NamedKey::F27 => SmolStr::new_inline("f27"),
        NamedKey::F28 => SmolStr::new_inline("f28"),
        NamedKey::F29 => SmolStr::new_inline("f29"),
        NamedKey::F30 => SmolStr::new_inline("f30"),
        NamedKey::F31 => SmolStr::new_inline("f31"),
        NamedKey::F32 => SmolStr::new_inline("f32"),
        NamedKey::F33 => SmolStr::new_inline("f33"),
        NamedKey::F34 => SmolStr::new_inline("f34"),
        NamedKey::F35 => SmolStr::new_inline("f35"),
        _ => unreachable!(),
    }
}
