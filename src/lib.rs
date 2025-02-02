#![feature(naked_functions)]
pub mod hook;

use libc::*;
use rand::Rng;
use std::arch::*;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use widestring::*;

static mut BASIC_WIDE_STRING: Option<extern "C" fn(*mut *mut c_void, *mut u32, *const c_void)> =
	None;
static mut BGM_SETS: Vec<BgmSet> = vec![];
static mut LAST_BGM_SET: i32 = 0;
static mut PLAYED_AUDIO_ID: i32 = 0;
static mut ORIGINAL_BGM_SET: u8 = 0;

fn original_bgm_sets() -> Vec<BgmSet> {
	let maxi3 = BgmSet {
		name: String::from("3"),
		imagepath: String::new(),
		songs: vec![
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3plus/wm3DX_P_race_01.wav"),
				name: String::from("Stay where you are"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3plus/wm3DX_P_race_02.wav"),
				name: String::from("Loop of Fortune"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3plus/wm3DX_P_race_03.wav"),
				name: String::from("Acid Runner remix 2010"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3plus/wm3DX_P_race_04.wav"),
				name: String::from("Feel the Passion remix 2010"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3dx/wm3DX_race_01_mainD.wav"),
				name: String::from("Enjoy the Process"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_01.wav"),
				name: String::from("Inexhaustible Energy"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_02.wav"),
				name: String::from("In Your Dream"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_03.wav"),
				name: String::from("Get Down to the Drive"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_04.wav"),
				name: String::from("Top-Flight Mechanics"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_05.wav"),
				name: String::from("Wanna Try One Last Time"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_06.wav"),
				name: String::from("Get you Cornered"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_07.wav"),
				name: String::from("Love to Rise in the Summer Morning"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_08.wav"),
				name: String::from("Evil Association"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_09.wav"),
				name: String::from("Supreme Folly"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_10.wav"),
				name: String::from("Control Your Body"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_11.wav"),
				name: String::from("Shrewd Critic"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_12.wav"),
				name: String::from("Cause You're Different"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_13.wav"),
				name: String::from("Feel the Moment"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_14.wav"),
				name: String::from("Black Pressure"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi3/wm3_15.wav"),
				name: String::from("Phantom of Blue"),
			},
		],
		stories: None,
	};
	let maxi2 = BgmSet {
		name: String::from("2"),
		imagepath: String::new(),
		songs: vec![
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_01.wav"),
				name: String::from("Overdrive Neurotransmitters"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_02.wav"),
				name: String::from("Maximum Acceleration"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi1/wm2_03.wav"),
				name: String::from("Stream Of Tears(more tranced remix)"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_04.wav"),
				name: String::from("Feel the Passion"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_06.wav"),
				name: String::from("Upbeat Gas Junkie"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_07.wav"),
				name: String::from("Fun-Loving Spirit"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi1/wm2_08.wav"),
				name: String::from("Speed Fenatic"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_09.wav"),
				name: String::from("Driver's Delight"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_10.wav"),
				name: String::from("Decoration For The Dement"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_11.wav"),
				name: String::from("Ghost in the Resonance"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_12.wav"),
				name: String::from("Stream Of Tears"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_13.wav"),
				name: String::from("Total Terror"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_14.wav"),
				name: String::from("Smoldery Guest"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_15.wav"),
				name: String::from("Atheist On The Highway"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_16.wav"),
				name: String::from("This is My Destiny"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_17.wav"),
				name: String::from("Unlawful Temptation"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi1/wm2_18.wav"),
				name: String::from("Acid Runner"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_19.wav"),
				name: String::from("Nothing Lives Forever"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_20.wav"),
				name: String::from("Destination Blackout"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_21.wav"),
				name: String::from("Blue Blazes"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_22.wav"),
				name: String::from("Another Dimension"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/maxi2/wm2_23.wav"),
				name: String::from("Holy Land Anthem"),
			},
		],
		stories: None,
	};
	let maxir = BgmSet {
		name: String::from("R"),
		imagepath: String::new(),
		songs: vec![
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_01.wav"),
				name: String::from("Rival's Theme 1"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_02.wav"),
				name: String::from("Rival's Theme 2"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_03.wav"),
				name: String::from("Hiramoto's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_04.wav"),
				name: String::from("Masaki's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_05.wav"),
				name: String::from("Kei's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_06.wav"),
				name: String::from("R200CLUB's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_07.wav"),
				name: String::from("Kuroki's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_08.wav"),
				name: String::from("Eiji's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_09.wav"),
				name: String::from("Kijima's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_10.wav"),
				name: String::from("Reina's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_11.wav"),
				name: String::from("BlackBird's Theme"),
			},
			BgmSong {
				filepath: String::from("data/sound/bgm/r/wmr_12.wav"),
				name: String::from("Akio's Theme"),
			},
		],
		stories: None,
	};
	vec![maxi3, maxi2, maxir]
}

struct BgmSet {
	name: String,
	imagepath: String,
	songs: Vec<BgmSong>,
	stories: Option<Vec<String>>,
}

struct BgmSong {
	filepath: String,
	name: String,
}

#[repr(C)]
struct NsAudio {
	name: *mut c_void,
}

static mut ORIGINAL_GET_TITLE_ID: Option<extern "C" fn(NsAudio, i32) -> NsAudio> = None;
unsafe extern "C" fn get_title_id(id: i32) -> NsAudio {
	if id < 1 << 8 {
		let this = NsAudio {
			name: std::ptr::null_mut(),
		};
		return ORIGINAL_GET_TITLE_ID.unwrap()(this, id);
	}
	let set = get_bgm_set(id);
	let index = get_bgm_index(set, id);
	get_title_index(set, index)
}

unsafe extern "C" fn get_title_index(set: i32, index: i32) -> NsAudio {
	let title = if index == 0 {
		U32CString::from_str_unchecked("RANDOM")
	} else {
		let set = BGM_SETS.get(set as usize);
		if let Some(set) = set {
			let song = set.songs.get(index as usize - 1);
			if let Some(song) = song {
				U32CString::from_str_unchecked(format!("{index} {}", song.name))
			} else {
				U32CString::from_str_unchecked("UNKNOWN")
			}
		} else {
			U32CString::from_str_unchecked("UNKNOWN")
		}
	};
	let mut this = NsAudio {
		name: std::ptr::null_mut(),
	};
	BASIC_WIDE_STRING.unwrap()(
		&mut this.name as *mut *mut _,
		title.into_raw(),
		std::ptr::null(),
	);
	this
}

unsafe extern "C" fn set_relative_volume(this: *mut f32) {
	this.byte_offset(0x2C).write(2.5);
	this.byte_offset(0x30).write(2.5);
}

unsafe fn get_random_song(set: i32) -> i32 {
	let Some(bgm_set) = BGM_SETS.get(set as usize) else {
		return 0;
	};
	let length = bgm_set.songs.len();
	let mut rng = rand::thread_rng();
	let index = rng.gen_range(1..=length) as i32;
	(set + 1) << 8 | index
}

unsafe extern "C" fn get_bgm_id(set: i32, index: i32) -> i32 {
	if index == 0 {
		get_random_song(set)
	} else {
		(set + 1) << 8 | index
	}
}

unsafe extern "C" fn get_bgm_set(id: i32) -> i32 {
	if id == -1 {
		0
	} else {
		(id >> 8) - 1
	}
}

unsafe extern "C" fn get_bgm_index(_set: i32, id: i32) -> i32 {
	if id == -1 || id == 0x6E {
		0
	} else {
		id & ((1 << 8) - 1)
	}
}

unsafe fn get_bgm_path(id: i32) -> Option<String> {
	let set = get_bgm_set(id);
	let index = get_bgm_index(set, id);
	let set = BGM_SETS.get(set as usize);
	if let Some(set) = set {
		let song = set.songs.get(index as usize - 1);
		if let Some(song) = song {
			Some(song.filepath.clone())
		} else {
			None
		}
	} else {
		None
	}
}

unsafe extern "C" fn get_next_bgm(set: i32, index: i32) -> i32 {
	let bgm_set = BGM_SETS.get(set as usize);
	if let Some(bgm_set) = bgm_set {
		let length = bgm_set.songs.len() as i32;
		if index == length {
			0
		} else {
			index + 1
		}
	} else {
		0
	}
}

static mut ORIGINAL_GET_BGM_ICON: Option<extern "C" fn(i32, i32) -> i32> = None;
unsafe extern "C" fn get_bgm_icon(set: i32, _id: i32) -> i32 {
	LAST_BGM_SET = set;
	if set > 2 {
		ORIGINAL_GET_BGM_ICON.unwrap()(0, 0)
	} else {
		ORIGINAL_GET_BGM_ICON.unwrap()(set, 0)
	}
}

static mut ORIGINAL_LOAD_IMAGE: Option<extern "C" fn(*mut c_void, *const c_char) -> i32> = None;
unsafe extern "C" fn load_image(this: *mut c_void, filepath: *const c_char) -> i32 {
	if filepath == std::ptr::null() {
		return ORIGINAL_LOAD_IMAGE.unwrap()(this, filepath);
	}
	let path = CStr::from_ptr(filepath).to_str().unwrap();
	if path.ends_with("menu_p_sound_logo_maxi3.png") && LAST_BGM_SET > 2 {
		let set = BGM_SETS.get(LAST_BGM_SET as usize);
		if let Some(set) = set {
			let path = CString::new(set.imagepath.clone()).unwrap();
			return ORIGINAL_LOAD_IMAGE.unwrap()(this, path.as_ptr());
		}
	}
	ORIGINAL_LOAD_IMAGE.unwrap()(this, filepath)
}

static mut ORIGINAL_PLAY_AUDIO: Option<extern "C" fn(*mut c_void, i32) -> i32> = None;
unsafe extern "C" fn play_audio(this: *mut c_void, id: i32) -> i32 {
	if id < 1 << 8 {
		PLAYED_AUDIO_ID = 0;
		return ORIGINAL_PLAY_AUDIO.unwrap()(this, id);
	}
	PLAYED_AUDIO_ID = id;
	println!("Playing song {id}");
	ORIGINAL_PLAY_AUDIO.unwrap()(this, 0)
}

#[no_mangle]
static mut LOAD_WAV_FILE: *const c_char = std::ptr::null();
#[naked]
unsafe extern "C" fn load_wav_impl(this: *mut c_void, filepath: *const c_char) {
	naked_asm!(
		r#"
		enter 24, 0
		mov eax, [ebp + 8]
		mov [esp], eax
		mov eax, [ebp + 12]
		mov [esp + 4], eax
		call load_wav
		cmp edi, 0
		jne 2f
		lea ebx, [LOAD_WAV_FILE]
		mov ebx, [ebx]
		2:
		leave
		ret 0
		"#
	)
}

static mut ORIGINAL_LOAD_WAV: Option<extern "C" fn(*mut c_void, *const c_char) -> i32> = None;
#[no_mangle]
unsafe extern "C" fn load_wav(this: *mut c_void, filepath: *const c_char) -> i32 {
	if PLAYED_AUDIO_ID == 0 {
		LOAD_WAV_FILE = filepath;
		return ORIGINAL_LOAD_WAV.unwrap()(this, filepath);
	}
	let path = CStr::from_ptr(filepath).to_str().unwrap();
	if path != "data/sound/bgm/maxi1/wm2_05.wav" {
		LOAD_WAV_FILE = filepath;
		return ORIGINAL_LOAD_WAV.unwrap()(this, filepath);
	}
	let path = get_bgm_path(PLAYED_AUDIO_ID).unwrap();
	println!("Playing song from {path}");
	let path = CString::new(path).unwrap();
	LOAD_WAV_FILE = path.as_ptr();
	let result = ORIGINAL_LOAD_WAV.unwrap()(this, path.as_ptr());
	std::mem::forget(path);
	result
}

static mut AUDIO_STREAM: *mut *mut c_void = std::ptr::null_mut();
static mut STREAM_MANAGER_PLAY: Option<extern "C" fn(*mut c_void, i32, i32)> = None;
unsafe extern "C" fn play_stream(_: *mut c_void, id: i32, sndtype: i32) {
	set_relative_volume(AUDIO_STREAM.read() as *mut f32);
	STREAM_MANAGER_PLAY.unwrap()(AUDIO_STREAM.read(), id, sndtype)
}

unsafe fn load_card() -> usize {
	#[derive(serde::Deserialize)]
	struct Bgm {
		bgm: String,
	}
	let Ok(text) = std::fs::read_to_string("plugins/audio.toml") else {
		return 0;
	};
	let Ok(bgm) = toml::from_str::<Bgm>(&text) else {
		return 0;
	};
	let Some((index, _)) = BGM_SETS
		.iter()
		.enumerate()
		.find(|(_, set)| set.name == bgm.bgm)
	else {
		return 0;
	};
	index
}

static mut ORIGINAL_LOAD_CARD_V386: Option<
	extern "C" fn(*mut u8, *const c_char, *const c_char, bool),
> = None;
unsafe extern "C" fn load_card_v386(data: *mut u8, a2: *const c_char, a3: *const c_char, a4: bool) {
	ORIGINAL_LOAD_CARD_V386.unwrap()(data, a2, a3, a4);
	ORIGINAL_BGM_SET = data.byte_offset(0x16D).read();
	data.byte_offset(0x16D).write(load_card() as u8);
}

static mut ORIGINAL_LOAD_CARD_V363: Option<extern "C" fn(*mut u8, *const c_char, *const c_char)> =
	None;
unsafe extern "C" fn load_card_v363(data: *mut u8, a2: *const c_char, a3: *const c_char) {
	ORIGINAL_LOAD_CARD_V363.unwrap()(data, a2, a3);
	ORIGINAL_BGM_SET = data.byte_offset(0x16D).read();
	data.byte_offset(0x16D).write(load_card() as u8);
}

static mut ORIGINAL_LOAD_CARD_V337: Option<extern "C" fn(*mut u8, *const c_char)> = None;
unsafe extern "C" fn load_card_v337(data: *mut u8, a2: *const c_char) {
	ORIGINAL_LOAD_CARD_V337.unwrap()(data, a2);
	ORIGINAL_BGM_SET = data.byte_offset(0x169).read();
	data.byte_offset(0x169).write(load_card() as u8);
}

static mut ORIGINAL_SAVE_CARD_V386: Option<extern "C" fn(*mut u8, bool, *const u8) -> *const u8> =
	None;
unsafe extern "C" fn save_card_v386(data: *mut u8, a2: bool, a3: *const u8) -> *const u8 {
	let bgm = data.byte_offset(0x16D);
	let original_bgm = bgm.read();
	bgm.write(ORIGINAL_BGM_SET);
	let ret = ORIGINAL_SAVE_CARD_V386.unwrap()(data, a2, a3);
	bgm.write(original_bgm);
	ret
}

static mut ORIGINAL_SAVE_CARD_V363: Option<extern "C" fn(*mut u8, bool, *const u8) -> *const u8> =
	None;
unsafe extern "C" fn save_card_v363(data: *mut u8, a2: bool, a3: *const u8) -> *const u8 {
	let bgm = data.byte_offset(0x16D);
	let original_bgm = bgm.read();
	bgm.write(ORIGINAL_BGM_SET);
	let ret = ORIGINAL_SAVE_CARD_V363.unwrap()(data, a2, a3);
	bgm.write(original_bgm);
	ret
}

static mut ORIGINAL_SAVE_CARD_V337: Option<extern "C" fn(*mut u8, bool) -> *const u8> = None;
unsafe extern "C" fn save_card_v337(data: *mut u8, a2: bool) -> *const u8 {
	let bgm = data.byte_offset(0x169);
	let original_bgm = bgm.read();
	bgm.write(ORIGINAL_BGM_SET);
	let ret = ORIGINAL_SAVE_CARD_V337.unwrap()(data, a2);
	bgm.write(original_bgm);
	ret
}

static mut ORIGINAL_GET_STORY_BGM: Option<extern "C" fn(*mut i32, i32, i32) -> i32> = None;
unsafe extern "C" fn get_story_bgm(episode: *mut i32, a2: i32, set: i32) -> i32 {
	if a2 > 0 {
		return -1;
	}
	if set < 3 || episode.is_null() {
		return ORIGINAL_GET_STORY_BGM.unwrap()(episode, a2, set);
	}
	let episode_no = episode.read();
	let Some(bgm_set) = BGM_SETS.get(set as usize) else {
		return ORIGINAL_GET_STORY_BGM.unwrap()(episode, a2, set);
	};
	let Some(stories) = &bgm_set.stories else {
		return get_random_song(set);
	};
	let Some(song_name) = stories.get(episode_no as usize) else {
		return get_random_song(set);
	};
	let Some((index, _)) = bgm_set
		.songs
		.iter()
		.enumerate()
		.find(|(_, song)| &song.name == song_name)
	else {
		return get_random_song(set);
	};
	get_bgm_id(set, index as i32)
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum GameMajor {
	WM3 = 0,
	W3X = 1,
	W3P = 2,
	Unknown,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum GameMinor {
	A = 0,
	B = 1,
	Unknown,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum GameRegion {
	JP = 1,
	EN2 = 2,
	EN3 = 3,
	EN4 = 4,
	Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct GameVersion {
	major: GameMajor,
	minor: GameMinor,
	region: GameRegion,
	revision: u32,
}

#[no_mangle]
pub unsafe extern "C" fn init(version: *const GameVersion) {
	let version = version.read();
	BGM_SETS = original_bgm_sets();

	if let Ok(folders) = glob::glob("plugins/bgms/*") {
		let folders = folders
			.map(|res| res.map(|e| e))
			.collect::<Result<Vec<_>, _>>();
		if let Ok(mut folders) = folders {
			folders.sort();
			for folder in folders {
				let mut image = folder.clone().to_path_buf();
				image.push("mod.png");
				let mut config = folder.clone().to_path_buf();
				config.push("mod.toml");
				if !image.exists() || !config.exists() {
					continue;
				}

				#[derive(serde::Deserialize)]
				struct Songs {
					song: Vec<Song>,
					stories: Option<Vec<String>>,
				}

				#[derive(serde::Deserialize)]
				struct Song {
					name: String,
					file: String,
				}

				let Ok(text) = std::fs::read_to_string(config) else {
					continue;
				};
				let Ok(songs) = toml::from_str::<Songs>(&text) else {
					continue;
				};

				let bgm_songs = songs
					.song
					.iter()
					.map(|song| {
						let mut folder = folder.clone().to_path_buf();
						folder.push(song.file.clone());
						BgmSong {
							name: song.name.clone(),
							filepath: folder.to_string_lossy().to_string(),
						}
					})
					.collect::<Vec<_>>();
				let bgm = BgmSet {
					name: folder
						.components()
						.last()
						.unwrap()
						.as_os_str()
						.to_string_lossy()
						.to_string(),
					imagepath: image.to_string_lossy().to_string(),
					songs: bgm_songs,
					stories: songs.stories,
				};
				BGM_SETS.push(bgm);
			}
		}
	}

	BASIC_WIDE_STRING = Some(transmute(hook::get_symbol(
		"_ZNSbIwSt11char_traitsIwESaIwEEC1EPKwRKS1_",
	)));
	hook::hook_symbol(
		"_ZN11nsMenuCheck16getBgmSelectNameEii",
		get_title_index as *const (),
	);
	ORIGINAL_GET_TITLE_ID = Some(transmute(hook::hook_symbol(
		"_ZN7nsAudio13getMusicTitleENS_9emBgmTypeE",
		get_title_id as *const (),
	)));
	hook::hook_symbol(
		"_ZN19clAudioStreamManage17setRelativeVolumeEN7nsAudio9emBgmTypeES1_",
		set_relative_volume as *const (),
	);
	hook::hook_symbol(
		"_ZN11nsMenuCheck14getBgmSelectIdEii",
		get_bgm_id as *const (),
	);
	hook::hook_symbol(
		"_ZN11nsMenuCheck15getBgmSetFromIdEN7nsAudio9emBgmTypeE",
		get_bgm_set as *const (),
	);
	hook::hook_symbol(
		"_ZN11nsMenuCheck15getBgmSelectIdxEiN7nsAudio9emBgmTypeE",
		get_bgm_index as *const (),
	);
	hook::hook_symbol(
		"_ZN11nsMenuCheck16getBgmSelectNextEii",
		get_next_bgm as *const (),
	);
	ORIGINAL_LOAD_IMAGE = Some(transmute(hook::hook_symbol(
		"_ZN3Gap3Gfx7igImage4loadEPKc",
		load_image as *const (),
	)));
	ORIGINAL_GET_BGM_ICON = Some(transmute(hook::hook_symbol(
		"_ZN11nsMenuCheck16getBgmSelectIconEii",
		get_bgm_icon as *const (),
	)));
	hook::hook_symbol(
		"_ZN11nsMenuCheck10getBgmIconEiN7nsAudio9emBgmTypeE",
		get_bgm_icon as *const (),
	);
	ORIGINAL_PLAY_AUDIO = Some(transmute(hook::hook_symbol(
		"_ZN13clAudioStream4playEN7nsAudio9emBgmTypeE",
		play_audio as *const (),
	)));
	ORIGINAL_LOAD_WAV = Some(transmute(hook::hook_symbol(
		"_ZN10clWaveFile8loadFileEPKc",
		load_wav_impl as *const (),
	)));
	hook::hook_symbol(
		"_ZN7nsAudio7clAudio10playStreamENS_9emBgmTypeENS_12emSndCtlTypeE",
		play_stream as *const (),
	);
	AUDIO_STREAM = transmute(hook::get_symbol("_ZN7nsAudio7clAudio11audioStreamE"));
	STREAM_MANAGER_PLAY = Some(transmute(hook::get_symbol(
		"_ZN19clAudioStreamManage4playEN7nsAudio9emBgmTypeENS0_12emSndCtlTypeE",
	)));
	ORIGINAL_GET_STORY_BGM = Some(transmute(hook::hook_symbol(
		"_ZNK14clEffStoryType6getBgmEii",
		get_story_bgm as *const (),
	)));

	if version.major == GameMajor::W3P {
		ORIGINAL_LOAD_CARD_V386 = Some(transmute(hook::hook_symbol(
			"_ZN14clV386CardData6assignEPKcPcb",
			load_card_v386 as *const (),
		)));
		ORIGINAL_SAVE_CARD_V386 = Some(transmute(hook::hook_symbol(
			"_ZN14clV386CardData4dataEbPc",
			save_card_v386 as *const (),
		)));
	} else if version.major == GameMajor::W3X {
		ORIGINAL_LOAD_CARD_V363 = Some(transmute(hook::hook_symbol(
			"_ZN14clV363CardData6assignEPKcPc",
			load_card_v363 as *const (),
		)));
		ORIGINAL_SAVE_CARD_V363 = Some(transmute(hook::hook_symbol(
			"_ZN14clV363CardData4dataEbPc",
			save_card_v363 as *const (),
		)));
	} else if version.major == GameMajor::WM3 {
		ORIGINAL_LOAD_CARD_V337 = Some(transmute(hook::hook_symbol(
			"_ZN10clV337Card6assignEPKc",
			load_card_v337 as *const (),
		)));
		ORIGINAL_SAVE_CARD_V337 = Some(transmute(hook::hook_symbol(
			"_ZN10clV337Card4dataEb",
			save_card_v337 as *const (),
		)));
	} else {
		panic!("Unknown game version");
	}

	match (
		version.major,
		version.region,
		version.minor,
		version.revision,
	) {
		(GameMajor::W3P, GameRegion::JP, GameMinor::B, 02) => {
			hook::write_memory(0x87EEA2C as *mut (), &[0x90, 0x90, 0x90]);
		}
		(GameMajor::W3P, GameRegion::EN2, GameMinor::B, 02) => {
			hook::write_memory(0x87EF4FC as *mut (), &[0x90, 0x90, 0x90]);
		}
		(GameMajor::W3X, GameRegion::EN4, GameMinor::A, 20) => {
			hook::write_memory(0x878168E as *mut (), &[0x90, 0x90, 0x90]);
		}
		(GameMajor::WM3, GameRegion::JP, GameMinor::A, 70) => {
			hook::write_memory(0x84FCA6A as *mut (), &[0x90, 0x90, 0x90]);
		}
		(GameMajor::WM3, GameRegion::EN3, GameMinor::A, 70) => {
			hook::write_memory(0x84FCC6A as *mut (), &[0x90, 0x90, 0x90]);
		}
		_ => {
			panic!("Unknown game version");
		}
	}
}
