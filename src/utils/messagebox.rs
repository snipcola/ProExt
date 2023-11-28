use windows::{Win32::{UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_STYLE, MESSAGEBOX_RESULT, MB_ICONINFORMATION, MB_ICONWARNING, MB_ICONERROR, MB_OKCANCEL, MB_YESNO}, Foundation::HWND}, core::HSTRING};

pub enum MessageBoxStyle {
    Info,
    Warning,
    Error
}

pub enum MessageBoxButtons {
    OkCancel,
    YesNo
}

pub enum MessageBoxResult {
    Ok,
    Cancel,
    Yes,
    No,
    None
}

pub fn convert_mbstyle(style: MessageBoxStyle) -> MESSAGEBOX_STYLE {
    return match style {
        MessageBoxStyle::Info => MB_ICONINFORMATION,
        MessageBoxStyle::Warning => MB_ICONWARNING,
        MessageBoxStyle::Error => MB_ICONERROR
    };
}

pub fn convert_mbbuttons(button: MessageBoxButtons) -> MESSAGEBOX_STYLE {
    return match button {
        MessageBoxButtons::OkCancel => MB_OKCANCEL,
        MessageBoxButtons::YesNo => MB_YESNO
    };
}

pub fn convert_mbresult(result: MESSAGEBOX_RESULT) -> MessageBoxResult {
    return match result {
        MESSAGEBOX_RESULT(1) => MessageBoxResult::Ok,
        MESSAGEBOX_RESULT(2) => MessageBoxResult::Cancel,
        MESSAGEBOX_RESULT(6) => MessageBoxResult::Yes,
        MESSAGEBOX_RESULT(7) => MessageBoxResult::No,
        _ => MessageBoxResult::None
    };
}

pub fn create_messagebox(style: MessageBoxStyle, caption: &str, text: &str) {
    let text = HSTRING::from(text);
    let caption = HSTRING::from(caption);
    let style = convert_mbstyle(style);

    unsafe { MessageBoxW(HWND::default(), &text, &caption, style); }
}

pub fn create_dialog(style: MessageBoxStyle, buttons: MessageBoxButtons, caption: &str, text: &str) -> MessageBoxResult {
    let text = HSTRING::from(text);
    let caption = HSTRING::from(caption);
    let style = convert_mbstyle(style);
    let buttons = convert_mbbuttons(buttons);

    return convert_mbresult( unsafe { MessageBoxW(HWND::default(), &text, &caption, style | buttons) });
}