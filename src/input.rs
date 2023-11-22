use std::io::BufReader;

use reqwest::blocking::Response;
use url::Url;

use crate::date::ChallengeDate;

/// Input data for a challenge.
pub struct Input(BufReader<Response>);

impl Input {
    /// Fetch the input for the given day's challenge from AoC's website.
    pub fn fetch(date: ChallengeDate) -> Self {
        let url = build_url(date);
        let response = reqwest::blocking::get(url).expect("failed to get AoC input data");
        Self(BufReader::new(response))
    }

    /// Get a reference to the inner `BufReader`.
    pub fn reader(&mut self) -> &mut BufReader<Response> {
        &mut self.0
    }
}

/// Builds the URL to get the input for the given day's challenge from AoC's website.
fn build_url(date: ChallengeDate) -> Url {
    const BASE_URL: &str = "https://adventofcode.com";

    let mut url = Url::parse(BASE_URL).expect("bad BASE_URL, this is a programmer error");
    url.path_segments_mut().unwrap().extend([
        &date.year.to_string(),
        "day",
        &date.day.to_string(),
        "input",
    ]);
    url
}
