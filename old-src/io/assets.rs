use std::{collections::HashMap, fs, path::Path};
use macroquad::prelude::*;

use crate::types::LightState;

pub struct Assets {
	pub car_textures: Vec<Texture2D>,
	pub light_textures: HashMap<LightState, Texture2D>,
}
impl Assets {
	pub async fn load(cars_dir: &str, lights_dir: &str) -> Self {
		let mut car_textures: Vec<Texture2D> = Vec::new();

		if let Ok(entries) = fs::read_dir(cars_dir) {
			for entry in entries.flatten() {
				let p = entry.path();
				if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
					let ext = ext.to_lowercase();
					if ext == "webp" {
						if let Some(pstr) = p.to_str() {
							if let Ok(tex) = Self::load_webp_texture(pstr).await {
								tex.set_filter(FilterMode::Linear);
								car_textures.push(tex);
							} else {
								eprintln!("Failed to load car texture: {}", pstr);
							}
						}
					}
				}
			}
		} else {
			eprintln!("Warning: could not read cars dir: {}", cars_dir);
		}

		let mut light_textures: HashMap<LightState, Texture2D> = HashMap::new();
		let mapping = [
			(LightState::Red, "red.webp"),
			(LightState::Yellow, "yellow.webp"),
			(LightState::Green, "green.webp"),
		];
		for (state, fname) in mapping.iter() {
			let p = Path::new(lights_dir).join(fname);
			if p.exists() {
				if let Some(pstr) = p.to_str() {
					match load_texture(pstr).await {
						Ok(t) => {
							t.set_filter(FilterMode::Linear);
							light_textures.insert(*state, t);
						}
						Err(_) => {
							eprintln!("Failed to load light texture: {}", pstr);
						}
					}
				}
			} else {
				eprintln!("Light texture not found: {}", p.display());
			}
		}

		Self {
			car_textures,
			light_textures,
		}
	}

	pub fn random_car_texture_idx(&self) -> usize {
		if self.car_textures.is_empty() {
			0
		}
		else {
			let n = self.car_textures.len() as i32;
			let idx = macroquad::rand::gen_range(0, n);
			idx as usize
		}
	}

	async fn load_webp_texture(path: &str) -> Result<Texture2D, String> {
		let bytes = load_file(path)
			.await
			.map_err(|e| format!("Не удалось прочитать файл '{}': {:?}", path, e))?;

		let img = image::io::Reader::new(std::io::Cursor::new(bytes))
			.with_guessed_format()
			.map_err(|e| format!("Ошибка формата файла '{}': {}", path, e))?
			.decode()
			.map_err(|e| format!("Ошибка декодирования WebP '{}': {}", path, e))?
			.to_rgba8();

		let (width, height) = (img.width() as u16, img.height() as u16);

		let texture = Texture2D::from_rgba8(width, height, &img.into_raw());

		Ok(texture)
	}
}