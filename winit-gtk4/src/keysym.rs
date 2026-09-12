//! Keysym → `Key`/`KeyLocation` lookup tables.
//!
//! Vendored from `winit_common::xkb::keymap`, which does not export them.

use winit_core::keyboard::{Key, KeyLocation, NamedKey, NativeKey};

pub fn keysym_to_key(keysym: u32) -> Key {
    use xkbcommon_dl::keysyms;
    Key::Named(match keysym {
        // TTY function keys
        keysyms::BackSpace => NamedKey::Backspace,
        keysyms::Tab => NamedKey::Tab,
        // keysyms::Linefeed => NamedKey::Linefeed,
        keysyms::Clear => NamedKey::Clear,
        keysyms::Return => NamedKey::Enter,
        keysyms::Pause => NamedKey::Pause,
        keysyms::Scroll_Lock => NamedKey::ScrollLock,
        keysyms::Sys_Req => NamedKey::PrintScreen,
        keysyms::Escape => NamedKey::Escape,
        keysyms::Delete => NamedKey::Delete,

        // IME keys
        keysyms::Multi_key => NamedKey::Compose,
        keysyms::Codeinput => NamedKey::CodeInput,
        keysyms::SingleCandidate => NamedKey::SingleCandidate,
        keysyms::MultipleCandidate => NamedKey::AllCandidates,
        keysyms::PreviousCandidate => NamedKey::PreviousCandidate,

        // Japanese keys
        keysyms::Kanji => NamedKey::KanjiMode,
        keysyms::Muhenkan => NamedKey::NonConvert,
        keysyms::Henkan_Mode => NamedKey::Convert,
        keysyms::Romaji => NamedKey::Romaji,
        keysyms::Hiragana => NamedKey::Hiragana,
        keysyms::Hiragana_Katakana => NamedKey::HiraganaKatakana,
        keysyms::Zenkaku => NamedKey::Zenkaku,
        keysyms::Hankaku => NamedKey::Hankaku,
        keysyms::Zenkaku_Hankaku => NamedKey::ZenkakuHankaku,
        // keysyms::Touroku => NamedKey::Touroku,
        // keysyms::Massyo => NamedKey::Massyo,
        keysyms::Kana_Lock => NamedKey::KanaMode,
        keysyms::Kana_Shift => NamedKey::KanaMode,
        keysyms::Eisu_Shift => NamedKey::Alphanumeric,
        keysyms::Eisu_toggle => NamedKey::Alphanumeric,
        // NOTE: The next three items are aliases for values we've already mapped.
        // keysyms::Kanji_Bangou => NamedKey::CodeInput,
        // keysyms::Zen_Koho => NamedKey::AllCandidates,
        // keysyms::Mae_Koho => NamedKey::PreviousCandidate,

        // Cursor control & motion
        keysyms::Home => NamedKey::Home,
        keysyms::Left => NamedKey::ArrowLeft,
        keysyms::Up => NamedKey::ArrowUp,
        keysyms::Right => NamedKey::ArrowRight,
        keysyms::Down => NamedKey::ArrowDown,
        // keysyms::Prior => NamedKey::PageUp,
        keysyms::Page_Up => NamedKey::PageUp,
        // keysyms::Next => NamedKey::PageDown,
        keysyms::Page_Down => NamedKey::PageDown,
        keysyms::End => NamedKey::End,
        // keysyms::Begin => NamedKey::Begin,

        // Misc. functions
        keysyms::Select => NamedKey::Select,
        keysyms::Print => NamedKey::PrintScreen,
        keysyms::Execute => NamedKey::Execute,
        keysyms::Insert => NamedKey::Insert,
        keysyms::Undo => NamedKey::Undo,
        keysyms::Redo => NamedKey::Redo,
        keysyms::Menu => NamedKey::ContextMenu,
        keysyms::Find => NamedKey::Find,
        keysyms::Cancel => NamedKey::Cancel,
        keysyms::Help => NamedKey::Help,
        keysyms::Break => NamedKey::Pause,
        keysyms::Mode_switch => NamedKey::ModeChange,
        // keysyms::script_switch => NamedKey::ModeChange,
        keysyms::Num_Lock => NamedKey::NumLock,

        // Keypad keys
        // keysyms::KP_Space => return Key::Character(" "),
        keysyms::KP_Tab => NamedKey::Tab,
        keysyms::KP_Enter => NamedKey::Enter,
        keysyms::KP_F1 => NamedKey::F1,
        keysyms::KP_F2 => NamedKey::F2,
        keysyms::KP_F3 => NamedKey::F3,
        keysyms::KP_F4 => NamedKey::F4,
        keysyms::KP_Home => NamedKey::Home,
        keysyms::KP_Left => NamedKey::ArrowLeft,
        keysyms::KP_Up => NamedKey::ArrowUp,
        keysyms::KP_Right => NamedKey::ArrowRight,
        keysyms::KP_Down => NamedKey::ArrowDown,
        // keysyms::KP_Prior => NamedKey::PageUp,
        keysyms::KP_Page_Up => NamedKey::PageUp,
        // keysyms::KP_Next => NamedKey::PageDown,
        keysyms::KP_Page_Down => NamedKey::PageDown,
        keysyms::KP_End => NamedKey::End,
        // This is the key labeled "5" on the numpad when NumLock is off.
        // keysyms::KP_Begin => NamedKey::Begin,
        keysyms::KP_Insert => NamedKey::Insert,
        keysyms::KP_Delete => NamedKey::Delete,
        // keysyms::KP_Equal => NamedKey::Equal,
        // keysyms::KP_Multiply => NamedKey::Multiply,
        // keysyms::KP_Add => NamedKey::Add,
        // keysyms::KP_Separator => NamedKey::Separator,
        // keysyms::KP_Subtract => NamedKey::Subtract,
        // keysyms::KP_Decimal => NamedKey::Decimal,
        // keysyms::KP_Divide => NamedKey::Divide,

        // keysyms::KP_0 => return Key::Character("0"),
        // keysyms::KP_1 => return Key::Character("1"),
        // keysyms::KP_2 => return Key::Character("2"),
        // keysyms::KP_3 => return Key::Character("3"),
        // keysyms::KP_4 => return Key::Character("4"),
        // keysyms::KP_5 => return Key::Character("5"),
        // keysyms::KP_6 => return Key::Character("6"),
        // keysyms::KP_7 => return Key::Character("7"),
        // keysyms::KP_8 => return Key::Character("8"),
        // keysyms::KP_9 => return Key::Character("9"),

        // Function keys
        keysyms::F1 => NamedKey::F1,
        keysyms::F2 => NamedKey::F2,
        keysyms::F3 => NamedKey::F3,
        keysyms::F4 => NamedKey::F4,
        keysyms::F5 => NamedKey::F5,
        keysyms::F6 => NamedKey::F6,
        keysyms::F7 => NamedKey::F7,
        keysyms::F8 => NamedKey::F8,
        keysyms::F9 => NamedKey::F9,
        keysyms::F10 => NamedKey::F10,
        keysyms::F11 => NamedKey::F11,
        keysyms::F12 => NamedKey::F12,
        keysyms::F13 => NamedKey::F13,
        keysyms::F14 => NamedKey::F14,
        keysyms::F15 => NamedKey::F15,
        keysyms::F16 => NamedKey::F16,
        keysyms::F17 => NamedKey::F17,
        keysyms::F18 => NamedKey::F18,
        keysyms::F19 => NamedKey::F19,
        keysyms::F20 => NamedKey::F20,
        keysyms::F21 => NamedKey::F21,
        keysyms::F22 => NamedKey::F22,
        keysyms::F23 => NamedKey::F23,
        keysyms::F24 => NamedKey::F24,
        keysyms::F25 => NamedKey::F25,
        keysyms::F26 => NamedKey::F26,
        keysyms::F27 => NamedKey::F27,
        keysyms::F28 => NamedKey::F28,
        keysyms::F29 => NamedKey::F29,
        keysyms::F30 => NamedKey::F30,
        keysyms::F31 => NamedKey::F31,
        keysyms::F32 => NamedKey::F32,
        keysyms::F33 => NamedKey::F33,
        keysyms::F34 => NamedKey::F34,
        keysyms::F35 => NamedKey::F35,

        // Modifiers
        keysyms::Shift_L => NamedKey::Shift,
        keysyms::Shift_R => NamedKey::Shift,
        keysyms::Control_L => NamedKey::Control,
        keysyms::Control_R => NamedKey::Control,
        keysyms::Caps_Lock => NamedKey::CapsLock,
        // keysyms::Shift_Lock => NamedKey::ShiftLock,
        keysyms::Alt_L => NamedKey::Alt,
        keysyms::Alt_R => NamedKey::Alt,
        #[allow(deprecated)]
        keysyms::Hyper_L => NamedKey::Hyper,
        #[allow(deprecated)]
        keysyms::Hyper_R => NamedKey::Hyper,

        // Browsers map X11's Super keys to Meta, so we do that as well.
        keysyms::Super_L => NamedKey::Meta,
        keysyms::Super_R => NamedKey::Meta,
        // The actual Meta keys do not seem to be used by browsers, so we don't do that either.
        // keysyms::Meta_L => NamedKey::Super,
        // keysyms::Meta_R => NamedKey::Super,

        // XKB function and modifier keys
        // keysyms::ISO_Lock => NamedKey::IsoLock,
        // keysyms::ISO_Level2_Latch => NamedKey::IsoLevel2Latch,
        keysyms::ISO_Level3_Shift => NamedKey::AltGraph,
        keysyms::ISO_Level3_Latch => NamedKey::AltGraph,
        keysyms::ISO_Level3_Lock => NamedKey::AltGraph,
        // keysyms::ISO_Level5_Shift => NamedKey::IsoLevel5Shift,
        // keysyms::ISO_Level5_Latch => NamedKey::IsoLevel5Latch,
        // keysyms::ISO_Level5_Lock => NamedKey::IsoLevel5Lock,
        // keysyms::ISO_Group_Shift => NamedKey::IsoGroupShift,
        // keysyms::ISO_Group_Latch => NamedKey::IsoGroupLatch,
        // keysyms::ISO_Group_Lock => NamedKey::IsoGroupLock,
        keysyms::ISO_Next_Group => NamedKey::GroupNext,
        // keysyms::ISO_Next_Group_Lock => NamedKey::GroupNextLock,
        keysyms::ISO_Prev_Group => NamedKey::GroupPrevious,
        // keysyms::ISO_Prev_Group_Lock => NamedKey::GroupPreviousLock,
        keysyms::ISO_First_Group => NamedKey::GroupFirst,
        // keysyms::ISO_First_Group_Lock => NamedKey::GroupFirstLock,
        keysyms::ISO_Last_Group => NamedKey::GroupLast,
        // keysyms::ISO_Last_Group_Lock => NamedKey::GroupLastLock,
        keysyms::ISO_Left_Tab => NamedKey::Tab,
        // keysyms::ISO_Move_Line_Up => NamedKey::IsoMoveLineUp,
        // keysyms::ISO_Move_Line_Down => NamedKey::IsoMoveLineDown,
        // keysyms::ISO_Partial_Line_Up => NamedKey::IsoPartialLineUp,
        // keysyms::ISO_Partial_Line_Down => NamedKey::IsoPartialLineDown,
        // keysyms::ISO_Partial_Space_Left => NamedKey::IsoPartialSpaceLeft,
        // keysyms::ISO_Partial_Space_Right => NamedKey::IsoPartialSpaceRight,
        // keysyms::ISO_Set_Margin_Left => NamedKey::IsoSetMarginLeft,
        // keysyms::ISO_Set_Margin_Right => NamedKey::IsoSetMarginRight,
        // keysyms::ISO_Release_Margin_Left => NamedKey::IsoReleaseMarginLeft,
        // keysyms::ISO_Release_Margin_Right => NamedKey::IsoReleaseMarginRight,
        // keysyms::ISO_Release_Both_Margins => NamedKey::IsoReleaseBothMargins,
        // keysyms::ISO_Fast_Cursor_Left => NamedKey::IsoFastPointerLeft,
        // keysyms::ISO_Fast_Cursor_Right => NamedKey::IsoFastCursorRight,
        // keysyms::ISO_Fast_Cursor_Up => NamedKey::IsoFastCursorUp,
        // keysyms::ISO_Fast_Cursor_Down => NamedKey::IsoFastCursorDown,
        // keysyms::ISO_Continuous_Underline => NamedKey::IsoContinuousUnderline,
        // keysyms::ISO_Discontinuous_Underline => NamedKey::IsoDiscontinuousUnderline,
        // keysyms::ISO_Emphasize => NamedKey::IsoEmphasize,
        // keysyms::ISO_Center_Object => NamedKey::IsoCenterObject,
        keysyms::ISO_Enter => NamedKey::Enter,

        // dead_grave..dead_currency

        // dead_lowline..dead_longsolidusoverlay

        // dead_a..dead_capital_schwa

        // dead_greek

        // First_Virtual_Screen..Terminate_Server

        // AccessX_Enable..AudibleBell_Enable

        // Pointer_Left..Pointer_Drag5

        // Pointer_EnableKeys..Pointer_DfltBtnPrev

        // ch..C_H

        // 3270 terminal keys
        // keysyms::3270_Duplicate => NamedKey::Duplicate,
        // keysyms::3270_FieldMark => NamedKey::FieldMark,
        // keysyms::3270_Right2 => NamedKey::Right2,
        // keysyms::3270_Left2 => NamedKey::Left2,
        // keysyms::3270_BackTab => NamedKey::BackTab,
        keysyms::_3270_EraseEOF => NamedKey::EraseEof,
        // keysyms::3270_EraseInput => NamedKey::EraseInput,
        // keysyms::3270_Reset => NamedKey::Reset,
        // keysyms::3270_Quit => NamedKey::Quit,
        // keysyms::3270_PA1 => NamedKey::Pa1,
        // keysyms::3270_PA2 => NamedKey::Pa2,
        // keysyms::3270_PA3 => NamedKey::Pa3,
        // keysyms::3270_Test => NamedKey::Test,
        keysyms::_3270_Attn => NamedKey::Attn,
        // keysyms::3270_CursorBlink => NamedKey::CursorBlink,
        // keysyms::3270_AltCursor => NamedKey::AltCursor,
        // keysyms::3270_KeyClick => NamedKey::KeyClick,
        // keysyms::3270_Jump => NamedKey::Jump,
        // keysyms::3270_Ident => NamedKey::Ident,
        // keysyms::3270_Rule => NamedKey::Rule,
        // keysyms::3270_Copy => NamedKey::Copy,
        keysyms::_3270_Play => NamedKey::Play,
        // keysyms::3270_Setup => NamedKey::Setup,
        // keysyms::3270_Record => NamedKey::Record,
        // keysyms::3270_ChangeScreen => NamedKey::ChangeScreen,
        // keysyms::3270_DeleteWord => NamedKey::DeleteWord,
        keysyms::_3270_ExSelect => NamedKey::ExSel,
        keysyms::_3270_CursorSelect => NamedKey::CrSel,
        keysyms::_3270_PrintScreen => NamedKey::PrintScreen,
        keysyms::_3270_Enter => NamedKey::Enter,

        keysyms::space => return Key::Character(" ".into()),
        // exclam..Sinh_kunddaliya

        // XFree86
        // keysyms::XF86_ModeLock => NamedKey::ModeLock,

        // XFree86 - Backlight controls
        keysyms::XF86_MonBrightnessUp => NamedKey::BrightnessUp,
        keysyms::XF86_MonBrightnessDown => NamedKey::BrightnessDown,
        // keysyms::XF86_KbdLightOnOff => NamedKey::LightOnOff,
        // keysyms::XF86_KbdBrightnessUp => NamedKey::KeyboardBrightnessUp,
        // keysyms::XF86_KbdBrightnessDown => NamedKey::KeyboardBrightnessDown,

        // XFree86 - "Internet"
        keysyms::XF86_Standby => NamedKey::Standby,
        keysyms::XF86_AudioLowerVolume => NamedKey::AudioVolumeDown,
        keysyms::XF86_AudioRaiseVolume => NamedKey::AudioVolumeUp,
        keysyms::XF86_AudioPlay => NamedKey::MediaPlay,
        keysyms::XF86_AudioStop => NamedKey::MediaStop,
        keysyms::XF86_AudioPrev => NamedKey::MediaTrackPrevious,
        keysyms::XF86_AudioNext => NamedKey::MediaTrackNext,
        keysyms::XF86_HomePage => NamedKey::BrowserHome,
        keysyms::XF86_Mail => NamedKey::LaunchMail,
        // keysyms::XF86_Start => NamedKey::Start,
        keysyms::XF86_Search => NamedKey::BrowserSearch,
        keysyms::XF86_AudioRecord => NamedKey::MediaRecord,

        // XFree86 - PDA
        keysyms::XF86_Calculator => NamedKey::LaunchApplication2,
        // keysyms::XF86_Memo => NamedKey::Memo,
        // keysyms::XF86_ToDoList => NamedKey::ToDoList,
        keysyms::XF86_Calendar => NamedKey::LaunchCalendar,
        keysyms::XF86_PowerDown => NamedKey::Power,
        // keysyms::XF86_ContrastAdjust => NamedKey::AdjustContrast,
        // keysyms::XF86_RockerUp => NamedKey::RockerUp,
        // keysyms::XF86_RockerDown => NamedKey::RockerDown,
        // keysyms::XF86_RockerEnter => NamedKey::RockerEnter,

        // XFree86 - More "Internet"
        keysyms::XF86_Back => NamedKey::BrowserBack,
        keysyms::XF86_Forward => NamedKey::BrowserForward,
        // keysyms::XF86_Stop => NamedKey::Stop,
        keysyms::XF86_Refresh => NamedKey::BrowserRefresh,
        keysyms::XF86_PowerOff => NamedKey::Power,
        keysyms::XF86_WakeUp => NamedKey::WakeUp,
        keysyms::XF86_Eject => NamedKey::Eject,
        keysyms::XF86_ScreenSaver => NamedKey::LaunchScreenSaver,
        keysyms::XF86_WWW => NamedKey::LaunchWebBrowser,
        keysyms::XF86_Sleep => NamedKey::Standby,
        keysyms::XF86_Favorites => NamedKey::BrowserFavorites,
        keysyms::XF86_AudioPause => NamedKey::MediaPause,
        // keysyms::XF86_AudioMedia => NamedKey::AudioMedia,
        keysyms::XF86_MyComputer => NamedKey::LaunchApplication1,
        // keysyms::XF86_VendorHome => NamedKey::VendorHome,
        // keysyms::XF86_LightBulb => NamedKey::LightBulb,
        // keysyms::XF86_Shop => NamedKey::BrowserShop,
        // keysyms::XF86_History => NamedKey::BrowserHistory,
        // keysyms::XF86_OpenURL => NamedKey::OpenUrl,
        // keysyms::XF86_AddFavorite => NamedKey::AddFavorite,
        // keysyms::XF86_HotLinks => NamedKey::HotLinks,
        // keysyms::XF86_BrightnessAdjust => NamedKey::BrightnessAdjust,
        // keysyms::XF86_Finance => NamedKey::BrowserFinance,
        // keysyms::XF86_Community => NamedKey::BrowserCommunity,
        keysyms::XF86_AudioRewind => NamedKey::MediaRewind,
        // keysyms::XF86_BackForward => Key::???,
        // XF86_Launch0..XF86_LaunchF

        // XF86_ApplicationLeft..XF86_CD
        keysyms::XF86_Calculater => NamedKey::LaunchApplication2, // Nice typo, libxkbcommon :)
        // XF86_Clear
        keysyms::XF86_Close => NamedKey::Close,
        keysyms::XF86_Copy => NamedKey::Copy,
        keysyms::XF86_Cut => NamedKey::Cut,
        // XF86_Display..XF86_Documents
        keysyms::XF86_Excel => NamedKey::LaunchSpreadsheet,
        // XF86_Explorer..XF86iTouch
        keysyms::XF86_LogOff => NamedKey::LogOff,
        // XF86_Market..XF86_MenuPB
        keysyms::XF86_MySites => NamedKey::BrowserFavorites,
        keysyms::XF86_New => NamedKey::New,
        // XF86_News..XF86_OfficeHome
        keysyms::XF86_Open => NamedKey::Open,
        // XF86_Option
        keysyms::XF86_Paste => NamedKey::Paste,
        keysyms::XF86_Phone => NamedKey::LaunchPhone,
        // XF86_Q
        keysyms::XF86_Reply => NamedKey::MailReply,
        keysyms::XF86_Reload => NamedKey::BrowserRefresh,
        // XF86_RotateWindows..XF86_RotationKB
        keysyms::XF86_Save => NamedKey::Save,
        // XF86_ScrollUp..XF86_ScrollClick
        keysyms::XF86_Send => NamedKey::MailSend,
        keysyms::XF86_Spell => NamedKey::SpellCheck,
        keysyms::XF86_SplitScreen => NamedKey::SplitScreenToggle,
        // XF86_Support..XF86_User2KB
        keysyms::XF86_Video => NamedKey::LaunchMediaPlayer,
        // XF86_WheelButton
        keysyms::XF86_Word => NamedKey::LaunchWordProcessor,
        // XF86_Xfer
        keysyms::XF86_ZoomIn => NamedKey::ZoomIn,
        keysyms::XF86_ZoomOut => NamedKey::ZoomOut,

        // XF86_Away..XF86_Messenger
        keysyms::XF86_WebCam => NamedKey::LaunchWebCam,
        keysyms::XF86_MailForward => NamedKey::MailForward,
        // XF86_Pictures
        keysyms::XF86_Music => NamedKey::LaunchMusicPlayer,

        // XF86_Battery..XF86_UWB
        keysyms::XF86_AudioForward => NamedKey::MediaFastForward,
        // XF86_AudioRepeat
        keysyms::XF86_AudioRandomPlay => NamedKey::RandomToggle,
        keysyms::XF86_Subtitle => NamedKey::Subtitle,
        keysyms::XF86_AudioCycleTrack => NamedKey::MediaAudioTrack,
        // XF86_CycleAngle..XF86_Blue
        keysyms::XF86_Suspend => NamedKey::Standby,
        keysyms::XF86_Hibernate => NamedKey::Hibernate,
        // XF86_TouchpadToggle..XF86_TouchpadOff
        keysyms::XF86_AudioMute => NamedKey::AudioVolumeMute,

        // XF86_Switch_VT_1..XF86_Switch_VT_12

        // XF86_Ungrab..XF86_ClearGrab
        keysyms::XF86_Next_VMode => NamedKey::VideoModeNext,
        // keysyms::XF86_Prev_VMode => NamedKey::VideoModePrevious,
        // XF86_LogWindowTree..XF86_LogGrabInfo

        // SunFA_Grave..SunFA_Cedilla

        // keysyms::SunF36 => NamedKey::F36 | NamedKey::F11,
        // keysyms::SunF37 => NamedKey::F37 | NamedKey::F12,

        // keysyms::SunSys_Req => NamedKey::PrintScreen,
        // The next couple of xkb (until SunStop) are already handled.
        // SunPrint_Screen..SunPageDown

        // SunUndo..SunFront
        keysyms::SUN_Copy => NamedKey::Copy,
        keysyms::SUN_Open => NamedKey::Open,
        keysyms::SUN_Paste => NamedKey::Paste,
        keysyms::SUN_Cut => NamedKey::Cut,

        // SunPowerSwitch
        keysyms::SUN_AudioLowerVolume => NamedKey::AudioVolumeDown,
        keysyms::SUN_AudioMute => NamedKey::AudioVolumeMute,
        keysyms::SUN_AudioRaiseVolume => NamedKey::AudioVolumeUp,
        // SUN_VideoDegauss
        keysyms::SUN_VideoLowerBrightness => NamedKey::BrightnessDown,
        keysyms::SUN_VideoRaiseBrightness => NamedKey::BrightnessUp,
        // SunPowerSwitchShift
        0 => return Key::Unidentified(NativeKey::Unidentified),
        _ => return Key::Unidentified(NativeKey::Xkb(keysym)),
    })
}

pub fn keysym_location(keysym: u32) -> KeyLocation {
    use xkbcommon_dl::keysyms;
    match keysym {
        keysyms::Shift_L
        | keysyms::Control_L
        | keysyms::Meta_L
        | keysyms::Alt_L
        | keysyms::Super_L
        | keysyms::Hyper_L => KeyLocation::Left,
        keysyms::Shift_R
        | keysyms::Control_R
        | keysyms::Meta_R
        | keysyms::Alt_R
        | keysyms::Super_R
        | keysyms::Hyper_R => KeyLocation::Right,
        keysyms::KP_0
        | keysyms::KP_1
        | keysyms::KP_2
        | keysyms::KP_3
        | keysyms::KP_4
        | keysyms::KP_5
        | keysyms::KP_6
        | keysyms::KP_7
        | keysyms::KP_8
        | keysyms::KP_9
        | keysyms::KP_Space
        | keysyms::KP_Tab
        | keysyms::KP_Enter
        | keysyms::KP_F1
        | keysyms::KP_F2
        | keysyms::KP_F3
        | keysyms::KP_F4
        | keysyms::KP_Home
        | keysyms::KP_Left
        | keysyms::KP_Up
        | keysyms::KP_Right
        | keysyms::KP_Down
        | keysyms::KP_Page_Up
        | keysyms::KP_Page_Down
        | keysyms::KP_End
        | keysyms::KP_Begin
        | keysyms::KP_Insert
        | keysyms::KP_Delete
        | keysyms::KP_Equal
        | keysyms::KP_Multiply
        | keysyms::KP_Add
        | keysyms::KP_Separator
        | keysyms::KP_Subtract
        | keysyms::KP_Decimal
        | keysyms::KP_Divide => KeyLocation::Numpad,
        _ => KeyLocation::Standard,
    }
}
