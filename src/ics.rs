use crate::{error::IcsError, talk::Talk};
use ical::{parser::ical::IcalParser, property::Property};
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

pub fn parse_file(path: &Path) -> Result<Vec<Talk>, IcsError> {
    let metadata =
        fs::metadata(path).map_err(|source| IcsError::ReadFile(path.to_owned(), source))?;
    if metadata.len() == 0 {
        return Err(IcsError::EmptyFile(path.to_owned()));
    }

    let file = File::open(path).map_err(|source| IcsError::ReadFile(path.to_owned(), source))?;
    IcalParser::new(BufReader::new(file))
        .map(|calendar| {
            calendar.map(|calendar| {
                calendar
                    .events
                    .into_iter()
                    .filter_map(|event| talk_from_properties(&event.properties))
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|events| events.into_iter().flatten().collect())
        .map_err(IcsError::from)
}

fn talk_from_properties(properties: &[Property]) -> Option<Talk> {
    property_value(properties, "URL")
        .zip(property_value(properties, "LOCATION"))
        .and_then(|(url, location)| path_elements(url).zip(room_name(location)))
        .map(|((year, id), room)| Talk {
            url: format!("https://video.fosdem.org/{year}/{room}/{id}.mp4"),
            year: year.to_owned(),
            id: id.to_owned(),
        })
}

fn path_elements(url: &str) -> Option<(&str, &str)> {
    let path = url
        .split_once("://")
        .map_or(url, |(_, remainder)| remainder);
    let path = path.split_once('/').map_or("", |(_, path)| path);
    let mut parts = path.split('/').filter(|part| !part.is_empty());
    let year = parts.next()?;
    let id = parts.next_back().unwrap_or(year);
    Some((year, id))
}

fn property_value<'a>(properties: &'a [Property], name: &str) -> Option<&'a str> {
    properties
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name))
        .and_then(|property| property.value.as_deref())
}

fn room_name(location: &str) -> Option<String> {
    location
        .replace('.', "")
        .split_whitespace()
        .next()
        .map(str::to_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_year_and_talk_id_from_url() {
        assert_eq!(
            path_elements("https://fosdem.org/2026/schedule/event/example/"),
            Some(("2026", "example"))
        );
    }

    #[test]
    fn extracts_room_prefix_without_dots() {
        assert_eq!(room_name("K.1.105 Building K"), Some("k1105".into()));
    }

    #[test]
    fn parses_talks_from_ics_events() {
        let talks = parse_file(Path::new("tests/fixtures/bookmarks.ics")).unwrap();
        assert_eq!(
            talks,
            vec![
                Talk {
                    url: "https://video.fosdem.org/2026/k1105/rust_example.mp4".into(),
                    year: "2026".into(),
                    id: "rust_example".into(),
                },
                Talk {
                    url: "https://video.fosdem.org/2025/ub2252a/another_talk.mp4".into(),
                    year: "2025".into(),
                    id: "another_talk".into(),
                },
            ]
        );
    }
}
