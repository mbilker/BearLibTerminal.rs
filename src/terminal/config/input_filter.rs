use std::fmt;
use terminal::config::{ConfigPart, escape_config_string};


/// One input filter element.
///
/// `both`s are equivalent in effect to `+`s -- let through both keypresses and keyreleases.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum InputFilter {
	Event{name: InputFilterEvent, both: bool},
	Group{group: InputFilterGroup, both: bool},
	Alnum{keys: String, both: bool},
}

/// From [here](http://foo.wyrd.name/en:bearlibterminal:reference:input#inputfilter).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum InputFilterGroup {
	Arrow,
	Keypad,
	Keyboard,
	Mouse,
	/// Close + Resized
	System,
}

// As enumerated [here](foo.wyrd.name/en:bearlibterminal:reference:input#event_and_state_constants)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum InputFilterEvent {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Row0,
	Row1,
	Row2,
	Row3,
	Row4,
	Row5,
	Row6,
	Row7,
	Row8,
	Row9,
	Space,
	Minus,
	Equals,
	LBracket,
	RBracket,
	Backslash,
	Semicolon,
	Apostrophe,
	Grave,
	Comma,
	Period,
	Slash,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Return,
	Escape,
	Backspace,
	Tab,
	Pause,
	Insert,
	Home,
	Pageup,
	Delete,
	End,
	Pagedown,
	Right,
	Left,
	Down,
	Up,
	Shift,
	Control,
	Pad0,
	Pad1,
	Pad2,
	Pad3,
	Pad4,
	Pad5,
	Pad6,
	Pad7,
	Pad8,
	Pad9,
	PadDivide,
	PadMultiply,
	PadMinus,
	PadPlus,
	PadPeriod,
	PadEnter,
	MouseLeft,
	MouseRight,
	MouseMiddle,
	MouseX1,
	MouseX2,
	MouseMove,
	MouseScroll,
	MouseWheel,
	MouseX,
	MouseY,
	MousePixelX,
	MousePixelY,
	MouseClicks,
	Width,
	Height,
	CellWidth,
	CellHeight,
	Color,
	Bkcolor,
	Layer,
	Composition,
	Char,
	Wchar,
	Event,
	Fullscreen,
	Close,
	Resized,
}


impl ConfigPart for Vec<InputFilter> {
	fn to_config_str(&self) -> String {
		escape_config_string(&format!("[{}]", {
			let mut elems = "".to_string();
			for filter in self {
				elems = format!("{}{}", elems, match filter {
					&InputFilter::Event{ref name,  both} => format!("{}{}", name,  if both {"+"} else {""}),
					&InputFilter::Group{ref group, both} => format!("{}{}", group, if both {"+"} else {""}),
					&InputFilter::Alnum{ref keys,  both} => format!("{}{}", keys,  if both {"+"} else {""}),
				});
			}
			elems
		}))
	}
}


impl fmt::Display for InputFilterGroup {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(match self {
			&InputFilterGroup::Arrow    => "arrow",
			&InputFilterGroup::Keypad   => "keypad",
			&InputFilterGroup::Keyboard => "keyboard",
			&InputFilterGroup::Mouse    => "mouse",
			&InputFilterGroup::System   => "system",
		})
	}
}

impl fmt::Display for InputFilterEvent {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(match self {
			&InputFilterEvent::A           => "A",
			&InputFilterEvent::B           => "B",
			&InputFilterEvent::C           => "C",
			&InputFilterEvent::D           => "D",
			&InputFilterEvent::E           => "E",
			&InputFilterEvent::F           => "F",
			&InputFilterEvent::G           => "G",
			&InputFilterEvent::H           => "H",
			&InputFilterEvent::I           => "I",
			&InputFilterEvent::J           => "J",
			&InputFilterEvent::K           => "K",
			&InputFilterEvent::L           => "L",
			&InputFilterEvent::M           => "M",
			&InputFilterEvent::N           => "N",
			&InputFilterEvent::O           => "O",
			&InputFilterEvent::P           => "P",
			&InputFilterEvent::Q           => "Q",
			&InputFilterEvent::R           => "R",
			&InputFilterEvent::S           => "S",
			&InputFilterEvent::T           => "T",
			&InputFilterEvent::U           => "U",
			&InputFilterEvent::V           => "V",
			&InputFilterEvent::W           => "W",
			&InputFilterEvent::X           => "X",
			&InputFilterEvent::Y           => "Y",
			&InputFilterEvent::Z           => "Z",
			&InputFilterEvent::Row0        => "0",
			&InputFilterEvent::Row1        => "1",
			&InputFilterEvent::Row2        => "2",
			&InputFilterEvent::Row3        => "3",
			&InputFilterEvent::Row4        => "4",
			&InputFilterEvent::Row5        => "5",
			&InputFilterEvent::Row6        => "6",
			&InputFilterEvent::Row7        => "7",
			&InputFilterEvent::Row8        => "8",
			&InputFilterEvent::Row9        => "9",
			&InputFilterEvent::Space       => "space",
			&InputFilterEvent::Minus       => "minus",
			&InputFilterEvent::Equals      => "equals",
			&InputFilterEvent::LBracket    => "lbracket",
			&InputFilterEvent::RBracket    => "rbracket",
			&InputFilterEvent::Backslash   => "backslash",
			&InputFilterEvent::Semicolon   => "semicolon",
			&InputFilterEvent::Apostrophe  => "apostrophe",
			&InputFilterEvent::Grave       => "grave",
			&InputFilterEvent::Comma       => "comma",
			&InputFilterEvent::Period      => "period",
			&InputFilterEvent::Slash       => "slash",
			&InputFilterEvent::F1          => "F1",
			&InputFilterEvent::F2          => "F2",
			&InputFilterEvent::F3          => "F3",
			&InputFilterEvent::F4          => "F4",
			&InputFilterEvent::F5          => "F5",
			&InputFilterEvent::F6          => "F6",
			&InputFilterEvent::F7          => "F7",
			&InputFilterEvent::F8          => "F8",
			&InputFilterEvent::F9          => "F9",
			&InputFilterEvent::F10         => "F10",
			&InputFilterEvent::F11         => "F11",
			&InputFilterEvent::F12         => "F12",
			&InputFilterEvent::Return      => "return",
			&InputFilterEvent::Escape      => "escape",
			&InputFilterEvent::Backspace   => "backspace",
			&InputFilterEvent::Tab         => "tab",
			&InputFilterEvent::Pause       => "pause",
			&InputFilterEvent::Insert      => "insert",
			&InputFilterEvent::Home        => "home",
			&InputFilterEvent::Pageup      => "pageup",
			&InputFilterEvent::Delete      => "delete",
			&InputFilterEvent::End         => "end",
			&InputFilterEvent::Pagedown    => "pagedown",
			&InputFilterEvent::Right       => "right",
			&InputFilterEvent::Left        => "left",
			&InputFilterEvent::Down        => "down",
			&InputFilterEvent::Up          => "up",
			&InputFilterEvent::Shift       => "shift",
			&InputFilterEvent::Control     => "control",
			&InputFilterEvent::Pad0        => "KP_0",
			&InputFilterEvent::Pad1        => "KP_1",
			&InputFilterEvent::Pad2        => "KP_2",
			&InputFilterEvent::Pad3        => "KP_3",
			&InputFilterEvent::Pad4        => "KP_4",
			&InputFilterEvent::Pad5        => "KP_5",
			&InputFilterEvent::Pad6        => "KP_6",
			&InputFilterEvent::Pad7        => "KP_7",
			&InputFilterEvent::Pad8        => "KP_8",
			&InputFilterEvent::Pad9        => "KP_9",
			&InputFilterEvent::PadDivide   => "KP_divide",
			&InputFilterEvent::PadMultiply => "KP_multiply",
			&InputFilterEvent::PadMinus    => "KP_minus",
			&InputFilterEvent::PadPlus     => "KP_plus",
			&InputFilterEvent::PadPeriod   => "KP_period",
			&InputFilterEvent::PadEnter    => "KP_enter",
			&InputFilterEvent::MouseLeft   => "mouse_left",
			&InputFilterEvent::MouseRight  => "mouse_right",
			&InputFilterEvent::MouseMiddle => "mouse_middle",
			&InputFilterEvent::MouseX1     => "mouse_x1",
			&InputFilterEvent::MouseX2     => "mouse_x2",
			&InputFilterEvent::MouseMove   => "mouse_move",
			&InputFilterEvent::MouseScroll => "mouse_scroll",
			&InputFilterEvent::MouseWheel  => "mouse_wheel",
			&InputFilterEvent::MouseX      => "mouse_x",
			&InputFilterEvent::MouseY      => "mouse_y",
			&InputFilterEvent::MousePixelX => "mouse_pixelx",
			&InputFilterEvent::MousePixelY => "mouse_pixely",
			&InputFilterEvent::MouseClicks => "mouse_clicks",
			&InputFilterEvent::Width       => "width",
			&InputFilterEvent::Height      => "height",
			&InputFilterEvent::CellWidth   => "cell_width",
			&InputFilterEvent::CellHeight  => "cell_height",
			&InputFilterEvent::Color       => "color",
			&InputFilterEvent::Bkcolor     => "bkcolor",
			&InputFilterEvent::Layer       => "layer",
			&InputFilterEvent::Composition => "composition",
			&InputFilterEvent::Char        => "char",
			&InputFilterEvent::Wchar       => "wchar",
			&InputFilterEvent::Event       => "event",
			&InputFilterEvent::Fullscreen  => "fullscreen",
			&InputFilterEvent::Close       => "close",
			&InputFilterEvent::Resized     => "resized",
		})
	}
}
