/*
 * File: temperature.rs
 * Project: knowitall
 * Created Date: Wednesday, December 9th 2020, 4:34:09 pm
 * Author: aspen
 * -----
 * Copyright (c) 2020 aspen
 *
 * This software is provided 'as-is', without any express or implied warranty. In
 * no event will the authors be held liable for any damages arising from the use of
 * this software.
 *
 * Permission is granted to anyone to use this software for any purpose, including
 * commercial applications, and to alter it and redistribute it freely, subject to
 * the following restrictions:
 *
 * 1.  The origin of this software must not be misrepresented; you must not claim
 *     that you wrote the original software. If you use this software in a product,
 *     an acknowledgment in the product documentation would be appreciated but is
 *     not required.
 *
 * 2.  Altered source versions must be plainly marked as such, and must not be
 *     misrepresented as being the original software.
 *
 * 3.  This notice may not be removed or altered from any source distribution.
 */

use crate::{
	patterns::TEMPERATURE,
	provider::{InfoSegment, Provider, Tooltip},
};

pub struct TemperatureProvider;

impl Provider for TemperatureProvider {
	fn name(&self) -> &'static str {
		"Temperature"
	}

	fn parse_message(&self, src: &str) -> Vec<InfoSegment> {
		TEMPERATURE
			.captures_iter(src)
			.filter_map(|capture| -> Option<InfoSegment> {
				let segment = capture.get(0)?;
				let (value, unit) = (
					lexical::parse(capture.name("value")?.as_str()).ok()?,
					capture.name("unit")?.as_str(),
				);

				let kelvin = match unit.to_lowercase().as_str() {
					"c" | "celsius" | "celcius" => celsius_to_kelvin(value),
					"f" | "fahrenheit" => fahrenheit_to_kelvin(value),
					"k" | "kelvin" => value,
					_ => return None,
				};

				InfoSegment {
					start: segment.start(),
					end: segment.end(),
					info: Tooltip::Temperature { kelvin },
				}
				.into()
			})
			.collect()
	}
}

fn celsius_to_kelvin(c: f32) -> f32 {
	c + 273.15
}

fn fahrenheit_to_kelvin(f: f32) -> f32 {
	const F_DIVISION: f32 = 5.0 / 9.0;
	(f + 459.67) * F_DIVISION
}
