use crate::{
	state::StrataComp,
	workspaces::FocusTarget,
};
use bitflags::bitflags;
use smithay::{
	backend::input::{
		AbsolutePositionEvent,
		Axis,
		AxisSource,
		Event,
		InputBackend,
		PointerAxisEvent,
		PointerButtonEvent,
		PointerMotionEvent,
	},
	input::{
		keyboard::{
			Keysym,
			ModifiersState,
		},
		pointer::{
			AxisFrame,
			ButtonEvent,
			MotionEvent,
			RelativeMotionEvent,
		},
	},
	utils::{
		Logical,
		Point,
		SERIAL_COUNTER,
	},
};

#[derive(Debug)]
pub struct Mods {
	pub flags: ModFlags,
	pub state: ModifiersState,
}

// complete list, for future reference
//
// Shift_L Shift_R
// Control_L Control_R
// Meta_L Meta_R
// Alt_L Alt_R
// Super_L Super_R
// Hyper_L Hyper_R
// ISO_Level2_Latch
// ISO_Level3_Shift ISO_Level3_Latch ISO_Level3_Lock
// ISO_Level5_Shift ISO_Level5_Latch ISO_Level5_Lock

// const KEY_Shift_L = 0xffe1;
// const KEY_Shift_R = 0xffe2;
// const KEY_Control_L = 0xffe3;
// const KEY_Control_R = 0xffe4;
// const KEY_Caps_Lock = 0xffe5;
// const KEY_Shift_Lock = 0xffe6;
//
// const KEY_Meta_L = 0xffe7;
// const KEY_Meta_R = 0xffe8;
// const KEY_Alt_L = 0xffe9;
// const KEY_Alt_R = 0xffea;
// const KEY_Super_L = 0xffeb;
// const KEY_Super_R = 0xffec;
// const KEY_Hyper_L = 0xffed;
// const KEY_Hyper_R = 0xffee;
//
//
// const KEY_ISO_Lock = 0xfe01;
// const KEY_ISO_Level2_Latch = 0xfe02;
// const KEY_ISO_Level3_Shift = 0xfe03;
// const KEY_ISO_Level3_Latch = 0xfe04;
// const KEY_ISO_Level3_Lock = 0xfe05;
// const KEY_ISO_Level5_Shift = 0xfe11;
// const KEY_ISO_Level5_Latch = 0xfe12;
// const KEY_ISO_Level5_Lock = 0xfe13;
// const KEY_ISO_Group_Shift = 0xff7e;
// const KEY_ISO_Group_Latch = 0xfe06;
// const KEY_ISO_Group_Lock = 0xfe07;
// const KEY_ISO_Next_Group = 0xfe08;
// const KEY_ISO_Next_Group_Lock = 0xfe09;
// const KEY_ISO_Prev_Group = 0xfe0a;
// const KEY_ISO_Prev_Group_Lock = 0xfe0b;
// const KEY_ISO_First_Group = 0xfe0c;
// const KEY_ISO_First_Group_Lock = 0xfe0d;
// const KEY_ISO_Last_Group = 0xfe0e;
// const KEY_ISO_Last_Group_Lock = 0xfe0f;
bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct ModFlags: u8 {
		const XK_Shift_L = 1;
		const XK_Shift_R = 1 << 1;
		const XK_Control_L = 1 << 1 + 1;
		const XK_Control_R = 1 << 2;
		const XK_Alt_L = 1 << 2 + 1;
		const XK_Alt_R = 1 << 3;
		const XK_Super_L = 1 << 3 + 1;
		const XK_Super_R = 1 << 4;
		const XK_ISO_Level3_Shift = 1 << 4 + 1;
		const XK_ISO_Level5_Shift = 1 << 5;
	}
}

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Key: u32 {
		const XK_NoSymbol = 0x0000_0000;
		const XK_VoidSymbol = 0x00ff_ffff;


		const XK_BackSpace = 0xff08;
		const XK_Tab = 0xff09;
		const XK_Linefeed = 0xff0a;
		const XK_Clear = 0xff0b;
		const XK_Return = 0xff0d;
		const XK_Pause = 0xff13;
		const XK_Scroll_Lock = 0xff14;
		const XK_Sys_Req = 0xff15;
		const XK_Escape = 0xff1b;
		const XK_Delete = 0xffff;


		const XK_Multi_key = 0xff20;
		const XK_Codeinput = 0xff37;
		const XK_SingleCandidate = 0xff3c;
		const XK_MultipleCandidate = 0xff3d;
		const XK_PreviousCandidate = 0xff3e;


		const XK_Kanji = 0xff21;
		const XK_Muhenkan = 0xff22;
		const XK_Henkan_Mode = 0xff23;
		const XK_Henkan = 0xff23;
		const XK_Romaji = 0xff24;
		const XK_Hiragana = 0xff25;
		const XK_Katakana = 0xff26;
		const XK_Hiragana_Katakana = 0xff27;
		const XK_Zenkaku = 0xff28;
		const XK_Hankaku = 0xff29;
		const XK_Zenkaku_Hankaku = 0xff2a;
		const XK_Touroku = 0xff2b;
		const XK_Massyo = 0xff2c;
		const XK_Kana_Lock = 0xff2d;
		const XK_Kana_Shift = 0xff2e;
		const XK_Eisu_Shift = 0xff2f;
		const XK_Eisu_toggle = 0xff30;
		const XK_Kanji_Bangou = 0xff37;
		const XK_Zen_Koho = 0xff3d;
		const XK_Mae_Koho = 0xff3e;



		const XK_Home = 0xff50;
		const XK_Left = 0xff51;
		const XK_Up = 0xff52;
		const XK_Right = 0xff53;
		const XK_Down = 0xff54;
		const XK_Prior = 0xff55;
		const XK_Page_Up = 0xff55;
		const XK_Next = 0xff56;
		const XK_Page_Down = 0xff56;
		const XK_End = 0xff57;
		const XK_Begin = 0xff58;


		const XK_Select = 0xff60;
		const XK_Print = 0xff61;
		const XK_Execute = 0xff62;
		const XK_Insert = 0xff63;
		const XK_Undo = 0xff65;
		const XK_Redo = 0xff66;
		const XK_Menu = 0xff67;
		const XK_Find = 0xff68;
		const XK_Cancel = 0xff69;
		const XK_Help = 0xff6a;
		const XK_Break = 0xff6b;
		const XK_Mode_switch = 0xff7e;
		const XK_script_switch = 0xff7e;
		const XK_Num_Lock = 0xff7f;


		const XK_KP_Space = 0xff80;
		const XK_KP_Tab = 0xff89;
		const XK_KP_Enter = 0xff8d;
		const XK_KP_F1 = 0xff91;
		const XK_KP_F2 = 0xff92;
		const XK_KP_F3 = 0xff93;
		const XK_KP_F4 = 0xff94;
		const XK_KP_Home = 0xff95;
		const XK_KP_Left = 0xff96;
		const XK_KP_Up = 0xff97;
		const XK_KP_Right = 0xff98;
		const XK_KP_Down = 0xff99;
		const XK_KP_Prior = 0xff9a;
		const XK_KP_Page_Up = 0xff9a;
		const XK_KP_Next = 0xff9b;
		const XK_KP_Page_Down = 0xff9b;
		const XK_KP_End = 0xff9c;
		const XK_KP_Begin = 0xff9d;
		const XK_KP_Insert = 0xff9e;
		const XK_KP_Delete = 0xff9f;
		const XK_KP_Equal = 0xffbd;
		const XK_KP_Multiply = 0xffaa;
		const XK_KP_Add = 0xffab;
		const XK_KP_Separator = 0xffac;
		const XK_KP_Subtract = 0xffad;
		const XK_KP_Decimal = 0xffae;
		const XK_KP_Divide = 0xffaf;

		const XK_KP_0 = 0xffb0;
		const XK_KP_1 = 0xffb1;
		const XK_KP_2 = 0xffb2;
		const XK_KP_3 = 0xffb3;
		const XK_KP_4 = 0xffb4;
		const XK_KP_5 = 0xffb5;
		const XK_KP_6 = 0xffb6;
		const XK_KP_7 = 0xffb7;
		const XK_KP_8 = 0xffb8;
		const XK_KP_9 = 0xffb9;


		const XK_F1 = 0xffbe;
		const XK_F2 = 0xffbf;
		const XK_F3 = 0xffc0;
		const XK_F4 = 0xffc1;
		const XK_F5 = 0xffc2;
		const XK_F6 = 0xffc3;
		const XK_F7 = 0xffc4;
		const XK_F8 = 0xffc5;
		const XK_F9 = 0xffc6;
		const XK_F10 = 0xffc7;
		const XK_F11 = 0xffc8;
		const XK_L1 = 0xffc8;
		const XK_F12 = 0xffc9;
		const XK_L2 = 0xffc9;
		const XK_F13 = 0xffca;
		const XK_L3 = 0xffca;
		const XK_F14 = 0xffcb;
		const XK_L4 = 0xffcb;
		const XK_F15 = 0xffcc;
		const XK_L5 = 0xffcc;
		const XK_F16 = 0xffcd;
		const XK_L6 = 0xffcd;
		const XK_F17 = 0xffce;
		const XK_L7 = 0xffce;
		const XK_F18 = 0xffcf;
		const XK_L8 = 0xffcf;
		const XK_F19 = 0xffd0;
		const XK_L9 = 0xffd0;
		const XK_F20 = 0xffd1;
		const XK_L10 = 0xffd1;
		const XK_F21 = 0xffd2;
		const XK_R1 = 0xffd2;
		const XK_F22 = 0xffd3;
		const XK_R2 = 0xffd3;
		const XK_F23 = 0xffd4;
		const XK_R3 = 0xffd4;
		const XK_F24 = 0xffd5;
		const XK_R4 = 0xffd5;
		const XK_F25 = 0xffd6;
		const XK_R5 = 0xffd6;
		const XK_F26 = 0xffd7;
		const XK_R6 = 0xffd7;
		const XK_F27 = 0xffd8;
		const XK_R7 = 0xffd8;
		const XK_F28 = 0xffd9;
		const XK_R8 = 0xffd9;
		const XK_F29 = 0xffda;
		const XK_R9 = 0xffda;
		const XK_F30 = 0xffdb;
		const XK_R10 = 0xffdb;
		const XK_F31 = 0xffdc;
		const XK_R11 = 0xffdc;
		const XK_F32 = 0xffdd;
		const XK_R12 = 0xffdd;
		const XK_F33 = 0xffde;
		const XK_R13 = 0xffde;
		const XK_F34 = 0xffdf;
		const XK_R14 = 0xffdf;
		const XK_F35 = 0xffe0;
		const XK_R15 = 0xffe0;



		const XK_ISO_Left_Tab = 0xfe20;
		const XK_ISO_Move_Line_Up = 0xfe21;
		const XK_ISO_Move_Line_Down = 0xfe22;
		const XK_ISO_Partial_Line_Up = 0xfe23;
		const XK_ISO_Partial_Line_Down = 0xfe24;
		const XK_ISO_Partial_Space_Left = 0xfe25;
		const XK_ISO_Partial_Space_Right = 0xfe26;
		const XK_ISO_Set_Margin_Left = 0xfe27;
		const XK_ISO_Set_Margin_Right = 0xfe28;
		const XK_ISO_Release_Margin_Left = 0xfe29;
		const XK_ISO_Release_Margin_Right = 0xfe2a;
		const XK_ISO_Release_Both_Margins = 0xfe2b;
		const XK_ISO_Fast_Cursor_Left = 0xfe2c;
		const XK_ISO_Fast_Cursor_Right = 0xfe2d;
		const XK_ISO_Fast_Cursor_Up = 0xfe2e;
		const XK_ISO_Fast_Cursor_Down = 0xfe2f;
		const XK_ISO_Continuous_Underline = 0xfe30;
		const XK_ISO_Discontinuous_Underline = 0xfe31;
		const XK_ISO_Emphasize = 0xfe32;
		const XK_ISO_Center_Object = 0xfe33;
		const XK_ISO_Enter = 0xfe34;

		const XK_dead_grave = 0xfe50;
		const XK_dead_acute = 0xfe51;
		const XK_dead_circumflex = 0xfe52;
		const XK_dead_tilde = 0xfe53;
		const XK_dead_perispomeni = 0xfe53;
		const XK_dead_macron = 0xfe54;
		const XK_dead_breve = 0xfe55;
		const XK_dead_abovedot = 0xfe56;
		const XK_dead_diaeresis = 0xfe57;
		const XK_dead_abovering = 0xfe58;
		const XK_dead_doubleacute = 0xfe59;
		const XK_dead_caron = 0xfe5a;
		const XK_dead_cedilla = 0xfe5b;
		const XK_dead_ogonek = 0xfe5c;
		const XK_dead_iota = 0xfe5d;
		const XK_dead_voiced_sound = 0xfe5e;
		const XK_dead_semivoiced_sound = 0xfe5f;
		const XK_dead_belowdot = 0xfe60;
		const XK_dead_hook = 0xfe61;
		const XK_dead_horn = 0xfe62;
		const XK_dead_stroke = 0xfe63;
		const XK_dead_abovecomma = 0xfe64;
		const XK_dead_psili = 0xfe64;
		const XK_dead_abovereversedcomma = 0xfe65;
		const XK_dead_dasia = 0xfe65;
		const XK_dead_doublegrave = 0xfe66;
		const XK_dead_belowring = 0xfe67;
		const XK_dead_belowmacron = 0xfe68;
		const XK_dead_belowcircumflex = 0xfe69;
		const XK_dead_belowtilde = 0xfe6a;
		const XK_dead_belowbreve = 0xfe6b;
		const XK_dead_belowdiaeresis = 0xfe6c;
		const XK_dead_invertedbreve = 0xfe6d;
		const XK_dead_belowcomma = 0xfe6e;
		const XK_dead_currency = 0xfe6f;

		const XK_dead_lowline = 0xfe90;
		const XK_dead_aboveverticalline = 0xfe91;
		const XK_dead_belowverticalline = 0xfe92;
		const XK_dead_longsolidusoverlay = 0xfe93;

		const XK_dead_a = 0xfe80;
		const XK_dead_A = 0xfe81;
		const XK_dead_e = 0xfe82;
		const XK_dead_E = 0xfe83;
		const XK_dead_i = 0xfe84;
		const XK_dead_I = 0xfe85;
		const XK_dead_o = 0xfe86;
		const XK_dead_O = 0xfe87;
		const XK_dead_u = 0xfe88;
		const XK_dead_U = 0xfe89;
		const XK_dead_small_schwa = 0xfe8a;
		const XK_dead_capital_schwa = 0xfe8b;

		const XK_dead_greek = 0xfe8c;

		const XK_First_Virtual_Screen = 0xfed0;
		const XK_Prev_Virtual_Screen = 0xfed1;
		const XK_Next_Virtual_Screen = 0xfed2;
		const XK_Last_Virtual_Screen = 0xfed4;
		const XK_Terminate_Server = 0xfed5;

		const XK_AccessX_Enable = 0xfe70;
		const XK_AccessX_Feedback_Enable = 0xfe71;
		const XK_RepeatKeys_Enable = 0xfe72;
		const XK_SlowKeys_Enable = 0xfe73;
		const XK_BounceKeys_Enable = 0xfe74;
		const XK_StickyKeys_Enable = 0xfe75;
		const XK_MouseKeys_Enable = 0xfe76;
		const XK_MouseKeys_Accel_Enable = 0xfe77;
		const XK_Overlay1_Enable = 0xfe78;
		const XK_Overlay2_Enable = 0xfe79;
		const XK_AudibleBell_Enable = 0xfe7a;

		const XK_Pointer_Left = 0xfee0;
		const XK_Pointer_Right = 0xfee1;
		const XK_Pointer_Up = 0xfee2;
		const XK_Pointer_Down = 0xfee3;
		const XK_Pointer_UpLeft = 0xfee4;
		const XK_Pointer_UpRight = 0xfee5;
		const XK_Pointer_DownLeft = 0xfee6;
		const XK_Pointer_DownRight = 0xfee7;
		const XK_Pointer_Button_Dflt = 0xfee8;
		const XK_Pointer_Button1 = 0xfee9;
		const XK_Pointer_Button2 = 0xfeea;
		const XK_Pointer_Button3 = 0xfeeb;
		const XK_Pointer_Button4 = 0xfeec;
		const XK_Pointer_Button5 = 0xfeed;
		const XK_Pointer_DblClick_Dflt = 0xfeee;
		const XK_Pointer_DblClick1 = 0xfeef;
		const XK_Pointer_DblClick2 = 0xfef0;
		const XK_Pointer_DblClick3 = 0xfef1;
		const XK_Pointer_DblClick4 = 0xfef2;
		const XK_Pointer_DblClick5 = 0xfef3;
		const XK_Pointer_Drag_Dflt = 0xfef4;
		const XK_Pointer_Drag1 = 0xfef5;
		const XK_Pointer_Drag2 = 0xfef6;
		const XK_Pointer_Drag3 = 0xfef7;
		const XK_Pointer_Drag4 = 0xfef8;
		const XK_Pointer_Drag5 = 0xfefd;

		const XK_Pointer_EnableKeys = 0xfef9;
		const XK_Pointer_Accelerate = 0xfefa;
		const XK_Pointer_DfltBtnNext = 0xfefb;
		const XK_Pointer_DfltBtnPrev = 0xfefc;


		const XK_ch = 0xfea0;
		const XK_Ch = 0xfea1;
		const XK_CH = 0xfea2;
		const XK_c_h = 0xfea3;
		const XK_C_h = 0xfea4;
		const XK_C_H = 0xfea5;


		const XK_3270_Duplicate = 0xfd01;
		const XK_3270_FieldMark = 0xfd02;
		const XK_3270_Right2 = 0xfd03;
		const XK_3270_Left2 = 0xfd04;
		const XK_3270_BackTab = 0xfd05;
		const XK_3270_EraseEOF = 0xfd06;
		const XK_3270_EraseInput = 0xfd07;
		const XK_3270_Reset = 0xfd08;
		const XK_3270_Quit = 0xfd09;
		const XK_3270_PA1 = 0xfd0a;
		const XK_3270_PA2 = 0xfd0b;
		const XK_3270_PA3 = 0xfd0c;
		const XK_3270_Test = 0xfd0d;
		const XK_3270_Attn = 0xfd0e;
		const XK_3270_CursorBlink = 0xfd0f;
		const XK_3270_AltCursor = 0xfd10;
		const XK_3270_KeyClick = 0xfd11;
		const XK_3270_Jump = 0xfd12;
		const XK_3270_Ident = 0xfd13;
		const XK_3270_Rule = 0xfd14;
		const XK_3270_Copy = 0xfd15;
		const XK_3270_Play = 0xfd16;
		const XK_3270_Setup = 0xfd17;
		const XK_3270_Record = 0xfd18;
		const XK_3270_ChangeScreen = 0xfd19;
		const XK_3270_DeleteWord = 0xfd1a;
		const XK_3270_ExSelect = 0xfd1b;
		const XK_3270_CursorSelect = 0xfd1c;
		const XK_3270_PrintScreen = 0xfd1d;
		const XK_3270_Enter = 0xfd1e;

		const XK_space = 0x0020;
		const XK_exclam = 0x0021;
		const XK_quotedbl = 0x0022;
		const XK_numbersign = 0x0023;
		const XK_dollar = 0x0024;
		const XK_percent = 0x0025;
		const XK_ampersand = 0x0026;
		const XK_apostrophe = 0x0027;
		const XK_quoteright = 0x0027;
		const XK_parenleft = 0x0028;
		const XK_parenright = 0x0029;
		const XK_asterisk = 0x002a;
		const XK_plus = 0x002b;
		const XK_comma = 0x002c;
		const XK_minus = 0x002d;
		const XK_period = 0x002e;
		const XK_slash = 0x002f;
		const XK_0 = 0x0030;
		const XK_1 = 0x0031;
		const XK_2 = 0x0032;
		const XK_3 = 0x0033;
		const XK_4 = 0x0034;
		const XK_5 = 0x0035;
		const XK_6 = 0x0036;
		const XK_7 = 0x0037;
		const XK_8 = 0x0038;
		const XK_9 = 0x0039;
		const XK_colon = 0x003a;
		const XK_semicolon = 0x003b;
		const XK_less = 0x003c;
		const XK_equal = 0x003d;
		const XK_greater = 0x003e;
		const XK_question = 0x003f;
		const XK_at = 0x0040;
		const XK_bracketleft = 0x005b;
		const XK_backslash = 0x005c;
		const XK_bracketright = 0x005d;
		const XK_asciicircum = 0x005e;
		const XK_underscore = 0x005f;
		const XK_grave = 0x0060;
		const XK_quoteleft = 0x0060;
		const XK_a = 0x0061;
		const XK_b = 0x0062;
		const XK_c = 0x0063;
		const XK_d = 0x0064;
		const XK_e = 0x0065;
		const XK_f = 0x0066;
		const XK_g = 0x0067;
		const XK_h = 0x0068;
		const XK_i = 0x0069;
		const XK_j = 0x006a;
		const XK_k = 0x006b;
		const XK_l = 0x006c;
		const XK_m = 0x006d;
		const XK_n = 0x006e;
		const XK_o = 0x006f;
		const XK_p = 0x0070;
		const XK_q = 0x0071;
		const XK_r = 0x0072;
		const XK_s = 0x0073;
		const XK_t = 0x0074;
		const XK_u = 0x0075;
		const XK_v = 0x0076;
		const XK_w = 0x0077;
		const XK_x = 0x0078;
		const XK_y = 0x0079;
		const XK_z = 0x007a;
		const XK_braceleft = 0x007b;
		const XK_bar = 0x007c;
		const XK_braceright = 0x007d;
		const XK_asciitilde = 0x007e;

		const XK_nobreakspace = 0x00a0;
		const XK_exclamdown = 0x00a1;
		const XK_cent = 0x00a2;
		const XK_sterling = 0x00a3;
		const XK_currency = 0x00a4;
		const XK_yen = 0x00a5;
		const XK_brokenbar = 0x00a6;
		const XK_section = 0x00a7;
		const XK_diaeresis = 0x00a8;
		const XK_copyright = 0x00a9;
		const XK_ordfeminine = 0x00aa;
		const XK_guillemotleft = 0x00ab;
		const XK_notsign = 0x00ac;
		const XK_hyphen = 0x00ad;
		const XK_registered = 0x00ae;
		const XK_macron = 0x00af;
		const XK_degree = 0x00b0;
		const XK_plusminus = 0x00b1;
		const XK_twosuperior = 0x00b2;
		const XK_threesuperior = 0x00b3;
		const XK_acute = 0x00b4;
		const XK_mu = 0x00b5;
		const XK_paragraph = 0x00b6;
		const XK_periodcentered = 0x00b7;
		const XK_cedilla = 0x00b8;
		const XK_onesuperior = 0x00b9;
		const XK_masculine = 0x00ba;
		const XK_guillemotright = 0x00bb;
		const XK_onequarter = 0x00bc;
		const XK_onehalf = 0x00bd;
		const XK_threequarters = 0x00be;
		const XK_questiondown = 0x00bf;
		const XK_Agrave = 0x00c0;
		const XK_Aacute = 0x00c1;
		const XK_Acircumflex = 0x00c2;
		const XK_Atilde = 0x00c3;
		const XK_Adiaeresis = 0x00c4;
		const XK_Aring = 0x00c5;
		const XK_AE = 0x00c6;
		const XK_Ccedilla = 0x00c7;
		const XK_Egrave = 0x00c8;
		const XK_Eacute = 0x00c9;
		const XK_Ecircumflex = 0x00ca;
		const XK_Ediaeresis = 0x00cb;
		const XK_Igrave = 0x00cc;
		const XK_Iacute = 0x00cd;
		const XK_Icircumflex = 0x00ce;
		const XK_Idiaeresis = 0x00cf;
		const XK_ETH = 0x00d0;
		const XK_Eth = 0x00d0;
		const XK_Ntilde = 0x00d1;
		const XK_Ograve = 0x00d2;
		const XK_Oacute = 0x00d3;
		const XK_Ocircumflex = 0x00d4;
		const XK_Otilde = 0x00d5;
		const XK_Odiaeresis = 0x00d6;
		const XK_multiply = 0x00d7;
		const XK_Oslash = 0x00d8;
		const XK_Ooblique = 0x00d8;
		const XK_Ugrave = 0x00d9;
		const XK_Uacute = 0x00da;
		const XK_Ucircumflex = 0x00db;
		const XK_Udiaeresis = 0x00dc;
		const XK_Yacute = 0x00dd;
		const XK_THORN = 0x00de;
		const XK_Thorn = 0x00de;
		const XK_ssharp = 0x00df;
		const XK_agrave = 0x00e0;
		const XK_aacute = 0x00e1;
		const XK_acircumflex = 0x00e2;
		const XK_atilde = 0x00e3;
		const XK_adiaeresis = 0x00e4;
		const XK_aring = 0x00e5;
		const XK_ae = 0x00e6;
		const XK_ccedilla = 0x00e7;
		const XK_egrave = 0x00e8;
		const XK_eacute = 0x00e9;
		const XK_ecircumflex = 0x00ea;
		const XK_ediaeresis = 0x00eb;
		const XK_igrave = 0x00ec;
		const XK_iacute = 0x00ed;
		const XK_icircumflex = 0x00ee;
		const XK_idiaeresis = 0x00ef;
		const XK_eth = 0x00f0;
		const XK_ntilde = 0x00f1;
		const XK_ograve = 0x00f2;
		const XK_oacute = 0x00f3;
		const XK_ocircumflex = 0x00f4;
		const XK_otilde = 0x00f5;
		const XK_odiaeresis = 0x00f6;
		const XK_division = 0x00f7;
		const XK_oslash = 0x00f8;
		const XK_ooblique = 0x00f8;
		const XK_ugrave = 0x00f9;
		const XK_uacute = 0x00fa;
		const XK_ucircumflex = 0x00fb;
		const XK_udiaeresis = 0x00fc;
		const XK_yacute = 0x00fd;
		const XK_thorn = 0x00fe;
		const XK_ydiaeresis = 0x00ff;


		const XK_Aogonek = 0x01a1;
		const XK_breve = 0x01a2;
		const XK_Lstroke = 0x01a3;
		const XK_Lcaron = 0x01a5;
		const XK_Sacute = 0x01a6;
		const XK_Scaron = 0x01a9;
		const XK_Scedilla = 0x01aa;
		const XK_Tcaron = 0x01ab;
		const XK_Zacute = 0x01ac;
		const XK_Zcaron = 0x01ae;
		const XK_Zabovedot = 0x01af;
		const XK_aogonek = 0x01b1;
		const XK_ogonek = 0x01b2;
		const XK_lstroke = 0x01b3;
		const XK_lcaron = 0x01b5;
		const XK_sacute = 0x01b6;
		const XK_caron = 0x01b7;
		const XK_scaron = 0x01b9;
		const XK_scedilla = 0x01ba;
		const XK_tcaron = 0x01bb;
		const XK_zacute = 0x01bc;
		const XK_doubleacute = 0x01bd;
		const XK_zcaron = 0x01be;
		const XK_zabovedot = 0x01bf;
		const XK_Racute = 0x01c0;
		const XK_Abreve = 0x01c3;
		const XK_Lacute = 0x01c5;
		const XK_Cacute = 0x01c6;
		const XK_Ccaron = 0x01c8;
		const XK_Eogonek = 0x01ca;
		const XK_Ecaron = 0x01cc;
		const XK_Dcaron = 0x01cf;
		const XK_Dstroke = 0x01d0;
		const XK_Nacute = 0x01d1;
		const XK_Ncaron = 0x01d2;
		const XK_Odoubleacute = 0x01d5;
		const XK_Rcaron = 0x01d8;
		const XK_Uring = 0x01d9;
		const XK_Udoubleacute = 0x01db;
		const XK_Tcedilla = 0x01de;
		const XK_racute = 0x01e0;
		const XK_abreve = 0x01e3;
		const XK_lacute = 0x01e5;
		const XK_cacute = 0x01e6;
		const XK_ccaron = 0x01e8;
		const XK_eogonek = 0x01ea;
		const XK_ecaron = 0x01ec;
		const XK_dcaron = 0x01ef;
		const XK_dstroke = 0x01f0;
		const XK_nacute = 0x01f1;
		const XK_ncaron = 0x01f2;
		const XK_odoubleacute = 0x01f5;
		const XK_rcaron = 0x01f8;
		const XK_uring = 0x01f9;
		const XK_udoubleacute = 0x01fb;
		const XK_tcedilla = 0x01fe;
		const XK_abovedot = 0x01ff;


		const XK_Hstroke = 0x02a1;
		const XK_Hcircumflex = 0x02a6;
		const XK_Iabovedot = 0x02a9;
		const XK_Gbreve = 0x02ab;
		const XK_Jcircumflex = 0x02ac;
		const XK_hstroke = 0x02b1;
		const XK_hcircumflex = 0x02b6;
		const XK_idotless = 0x02b9;
		const XK_gbreve = 0x02bb;
		const XK_jcircumflex = 0x02bc;
		const XK_Cabovedot = 0x02c5;
		const XK_Ccircumflex = 0x02c6;
		const XK_Gabovedot = 0x02d5;
		const XK_Gcircumflex = 0x02d8;
		const XK_Ubreve = 0x02dd;
		const XK_Scircumflex = 0x02de;
		const XK_cabovedot = 0x02e5;
		const XK_ccircumflex = 0x02e6;
		const XK_gabovedot = 0x02f5;
		const XK_gcircumflex = 0x02f8;
		const XK_ubreve = 0x02fd;
		const XK_scircumflex = 0x02fe;


		const XK_kra = 0x03a2;
		const XK_kappa = 0x03a2;
		const XK_Rcedilla = 0x03a3;
		const XK_Itilde = 0x03a5;
		const XK_Lcedilla = 0x03a6;
		const XK_Emacron = 0x03aa;
		const XK_Gcedilla = 0x03ab;
		const XK_Tslash = 0x03ac;
		const XK_rcedilla = 0x03b3;
		const XK_itilde = 0x03b5;
		const XK_lcedilla = 0x03b6;
		const XK_emacron = 0x03ba;
		const XK_gcedilla = 0x03bb;
		const XK_tslash = 0x03bc;
		const XK_ENG = 0x03bd;
		const XK_eng = 0x03bf;
		const XK_Amacron = 0x03c0;
		const XK_Iogonek = 0x03c7;
		const XK_Eabovedot = 0x03cc;
		const XK_Imacron = 0x03cf;
		const XK_Ncedilla = 0x03d1;
		const XK_Omacron = 0x03d2;
		const XK_Kcedilla = 0x03d3;
		const XK_Uogonek = 0x03d9;
		const XK_Utilde = 0x03dd;
		const XK_Umacron = 0x03de;
		const XK_amacron = 0x03e0;
		const XK_iogonek = 0x03e7;
		const XK_eabovedot = 0x03ec;
		const XK_imacron = 0x03ef;
		const XK_ncedilla = 0x03f1;
		const XK_omacron = 0x03f2;
		const XK_kcedilla = 0x03f3;
		const XK_uogonek = 0x03f9;
		const XK_utilde = 0x03fd;
		const XK_umacron = 0x03fe;

		const XK_Wcircumflex = 0x0100_0174;
		const XK_wcircumflex = 0x0100_0175;
		const XK_Ycircumflex = 0x0100_0176;
		const XK_ycircumflex = 0x0100_0177;
		const XK_Babovedot = 0x0100_1e02;
		const XK_babovedot = 0x0100_1e03;
		const XK_Dabovedot = 0x0100_1e0a;
		const XK_dabovedot = 0x0100_1e0b;
		const XK_Fabovedot = 0x0100_1e1e;
		const XK_fabovedot = 0x0100_1e1f;
		const XK_Mabovedot = 0x0100_1e40;
		const XK_mabovedot = 0x0100_1e41;
		const XK_Pabovedot = 0x0100_1e56;
		const XK_pabovedot = 0x0100_1e57;
		const XK_Sabovedot = 0x0100_1e60;
		const XK_sabovedot = 0x0100_1e61;
		const XK_Tabovedot = 0x0100_1e6a;
		const XK_tabovedot = 0x0100_1e6b;
		const XK_Wgrave = 0x0100_1e80;
		const XK_wgrave = 0x0100_1e81;
		const XK_Wacute = 0x0100_1e82;
		const XK_wacute = 0x0100_1e83;
		const XK_Wdiaeresis = 0x0100_1e84;
		const XK_wdiaeresis = 0x0100_1e85;
		const XK_Ygrave = 0x0100_1ef2;
		const XK_ygrave = 0x0100_1ef3;


		const XK_OE = 0x13bc;
		const XK_oe = 0x13bd;
		const XK_Ydiaeresis = 0x13be;


		const XK_overline = 0x047e;
		const XK_kana_fullstop = 0x04a1;
		const XK_kana_openingbracket = 0x04a2;
		const XK_kana_closingbracket = 0x04a3;
		const XK_kana_comma = 0x04a4;
		const XK_kana_conjunctive = 0x04a5;
		const XK_kana_middledot = 0x04a5;
		const XK_kana_WO = 0x04a6;
		const XK_kana_a = 0x04a7;
		const XK_kana_i = 0x04a8;
		const XK_kana_u = 0x04a9;
		const XK_kana_e = 0x04aa;
		const XK_kana_o = 0x04ab;
		const XK_kana_ya = 0x04ac;
		const XK_kana_yu = 0x04ad;
		const XK_kana_yo = 0x04ae;
		const XK_kana_tsu = 0x04af;
		const XK_kana_tu = 0x04af;
		const XK_prolongedsound = 0x04b0;
		const XK_kana_A = 0x04b1;
		const XK_kana_I = 0x04b2;
		const XK_kana_U = 0x04b3;
		const XK_kana_E = 0x04b4;
		const XK_kana_O = 0x04b5;
		const XK_kana_KA = 0x04b6;
		const XK_kana_KI = 0x04b7;
		const XK_kana_KU = 0x04b8;
		const XK_kana_KE = 0x04b9;
		const XK_kana_KO = 0x04ba;
		const XK_kana_SA = 0x04bb;
		const XK_kana_SHI = 0x04bc;
		const XK_kana_SU = 0x04bd;
		const XK_kana_SE = 0x04be;
		const XK_kana_SO = 0x04bf;
		const XK_kana_TA = 0x04c0;
		const XK_kana_CHI = 0x04c1;
		const XK_kana_TI = 0x04c1;
		const XK_kana_TSU = 0x04c2;
		const XK_kana_TU = 0x04c2;
		const XK_kana_TE = 0x04c3;
		const XK_kana_TO = 0x04c4;
		const XK_kana_NA = 0x04c5;
		const XK_kana_NI = 0x04c6;
		const XK_kana_NU = 0x04c7;
		const XK_kana_NE = 0x04c8;
		const XK_kana_NO = 0x04c9;
		const XK_kana_HA = 0x04ca;
		const XK_kana_HI = 0x04cb;
		const XK_kana_FU = 0x04cc;
		const XK_kana_HU = 0x04cc;
		const XK_kana_HE = 0x04cd;
		const XK_kana_HO = 0x04ce;
		const XK_kana_MA = 0x04cf;
		const XK_kana_MI = 0x04d0;
		const XK_kana_MU = 0x04d1;
		const XK_kana_ME = 0x04d2;
		const XK_kana_MO = 0x04d3;
		const XK_kana_YA = 0x04d4;
		const XK_kana_YU = 0x04d5;
		const XK_kana_YO = 0x04d6;
		const XK_kana_RA = 0x04d7;
		const XK_kana_RI = 0x04d8;
		const XK_kana_RU = 0x04d9;
		const XK_kana_RE = 0x04da;
		const XK_kana_RO = 0x04db;
		const XK_kana_WA = 0x04dc;
		const XK_kana_N = 0x04dd;
		const XK_voicedsound = 0x04de;
		const XK_semivoicedsound = 0x04df;
		const XK_kana_switch = 0xff7e;


		const XK_Farsi_0 = 0x0100_06f0;
		const XK_Farsi_1 = 0x0100_06f1;
		const XK_Farsi_2 = 0x0100_06f2;
		const XK_Farsi_3 = 0x0100_06f3;
		const XK_Farsi_4 = 0x0100_06f4;
		const XK_Farsi_5 = 0x0100_06f5;
		const XK_Farsi_6 = 0x0100_06f6;
		const XK_Farsi_7 = 0x0100_06f7;
		const XK_Farsi_8 = 0x0100_06f8;
		const XK_Farsi_9 = 0x0100_06f9;
		const XK_Arabic_percent = 0x0100_066a;
		const XK_Arabic_superscript_alef = 0x0100_0670;
		const XK_Arabic_tteh = 0x0100_0679;
		const XK_Arabic_peh = 0x0100_067e;
		const XK_Arabic_tcheh = 0x0100_0686;
		const XK_Arabic_ddal = 0x0100_0688;
		const XK_Arabic_rreh = 0x0100_0691;
		const XK_Arabic_comma = 0x05ac;
		const XK_Arabic_fullstop = 0x0100_06d4;
		const XK_Arabic_0 = 0x0100_0660;
		const XK_Arabic_1 = 0x0100_0661;
		const XK_Arabic_2 = 0x0100_0662;
		const XK_Arabic_3 = 0x0100_0663;
		const XK_Arabic_4 = 0x0100_0664;
		const XK_Arabic_5 = 0x0100_0665;
		const XK_Arabic_6 = 0x0100_0666;
		const XK_Arabic_7 = 0x0100_0667;
		const XK_Arabic_8 = 0x0100_0668;
		const XK_Arabic_9 = 0x0100_0669;
		const XK_Arabic_semicolon = 0x05bb;
		const XK_Arabic_question_mark = 0x05bf;
		const XK_Arabic_hamza = 0x05c1;
		const XK_Arabic_maddaonalef = 0x05c2;
		const XK_Arabic_hamzaonalef = 0x05c3;
		const XK_Arabic_hamzaonwaw = 0x05c4;
		const XK_Arabic_hamzaunderalef = 0x05c5;
		const XK_Arabic_hamzaonyeh = 0x05c6;
		const XK_Arabic_alef = 0x05c7;
		const XK_Arabic_beh = 0x05c8;
		const XK_Arabic_tehmarbuta = 0x05c9;
		const XK_Arabic_teh = 0x05ca;
		const XK_Arabic_theh = 0x05cb;
		const XK_Arabic_jeem = 0x05cc;
		const XK_Arabic_hah = 0x05cd;
		const XK_Arabic_khah = 0x05ce;
		const XK_Arabic_dal = 0x05cf;
		const XK_Arabic_thal = 0x05d0;
		const XK_Arabic_ra = 0x05d1;
		const XK_Arabic_zain = 0x05d2;
		const XK_Arabic_seen = 0x05d3;
		const XK_Arabic_sheen = 0x05d4;
		const XK_Arabic_sad = 0x05d5;
		const XK_Arabic_dad = 0x05d6;
		const XK_Arabic_tah = 0x05d7;
		const XK_Arabic_zah = 0x05d8;
		const XK_Arabic_ain = 0x05d9;
		const XK_Arabic_ghain = 0x05da;
		const XK_Arabic_tatweel = 0x05e0;
		const XK_Arabic_feh = 0x05e1;
		const XK_Arabic_qaf = 0x05e2;
		const XK_Arabic_kaf = 0x05e3;
		const XK_Arabic_lam = 0x05e4;
		const XK_Arabic_meem = 0x05e5;
		const XK_Arabic_noon = 0x05e6;
		const XK_Arabic_ha = 0x05e7;
		const XK_Arabic_heh = 0x05e7;
		const XK_Arabic_waw = 0x05e8;
		const XK_Arabic_alefmaksura = 0x05e9;
		const XK_Arabic_yeh = 0x05ea;
		const XK_Arabic_fathatan = 0x05eb;
		const XK_Arabic_dammatan = 0x05ec;
		const XK_Arabic_kasratan = 0x05ed;
		const XK_Arabic_fatha = 0x05ee;
		const XK_Arabic_damma = 0x05ef;
		const XK_Arabic_kasra = 0x05f0;
		const XK_Arabic_shadda = 0x05f1;
		const XK_Arabic_sukun = 0x05f2;
		const XK_Arabic_madda_above = 0x0100_0653;
		const XK_Arabic_hamza_above = 0x0100_0654;
		const XK_Arabic_hamza_below = 0x0100_0655;
		const XK_Arabic_jeh = 0x0100_0698;
		const XK_Arabic_veh = 0x0100_06a4;
		const XK_Arabic_keheh = 0x0100_06a9;
		const XK_Arabic_gaf = 0x0100_06af;
		const XK_Arabic_noon_ghunna = 0x0100_06ba;
		const XK_Arabic_heh_doachashmee = 0x0100_06be;
		const XK_Farsi_yeh = 0x0100_06cc;
		const XK_Arabic_farsi_yeh = 0x0100_06cc;
		const XK_Arabic_yeh_baree = 0x0100_06d2;
		const XK_Arabic_heh_goal = 0x0100_06c1;
		const XK_Arabic_switch = 0xff7e;

		const XK_Cyrillic_GHE_bar = 0x0100_0492;
		const XK_Cyrillic_ghe_bar = 0x0100_0493;
		const XK_Cyrillic_ZHE_descender = 0x0100_0496;
		const XK_Cyrillic_zhe_descender = 0x0100_0497;
		const XK_Cyrillic_KA_descender = 0x0100_049a;
		const XK_Cyrillic_ka_descender = 0x0100_049b;
		const XK_Cyrillic_KA_vertstroke = 0x0100_049c;
		const XK_Cyrillic_ka_vertstroke = 0x0100_049d;
		const XK_Cyrillic_EN_descender = 0x0100_04a2;
		const XK_Cyrillic_en_descender = 0x0100_04a3;
		const XK_Cyrillic_U_straight = 0x0100_04ae;
		const XK_Cyrillic_u_straight = 0x0100_04af;
		const XK_Cyrillic_U_straight_bar = 0x0100_04b0;
		const XK_Cyrillic_u_straight_bar = 0x0100_04b1;
		const XK_Cyrillic_HA_descender = 0x0100_04b2;
		const XK_Cyrillic_ha_descender = 0x0100_04b3;
		const XK_Cyrillic_CHE_descender = 0x0100_04b6;
		const XK_Cyrillic_che_descender = 0x0100_04b7;
		const XK_Cyrillic_CHE_vertstroke = 0x0100_04b8;
		const XK_Cyrillic_che_vertstroke = 0x0100_04b9;
		const XK_Cyrillic_SHHA = 0x0100_04ba;
		const XK_Cyrillic_shha = 0x0100_04bb;

		const XK_Cyrillic_SCHWA = 0x0100_04d8;
		const XK_Cyrillic_schwa = 0x0100_04d9;
		const XK_Cyrillic_I_macron = 0x0100_04e2;
		const XK_Cyrillic_i_macron = 0x0100_04e3;
		const XK_Cyrillic_O_bar = 0x0100_04e8;
		const XK_Cyrillic_o_bar = 0x0100_04e9;
		const XK_Cyrillic_U_macron = 0x0100_04ee;
		const XK_Cyrillic_u_macron = 0x0100_04ef;

		const XK_Serbian_dje = 0x06a1;
		const XK_Macedonia_gje = 0x06a2;
		const XK_Cyrillic_io = 0x06a3;
		const XK_Ukrainian_ie = 0x06a4;
		const XK_Ukranian_je = 0x06a4;
		const XK_Macedonia_dse = 0x06a5;
		const XK_Ukrainian_i = 0x06a6;
		const XK_Ukranian_i = 0x06a6;
		const XK_Ukrainian_yi = 0x06a7;
		const XK_Ukranian_yi = 0x06a7;
		const XK_Cyrillic_je = 0x06a8;
		const XK_Serbian_je = 0x06a8;
		const XK_Cyrillic_lje = 0x06a9;
		const XK_Serbian_lje = 0x06a9;
		const XK_Cyrillic_nje = 0x06aa;
		const XK_Serbian_nje = 0x06aa;
		const XK_Serbian_tshe = 0x06ab;
		const XK_Macedonia_kje = 0x06ac;
		const XK_Ukrainian_ghe_with_upturn = 0x06ad;
		const XK_Byelorussian_shortu = 0x06ae;
		const XK_Cyrillic_dzhe = 0x06af;
		const XK_Serbian_dze = 0x06af;
		const XK_numerosign = 0x06b0;
		const XK_Serbian_DJE = 0x06b1;
		const XK_Macedonia_GJE = 0x06b2;
		const XK_Cyrillic_IO = 0x06b3;
		const XK_Ukrainian_IE = 0x06b4;
		const XK_Ukranian_JE = 0x06b4;
		const XK_Macedonia_DSE = 0x06b5;
		const XK_Ukrainian_I = 0x06b6;
		const XK_Ukranian_I = 0x06b6;
		const XK_Ukrainian_YI = 0x06b7;
		const XK_Ukranian_YI = 0x06b7;
		const XK_Cyrillic_JE = 0x06b8;
		const XK_Serbian_JE = 0x06b8;
		const XK_Cyrillic_LJE = 0x06b9;
		const XK_Serbian_LJE = 0x06b9;
		const XK_Cyrillic_NJE = 0x06ba;
		const XK_Serbian_NJE = 0x06ba;
		const XK_Serbian_TSHE = 0x06bb;
		const XK_Macedonia_KJE = 0x06bc;
		const XK_Ukrainian_GHE_WITH_UPTURN = 0x06bd;
		const XK_Byelorussian_SHORTU = 0x06be;
		const XK_Cyrillic_DZHE = 0x06bf;
		const XK_Serbian_DZE = 0x06bf;
		const XK_Cyrillic_yu = 0x06c0;
		const XK_Cyrillic_a = 0x06c1;
		const XK_Cyrillic_be = 0x06c2;
		const XK_Cyrillic_tse = 0x06c3;
		const XK_Cyrillic_de = 0x06c4;
		const XK_Cyrillic_ie = 0x06c5;
		const XK_Cyrillic_ef = 0x06c6;
		const XK_Cyrillic_ghe = 0x06c7;
		const XK_Cyrillic_ha = 0x06c8;
		const XK_Cyrillic_i = 0x06c9;
		const XK_Cyrillic_shorti = 0x06ca;
		const XK_Cyrillic_ka = 0x06cb;
		const XK_Cyrillic_el = 0x06cc;
		const XK_Cyrillic_em = 0x06cd;
		const XK_Cyrillic_en = 0x06ce;
		const XK_Cyrillic_o = 0x06cf;
		const XK_Cyrillic_pe = 0x06d0;
		const XK_Cyrillic_ya = 0x06d1;
		const XK_Cyrillic_er = 0x06d2;
		const XK_Cyrillic_es = 0x06d3;
		const XK_Cyrillic_te = 0x06d4;
		const XK_Cyrillic_u = 0x06d5;
		const XK_Cyrillic_zhe = 0x06d6;
		const XK_Cyrillic_ve = 0x06d7;
		const XK_Cyrillic_softsign = 0x06d8;
		const XK_Cyrillic_yeru = 0x06d9;
		const XK_Cyrillic_ze = 0x06da;
		const XK_Cyrillic_sha = 0x06db;
		const XK_Cyrillic_e = 0x06dc;
		const XK_Cyrillic_shcha = 0x06dd;
		const XK_Cyrillic_che = 0x06de;
		const XK_Cyrillic_hardsign = 0x06df;
		const XK_Cyrillic_YU = 0x06e0;
		const XK_Cyrillic_A = 0x06e1;
		const XK_Cyrillic_BE = 0x06e2;
		const XK_Cyrillic_TSE = 0x06e3;
		const XK_Cyrillic_DE = 0x06e4;
		const XK_Cyrillic_IE = 0x06e5;
		const XK_Cyrillic_EF = 0x06e6;
		const XK_Cyrillic_GHE = 0x06e7;
		const XK_Cyrillic_HA = 0x06e8;
		const XK_Cyrillic_I = 0x06e9;
		const XK_Cyrillic_SHORTI = 0x06ea;
		const XK_Cyrillic_KA = 0x06eb;
		const XK_Cyrillic_EL = 0x06ec;
		const XK_Cyrillic_EM = 0x06ed;
		const XK_Cyrillic_EN = 0x06ee;
		const XK_Cyrillic_O = 0x06ef;
		const XK_Cyrillic_PE = 0x06f0;
		const XK_Cyrillic_YA = 0x06f1;
		const XK_Cyrillic_ER = 0x06f2;
		const XK_Cyrillic_ES = 0x06f3;
		const XK_Cyrillic_TE = 0x06f4;
		const XK_Cyrillic_U = 0x06f5;
		const XK_Cyrillic_ZHE = 0x06f6;
		const XK_Cyrillic_VE = 0x06f7;
		const XK_Cyrillic_SOFTSIGN = 0x06f8;
		const XK_Cyrillic_YERU = 0x06f9;
		const XK_Cyrillic_ZE = 0x06fa;
		const XK_Cyrillic_SHA = 0x06fb;
		const XK_Cyrillic_E = 0x06fc;
		const XK_Cyrillic_SHCHA = 0x06fd;
		const XK_Cyrillic_CHE = 0x06fe;
		const XK_Cyrillic_HARDSIGN = 0x06ff;


		const XK_Greek_ALPHAaccent = 0x07a1;
		const XK_Greek_EPSILONaccent = 0x07a2;
		const XK_Greek_ETAaccent = 0x07a3;
		const XK_Greek_IOTAaccent = 0x07a4;
		const XK_Greek_IOTAdieresis = 0x07a5;
		const XK_Greek_IOTAdiaeresis = 0x07a5;
		const XK_Greek_OMICRONaccent = 0x07a7;
		const XK_Greek_UPSILONaccent = 0x07a8;
		const XK_Greek_UPSILONdieresis = 0x07a9;
		const XK_Greek_OMEGAaccent = 0x07ab;
		const XK_Greek_accentdieresis = 0x07ae;
		const XK_Greek_horizbar = 0x07af;
		const XK_Greek_alphaaccent = 0x07b1;
		const XK_Greek_epsilonaccent = 0x07b2;
		const XK_Greek_etaaccent = 0x07b3;
		const XK_Greek_iotaaccent = 0x07b4;
		const XK_Greek_iotadieresis = 0x07b5;
		const XK_Greek_iotaaccentdieresis = 0x07b6;
		const XK_Greek_omicronaccent = 0x07b7;
		const XK_Greek_upsilonaccent = 0x07b8;
		const XK_Greek_upsilondieresis = 0x07b9;
		const XK_Greek_upsilonaccentdieresis = 0x07ba;
		const XK_Greek_omegaaccent = 0x07bb;
		const XK_Greek_ALPHA = 0x07c1;
		const XK_Greek_BETA = 0x07c2;
		const XK_Greek_GAMMA = 0x07c3;
		const XK_Greek_DELTA = 0x07c4;
		const XK_Greek_EPSILON = 0x07c5;
		const XK_Greek_ZETA = 0x07c6;
		const XK_Greek_ETA = 0x07c7;
		const XK_Greek_THETA = 0x07c8;
		const XK_Greek_IOTA = 0x07c9;
		const XK_Greek_KAPPA = 0x07ca;
		const XK_Greek_LAMDA = 0x07cb;
		const XK_Greek_LAMBDA = 0x07cb;
		const XK_Greek_MU = 0x07cc;
		const XK_Greek_NU = 0x07cd;
		const XK_Greek_XI = 0x07ce;
		const XK_Greek_OMICRON = 0x07cf;
		const XK_Greek_PI = 0x07d0;
		const XK_Greek_RHO = 0x07d1;
		const XK_Greek_SIGMA = 0x07d2;
		const XK_Greek_TAU = 0x07d4;
		const XK_Greek_UPSILON = 0x07d5;
		const XK_Greek_PHI = 0x07d6;
		const XK_Greek_CHI = 0x07d7;
		const XK_Greek_PSI = 0x07d8;
		const XK_Greek_OMEGA = 0x07d9;
		const XK_Greek_alpha = 0x07e1;
		const XK_Greek_beta = 0x07e2;
		const XK_Greek_gamma = 0x07e3;
		const XK_Greek_delta = 0x07e4;
		const XK_Greek_epsilon = 0x07e5;
		const XK_Greek_zeta = 0x07e6;
		const XK_Greek_eta = 0x07e7;
		const XK_Greek_theta = 0x07e8;
		const XK_Greek_iota = 0x07e9;
		const XK_Greek_kappa = 0x07ea;
		const XK_Greek_lamda = 0x07eb;
		const XK_Greek_lambda = 0x07eb;
		const XK_Greek_mu = 0x07ec;
		const XK_Greek_nu = 0x07ed;
		const XK_Greek_xi = 0x07ee;
		const XK_Greek_omicron = 0x07ef;
		const XK_Greek_pi = 0x07f0;
		const XK_Greek_rho = 0x07f1;
		const XK_Greek_sigma = 0x07f2;
		const XK_Greek_finalsmallsigma = 0x07f3;
		const XK_Greek_tau = 0x07f4;
		const XK_Greek_upsilon = 0x07f5;
		const XK_Greek_phi = 0x07f6;
		const XK_Greek_chi = 0x07f7;
		const XK_Greek_psi = 0x07f8;
		const XK_Greek_omega = 0x07f9;
		const XK_Greek_switch = 0xff7e;


		const XK_leftradical = 0x08a1;
		const XK_topleftradical = 0x08a2;
		const XK_horizconnector = 0x08a3;
		const XK_topintegral = 0x08a4;
		const XK_botintegral = 0x08a5;
		const XK_vertconnector = 0x08a6;
		const XK_topleftsqbracket = 0x08a7;
		const XK_botleftsqbracket = 0x08a8;
		const XK_toprightsqbracket = 0x08a9;
		const XK_botrightsqbracket = 0x08aa;
		const XK_topleftparens = 0x08ab;
		const XK_botleftparens = 0x08ac;
		const XK_toprightparens = 0x08ad;
		const XK_botrightparens = 0x08ae;
		const XK_leftmiddlecurlybrace = 0x08af;
		const XK_rightmiddlecurlybrace = 0x08b0;
		const XK_topleftsummation = 0x08b1;
		const XK_botleftsummation = 0x08b2;
		const XK_topvertsummationconnector = 0x08b3;
		const XK_botvertsummationconnector = 0x08b4;
		const XK_toprightsummation = 0x08b5;
		const XK_botrightsummation = 0x08b6;
		const XK_rightmiddlesummation = 0x08b7;
		const XK_lessthanequal = 0x08bc;
		const XK_notequal = 0x08bd;
		const XK_greaterthanequal = 0x08be;
		const XK_integral = 0x08bf;
		const XK_therefore = 0x08c0;
		const XK_variation = 0x08c1;
		const XK_infinity = 0x08c2;
		const XK_nabla = 0x08c5;
		const XK_approximate = 0x08c8;
		const XK_similarequal = 0x08c9;
		const XK_ifonlyif = 0x08cd;
		const XK_implies = 0x08ce;
		const XK_identical = 0x08cf;
		const XK_radical = 0x08d6;
		const XK_includedin = 0x08da;
		const XK_includes = 0x08db;
		const XK_intersection = 0x08dc;
		const XK_union = 0x08dd;
		const XK_logicaland = 0x08de;
		const XK_logicalor = 0x08df;
		const XK_partialderivative = 0x08ef;
		const XK_function = 0x08f6;
		const XK_leftarrow = 0x08fb;
		const XK_uparrow = 0x08fc;
		const XK_rightarrow = 0x08fd;
		const XK_downarrow = 0x08fe;


		const XK_blank = 0x09df;
		const XK_soliddiamond = 0x09e0;
		const XK_checkerboard = 0x09e1;
		const XK_ht = 0x09e2;
		const XK_ff = 0x09e3;
		const XK_cr = 0x09e4;
		const XK_lf = 0x09e5;
		const XK_nl = 0x09e8;
		const XK_vt = 0x09e9;
		const XK_lowrightcorner = 0x09ea;
		const XK_uprightcorner = 0x09eb;
		const XK_upleftcorner = 0x09ec;
		const XK_lowleftcorner = 0x09ed;
		const XK_crossinglines = 0x09ee;
		const XK_horizlinescan1 = 0x09ef;
		const XK_horizlinescan3 = 0x09f0;
		const XK_horizlinescan5 = 0x09f1;
		const XK_horizlinescan7 = 0x09f2;
		const XK_horizlinescan9 = 0x09f3;
		const XK_leftt = 0x09f4;
		const XK_rightt = 0x09f5;
		const XK_bott = 0x09f6;
		const XK_topt = 0x09f7;
		const XK_vertbar = 0x09f8;


		const XK_emspace = 0x0aa1;
		const XK_enspace = 0x0aa2;
		const XK_em3space = 0x0aa3;
		const XK_em4space = 0x0aa4;
		const XK_digitspace = 0x0aa5;
		const XK_punctspace = 0x0aa6;
		const XK_thinspace = 0x0aa7;
		const XK_hairspace = 0x0aa8;
		const XK_emdash = 0x0aa9;
		const XK_endash = 0x0aaa;
		const XK_signifblank = 0x0aac;
		const XK_ellipsis = 0x0aae;
		const XK_doubbaselinedot = 0x0aaf;
		const XK_onethird = 0x0ab0;
		const XK_twothirds = 0x0ab1;
		const XK_onefifth = 0x0ab2;
		const XK_twofifths = 0x0ab3;
		const XK_threefifths = 0x0ab4;
		const XK_fourfifths = 0x0ab5;
		const XK_onesixth = 0x0ab6;
		const XK_fivesixths = 0x0ab7;
		const XK_careof = 0x0ab8;
		const XK_figdash = 0x0abb;
		const XK_leftanglebracket = 0x0abc;
		const XK_decimalpoint = 0x0abd;
		const XK_rightanglebracket = 0x0abe;
		const XK_marker = 0x0abf;
		const XK_oneeighth = 0x0ac3;
		const XK_threeeighths = 0x0ac4;
		const XK_fiveeighths = 0x0ac5;
		const XK_seveneighths = 0x0ac6;
		const XK_trademark = 0x0ac9;
		const XK_signaturemark = 0x0aca;
		const XK_trademarkincircle = 0x0acb;
		const XK_leftopentriangle = 0x0acc;
		const XK_rightopentriangle = 0x0acd;
		const XK_emopencircle = 0x0ace;
		const XK_emopenrectangle = 0x0acf;
		const XK_leftsinglequotemark = 0x0ad0;
		const XK_rightsinglequotemark = 0x0ad1;
		const XK_leftdoublequotemark = 0x0ad2;
		const XK_rightdoublequotemark = 0x0ad3;
		const XK_prescription = 0x0ad4;
		const XK_permille = 0x0ad5;
		const XK_minutes = 0x0ad6;
		const XK_seconds = 0x0ad7;
		const XK_latincross = 0x0ad9;
		const XK_hexagram = 0x0ada;
		const XK_filledrectbullet = 0x0adb;
		const XK_filledlefttribullet = 0x0adc;
		const XK_filledrighttribullet = 0x0add;
		const XK_emfilledcircle = 0x0ade;
		const XK_emfilledrect = 0x0adf;
		const XK_enopencircbullet = 0x0ae0;
		const XK_enopensquarebullet = 0x0ae1;
		const XK_openrectbullet = 0x0ae2;
		const XK_opentribulletup = 0x0ae3;
		const XK_opentribulletdown = 0x0ae4;
		const XK_openstar = 0x0ae5;
		const XK_enfilledcircbullet = 0x0ae6;
		const XK_enfilledsqbullet = 0x0ae7;
		const XK_filledtribulletup = 0x0ae8;
		const XK_filledtribulletdown = 0x0ae9;
		const XK_leftpointer = 0x0aea;
		const XK_rightpointer = 0x0aeb;
		const XK_club = 0x0aec;
		const XK_diamond = 0x0aed;
		const XK_heart = 0x0aee;
		const XK_maltesecross = 0x0af0;
		const XK_dagger = 0x0af1;
		const XK_doubledagger = 0x0af2;
		const XK_checkmark = 0x0af3;
		const XK_ballotcross = 0x0af4;
		const XK_musicalsharp = 0x0af5;
		const XK_musicalflat = 0x0af6;
		const XK_malesymbol = 0x0af7;
		const XK_femalesymbol = 0x0af8;
		const XK_telephone = 0x0af9;
		const XK_telephonerecorder = 0x0afa;
		const XK_phonographcopyright = 0x0afb;
		const XK_caret = 0x0afc;
		const XK_singlelowquotemark = 0x0afd;
		const XK_doublelowquotemark = 0x0afe;
		const XK_cursor = 0x0aff;


		const XK_leftcaret = 0x0ba3;
		const XK_rightcaret = 0x0ba6;
		const XK_downcaret = 0x0ba8;
		const XK_upcaret = 0x0ba9;
		const XK_overbar = 0x0bc0;
		const XK_downtack = 0x0bc2;
		const XK_upshoe = 0x0bc3;
		const XK_downstile = 0x0bc4;
		const XK_underbar = 0x0bc6;
		const XK_jot = 0x0bca;
		const XK_quad = 0x0bcc;
		const XK_uptack = 0x0bce;
		const XK_circle = 0x0bcf;
		const XK_upstile = 0x0bd3;
		const XK_downshoe = 0x0bd6;
		const XK_rightshoe = 0x0bd8;
		const XK_leftshoe = 0x0bda;
		const XK_lefttack = 0x0bdc;
		const XK_righttack = 0x0bfc;


		const XK_hebrew_doublelowline = 0x0cdf;
		const XK_hebrew_aleph = 0x0ce0;
		const XK_hebrew_bet = 0x0ce1;
		const XK_hebrew_beth = 0x0ce1;
		const XK_hebrew_gimel = 0x0ce2;
		const XK_hebrew_gimmel = 0x0ce2;
		const XK_hebrew_dalet = 0x0ce3;
		const XK_hebrew_daleth = 0x0ce3;
		const XK_hebrew_he = 0x0ce4;
		const XK_hebrew_waw = 0x0ce5;
		const XK_hebrew_zain = 0x0ce6;
		const XK_hebrew_zayin = 0x0ce6;
		const XK_hebrew_chet = 0x0ce7;
		const XK_hebrew_het = 0x0ce7;
		const XK_hebrew_tet = 0x0ce8;
		const XK_hebrew_teth = 0x0ce8;
		const XK_hebrew_yod = 0x0ce9;
		const XK_hebrew_finalkaph = 0x0cea;
		const XK_hebrew_kaph = 0x0ceb;
		const XK_hebrew_lamed = 0x0cec;
		const XK_hebrew_finalmem = 0x0ced;
		const XK_hebrew_mem = 0x0cee;
		const XK_hebrew_finalnun = 0x0cef;
		const XK_hebrew_nun = 0x0cf0;
		const XK_hebrew_samech = 0x0cf1;
		const XK_hebrew_samekh = 0x0cf1;
		const XK_hebrew_ayin = 0x0cf2;
		const XK_hebrew_finalpe = 0x0cf3;
		const XK_hebrew_pe = 0x0cf4;
		const XK_hebrew_finalzade = 0x0cf5;
		const XK_hebrew_finalzadi = 0x0cf5;
		const XK_hebrew_zade = 0x0cf6;
		const XK_hebrew_zadi = 0x0cf6;
		const XK_hebrew_qoph = 0x0cf7;
		const XK_hebrew_kuf = 0x0cf7;
		const XK_hebrew_resh = 0x0cf8;
		const XK_hebrew_shin = 0x0cf9;
		const XK_hebrew_taw = 0x0cfa;
		const XK_hebrew_taf = 0x0cfa;
		const XK_Hebrew_switch = 0xff7e;


		const XK_Thai_kokai = 0x0da1;
		const XK_Thai_khokhai = 0x0da2;
		const XK_Thai_khokhuat = 0x0da3;
		const XK_Thai_khokhwai = 0x0da4;
		const XK_Thai_khokhon = 0x0da5;
		const XK_Thai_khorakhang = 0x0da6;
		const XK_Thai_ngongu = 0x0da7;
		const XK_Thai_chochan = 0x0da8;
		const XK_Thai_choching = 0x0da9;
		const XK_Thai_chochang = 0x0daa;
		const XK_Thai_soso = 0x0dab;
		const XK_Thai_chochoe = 0x0dac;
		const XK_Thai_yoying = 0x0dad;
		const XK_Thai_dochada = 0x0dae;
		const XK_Thai_topatak = 0x0daf;
		const XK_Thai_thothan = 0x0db0;
		const XK_Thai_thonangmontho = 0x0db1;
		const XK_Thai_thophuthao = 0x0db2;
		const XK_Thai_nonen = 0x0db3;
		const XK_Thai_dodek = 0x0db4;
		const XK_Thai_totao = 0x0db5;
		const XK_Thai_thothung = 0x0db6;
		const XK_Thai_thothahan = 0x0db7;
		const XK_Thai_thothong = 0x0db8;
		const XK_Thai_nonu = 0x0db9;
		const XK_Thai_bobaimai = 0x0dba;
		const XK_Thai_popla = 0x0dbb;
		const XK_Thai_phophung = 0x0dbc;
		const XK_Thai_fofa = 0x0dbd;
		const XK_Thai_phophan = 0x0dbe;
		const XK_Thai_fofan = 0x0dbf;
		const XK_Thai_phosamphao = 0x0dc0;
		const XK_Thai_moma = 0x0dc1;
		const XK_Thai_yoyak = 0x0dc2;
		const XK_Thai_rorua = 0x0dc3;
		const XK_Thai_ru = 0x0dc4;
		const XK_Thai_loling = 0x0dc5;
		const XK_Thai_lu = 0x0dc6;
		const XK_Thai_wowaen = 0x0dc7;
		const XK_Thai_sosala = 0x0dc8;
		const XK_Thai_sorusi = 0x0dc9;
		const XK_Thai_sosua = 0x0dca;
		const XK_Thai_hohip = 0x0dcb;
		const XK_Thai_lochula = 0x0dcc;
		const XK_Thai_oang = 0x0dcd;
		const XK_Thai_honokhuk = 0x0dce;
		const XK_Thai_paiyannoi = 0x0dcf;
		const XK_Thai_saraa = 0x0dd0;
		const XK_Thai_maihanakat = 0x0dd1;
		const XK_Thai_saraaa = 0x0dd2;
		const XK_Thai_saraam = 0x0dd3;
		const XK_Thai_sarai = 0x0dd4;
		const XK_Thai_saraii = 0x0dd5;
		const XK_Thai_saraue = 0x0dd6;
		const XK_Thai_sarauee = 0x0dd7;
		const XK_Thai_sarau = 0x0dd8;
		const XK_Thai_sarauu = 0x0dd9;
		const XK_Thai_phinthu = 0x0dda;
		const XK_Thai_maihanakat_maitho = 0x0dde;
		const XK_Thai_baht = 0x0ddf;
		const XK_Thai_sarae = 0x0de0;
		const XK_Thai_saraae = 0x0de1;
		const XK_Thai_sarao = 0x0de2;
		const XK_Thai_saraaimaimuan = 0x0de3;
		const XK_Thai_saraaimaimalai = 0x0de4;
		const XK_Thai_lakkhangyao = 0x0de5;
		const XK_Thai_maiyamok = 0x0de6;
		const XK_Thai_maitaikhu = 0x0de7;
		const XK_Thai_maiek = 0x0de8;
		const XK_Thai_maitho = 0x0de9;
		const XK_Thai_maitri = 0x0dea;
		const XK_Thai_maichattawa = 0x0deb;
		const XK_Thai_thanthakhat = 0x0dec;
		const XK_Thai_nikhahit = 0x0ded;
		const XK_Thai_leksun = 0x0df0;
		const XK_Thai_leknung = 0x0df1;
		const XK_Thai_leksong = 0x0df2;
		const XK_Thai_leksam = 0x0df3;
		const XK_Thai_leksi = 0x0df4;
		const XK_Thai_lekha = 0x0df5;
		const XK_Thai_lekhok = 0x0df6;
		const XK_Thai_lekchet = 0x0df7;
		const XK_Thai_lekpaet = 0x0df8;
		const XK_Thai_lekkao = 0x0df9;


		const XK_Hangul = 0xff31;
		const XK_Hangul_Start = 0xff32;
		const XK_Hangul_End = 0xff33;
		const XK_Hangul_Hanja = 0xff34;
		const XK_Hangul_Jamo = 0xff35;
		const XK_Hangul_Romaja = 0xff36;
		const XK_Hangul_Codeinput = 0xff37;
		const XK_Hangul_Jeonja = 0xff38;
		const XK_Hangul_Banja = 0xff39;
		const XK_Hangul_PreHanja = 0xff3a;
		const XK_Hangul_PostHanja = 0xff3b;
		const XK_Hangul_SingleCandidate = 0xff3c;
		const XK_Hangul_MultipleCandidate = 0xff3d;
		const XK_Hangul_PreviousCandidate = 0xff3e;
		const XK_Hangul_Special = 0xff3f;
		const XK_Hangul_switch = 0xff7e;

		const XK_Hangul_Kiyeog = 0x0ea1;
		const XK_Hangul_SsangKiyeog = 0x0ea2;
		const XK_Hangul_KiyeogSios = 0x0ea3;
		const XK_Hangul_Nieun = 0x0ea4;
		const XK_Hangul_NieunJieuj = 0x0ea5;
		const XK_Hangul_NieunHieuh = 0x0ea6;
		const XK_Hangul_Dikeud = 0x0ea7;
		const XK_Hangul_SsangDikeud = 0x0ea8;
		const XK_Hangul_Rieul = 0x0ea9;
		const XK_Hangul_RieulKiyeog = 0x0eaa;
		const XK_Hangul_RieulMieum = 0x0eab;
		const XK_Hangul_RieulPieub = 0x0eac;
		const XK_Hangul_RieulSios = 0x0ead;
		const XK_Hangul_RieulTieut = 0x0eae;
		const XK_Hangul_RieulPhieuf = 0x0eaf;
		const XK_Hangul_RieulHieuh = 0x0eb0;
		const XK_Hangul_Mieum = 0x0eb1;
		const XK_Hangul_Pieub = 0x0eb2;
		const XK_Hangul_SsangPieub = 0x0eb3;
		const XK_Hangul_PieubSios = 0x0eb4;
		const XK_Hangul_Sios = 0x0eb5;
		const XK_Hangul_SsangSios = 0x0eb6;
		const XK_Hangul_Ieung = 0x0eb7;
		const XK_Hangul_Jieuj = 0x0eb8;
		const XK_Hangul_SsangJieuj = 0x0eb9;
		const XK_Hangul_Cieuc = 0x0eba;
		const XK_Hangul_Khieuq = 0x0ebb;
		const XK_Hangul_Tieut = 0x0ebc;
		const XK_Hangul_Phieuf = 0x0ebd;
		const XK_Hangul_Hieuh = 0x0ebe;

		const XK_Hangul_A = 0x0ebf;
		const XK_Hangul_AE = 0x0ec0;
		const XK_Hangul_YA = 0x0ec1;
		const XK_Hangul_YAE = 0x0ec2;
		const XK_Hangul_EO = 0x0ec3;
		const XK_Hangul_E = 0x0ec4;
		const XK_Hangul_YEO = 0x0ec5;
		const XK_Hangul_YE = 0x0ec6;
		const XK_Hangul_O = 0x0ec7;
		const XK_Hangul_WA = 0x0ec8;
		const XK_Hangul_WAE = 0x0ec9;
		const XK_Hangul_OE = 0x0eca;
		const XK_Hangul_YO = 0x0ecb;
		const XK_Hangul_U = 0x0ecc;
		const XK_Hangul_WEO = 0x0ecd;
		const XK_Hangul_WE = 0x0ece;
		const XK_Hangul_WI = 0x0ecf;
		const XK_Hangul_YU = 0x0ed0;
		const XK_Hangul_EU = 0x0ed1;
		const XK_Hangul_YI = 0x0ed2;
		const XK_Hangul_I = 0x0ed3;

		const XK_Hangul_J_Kiyeog = 0x0ed4;
		const XK_Hangul_J_SsangKiyeog = 0x0ed5;
		const XK_Hangul_J_KiyeogSios = 0x0ed6;
		const XK_Hangul_J_Nieun = 0x0ed7;
		const XK_Hangul_J_NieunJieuj = 0x0ed8;
		const XK_Hangul_J_NieunHieuh = 0x0ed9;
		const XK_Hangul_J_Dikeud = 0x0eda;
		const XK_Hangul_J_Rieul = 0x0edb;
		const XK_Hangul_J_RieulKiyeog = 0x0edc;
		const XK_Hangul_J_RieulMieum = 0x0edd;
		const XK_Hangul_J_RieulPieub = 0x0ede;
		const XK_Hangul_J_RieulSios = 0x0edf;
		const XK_Hangul_J_RieulTieut = 0x0ee0;
		const XK_Hangul_J_RieulPhieuf = 0x0ee1;
		const XK_Hangul_J_RieulHieuh = 0x0ee2;
		const XK_Hangul_J_Mieum = 0x0ee3;
		const XK_Hangul_J_Pieub = 0x0ee4;
		const XK_Hangul_J_PieubSios = 0x0ee5;
		const XK_Hangul_J_Sios = 0x0ee6;
		const XK_Hangul_J_SsangSios = 0x0ee7;
		const XK_Hangul_J_Ieung = 0x0ee8;
		const XK_Hangul_J_Jieuj = 0x0ee9;
		const XK_Hangul_J_Cieuc = 0x0eea;
		const XK_Hangul_J_Khieuq = 0x0eeb;
		const XK_Hangul_J_Tieut = 0x0eec;
		const XK_Hangul_J_Phieuf = 0x0eed;
		const XK_Hangul_J_Hieuh = 0x0eee;

		const XK_Hangul_RieulYeorinHieuh = 0x0eef;
		const XK_Hangul_SunkyeongeumMieum = 0x0ef0;
		const XK_Hangul_SunkyeongeumPieub = 0x0ef1;
		const XK_Hangul_PanSios = 0x0ef2;
		const XK_Hangul_KkogjiDalrinIeung = 0x0ef3;
		const XK_Hangul_SunkyeongeumPhieuf = 0x0ef4;
		const XK_Hangul_YeorinHieuh = 0x0ef5;

		const XK_Hangul_AraeA = 0x0ef6;
		const XK_Hangul_AraeAE = 0x0ef7;

		const XK_Hangul_J_PanSios = 0x0ef8;
		const XK_Hangul_J_KkogjiDalrinIeung = 0x0ef9;
		const XK_Hangul_J_YeorinHieuh = 0x0efa;

		const XK_Korean_Won = 0x0eff;


		const XK_Armenian_ligature_ew = 0x0100_0587;
		const XK_Armenian_full_stop = 0x0100_0589;
		const XK_Armenian_verjaket = 0x0100_0589;
		const XK_Armenian_separation_mark = 0x0100_055d;
		const XK_Armenian_but = 0x0100_055d;
		const XK_Armenian_hyphen = 0x0100_058a;
		const XK_Armenian_yentamna = 0x0100_058a;
		const XK_Armenian_exclam = 0x0100_055c;
		const XK_Armenian_amanak = 0x0100_055c;
		const XK_Armenian_accent = 0x0100_055b;
		const XK_Armenian_shesht = 0x0100_055b;
		const XK_Armenian_question = 0x0100_055e;
		const XK_Armenian_paruyk = 0x0100_055e;
		const XK_Armenian_AYB = 0x0100_0531;
		const XK_Armenian_ayb = 0x0100_0561;
		const XK_Armenian_BEN = 0x0100_0532;
		const XK_Armenian_ben = 0x0100_0562;
		const XK_Armenian_GIM = 0x0100_0533;
		const XK_Armenian_gim = 0x0100_0563;
		const XK_Armenian_DA = 0x0100_0534;
		const XK_Armenian_da = 0x0100_0564;
		const XK_Armenian_YECH = 0x0100_0535;
		const XK_Armenian_yech = 0x0100_0565;
		const XK_Armenian_ZA = 0x0100_0536;
		const XK_Armenian_za = 0x0100_0566;
		const XK_Armenian_E = 0x0100_0537;
		const XK_Armenian_e = 0x0100_0567;
		const XK_Armenian_AT = 0x0100_0538;
		const XK_Armenian_at = 0x0100_0568;
		const XK_Armenian_TO = 0x0100_0539;
		const XK_Armenian_to = 0x0100_0569;
		const XK_Armenian_ZHE = 0x0100_053a;
		const XK_Armenian_zhe = 0x0100_056a;
		const XK_Armenian_INI = 0x0100_053b;
		const XK_Armenian_ini = 0x0100_056b;
		const XK_Armenian_LYUN = 0x0100_053c;
		const XK_Armenian_lyun = 0x0100_056c;
		const XK_Armenian_KHE = 0x0100_053d;
		const XK_Armenian_khe = 0x0100_056d;
		const XK_Armenian_TSA = 0x0100_053e;
		const XK_Armenian_tsa = 0x0100_056e;
		const XK_Armenian_KEN = 0x0100_053f;
		const XK_Armenian_ken = 0x0100_056f;
		const XK_Armenian_HO = 0x0100_0540;
		const XK_Armenian_ho = 0x0100_0570;
		const XK_Armenian_DZA = 0x0100_0541;
		const XK_Armenian_dza = 0x0100_0571;
		const XK_Armenian_GHAT = 0x0100_0542;
		const XK_Armenian_ghat = 0x0100_0572;
		const XK_Armenian_TCHE = 0x0100_0543;
		const XK_Armenian_tche = 0x0100_0573;
		const XK_Armenian_MEN = 0x0100_0544;
		const XK_Armenian_men = 0x0100_0574;
		const XK_Armenian_HI = 0x0100_0545;
		const XK_Armenian_hi = 0x0100_0575;
		const XK_Armenian_NU = 0x0100_0546;
		const XK_Armenian_nu = 0x0100_0576;
		const XK_Armenian_SHA = 0x0100_0547;
		const XK_Armenian_sha = 0x0100_0577;
		const XK_Armenian_VO = 0x0100_0548;
		const XK_Armenian_vo = 0x0100_0578;
		const XK_Armenian_CHA = 0x0100_0549;
		const XK_Armenian_cha = 0x0100_0579;
		const XK_Armenian_PE = 0x0100_054a;
		const XK_Armenian_pe = 0x0100_057a;
		const XK_Armenian_JE = 0x0100_054b;
		const XK_Armenian_je = 0x0100_057b;
		const XK_Armenian_RA = 0x0100_054c;
		const XK_Armenian_ra = 0x0100_057c;
		const XK_Armenian_SE = 0x0100_054d;
		const XK_Armenian_se = 0x0100_057d;
		const XK_Armenian_VEV = 0x0100_054e;
		const XK_Armenian_vev = 0x0100_057e;
		const XK_Armenian_TYUN = 0x0100_054f;
		const XK_Armenian_tyun = 0x0100_057f;
		const XK_Armenian_RE = 0x0100_0550;
		const XK_Armenian_re = 0x0100_0580;
		const XK_Armenian_TSO = 0x0100_0551;
		const XK_Armenian_tso = 0x0100_0581;
		const XK_Armenian_VYUN = 0x0100_0552;
		const XK_Armenian_vyun = 0x0100_0582;
		const XK_Armenian_PYUR = 0x0100_0553;
		const XK_Armenian_pyur = 0x0100_0583;
		const XK_Armenian_KE = 0x0100_0554;
		const XK_Armenian_ke = 0x0100_0584;
		const XK_Armenian_O = 0x0100_0555;
		const XK_Armenian_o = 0x0100_0585;
		const XK_Armenian_FE = 0x0100_0556;
		const XK_Armenian_fe = 0x0100_0586;
		const XK_Armenian_apostrophe = 0x0100_055a;


		const XK_Georgian_an = 0x0100_10d0;
		const XK_Georgian_ban = 0x0100_10d1;
		const XK_Georgian_gan = 0x0100_10d2;
		const XK_Georgian_don = 0x0100_10d3;
		const XK_Georgian_en = 0x0100_10d4;
		const XK_Georgian_vin = 0x0100_10d5;
		const XK_Georgian_zen = 0x0100_10d6;
		const XK_Georgian_tan = 0x0100_10d7;
		const XK_Georgian_in = 0x0100_10d8;
		const XK_Georgian_kan = 0x0100_10d9;
		const XK_Georgian_las = 0x0100_10da;
		const XK_Georgian_man = 0x0100_10db;
		const XK_Georgian_nar = 0x0100_10dc;
		const XK_Georgian_on = 0x0100_10dd;
		const XK_Georgian_par = 0x0100_10de;
		const XK_Georgian_zhar = 0x0100_10df;
		const XK_Georgian_rae = 0x0100_10e0;
		const XK_Georgian_san = 0x0100_10e1;
		const XK_Georgian_tar = 0x0100_10e2;
		const XK_Georgian_un = 0x0100_10e3;
		const XK_Georgian_phar = 0x0100_10e4;
		const XK_Georgian_khar = 0x0100_10e5;
		const XK_Georgian_ghan = 0x0100_10e6;
		const XK_Georgian_qar = 0x0100_10e7;
		const XK_Georgian_shin = 0x0100_10e8;
		const XK_Georgian_chin = 0x0100_10e9;
		const XK_Georgian_can = 0x0100_10ea;
		const XK_Georgian_jil = 0x0100_10eb;
		const XK_Georgian_cil = 0x0100_10ec;
		const XK_Georgian_char = 0x0100_10ed;
		const XK_Georgian_xan = 0x0100_10ee;
		const XK_Georgian_jhan = 0x0100_10ef;
		const XK_Georgian_hae = 0x0100_10f0;
		const XK_Georgian_he = 0x0100_10f1;
		const XK_Georgian_hie = 0x0100_10f2;
		const XK_Georgian_we = 0x0100_10f3;
		const XK_Georgian_har = 0x0100_10f4;
		const XK_Georgian_hoe = 0x0100_10f5;
		const XK_Georgian_fi = 0x0100_10f6;


		const XK_Xabovedot = 0x0100_1e8a;
		const XK_Ibreve = 0x0100_012c;
		const XK_Zstroke = 0x0100_01b5;
		const XK_Gcaron = 0x0100_01e6;
		const XK_Ocaron = 0x0100_01d1;
		const XK_Obarred = 0x0100_019f;
		const XK_xabovedot = 0x0100_1e8b;
		const XK_ibreve = 0x0100_012d;
		const XK_zstroke = 0x0100_01b6;
		const XK_gcaron = 0x0100_01e7;
		const XK_ocaron = 0x0100_01d2;
		const XK_obarred = 0x0100_0275;
		const XK_SCHWA = 0x0100_018f;
		const XK_schwa = 0x0100_0259;
		const XK_EZH = 0x0100_01b7;
		const XK_ezh = 0x0100_0292;
		const XK_Lbelowdot = 0x0100_1e36;
		const XK_lbelowdot = 0x0100_1e37;


		const XK_Abelowdot = 0x0100_1ea0;
		const XK_abelowdot = 0x0100_1ea1;
		const XK_Ahook = 0x0100_1ea2;
		const XK_ahook = 0x0100_1ea3;
		const XK_Acircumflexacute = 0x0100_1ea4;
		const XK_acircumflexacute = 0x0100_1ea5;
		const XK_Acircumflexgrave = 0x0100_1ea6;
		const XK_acircumflexgrave = 0x0100_1ea7;
		const XK_Acircumflexhook = 0x0100_1ea8;
		const XK_acircumflexhook = 0x0100_1ea9;
		const XK_Acircumflextilde = 0x0100_1eaa;
		const XK_acircumflextilde = 0x0100_1eab;
		const XK_Acircumflexbelowdot = 0x0100_1eac;
		const XK_acircumflexbelowdot = 0x0100_1ead;
		const XK_Abreveacute = 0x0100_1eae;
		const XK_abreveacute = 0x0100_1eaf;
		const XK_Abrevegrave = 0x0100_1eb0;
		const XK_abrevegrave = 0x0100_1eb1;
		const XK_Abrevehook = 0x0100_1eb2;
		const XK_abrevehook = 0x0100_1eb3;
		const XK_Abrevetilde = 0x0100_1eb4;
		const XK_abrevetilde = 0x0100_1eb5;
		const XK_Abrevebelowdot = 0x0100_1eb6;
		const XK_abrevebelowdot = 0x0100_1eb7;
		const XK_Ebelowdot = 0x0100_1eb8;
		const XK_ebelowdot = 0x0100_1eb9;
		const XK_Ehook = 0x0100_1eba;
		const XK_ehook = 0x0100_1ebb;
		const XK_Etilde = 0x0100_1ebc;
		const XK_etilde = 0x0100_1ebd;
		const XK_Ecircumflexacute = 0x0100_1ebe;
		const XK_ecircumflexacute = 0x0100_1ebf;
		const XK_Ecircumflexgrave = 0x0100_1ec0;
		const XK_ecircumflexgrave = 0x0100_1ec1;
		const XK_Ecircumflexhook = 0x0100_1ec2;
		const XK_ecircumflexhook = 0x0100_1ec3;
		const XK_Ecircumflextilde = 0x0100_1ec4;
		const XK_ecircumflextilde = 0x0100_1ec5;
		const XK_Ecircumflexbelowdot = 0x0100_1ec6;
		const XK_ecircumflexbelowdot = 0x0100_1ec7;
		const XK_Ihook = 0x0100_1ec8;
		const XK_ihook = 0x0100_1ec9;
		const XK_Ibelowdot = 0x0100_1eca;
		const XK_ibelowdot = 0x0100_1ecb;
		const XK_Obelowdot = 0x0100_1ecc;
		const XK_obelowdot = 0x0100_1ecd;
		const XK_Ohook = 0x0100_1ece;
		const XK_ohook = 0x0100_1ecf;
		const XK_Ocircumflexacute = 0x0100_1ed0;
		const XK_ocircumflexacute = 0x0100_1ed1;
		const XK_Ocircumflexgrave = 0x0100_1ed2;
		const XK_ocircumflexgrave = 0x0100_1ed3;
		const XK_Ocircumflexhook = 0x0100_1ed4;
		const XK_ocircumflexhook = 0x0100_1ed5;
		const XK_Ocircumflextilde = 0x0100_1ed6;
		const XK_ocircumflextilde = 0x0100_1ed7;
		const XK_Ocircumflexbelowdot = 0x0100_1ed8;
		const XK_ocircumflexbelowdot = 0x0100_1ed9;
		const XK_Ohornacute = 0x0100_1eda;
		const XK_ohornacute = 0x0100_1edb;
		const XK_Ohorngrave = 0x0100_1edc;
		const XK_ohorngrave = 0x0100_1edd;
		const XK_Ohornhook = 0x0100_1ede;
		const XK_ohornhook = 0x0100_1edf;
		const XK_Ohorntilde = 0x0100_1ee0;
		const XK_ohorntilde = 0x0100_1ee1;
		const XK_Ohornbelowdot = 0x0100_1ee2;
		const XK_ohornbelowdot = 0x0100_1ee3;
		const XK_Ubelowdot = 0x0100_1ee4;
		const XK_ubelowdot = 0x0100_1ee5;
		const XK_Uhook = 0x0100_1ee6;
		const XK_uhook = 0x0100_1ee7;
		const XK_Uhornacute = 0x0100_1ee8;
		const XK_uhornacute = 0x0100_1ee9;
		const XK_Uhorngrave = 0x0100_1eea;
		const XK_uhorngrave = 0x0100_1eeb;
		const XK_Uhornhook = 0x0100_1eec;
		const XK_uhornhook = 0x0100_1eed;
		const XK_Uhorntilde = 0x0100_1eee;
		const XK_uhorntilde = 0x0100_1eef;
		const XK_Uhornbelowdot = 0x0100_1ef0;
		const XK_uhornbelowdot = 0x0100_1ef1;
		const XK_Ybelowdot = 0x0100_1ef4;
		const XK_ybelowdot = 0x0100_1ef5;
		const XK_Yhook = 0x0100_1ef6;
		const XK_yhook = 0x0100_1ef7;
		const XK_Ytilde = 0x0100_1ef8;
		const XK_ytilde = 0x0100_1ef9;
		const XK_Ohorn = 0x0100_01a0;
		const XK_ohorn = 0x0100_01a1;
		const XK_Uhorn = 0x0100_01af;
		const XK_uhorn = 0x0100_01b0;

		const XK_EcuSign = 0x0100_20a0;
		const XK_ColonSign = 0x0100_20a1;
		const XK_CruzeiroSign = 0x0100_20a2;
		const XK_FFrancSign = 0x0100_20a3;
		const XK_LiraSign = 0x0100_20a4;
		const XK_MillSign = 0x0100_20a5;
		const XK_NairaSign = 0x0100_20a6;
		const XK_PesetaSign = 0x0100_20a7;
		const XK_RupeeSign = 0x0100_20a8;
		const XK_WonSign = 0x0100_20a9;
		const XK_NewSheqelSign = 0x0100_20aa;
		const XK_DongSign = 0x0100_20ab;
		const XK_EuroSign = 0x20ac;

		const XK_zerosuperior = 0x0100_2070;
		const XK_foursuperior = 0x0100_2074;
		const XK_fivesuperior = 0x0100_2075;
		const XK_sixsuperior = 0x0100_2076;
		const XK_sevensuperior = 0x0100_2077;
		const XK_eightsuperior = 0x0100_2078;
		const XK_ninesuperior = 0x0100_2079;
		const XK_zerosubscript = 0x0100_2080;
		const XK_onesubscript = 0x0100_2081;
		const XK_twosubscript = 0x0100_2082;
		const XK_threesubscript = 0x0100_2083;
		const XK_foursubscript = 0x0100_2084;
		const XK_fivesubscript = 0x0100_2085;
		const XK_sixsubscript = 0x0100_2086;
		const XK_sevensubscript = 0x0100_2087;
		const XK_eightsubscript = 0x0100_2088;
		const XK_ninesubscript = 0x0100_2089;
		const XK_partdifferential = 0x0100_2202;
		const XK_emptyset = 0x0100_2205;
		const XK_elementof = 0x0100_2208;
		const XK_notelementof = 0x0100_2209;
		const XK_containsas = 0x0100_220B;
		const XK_squareroot = 0x0100_221A;
		const XK_cuberoot = 0x0100_221B;
		const XK_fourthroot = 0x0100_221C;
		const XK_dintegral = 0x0100_222C;
		const XK_tintegral = 0x0100_222D;
		const XK_because = 0x0100_2235;
		const XK_approxeq = 0x0100_2248;
		const XK_notapproxeq = 0x0100_2247;
		const XK_notidentical = 0x0100_2262;
		const XK_stricteq = 0x0100_2263;

		const XK_braille_dot_1 = 0xfff1;
		const XK_braille_dot_2 = 0xfff2;
		const XK_braille_dot_3 = 0xfff3;
		const XK_braille_dot_4 = 0xfff4;
		const XK_braille_dot_5 = 0xfff5;
		const XK_braille_dot_6 = 0xfff6;
		const XK_braille_dot_7 = 0xfff7;
		const XK_braille_dot_8 = 0xfff8;
		const XK_braille_dot_9 = 0xfff9;
		const XK_braille_dot_10 = 0xfffa;
		const XK_braille_blank = 0x0100_2800;
		const XK_braille_dots_1 = 0x0100_2801;
		const XK_braille_dots_2 = 0x0100_2802;
		const XK_braille_dots_12 = 0x0100_2803;
		const XK_braille_dots_3 = 0x0100_2804;
		const XK_braille_dots_13 = 0x0100_2805;
		const XK_braille_dots_23 = 0x0100_2806;
		const XK_braille_dots_123 = 0x0100_2807;
		const XK_braille_dots_4 = 0x0100_2808;
		const XK_braille_dots_14 = 0x0100_2809;
		const XK_braille_dots_24 = 0x0100_280a;
		const XK_braille_dots_124 = 0x0100_280b;
		const XK_braille_dots_34 = 0x0100_280c;
		const XK_braille_dots_134 = 0x0100_280d;
		const XK_braille_dots_234 = 0x0100_280e;
		const XK_braille_dots_1234 = 0x0100_280f;
		const XK_braille_dots_5 = 0x0100_2810;
		const XK_braille_dots_15 = 0x0100_2811;
		const XK_braille_dots_25 = 0x0100_2812;
		const XK_braille_dots_125 = 0x0100_2813;
		const XK_braille_dots_35 = 0x0100_2814;
		const XK_braille_dots_135 = 0x0100_2815;
		const XK_braille_dots_235 = 0x0100_2816;
		const XK_braille_dots_1235 = 0x0100_2817;
		const XK_braille_dots_45 = 0x0100_2818;
		const XK_braille_dots_145 = 0x0100_2819;
		const XK_braille_dots_245 = 0x0100_281a;
		const XK_braille_dots_1245 = 0x0100_281b;
		const XK_braille_dots_345 = 0x0100_281c;
		const XK_braille_dots_1345 = 0x0100_281d;
		const XK_braille_dots_2345 = 0x0100_281e;
		const XK_braille_dots_12345 = 0x0100_281f;
		const XK_braille_dots_6 = 0x0100_2820;
		const XK_braille_dots_16 = 0x0100_2821;
		const XK_braille_dots_26 = 0x0100_2822;
		const XK_braille_dots_126 = 0x0100_2823;
		const XK_braille_dots_36 = 0x0100_2824;
		const XK_braille_dots_136 = 0x0100_2825;
		const XK_braille_dots_236 = 0x0100_2826;
		const XK_braille_dots_1236 = 0x0100_2827;
		const XK_braille_dots_46 = 0x0100_2828;
		const XK_braille_dots_146 = 0x0100_2829;
		const XK_braille_dots_246 = 0x0100_282a;
		const XK_braille_dots_1246 = 0x0100_282b;
		const XK_braille_dots_346 = 0x0100_282c;
		const XK_braille_dots_1346 = 0x0100_282d;
		const XK_braille_dots_2346 = 0x0100_282e;
		const XK_braille_dots_12346 = 0x0100_282f;
		const XK_braille_dots_56 = 0x0100_2830;
		const XK_braille_dots_156 = 0x0100_2831;
		const XK_braille_dots_256 = 0x0100_2832;
		const XK_braille_dots_1256 = 0x0100_2833;
		const XK_braille_dots_356 = 0x0100_2834;
		const XK_braille_dots_1356 = 0x0100_2835;
		const XK_braille_dots_2356 = 0x0100_2836;
		const XK_braille_dots_12356 = 0x0100_2837;
		const XK_braille_dots_456 = 0x0100_2838;
		const XK_braille_dots_1456 = 0x0100_2839;
		const XK_braille_dots_2456 = 0x0100_283a;
		const XK_braille_dots_12456 = 0x0100_283b;
		const XK_braille_dots_3456 = 0x0100_283c;
		const XK_braille_dots_13456 = 0x0100_283d;
		const XK_braille_dots_23456 = 0x0100_283e;
		const XK_braille_dots_123456 = 0x0100_283f;
		const XK_braille_dots_7 = 0x0100_2840;
		const XK_braille_dots_17 = 0x0100_2841;
		const XK_braille_dots_27 = 0x0100_2842;
		const XK_braille_dots_127 = 0x0100_2843;
		const XK_braille_dots_37 = 0x0100_2844;
		const XK_braille_dots_137 = 0x0100_2845;
		const XK_braille_dots_237 = 0x0100_2846;
		const XK_braille_dots_1237 = 0x0100_2847;
		const XK_braille_dots_47 = 0x0100_2848;
		const XK_braille_dots_147 = 0x0100_2849;
		const XK_braille_dots_247 = 0x0100_284a;
		const XK_braille_dots_1247 = 0x0100_284b;
		const XK_braille_dots_347 = 0x0100_284c;
		const XK_braille_dots_1347 = 0x0100_284d;
		const XK_braille_dots_2347 = 0x0100_284e;
		const XK_braille_dots_12347 = 0x0100_284f;
		const XK_braille_dots_57 = 0x0100_2850;
		const XK_braille_dots_157 = 0x0100_2851;
		const XK_braille_dots_257 = 0x0100_2852;
		const XK_braille_dots_1257 = 0x0100_2853;
		const XK_braille_dots_357 = 0x0100_2854;
		const XK_braille_dots_1357 = 0x0100_2855;
		const XK_braille_dots_2357 = 0x0100_2856;
		const XK_braille_dots_12357 = 0x0100_2857;
		const XK_braille_dots_457 = 0x0100_2858;
		const XK_braille_dots_1457 = 0x0100_2859;
		const XK_braille_dots_2457 = 0x0100_285a;
		const XK_braille_dots_12457 = 0x0100_285b;
		const XK_braille_dots_3457 = 0x0100_285c;
		const XK_braille_dots_13457 = 0x0100_285d;
		const XK_braille_dots_23457 = 0x0100_285e;
		const XK_braille_dots_123457 = 0x0100_285f;
		const XK_braille_dots_67 = 0x0100_2860;
		const XK_braille_dots_167 = 0x0100_2861;
		const XK_braille_dots_267 = 0x0100_2862;
		const XK_braille_dots_1267 = 0x0100_2863;
		const XK_braille_dots_367 = 0x0100_2864;
		const XK_braille_dots_1367 = 0x0100_2865;
		const XK_braille_dots_2367 = 0x0100_2866;
		const XK_braille_dots_12367 = 0x0100_2867;
		const XK_braille_dots_467 = 0x0100_2868;
		const XK_braille_dots_1467 = 0x0100_2869;
		const XK_braille_dots_2467 = 0x0100_286a;
		const XK_braille_dots_12467 = 0x0100_286b;
		const XK_braille_dots_3467 = 0x0100_286c;
		const XK_braille_dots_13467 = 0x0100_286d;
		const XK_braille_dots_23467 = 0x0100_286e;
		const XK_braille_dots_123467 = 0x0100_286f;
		const XK_braille_dots_567 = 0x0100_2870;
		const XK_braille_dots_1567 = 0x0100_2871;
		const XK_braille_dots_2567 = 0x0100_2872;
		const XK_braille_dots_12567 = 0x0100_2873;
		const XK_braille_dots_3567 = 0x0100_2874;
		const XK_braille_dots_13567 = 0x0100_2875;
		const XK_braille_dots_23567 = 0x0100_2876;
		const XK_braille_dots_123567 = 0x0100_2877;
		const XK_braille_dots_4567 = 0x0100_2878;
		const XK_braille_dots_14567 = 0x0100_2879;
		const XK_braille_dots_24567 = 0x0100_287a;
		const XK_braille_dots_124567 = 0x0100_287b;
		const XK_braille_dots_34567 = 0x0100_287c;
		const XK_braille_dots_134567 = 0x0100_287d;
		const XK_braille_dots_234567 = 0x0100_287e;
		const XK_braille_dots_1234567 = 0x0100_287f;
		const XK_braille_dots_8 = 0x0100_2880;
		const XK_braille_dots_18 = 0x0100_2881;
		const XK_braille_dots_28 = 0x0100_2882;
		const XK_braille_dots_128 = 0x0100_2883;
		const XK_braille_dots_38 = 0x0100_2884;
		const XK_braille_dots_138 = 0x0100_2885;
		const XK_braille_dots_238 = 0x0100_2886;
		const XK_braille_dots_1238 = 0x0100_2887;
		const XK_braille_dots_48 = 0x0100_2888;
		const XK_braille_dots_148 = 0x0100_2889;
		const XK_braille_dots_248 = 0x0100_288a;
		const XK_braille_dots_1248 = 0x0100_288b;
		const XK_braille_dots_348 = 0x0100_288c;
		const XK_braille_dots_1348 = 0x0100_288d;
		const XK_braille_dots_2348 = 0x0100_288e;
		const XK_braille_dots_12348 = 0x0100_288f;
		const XK_braille_dots_58 = 0x0100_2890;
		const XK_braille_dots_158 = 0x0100_2891;
		const XK_braille_dots_258 = 0x0100_2892;
		const XK_braille_dots_1258 = 0x0100_2893;
		const XK_braille_dots_358 = 0x0100_2894;
		const XK_braille_dots_1358 = 0x0100_2895;
		const XK_braille_dots_2358 = 0x0100_2896;
		const XK_braille_dots_12358 = 0x0100_2897;
		const XK_braille_dots_458 = 0x0100_2898;
		const XK_braille_dots_1458 = 0x0100_2899;
		const XK_braille_dots_2458 = 0x0100_289a;
		const XK_braille_dots_12458 = 0x0100_289b;
		const XK_braille_dots_3458 = 0x0100_289c;
		const XK_braille_dots_13458 = 0x0100_289d;
		const XK_braille_dots_23458 = 0x0100_289e;
		const XK_braille_dots_123458 = 0x0100_289f;
		const XK_braille_dots_68 = 0x0100_28a0;
		const XK_braille_dots_168 = 0x0100_28a1;
		const XK_braille_dots_268 = 0x0100_28a2;
		const XK_braille_dots_1268 = 0x0100_28a3;
		const XK_braille_dots_368 = 0x0100_28a4;
		const XK_braille_dots_1368 = 0x0100_28a5;
		const XK_braille_dots_2368 = 0x0100_28a6;
		const XK_braille_dots_12368 = 0x0100_28a7;
		const XK_braille_dots_468 = 0x0100_28a8;
		const XK_braille_dots_1468 = 0x0100_28a9;
		const XK_braille_dots_2468 = 0x0100_28aa;
		const XK_braille_dots_12468 = 0x0100_28ab;
		const XK_braille_dots_3468 = 0x0100_28ac;
		const XK_braille_dots_13468 = 0x0100_28ad;
		const XK_braille_dots_23468 = 0x0100_28ae;
		const XK_braille_dots_123468 = 0x0100_28af;
		const XK_braille_dots_568 = 0x0100_28b0;
		const XK_braille_dots_1568 = 0x0100_28b1;
		const XK_braille_dots_2568 = 0x0100_28b2;
		const XK_braille_dots_12568 = 0x0100_28b3;
		const XK_braille_dots_3568 = 0x0100_28b4;
		const XK_braille_dots_13568 = 0x0100_28b5;
		const XK_braille_dots_23568 = 0x0100_28b6;
		const XK_braille_dots_123568 = 0x0100_28b7;
		const XK_braille_dots_4568 = 0x0100_28b8;
		const XK_braille_dots_14568 = 0x0100_28b9;
		const XK_braille_dots_24568 = 0x0100_28ba;
		const XK_braille_dots_124568 = 0x0100_28bb;
		const XK_braille_dots_34568 = 0x0100_28bc;
		const XK_braille_dots_134568 = 0x0100_28bd;
		const XK_braille_dots_234568 = 0x0100_28be;
		const XK_braille_dots_1234568 = 0x0100_28bf;
		const XK_braille_dots_78 = 0x0100_28c0;
		const XK_braille_dots_178 = 0x0100_28c1;
		const XK_braille_dots_278 = 0x0100_28c2;
		const XK_braille_dots_1278 = 0x0100_28c3;
		const XK_braille_dots_378 = 0x0100_28c4;
		const XK_braille_dots_1378 = 0x0100_28c5;
		const XK_braille_dots_2378 = 0x0100_28c6;
		const XK_braille_dots_12378 = 0x0100_28c7;
		const XK_braille_dots_478 = 0x0100_28c8;
		const XK_braille_dots_1478 = 0x0100_28c9;
		const XK_braille_dots_2478 = 0x0100_28ca;
		const XK_braille_dots_12478 = 0x0100_28cb;
		const XK_braille_dots_3478 = 0x0100_28cc;
		const XK_braille_dots_13478 = 0x0100_28cd;
		const XK_braille_dots_23478 = 0x0100_28ce;
		const XK_braille_dots_123478 = 0x0100_28cf;
		const XK_braille_dots_578 = 0x0100_28d0;
		const XK_braille_dots_1578 = 0x0100_28d1;
		const XK_braille_dots_2578 = 0x0100_28d2;
		const XK_braille_dots_12578 = 0x0100_28d3;
		const XK_braille_dots_3578 = 0x0100_28d4;
		const XK_braille_dots_13578 = 0x0100_28d5;
		const XK_braille_dots_23578 = 0x0100_28d6;
		const XK_braille_dots_123578 = 0x0100_28d7;
		const XK_braille_dots_4578 = 0x0100_28d8;
		const XK_braille_dots_14578 = 0x0100_28d9;
		const XK_braille_dots_24578 = 0x0100_28da;
		const XK_braille_dots_124578 = 0x0100_28db;
		const XK_braille_dots_34578 = 0x0100_28dc;
		const XK_braille_dots_134578 = 0x0100_28dd;
		const XK_braille_dots_234578 = 0x0100_28de;
		const XK_braille_dots_1234578 = 0x0100_28df;
		const XK_braille_dots_678 = 0x0100_28e0;
		const XK_braille_dots_1678 = 0x0100_28e1;
		const XK_braille_dots_2678 = 0x0100_28e2;
		const XK_braille_dots_12678 = 0x0100_28e3;
		const XK_braille_dots_3678 = 0x0100_28e4;
		const XK_braille_dots_13678 = 0x0100_28e5;
		const XK_braille_dots_23678 = 0x0100_28e6;
		const XK_braille_dots_123678 = 0x0100_28e7;
		const XK_braille_dots_4678 = 0x0100_28e8;
		const XK_braille_dots_14678 = 0x0100_28e9;
		const XK_braille_dots_24678 = 0x0100_28ea;
		const XK_braille_dots_124678 = 0x0100_28eb;
		const XK_braille_dots_34678 = 0x0100_28ec;
		const XK_braille_dots_134678 = 0x0100_28ed;
		const XK_braille_dots_234678 = 0x0100_28ee;
		const XK_braille_dots_1234678 = 0x0100_28ef;
		const XK_braille_dots_5678 = 0x0100_28f0;
		const XK_braille_dots_15678 = 0x0100_28f1;
		const XK_braille_dots_25678 = 0x0100_28f2;
		const XK_braille_dots_125678 = 0x0100_28f3;
		const XK_braille_dots_35678 = 0x0100_28f4;
		const XK_braille_dots_135678 = 0x0100_28f5;
		const XK_braille_dots_235678 = 0x0100_28f6;
		const XK_braille_dots_1235678 = 0x0100_28f7;
		const XK_braille_dots_45678 = 0x0100_28f8;
		const XK_braille_dots_145678 = 0x0100_28f9;
		const XK_braille_dots_245678 = 0x0100_28fa;
		const XK_braille_dots_1245678 = 0x0100_28fb;
		const XK_braille_dots_345678 = 0x0100_28fc;
		const XK_braille_dots_1345678 = 0x0100_28fd;
		const XK_braille_dots_2345678 = 0x0100_28fe;
		const XK_braille_dots_12345678 = 0x0100_28ff;


		const XK_Sinh_ng = 0x0100_0d82;
		const XK_Sinh_h2 = 0x0100_0d83;
		const XK_Sinh_a = 0x0100_0d85;
		const XK_Sinh_aa = 0x0100_0d86;
		const XK_Sinh_ae = 0x0100_0d87;
		const XK_Sinh_aee = 0x0100_0d88;
		const XK_Sinh_i = 0x0100_0d89;
		const XK_Sinh_ii = 0x0100_0d8a;
		const XK_Sinh_u = 0x0100_0d8b;
		const XK_Sinh_uu = 0x0100_0d8c;
		const XK_Sinh_ri = 0x0100_0d8d;
		const XK_Sinh_rii = 0x0100_0d8e;
		const XK_Sinh_lu = 0x0100_0d8f;
		const XK_Sinh_luu = 0x0100_0d90;
		const XK_Sinh_e = 0x0100_0d91;
		const XK_Sinh_ee = 0x0100_0d92;
		const XK_Sinh_ai = 0x0100_0d93;
		const XK_Sinh_o = 0x0100_0d94;
		const XK_Sinh_oo = 0x0100_0d95;
		const XK_Sinh_au = 0x0100_0d96;
		const XK_Sinh_ka = 0x0100_0d9a;
		const XK_Sinh_kha = 0x0100_0d9b;
		const XK_Sinh_ga = 0x0100_0d9c;
		const XK_Sinh_gha = 0x0100_0d9d;
		const XK_Sinh_ng2 = 0x0100_0d9e;
		const XK_Sinh_nga = 0x0100_0d9f;
		const XK_Sinh_ca = 0x0100_0da0;
		const XK_Sinh_cha = 0x0100_0da1;
		const XK_Sinh_ja = 0x0100_0da2;
		const XK_Sinh_jha = 0x0100_0da3;
		const XK_Sinh_nya = 0x0100_0da4;
		const XK_Sinh_jnya = 0x0100_0da5;
		const XK_Sinh_nja = 0x0100_0da6;
		const XK_Sinh_tta = 0x0100_0da7;
		const XK_Sinh_ttha = 0x0100_0da8;
		const XK_Sinh_dda = 0x0100_0da9;
		const XK_Sinh_ddha = 0x0100_0daa;
		const XK_Sinh_nna = 0x0100_0dab;
		const XK_Sinh_ndda = 0x0100_0dac;
		const XK_Sinh_tha = 0x0100_0dad;
		const XK_Sinh_thha = 0x0100_0dae;
		const XK_Sinh_dha = 0x0100_0daf;
		const XK_Sinh_dhha = 0x0100_0db0;
		const XK_Sinh_na = 0x0100_0db1;
		const XK_Sinh_ndha = 0x0100_0db3;
		const XK_Sinh_pa = 0x0100_0db4;
		const XK_Sinh_pha = 0x0100_0db5;
		const XK_Sinh_ba = 0x0100_0db6;
		const XK_Sinh_bha = 0x0100_0db7;
		const XK_Sinh_ma = 0x0100_0db8;
		const XK_Sinh_mba = 0x0100_0db9;
		const XK_Sinh_ya = 0x0100_0dba;
		const XK_Sinh_ra = 0x0100_0dbb;
		const XK_Sinh_la = 0x0100_0dbd;
		const XK_Sinh_va = 0x0100_0dc0;
		const XK_Sinh_sha = 0x0100_0dc1;
		const XK_Sinh_ssha = 0x0100_0dc2;
		const XK_Sinh_sa = 0x0100_0dc3;
		const XK_Sinh_ha = 0x0100_0dc4;
		const XK_Sinh_lla = 0x0100_0dc5;
		const XK_Sinh_fa = 0x0100_0dc6;
		const XK_Sinh_al = 0x0100_0dca;
		const XK_Sinh_aa2 = 0x0100_0dcf;
		const XK_Sinh_ae2 = 0x0100_0dd0;
		const XK_Sinh_aee2 = 0x0100_0dd1;
		const XK_Sinh_i2 = 0x0100_0dd2;
		const XK_Sinh_ii2 = 0x0100_0dd3;
		const XK_Sinh_u2 = 0x0100_0dd4;
		const XK_Sinh_uu2 = 0x0100_0dd6;
		const XK_Sinh_ru2 = 0x0100_0dd8;
		const XK_Sinh_e2 = 0x0100_0dd9;
		const XK_Sinh_ee2 = 0x0100_0dda;
		const XK_Sinh_ai2 = 0x0100_0ddb;
		const XK_Sinh_au2 = 0x0100_0dde;
		const XK_Sinh_lu2 = 0x0100_0ddf;
		const XK_Sinh_ruu2 = 0x0100_0df2;
		const XK_Sinh_luu2 = 0x0100_0df3;
		const XK_Sinh_kunddaliya = 0x0100_0df4;


		const XK_XF86ModeLock = 0x1008_FF01;


		const XK_XF86MonBrightnessUp = 0x1008_FF02;
		const XK_XF86MonBrightnessDown = 0x1008_FF03;
		const XK_XF86KbdLightOnOff = 0x1008_FF04;
		const XK_XF86KbdBrightnessUp = 0x1008_FF05;
		const XK_XF86KbdBrightnessDown = 0x1008_FF06;

		const XK_XF86Standby = 0x1008_FF10;
		const XK_XF86AudioLowerVolume = 0x1008_FF11;
		const XK_XF86AudioMute = 0x1008_FF12;
		const XK_XF86AudioRaiseVolume = 0x1008_FF13;
		const XK_XF86AudioPlay = 0x1008_FF14;
		const XK_XF86AudioStop = 0x1008_FF15;
		const XK_XF86AudioPrev = 0x1008_FF16;
		const XK_XF86AudioNext = 0x1008_FF17;
		const XK_XF86HomePage = 0x1008_FF18;
		const XK_XF86Mail = 0x1008_FF19;
		const XK_XF86Start = 0x1008_FF1A;
		const XK_XF86Search = 0x1008_FF1B;
		const XK_XF86AudioRecord = 0x1008_FF1C;

		const XK_XF86Calculator = 0x1008_FF1D;
		const XK_XF86Memo = 0x1008_FF1E;
		const XK_XF86ToDoList = 0x1008_FF1F;
		const XK_XF86Calendar = 0x1008_FF20;
		const XK_XF86PowerDown = 0x1008_FF21;
		const XK_XF86ContrastAdjust = 0x1008_FF22;
		const XK_XF86RockerUp = 0x1008_FF23;
		const XK_XF86RockerDown = 0x1008_FF24;
		const XK_XF86RockerEnter = 0x1008_FF25;

		const XK_XF86Back = 0x1008_FF26;
		const XK_XF86Forward = 0x1008_FF27;
		const XK_XF86Stop = 0x1008_FF28;
		const XK_XF86Refresh = 0x1008_FF29;
		const XK_XF86PowerOff = 0x1008_FF2A;
		const XK_XF86WakeUp = 0x1008_FF2B;
		const XK_XF86Eject = 0x1008_FF2C;
		const XK_XF86ScreenSaver = 0x1008_FF2D;
		const XK_XF86WWW = 0x1008_FF2E;
		const XK_XF86Sleep = 0x1008_FF2F;
		const XK_XF86Favorites = 0x1008_FF30;
		const XK_XF86AudioPause = 0x1008_FF31;
		const XK_XF86AudioMedia = 0x1008_FF32;
		const XK_XF86MyComputer = 0x1008_FF33;
		const XK_XF86VendorHome = 0x1008_FF34;
		const XK_XF86LightBulb = 0x1008_FF35;
		const XK_XF86Shop = 0x1008_FF36;
		const XK_XF86History = 0x1008_FF37;
		const XK_XF86OpenURL = 0x1008_FF38;
		const XK_XF86AddFavorite = 0x1008_FF39;
		const XK_XF86HotLinks = 0x1008_FF3A;
		const XK_XF86BrightnessAdjust = 0x1008_FF3B;
		const XK_XF86Finance = 0x1008_FF3C;
		const XK_XF86Community = 0x1008_FF3D;
		const XK_XF86AudioRewind = 0x1008_FF3E;
		const XK_XF86BackForward = 0x1008_FF3F;
		const XK_XF86Launch0 = 0x1008_FF40;
		const XK_XF86Launch1 = 0x1008_FF41;
		const XK_XF86Launch2 = 0x1008_FF42;
		const XK_XF86Launch3 = 0x1008_FF43;
		const XK_XF86Launch4 = 0x1008_FF44;
		const XK_XF86Launch5 = 0x1008_FF45;
		const XK_XF86Launch6 = 0x1008_FF46;
		const XK_XF86Launch7 = 0x1008_FF47;
		const XK_XF86Launch8 = 0x1008_FF48;
		const XK_XF86Launch9 = 0x1008_FF49;
		const XK_XF86LaunchA = 0x1008_FF4A;
		const XK_XF86LaunchB = 0x1008_FF4B;
		const XK_XF86LaunchC = 0x1008_FF4C;
		const XK_XF86LaunchD = 0x1008_FF4D;
		const XK_XF86LaunchE = 0x1008_FF4E;
		const XK_XF86LaunchF = 0x1008_FF4F;

		const XK_XF86ApplicationLeft = 0x1008_FF50;
		const XK_XF86Book = 0x1008_FF52;
		const XK_XF86CD = 0x1008_FF53;
		const XK_XF86Calculater = 0x1008_FF54;
		const XK_XF86Clear = 0x1008_FF55;
		const XK_XF86Close = 0x1008_FF56;
		const XK_XF86Copy = 0x1008_FF57;
		const XK_XF86Cut = 0x1008_FF58;
		const XK_XF86Display = 0x1008_FF59;
		const XK_XF86DOS = 0x1008_FF5A;
		const XK_XF86Documents = 0x1008_FF5B;
		const XK_XF86Excel = 0x1008_FF5C;
		const XK_XF86Explorer = 0x1008_FF5D;
		const XK_XF86Game = 0x1008_FF5E;
		const XK_XF86Go = 0x1008_FF5F;
		const XK_XF86iTouch = 0x1008_FF60;
		const XK_XF86LogOff = 0x1008_FF61;
		const XK_XF86Market = 0x1008_FF62;
		const XK_XF86Meeting = 0x1008_FF63;
		const XK_XF86MenuKB = 0x1008_FF65;
		const XK_XF86MenuPB = 0x1008_FF66;
		const XK_XF86MySites = 0x1008_FF67;
		const XK_XF86New = 0x1008_FF68;
		const XK_XF86News = 0x1008_FF69;
		const XK_XF86Open = 0x1008_FF6B;
		const XK_XF86Option = 0x1008_FF6C;
		const XK_XF86Paste = 0x1008_FF6D;
		const XK_XF86Phone = 0x1008_FF6E;
		const XK_XF86Q = 0x1008_FF70;
		const XK_XF86Reply = 0x1008_FF72;
		const XK_XF86Reload = 0x1008_FF73;
		const XK_XF86RotateWindows = 0x1008_FF74;
		const XK_XF86RotationPB = 0x1008_FF75;
		const XK_XF86RotationKB = 0x1008_FF76;
		const XK_XF86Save = 0x1008_FF77;
		const XK_XF86ScrollUp = 0x1008_FF78;
		const XK_XF86ScrollDown = 0x1008_FF79;
		const XK_XF86ScrollClick = 0x1008_FF7A;
		const XK_XF86Send = 0x1008_FF7B;
		const XK_XF86Spell = 0x1008_FF7C;
		const XK_XF86SplitScreen = 0x1008_FF7D;
		const XK_XF86Support = 0x1008_FF7E;
		const XK_XF86TaskPane = 0x1008_FF7F;
		const XK_XF86Terminal = 0x1008_FF80;
		const XK_XF86Tools = 0x1008_FF81;
		const XK_XF86Travel = 0x1008_FF82;
		const XK_XF86UserPB = 0x1008_FF84;
		const XK_XF86User1KB = 0x1008_FF85;
		const XK_XF86User2KB = 0x1008_FF86;
		const XK_XF86Video = 0x1008_FF87;
		const XK_XF86WheelButton = 0x1008_FF88;
		const XK_XF86Word = 0x1008_FF89;
		const XK_XF86Xfer = 0x1008_FF8A;
		const XK_XF86ZoomIn = 0x1008_FF8B;
		const XK_XF86ZoomOut = 0x1008_FF8C;

		const XK_XF86Away = 0x1008_FF8D;
		const XK_XF86Messenger = 0x1008_FF8E;
		const XK_XF86WebCam = 0x1008_FF8F;
		const XK_XF86MailForward = 0x1008_FF90;
		const XK_XF86Pictures = 0x1008_FF91;
		const XK_XF86Music = 0x1008_FF92;

		const XK_XF86Battery = 0x1008_FF93;
		const XK_XF86Bluetooth = 0x1008_FF94;
		const XK_XF86WLAN = 0x1008_FF95;
		const XK_XF86UWB = 0x1008_FF96;

		const XK_XF86AudioForward = 0x1008_FF97;
		const XK_XF86AudioRepeat = 0x1008_FF98;
		const XK_XF86AudioRandomPlay = 0x1008_FF99;
		const XK_XF86Subtitle = 0x1008_FF9A;
		const XK_XF86AudioCycleTrack = 0x1008_FF9B;
		const XK_XF86CycleAngle = 0x1008_FF9C;
		const XK_XF86FrameBack = 0x1008_FF9D;
		const XK_XF86FrameForward = 0x1008_FF9E;
		const XK_XF86Time = 0x1008_FF9F;
		const XK_XF86Select = 0x1008_FFA0;
		const XK_XF86View = 0x1008_FFA1;
		const XK_XF86TopMenu = 0x1008_FFA2;

		const XK_XF86Red = 0x1008_FFA3;
		const XK_XF86Green = 0x1008_FFA4;
		const XK_XF86Yellow = 0x1008_FFA5;
		const XK_XF86Blue = 0x1008_FFA6;

		const XK_XF86Suspend = 0x1008_FFA7;
		const XK_XF86Hibernate = 0x1008_FFA8;
		const XK_XF86TouchpadToggle = 0x1008_FFA9;
		const XK_XF86TouchpadOn = 0x1008_FFB0;
		const XK_XF86TouchpadOff = 0x1008_FFB1;

		const XK_XF86AudioMicMute = 0x1008_FFB2;

		const XK_XF86Switch_VT_1 = 0x1008_FE01;
		const XK_XF86Switch_VT_2 = 0x1008_FE02;
		const XK_XF86Switch_VT_3 = 0x1008_FE03;
		const XK_XF86Switch_VT_4 = 0x1008_FE04;
		const XK_XF86Switch_VT_5 = 0x1008_FE05;
		const XK_XF86Switch_VT_6 = 0x1008_FE06;
		const XK_XF86Switch_VT_7 = 0x1008_FE07;
		const XK_XF86Switch_VT_8 = 0x1008_FE08;
		const XK_XF86Switch_VT_9 = 0x1008_FE09;
		const XK_XF86Switch_VT_10 = 0x1008_FE0A;
		const XK_XF86Switch_VT_11 = 0x1008_FE0B;
		const XK_XF86Switch_VT_12 = 0x1008_FE0C;

		const XK_XF86Ungrab = 0x1008_FE20;
		const XK_XF86ClearGrab = 0x1008_FE21;
		const XK_XF86Next_VMode = 0x1008_FE22;
		const XK_XF86Prev_VMode = 0x1008_FE23;
		const XK_XF86LogWindowTree = 0x1008_FE24;
		const XK_XF86LogGrabInfo = 0x1008_FE25;



		const XK_SunFA_Grave = 0x1005_FF00;
		const XK_SunFA_Circum = 0x1005_FF01;
		const XK_SunFA_Tilde = 0x1005_FF02;
		const XK_SunFA_Acute = 0x1005_FF03;
		const XK_SunFA_Diaeresis = 0x1005_FF04;
		const XK_SunFA_Cedilla = 0x1005_FF05;


		const XK_SunF36 = 0x1005_FF10;
		const XK_SunF37 = 0x1005_FF11;

		const XK_SunSys_Req = 0x1005_FF60;
		const XK_SunPrint_Screen = 0x0000_FF61;


		const XK_SunCompose = 0x0000_FF20;
		const XK_SunAltGraph = 0x0000_FF7E;


		const XK_SunPageUp = 0x0000_FF55;
		const XK_SunPageDown = 0x0000_FF56;


		const XK_SunUndo = 0x0000_FF65;
		const XK_SunAgain = 0x0000_FF66;
		const XK_SunFind = 0x0000_FF68;
		const XK_SunStop = 0x0000_FF69;
		const XK_SunProps = 0x1005_FF70;
		const XK_SunFront = 0x1005_FF71;
		const XK_SunCopy = 0x1005_FF72;
		const XK_SunOpen = 0x1005_FF73;
		const XK_SunPaste = 0x1005_FF74;
		const XK_SunCut = 0x1005_FF75;

		const XK_SunPowerSwitch = 0x1005_FF76;
		const XK_SunAudioLowerVolume = 0x1005_FF77;
		const XK_SunAudioMute = 0x1005_FF78;
		const XK_SunAudioRaiseVolume = 0x1005_FF79;
		const XK_SunVideoDegauss = 0x1005_FF7A;
		const XK_SunVideoLowerBrightness = 0x1005_FF7B;
		const XK_SunVideoRaiseBrightness = 0x1005_FF7C;
		const XK_SunPowerSwitchShift = 0x1005_FF7D;



		const XK_Dring_accent = 0x1000_FEB0;
		const XK_Dcircumflex_accent = 0x1000_FE5E;
		const XK_Dcedilla_accent = 0x1000_FE2C;
		const XK_Dacute_accent = 0x1000_FE27;
		const XK_Dgrave_accent = 0x1000_FE60;
		const XK_Dtilde = 0x1000_FE7E;
		const XK_Ddiaeresis = 0x1000_FE22;


		const XK_DRemove = 0x1000_FF00;


		const XK_hpClearLine = 0x1000_FF6F;
		const XK_hpInsertLine = 0x1000_FF70;
		const XK_hpDeleteLine = 0x1000_FF71;
		const XK_hpInsertChar = 0x1000_FF72;
		const XK_hpDeleteChar = 0x1000_FF73;
		const XK_hpBackTab = 0x1000_FF74;
		const XK_hpKP_BackTab = 0x1000_FF75;
		const XK_hpModelock1 = 0x1000_FF48;
		const XK_hpModelock2 = 0x1000_FF49;
		const XK_hpReset = 0x1000_FF6C;
		const XK_hpSystem = 0x1000_FF6D;
		const XK_hpUser = 0x1000_FF6E;
		const XK_hpmute_acute = 0x1000_00A8;
		const XK_hpmute_grave = 0x1000_00A9;
		const XK_hpmute_asciicircum = 0x1000_00AA;
		const XK_hpmute_diaeresis = 0x1000_00AB;
		const XK_hpmute_asciitilde = 0x1000_00AC;
		const XK_hplira = 0x1000_00AF;
		const XK_hpguilder = 0x1000_00BE;
		const XK_hpYdiaeresis = 0x1000_00EE;
		const XK_hpIO = 0x1000_00EE;
		const XK_hplongminus = 0x1000_00F6;
		const XK_hpblock = 0x1000_00FC;

		const XK_osfCopy = 0x1004_FF02;
		const XK_osfCut = 0x1004_FF03;
		const XK_osfPaste = 0x1004_FF04;
		const XK_osfBackTab = 0x1004_FF07;
		const XK_osfBackSpace = 0x1004_FF08;
		const XK_osfClear = 0x1004_FF0B;
		const XK_osfEscape = 0x1004_FF1B;
		const XK_osfAddMode = 0x1004_FF31;
		const XK_osfPrimaryPaste = 0x1004_FF32;
		const XK_osfQuickPaste = 0x1004_FF33;
		const XK_osfPageLeft = 0x1004_FF40;
		const XK_osfPageUp = 0x1004_FF41;
		const XK_osfPageDown = 0x1004_FF42;
		const XK_osfPageRight = 0x1004_FF43;
		const XK_osfActivate = 0x1004_FF44;
		const XK_osfMenuBar = 0x1004_FF45;
		const XK_osfLeft = 0x1004_FF51;
		const XK_osfUp = 0x1004_FF52;
		const XK_osfRight = 0x1004_FF53;
		const XK_osfDown = 0x1004_FF54;
		const XK_osfEndLine = 0x1004_FF57;
		const XK_osfBeginLine = 0x1004_FF58;
		const XK_osfEndData = 0x1004_FF59;
		const XK_osfBeginData = 0x1004_FF5A;
		const XK_osfPrevMenu = 0x1004_FF5B;
		const XK_osfNextMenu = 0x1004_FF5C;
		const XK_osfPrevField = 0x1004_FF5D;
		const XK_osfNextField = 0x1004_FF5E;
		const XK_osfSelect = 0x1004_FF60;
		const XK_osfInsert = 0x1004_FF63;
		const XK_osfUndo = 0x1004_FF65;
		const XK_osfMenu = 0x1004_FF67;
		const XK_osfCancel = 0x1004_FF69;
		const XK_osfHelp = 0x1004_FF6A;
		const XK_osfSelectAll = 0x1004_FF71;
		const XK_osfDeselectAll = 0x1004_FF72;
		const XK_osfReselect = 0x1004_FF73;
		const XK_osfExtend = 0x1004_FF74;
		const XK_osfRestore = 0x1004_FF78;
		const XK_osfDelete = 0x1004_FFFF;

		const XK_Reset = 0x1000_FF6C;
		const XK_System = 0x1000_FF6D;
		const XK_User = 0x1000_FF6E;
		const XK_ClearLine = 0x1000_FF6F;
		const XK_InsertLine = 0x1000_FF70;
		const XK_DeleteLine = 0x1000_FF71;
		const XK_InsertChar = 0x1000_FF72;
		const XK_DeleteChar = 0x1000_FF73;
		const XK_BackTab = 0x1000_FF74;
		const XK_KP_BackTab = 0x1000_FF75;
		const XK_Ext16bit_L = 0x1000_FF76;
		const XK_Ext16bit_R = 0x1000_FF77;
		const XK_mute_acute = 0x1000_00a8;
		const XK_mute_grave = 0x1000_00a9;
		const XK_mute_asciicircum = 0x1000_00aa;
		const XK_mute_diaeresis = 0x1000_00ab;
		const XK_mute_asciitilde = 0x1000_00ac;
		const XK_lira = 0x1000_00af;
		const XK_guilder = 0x1000_00be;
		const XK_IO = 0x1000_00ee;
		const XK_longminus = 0x1000_00f6;
		const XK_block = 0x1000_00fc;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyPattern {
	pub mods: ModFlags,
	pub key: Keysym,
}

impl StrataComp {
	pub fn clamp_coords(&self, pos: Point<f64, Logical>) -> Point<f64, Logical> {
		if self.workspaces.current().outputs().next().is_none() {
			return pos;
		}

		let (pos_x, pos_y) = pos.into();
		let (max_x, max_y) = self
			.workspaces
			.current()
			.output_geometry(self.workspaces.current().outputs().next().unwrap())
			.unwrap()
			.size
			.into();
		let clamped_x = pos_x.max(0.0).min(max_x as f64);
		let clamped_y = pos_y.max(0.0).min(max_y as f64);
		(clamped_x, clamped_y).into()
	}

	pub fn set_input_focus(&mut self, target: FocusTarget) {
		let keyboard = self.seat.get_keyboard().unwrap();
		let serial = SERIAL_COUNTER.next_serial();
		keyboard.set_focus(self, Some(target), serial);
	}

	pub fn set_input_focus_auto(&mut self) {
		let under = self.surface_under();
		if let Some(d) = under {
			self.set_input_focus(d.0);
		}
	}

	pub fn pointer_motion<I: InputBackend>(
		&mut self,
		event: I::PointerMotionEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let delta = (event.delta_x(), event.delta_y()).into();
		self.pointer_location += delta;
		self.pointer_location = self.clamp_coords(self.pointer_location);

		self.set_input_focus_auto();

		if let Some(ptr) = self.seat.get_pointer() {
			let under = self.surface_under();

			let location = self.pointer_location;
			ptr.motion(
				self,
				under.clone(),
				&MotionEvent { location, serial, time: event.time_msec() },
			);

			ptr.relative_motion(
				self,
				under,
				&RelativeMotionEvent {
					delta,
					delta_unaccel: event.delta_unaccel(),
					utime: event.time(),
				},
			)
		}

		Ok(())
	}

	pub fn pointer_motion_absolute<I: InputBackend>(
		&mut self,
		event: I::PointerMotionAbsoluteEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();

		let curr_workspace = self.workspaces.current();
		let output = curr_workspace.outputs().next().unwrap().clone();
		let output_geo = curr_workspace.output_geometry(&output).unwrap();
		let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();

		self.pointer_location = self.clamp_coords(pos);

		self.set_input_focus_auto();

		let under = self.surface_under();
		if let Some(ptr) = self.seat.get_pointer() {
			ptr.motion(
				self,
				under,
				&MotionEvent { location: pos, serial, time: event.time_msec() },
			);
		}

		Ok(())
	}
	pub fn pointer_button<I: InputBackend>(
		&mut self,
		event: I::PointerButtonEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();

		let button = event.button_code();
		let button_state = event.state();
		self.set_input_focus_auto();
		if let Some(ptr) = self.seat.get_pointer() {
			ptr.button(
				self,
				&ButtonEvent { button, state: button_state, serial, time: event.time_msec() },
			);
		}

		Ok(())
	}

	pub fn pointer_axis<I: InputBackend>(
		&mut self,
		event: I::PointerAxisEvent,
	) -> anyhow::Result<()> {
		let horizontal_amount = event
			.amount(Axis::Horizontal)
			.unwrap_or_else(|| event.amount(Axis::Horizontal).unwrap_or(0.0) * 3.0);
		let vertical_amount = event
			.amount(Axis::Vertical)
			.unwrap_or_else(|| event.amount(Axis::Vertical).unwrap_or(0.0) * 3.0);

		let mut frame = AxisFrame::new(event.time_msec()).source(event.source());
		if horizontal_amount != 0.0 {
			frame = frame.value(Axis::Horizontal, horizontal_amount);
		} else if event.source() == AxisSource::Finger {
			frame = frame.stop(Axis::Horizontal);
		}
		if vertical_amount != 0.0 {
			frame = frame.value(Axis::Vertical, vertical_amount);
		} else if event.source() == AxisSource::Finger {
			frame = frame.stop(Axis::Vertical);
		}

		if let Some(ptr) = self.seat.get_pointer() {
			ptr.axis(self, frame);
		}

		Ok(())
	}
}
