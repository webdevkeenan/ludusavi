use iced::{alignment::Horizontal as HorizontalAlignment, Font};

use crate::gui::widget::Text;

pub const ICONS: Font = Font::External {
    name: "Material Icons",
    bytes: include_bytes!("../../assets/MaterialIcons-Regular.ttf"),
};

pub enum Icon {
    Add,
    AddCircle,
    Edit,
    FolderOpen,
    KeyboardArrowRight,
    KeyboardArrowDown,
    Language,
    OpenInNew,
    Remove,
    RemoveCircle,
    SubdirectoryArrowRight,
    Delete,
    PlayCircleOutline,
    Settings,
    MoreVert,
    Refresh,
    FastForward,
    ArrowUpward,
    ArrowDownward,
    ArrowForward,
    ArrowBack,
    Comment,
    Close,
    Filter,
    Error,
    Upload,
    Download,
    Lock,
    LockOpen,
    Search,
}

impl Icon {
    pub fn as_char(&self) -> char {
        match self {
            Self::Add => '\u{E145}',
            Self::AddCircle => '\u{E147}',
            Self::Edit => '\u{E150}',
            Self::FolderOpen => '\u{E2C8}',
            Self::KeyboardArrowRight => '\u{E315}',
            Self::KeyboardArrowDown => '\u{E313}',
            Self::Language => '\u{E894}',
            Self::OpenInNew => '\u{E89E}',
            Self::Remove => '\u{E15B}',
            Self::RemoveCircle => '\u{E15C}',
            Self::SubdirectoryArrowRight => '\u{E5DA}',
            Self::Delete => '\u{E872}',
            Self::PlayCircleOutline => '\u{E039}',
            Self::Settings => '\u{E8B8}',
            Self::MoreVert => '\u{E5D4}',
            Self::Refresh => '\u{E5D5}',
            Self::FastForward => '\u{E01F}',
            Self::ArrowUpward => '\u{E5D8}',
            Self::ArrowDownward => '\u{E5DB}',
            Self::ArrowForward => '\u{e5c8}',
            Self::ArrowBack => '\u{e5c4}',
            Self::Comment => '\u{E0B9}',
            Self::Close => '\u{E5CD}',
            Self::Filter => '\u{ef4f}',
            Self::Error => '\u{e000}',
            Self::Upload => '\u{f09b}',
            Self::Download => '\u{f090}',
            Self::Lock => '\u{e897}',
            Self::LockOpen => '\u{e898}',
            Self::Search => '\u{e8b6}',
        }
    }

    pub fn as_text(&self) -> Text {
        Text::new(self.as_char().to_string())
            .font(ICONS)
            .width(60)
            .horizontal_alignment(HorizontalAlignment::Center)
    }

    pub fn into_text(self) -> Text<'static> {
        Text::new(self.as_char().to_string())
            .font(ICONS)
            .width(60)
            .horizontal_alignment(HorizontalAlignment::Center)
    }
}
