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
                Talk { url: "https://video.fosdem.org/2026/janson/SFKNTZ-welcome_to_fosdem_2026.mp4".into(), year: "2026".into(), id: "SFKNTZ-welcome_to_fosdem_2026".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/FE7ULY-foss-in-times-of-war-scarcity-and-ai.mp4".into(), year: "2026".into(), id: "FE7ULY-foss-in-times-of-war-scarcity-and-ai".into() },
                Talk { url: "https://video.fosdem.org/2026/k1105/BTMDHW-wayland-is-fun.mp4".into(), year: "2026".into(), id: "BTMDHW-wayland-is-fun".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/JWX9UM-postgres-mysql-two-databases-three-perspectives.mp4".into(), year: "2026".into(), id: "JWX9UM-postgres-mysql-two-databases-three-perspectives".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/HTJK33-evolving_git_for_the_next_decade.mp4".into(), year: "2026".into(), id: "HTJK33-evolving_git_for_the_next_decade".into() },
                Talk { url: "https://video.fosdem.org/2026/k3201/BCFZP7-current-state-programming-language-attestations.mp4".into(), year: "2026".into(), id: "BCFZP7-current-state-programming-language-attestations".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/ZMDB3S-defining-compatibility-postgres-mysql-derivatives.mp4".into(), year: "2026".into(), id: "ZMDB3S-defining-compatibility-postgres-mysql-derivatives".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2118/RYM8SF-repro-build.mp4".into(), year: "2026".into(), id: "RYM8SF-repro-build".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/9SU3P3-cracking-open-postgresql-select-execution-pipeline.mp4".into(), year: "2026".into(), id: "9SU3P3-cracking-open-postgresql-select-execution-pipeline".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2118/ZKKQWC-native_oci_container_support_in_systemd.mp4".into(), year: "2026".into(), id: "ZKKQWC-native_oci_container_support_in_systemd".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/3VNNBK-efficient-git-for-high-stakes.mp4".into(), year: "2026".into(), id: "3VNNBK-efficient-git-for-high-stakes".into() },
                Talk { url: "https://video.fosdem.org/2026/k3201/8WJKEH-package-registry-economics.mp4".into(), year: "2026".into(), id: "8WJKEH-package-registry-economics".into() },
                Talk { url: "https://video.fosdem.org/2026/k3201/FGBYKV-package_management_learnings_from_homebrew.mp4".into(), year: "2026".into(), id: "FGBYKV-package_management_learnings_from_homebrew".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/F9Y7ZY-you-do-not-need-an-orm.mp4".into(), year: "2026".into(), id: "F9Y7ZY-you-do-not-need-an-orm".into() },
                Talk { url: "https://video.fosdem.org/2026/k1105/YDLGLN-running_wikipedia_on_bare_metal_open_source_and_a_healthy_dose_of_caching.mp4".into(), year: "2026".into(), id: "YDLGLN-running_wikipedia_on_bare_metal_open_source_and_a_healthy_dose_of_caching".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2220/HAAABD-the_bakery_how_pep810_sped_up_my_bread_operations_business.mp4".into(), year: "2026".into(), id: "HAAABD-the_bakery_how_pep810_sped_up_my_bread_operations_business".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2220/ABJMWD-the_gil_and_api_performance_past_present_and_free-threaded_future.mp4".into(), year: "2026".into(), id: "ABJMWD-the_gil_and_api_performance_past_present_and_free-threaded_future".into() },
                Talk { url: "https://video.fosdem.org/2026/k1105/YUJUKD-what_happened_to_rubygems_and_what_can_we_learn.mp4".into(), year: "2026".into(), id: "YUJUKD-what_happened_to_rubygems_and_what_can_we_learn".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/CNPVJL-lightning_lightning_talks_1.mp4".into(), year: "2026".into(), id: "CNPVJL-lightning_lightning_talks_1".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2220/WE7NHM-modern-python-monorepo-apache-airflow.mp4".into(), year: "2026".into(), id: "WE7NHM-modern-python-monorepo-apache-airflow".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/N38ZF9-inverted_database_indexes_the_why_the_what_and_the_how.mp4".into(), year: "2026".into(), id: "N38ZF9-inverted_database_indexes_the_why_the_what_and_the_how".into() },
                Talk { url: "https://video.fosdem.org/2026/k1105/DCAVDC-how_to_make_package_managers_scream.mp4".into(), year: "2026".into(), id: "DCAVDC-how_to_make_package_managers_scream".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2220/UBNWNL-django-admin-deux.mp4".into(), year: "2026".into(), id: "UBNWNL-django-admin-deux".into() },
                Talk { url: "https://video.fosdem.org/2026/ua2220/YYTRKQ-lightning_talks.mp4".into(), year: "2026".into(), id: "YYTRKQ-lightning_talks".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/L3BK7S-free-as-in-burned-out.mp4".into(), year: "2026".into(), id: "L3BK7S-free-as-in-burned-out".into() },
                Talk { url: "https://video.fosdem.org/2026/h2215/ZUVAZD-four_year_bus.mp4".into(), year: "2026".into(), id: "ZUVAZD-four_year_bus".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/W3UFSK-rust-game-boy.mp4".into(), year: "2026".into(), id: "W3UFSK-rust-game-boy".into() },
                Talk { url: "https://video.fosdem.org/2026/h2215/J8UCRH-why_i_volunteer_at_fosdem_and_you_should_too.mp4".into(), year: "2026".into(), id: "J8UCRH-why_i_volunteer_at_fosdem_and_you_should_too".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/UHDG9Z-who_funds_the_egg_cracking_the_foss_funding_paradox.mp4".into(), year: "2026".into(), id: "UHDG9Z-who_funds_the_egg_cracking_the_foss_funding_paradox".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/DS7LFX-rust-roto-calling-jit-compiled-scripts.mp4".into(), year: "2026".into(), id: "DS7LFX-rust-roto-calling-jit-compiled-scripts".into() },
                Talk { url: "https://video.fosdem.org/2026/h1301/8AS3XD-how-to-reliably-measure-software-performance.mp4".into(), year: "2026".into(), id: "8AS3XD-how-to-reliably-measure-software-performance".into() },
                Talk { url: "https://video.fosdem.org/2026/k1105/TYZH97-fear-loathing-app-stores.mp4".into(), year: "2026".into(), id: "TYZH97-fear-loathing-app-stores".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/JDZXFE-next-generation-contributors-lessons-from-postgres.mp4".into(), year: "2026".into(), id: "JDZXFE-next-generation-contributors-lessons-from-postgres".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/NBLNRY-rust-clickhouse.mp4".into(), year: "2026".into(), id: "NBLNRY-rust-clickhouse".into() },
                Talk { url: "https://video.fosdem.org/2026/k3401/SSXDBS-dns_a_love_affair_with_lovecraftian_horrors.mp4".into(), year: "2026".into(), id: "SSXDBS-dns_a_love_affair_with_lovecraftian_horrors".into() },
                Talk { url: "https://video.fosdem.org/2026/h1301/77QF9T-hg-100x.mp4".into(), year: "2026".into(), id: "77QF9T-hg-100x".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/YVALSG-rust-profiling-with-parca.mp4".into(), year: "2026".into(), id: "YVALSG-rust-profiling-with-parca".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/RCFALN-rust-building-performance-critical-python-apps.mp4".into(), year: "2026".into(), id: "RCFALN-rust-building-performance-critical-python-apps".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/GWRDNT-rust-type-checking-python.mp4".into(), year: "2026".into(), id: "GWRDNT-rust-type-checking-python".into() },
                Talk { url: "https://video.fosdem.org/2026/h1301/YNB7KR-continuous-perf-engineering.mp4".into(), year: "2026".into(), id: "YNB7KR-continuous-perf-engineering".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/EQAECR-rust-gitoxide.mp4".into(), year: "2026".into(), id: "EQAECR-rust-gitoxide".into() },
                Talk { url: "https://video.fosdem.org/2026/h2215/E7BQKK-git_blame_for_your_dependencies.mp4".into(), year: "2026".into(), id: "E7BQKK-git_blame_for_your_dependencies".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/KX9P7J-art-of-esoteric-code.mp4".into(), year: "2026".into(), id: "KX9P7J-art-of-esoteric-code".into() },
                Talk { url: "https://video.fosdem.org/2026/ub2252a/DTYYL9-rust-coreutils.mp4".into(), year: "2026".into(), id: "DTYYL9-rust-coreutils".into() },
                Talk { url: "https://video.fosdem.org/2026/h2215/GGLZS9-amber-lang-bash-transpiler.mp4".into(), year: "2026".into(), id: "GGLZS9-amber-lang-bash-transpiler".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/G3ZWYU-lightning_lightning_talks_2.mp4".into(), year: "2026".into(), id: "G3ZWYU-lightning_lightning_talks_2".into() },
                Talk { url: "https://video.fosdem.org/2026/h2215/WQ3F9A-fosdem_infrastructure_review.mp4".into(), year: "2026".into(), id: "WQ3F9A-fosdem_infrastructure_review".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/B7YKQ7-oss-in-spite-of-ai.mp4".into(), year: "2026".into(), id: "B7YKQ7-oss-in-spite-of-ai".into() },
                Talk { url: "https://video.fosdem.org/2026/janson/XN9DAW-closing_fosdem_2026.mp4".into(), year: "2026".into(), id: "XN9DAW-closing_fosdem_2026".into() },
            ]
        );
    }
}
