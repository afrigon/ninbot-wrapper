use super::Palette;
use super::Piece;
use super::Segment;
use super::Weight;
use crate::game::GameState;
use crate::ninjabrain::Boat;
use crate::ninjabrain::InformationMessages;
use crate::ninjabrain::Prediction;
use crate::ninjabrain::ResultType;
use crate::ninjabrain::Severity;
use crate::ninjabrain::Stronghold;

const HIGH_CERTAINTY: f64 = 0.95;
const MEDIUM_CERTAINTY: f64 = 0.9;
const STRONGHOLD_NETHER_Y: &str = "40.00000";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Line {
    pub segments: Vec<Segment>
}

impl Line {
    pub fn new(
        stronghold: &Stronghold,
        boat: &Boat,
        messages: &InformationMessages,
        state: Option<&GameState>
    ) -> Self {
        if !state.is_some_and(GameState::is_playing) {
            return Self::default();
        }

        let prediction = stronghold.prediction();
        let has_details = stronghold.throws() > 0 || messages.warnings().next().is_some();
        let mut segments = vec![Self::boat(boat)];

        if let Some(prediction) = prediction {
            segments.push(Self::coordinates(prediction));

            if has_details {
                segments.push(Segment::text("|", Palette::Text, Weight::Regular));
            }
        }

        if has_details {
            segments.extend(Self::details(stronghold, prediction, messages));
        }

        Self { segments }
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    fn boat(boat: &Boat) -> Segment {
        Segment::new(vec![
            Piece::text(" ", Palette::Text, Weight::Regular),
            Piece::Boat(boat.boat_state),
            Piece::text(" ", Palette::Text, Weight::Regular),
        ])
    }

    fn coordinates(prediction: &Prediction) -> Segment {
        Segment::new(vec![
            Piece::text(" ", Palette::Text, Weight::Regular),
            Self::coordinate(prediction.nether_x()),
            Piece::text(
                format!(" / {STRONGHOLD_NETHER_Y} / "),
                Palette::Text,
                Weight::Regular
            ),
            Self::coordinate(prediction.nether_z()),
        ])
    }

    fn coordinate(value: i32) -> Piece {
        Piece::text(
            format!("{value}.000"),
            Self::sign_color(value),
            Weight::Bold
        )
    }

    fn details(
        stronghold: &Stronghold,
        prediction: Option<&Prediction>,
        messages: &InformationMessages
    ) -> Vec<Segment> {
        let mut segments = Vec::new();

        if let Some(prediction) = prediction {
            segments.push(Segment::text(
                format!("{:.1}%", prediction.certainty * 100.0),
                Self::certainty_color(prediction.certainty),
                Weight::Regular
            ));
            segments.push(Segment::text(
                format!(
                    "{} ({})",
                    prediction.distance(),
                    prediction.nether_distance()
                ),
                Palette::Text,
                Weight::Regular
            ));
        }

        let correction = stronghold.correction_increments();

        if correction != 0 {
            segments.push(Segment::text(
                format!("{correction:+}"),
                Self::sign_color(correction),
                Weight::Bold
            ));
        }

        if stronghold.throws() > 0 {
            segments.push(Segment::text(
                format!("t: {}", stronghold.throws()),
                Palette::Meta,
                Weight::Regular
            ));
        }

        let failed = stronghold.result_type == ResultType::Failed;

        if failed || (stronghold.throws() > 0 && stronghold.predictions.is_empty()) {
            segments.push(Segment::text(
                "reset measurements",
                Palette::Error,
                Weight::Bold
            ));
        }

        for message in messages.warnings() {
            let color = match message.severity {
                Severity::Error => Palette::Error,
                Severity::Warning | Severity::Info => Palette::Warning
            };
            segments.push(Segment::text(message.label(), color, Weight::Bold));
        }

        segments
    }

    fn sign_color(value: i32) -> Palette {
        if value < 0 {
            Palette::Negative
        } else {
            Palette::Positive
        }
    }

    fn certainty_color(certainty: f64) -> Palette {
        if certainty >= HIGH_CERTAINTY {
            Palette::HighCertainty
        } else if certainty >= MEDIUM_CERTAINTY {
            Palette::MediumCertainty
        } else {
            Palette::LowCertainty
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::game::InWorldState;
    use crate::ninjabrain::BoatState;

    const PLAYING: GameState = GameState::InWorld(InWorldState::Unpaused);

    fn stronghold(value: serde_json::Value) -> Stronghold {
        serde_json::from_value(value).unwrap()
    }

    fn triangulated() -> Stronghold {
        stronghold(json!({
            "resultType": "TRIANGULATION",
            "eyeThrows": [{ "correctionIncrements": 0 }, { "correctionIncrements": -2 }],
            "predictions": [
                { "chunkX": -12, "chunkZ": 85, "certainty": 0.9634, "overworldDistance": 1530.7 },
                { "chunkX": 10, "chunkZ": 80, "certainty": 0.02, "overworldDistance": 1700.0 }
            ]
        }))
    }

    fn texts(line: &Line) -> Vec<String> {
        line.segments
            .iter()
            .map(|segment| {
                segment
                    .pieces
                    .iter()
                    .map(|piece| match piece {
                        Piece::Text { text, .. } => text.clone(),
                        Piece::Boat(_) => "[boat]".to_string()
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn hidden_unless_playing() {
        let paused = GameState::InWorld(InWorldState::Paused);
        let empty = InformationMessages::default();
        let boat = Boat::default();

        assert!(Line::new(&triangulated(), &boat, &empty, Some(&paused)).is_empty());
        assert!(Line::new(&triangulated(), &boat, &empty, None).is_empty());
    }

    #[test]
    fn shows_only_the_boat_without_data() {
        let line = Line::new(
            &Stronghold::default(),
            &Boat::default(),
            &InformationMessages::default(),
            Some(&PLAYING)
        );

        assert_eq!(texts(&line), [" [boat] "]);
    }

    #[test]
    fn shows_triangulation_details() {
        let line = Line::new(
            &triangulated(),
            &Boat {
                boat_state: BoatState::Valid
            },
            &InformationMessages::default(),
            Some(&PLAYING)
        );

        assert_eq!(
            texts(&line),
            [
                " [boat] ",
                " -24.000 / 40.00000 / 170.000",
                "|",
                "96.3%",
                "1530 (191)",
                "-2",
                "t: 2"
            ]
        );
        assert_eq!(
            line.segments[3].pieces[0],
            Piece::text("96.3%", Palette::HighCertainty, Weight::Regular)
        );
    }

    #[test]
    fn asks_for_reset_when_throws_give_no_prediction() {
        let failed = stronghold(json!({
            "resultType": "FAILED",
            "eyeThrows": [{ "correctionIncrements": 0 }],
            "predictions": []
        }));
        let line = Line::new(
            &failed,
            &Boat::default(),
            &InformationMessages::default(),
            Some(&PLAYING)
        );

        assert_eq!(texts(&line), [" [boat] ", "t: 1", "reset measurements"]);
    }

    #[test]
    fn lists_warnings_but_not_info() {
        let messages: InformationMessages = serde_json::from_value(json!({
            "informationMessages": [
                { "severity": "INFO", "type": "NEXT_THROW_DIRECTION" },
                { "severity": "WARNING", "type": "MISMEASURE" },
                { "severity": "ERROR", "type": "PORTAL_LINKING" }
            ]
        }))
        .unwrap();
        let line = Line::new(
            &Stronghold::default(),
            &Boat::default(),
            &messages,
            Some(&PLAYING)
        );

        assert_eq!(texts(&line), [" [boat] ", "mismeasure", "portal linking"]);
        assert_eq!(
            line.segments[2].pieces[0],
            Piece::text("portal linking", Palette::Error, Weight::Bold)
        );
    }
}
