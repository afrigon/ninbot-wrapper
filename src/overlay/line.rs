use super::Palette;
use super::Piece;
use super::Segment;
use crate::game::GameState;
use crate::ninjabrain::Boat;
use crate::ninjabrain::Dimension;
use crate::ninjabrain::InformationMessages;
use crate::ninjabrain::PlayerPosition;
use crate::ninjabrain::Prediction;
use crate::ninjabrain::ResultType;
use crate::ninjabrain::Stronghold;

const HIGH_CERTAINTY: f64 = 0.95;
const MEDIUM_CERTAINTY: f64 = 0.9;
const STRONGHOLD_Y: i32 = 40;
const SEPARATOR: &str = "|";
const LEFT_TURN_ARROW: &str = "< ";
const RIGHT_TURN_ARROW: &str = " >";

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

        if let Some(segments) = Self::fatal(stronghold, messages) {
            return Self { segments };
        }

        let dimension = stronghold
            .player_position
            .as_ref()
            .map(PlayerPosition::dimension);
        let sections = match dimension {
            Some(Dimension::Nether) => Self::nether_sections(stronghold),
            Some(Dimension::Overworld | Dimension::End) | None => {
                Self::overworld_sections(stronghold, messages)
            }
        };
        let mut segments = vec![Self::boat(boat)];

        for (index, section) in sections
            .into_iter()
            .filter(|section| !section.is_empty())
            .enumerate()
        {
            if index > 0 {
                segments.push(Segment::text(SEPARATOR, Palette::Text));
            }

            segments.extend(section);
        }

        Self { segments }
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    fn overworld_sections(
        stronghold: &Stronghold,
        messages: &InformationMessages
    ) -> Vec<Vec<Segment>> {
        let prediction = stronghold.prediction();

        vec![
            prediction.map(Self::coordinates).into_iter().collect(),
            prediction
                .map(|prediction| {
                    Segment::text(
                        format!(
                            "{} ({})",
                            prediction.distance(),
                            prediction.nether_distance()
                        ),
                        Palette::Text
                    )
                })
                .into_iter()
                .collect(),
            Self::turn(stronghold).into_iter().collect(),
            Self::accuracy(stronghold),
            Self::warnings(messages),
            Self::next_throw(messages).into_iter().collect(),
        ]
    }

    fn nether_sections(stronghold: &Stronghold) -> Vec<Vec<Segment>> {
        let prediction = stronghold.prediction();

        vec![
            prediction.map(Self::coordinates).into_iter().collect(),
            prediction
                .map(|prediction| {
                    Segment::text(prediction.nether_distance().to_string(), Palette::Text)
                })
                .into_iter()
                .collect(),
            Self::turn(stronghold).into_iter().collect(),
        ]
    }

    fn boat(boat: &Boat) -> Segment {
        Segment::new(vec![Piece::Boat(boat.boat_state)])
    }

    fn coordinates(prediction: &Prediction) -> Segment {
        Segment::new(vec![
            Self::coordinate(prediction.nether_x()),
            Piece::text(format!(" {STRONGHOLD_Y} "), Palette::Text),
            Self::coordinate(prediction.nether_z()),
        ])
    }

    fn coordinate(value: i32) -> Piece {
        Piece::text(value.to_string(), Self::sign_color(value))
    }

    fn turn(stronghold: &Stronghold) -> Option<Segment> {
        let prediction = stronghold.prediction()?;
        let player = stronghold.player_position.as_ref()?;
        let turn = prediction.turn_angle(player);
        let degrees = turn.abs().round() as u8;
        let color = Palette::Turn(degrees);

        if turn > 0.0 {
            Some(Segment::new(vec![
                Piece::Hidden(LEFT_TURN_ARROW.to_string()),
                Piece::text(format!("{degrees}{RIGHT_TURN_ARROW}"), color),
            ]))
        } else {
            Some(Segment::new(vec![
                Piece::text(format!("{LEFT_TURN_ARROW}{degrees}"), color),
                Piece::Hidden(RIGHT_TURN_ARROW.to_string()),
            ]))
        }
    }

    fn accuracy(stronghold: &Stronghold) -> Vec<Segment> {
        let mut segments = Vec::new();

        if let Some(prediction) = stronghold.prediction() {
            segments.push(Segment::text(
                format!("{:.1}%", prediction.certainty * 100.0),
                Self::certainty_color(prediction.certainty)
            ));
        }

        segments.extend(Self::adjustment(stronghold));
        segments
    }

    fn adjustment(stronghold: &Stronghold) -> Option<Segment> {
        let correction = stronghold.correction_increments();

        if correction == 0 {
            return None;
        }

        Some(Segment::text(
            format!("{correction:+}"),
            Self::sign_color(correction)
        ))
    }

    fn fatal(stronghold: &Stronghold, messages: &InformationMessages) -> Option<Vec<Segment>> {
        let mut segments = Vec::new();
        let failed = stronghold.result_type == ResultType::Failed;

        if failed || (stronghold.throws() > 0 && stronghold.predictions.is_empty()) {
            segments.push(Segment::text("reset measurements", Palette::Error));
        }

        for message in messages.fatal() {
            segments.push(Segment::text(message.label(), Palette::Error));
        }

        if segments.is_empty() {
            return None;
        }

        if messages.fatal().any(|message| message.keeps_adjustment()) {
            segments.extend(Self::adjustment(stronghold));
        }

        Some(segments)
    }

    fn warnings(messages: &InformationMessages) -> Vec<Segment> {
        messages
            .warnings()
            .map(|message| Segment::text(message.label(), Palette::Warning))
            .collect()
    }

    fn next_throw(messages: &InformationMessages) -> Option<Segment> {
        let next_throw = messages.next_throw()?;

        Some(Segment::text(
            format!(
                "{LEFT_TURN_ARROW}{} {SEPARATOR} {}{RIGHT_TURN_ARROW} for {}%",
                next_throw.left, next_throw.right, next_throw.certainty
            ),
            Palette::Text
        ))
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

    fn triangulated(in_nether: bool) -> Stronghold {
        stronghold(json!({
            "resultType": "TRIANGULATION",
            "eyeThrows": [{ "correctionIncrements": 0 }, { "correctionIncrements": -2 }],
            "predictions": [
                { "chunkX": -12, "chunkZ": 85, "certainty": 0.9634, "overworldDistance": 1530.7 },
                { "chunkX": 10, "chunkZ": 80, "certainty": 0.02, "overworldDistance": 1700.0 }
            ],
            "playerPosition": {
                "xInOverworld": 0.0,
                "zInOverworld": 0.0,
                "horizontalAngle": 0.0,
                "isInOverworld": !in_nether,
                "isInNether": in_nether
            }
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
                        Piece::Hidden(text) => " ".repeat(text.len()),
                        Piece::Boat(_) => "[boat]".to_string()
                    })
                    .collect()
            })
            .collect()
    }

    fn line(stronghold: &Stronghold, messages: &InformationMessages) -> Line {
        Line::new(
            stronghold,
            &Boat {
                boat_state: BoatState::Valid
            },
            messages,
            Some(&PLAYING)
        )
    }

    #[test]
    fn hidden_unless_playing() {
        let paused = GameState::InWorld(InWorldState::Paused);
        let empty = InformationMessages::default();
        let boat = Boat::default();

        assert!(Line::new(&triangulated(false), &boat, &empty, Some(&paused)).is_empty());
        assert!(Line::new(&triangulated(false), &boat, &empty, None).is_empty());
    }

    #[test]
    fn shows_only_the_boat_without_data() {
        let line = line(&Stronghold::default(), &InformationMessages::default());

        assert_eq!(texts(&line), ["[boat]"]);
    }

    fn messages(value: serde_json::Value) -> InformationMessages {
        serde_json::from_value(json!({ "informationMessages": value })).unwrap()
    }

    #[test]
    fn shows_every_section_after_an_overworld_measurement() {
        let messages = messages(json!([
            { "severity": "WARNING", "type": "PORTAL_LINKING", "message": "" },
            {
                "severity": "INFO",
                "type": "NEXT_THROW_DIRECTION",
                "message": "Go left 12 blocks, or right 15 blocks, for ~95% certainty after next measurement."
            },
            { "severity": "INFO", "type": "COMBINED_CERTAINTY", "message": "" }
        ]));
        let line = line(&triangulated(false), &messages);

        assert_eq!(
            texts(&line),
            [
                "[boat]",
                "-24 40 170",
                "|",
                "1530 (191)",
                "|",
                "  8 >",
                "|",
                "96.3%",
                "-2",
                "|",
                "portal linking",
                "|",
                "< 12 | 15 > for 95%"
            ]
        );
        assert_eq!(
            line.segments[5].pieces[1],
            Piece::text("8 >", Palette::Turn(8))
        );
    }

    #[test]
    fn shows_only_travel_details_after_a_nether_measurement() {
        let messages = messages(json!([
            { "severity": "WARNING", "type": "PORTAL_LINKING", "message": "" }
        ]));
        let line = line(&triangulated(true), &messages);

        assert_eq!(
            texts(&line),
            ["[boat]", "-24 40 170", "|", "191", "|", "  8 >"]
        );
    }

    #[test]
    fn turns_left_with_the_arrow_first() {
        let mut stronghold = triangulated(false);
        stronghold
            .player_position
            .as_mut()
            .unwrap()
            .horizontal_angle = 90.0;
        let line = line(&stronghold, &InformationMessages::default());

        assert_eq!(texts(&line)[5], "< 82  ");
    }

    #[test]
    fn omits_the_turn_without_a_player_position() {
        let mut stronghold = triangulated(false);
        stronghold.player_position = None;
        let line = line(&stronghold, &InformationMessages::default());

        assert_eq!(
            texts(&line),
            [
                "[boat]",
                "-24 40 170",
                "|",
                "1530 (191)",
                "|",
                "96.3%",
                "-2"
            ]
        );
    }

    #[test]
    fn reset_is_fatal() {
        let failed = stronghold(json!({
            "resultType": "FAILED",
            "eyeThrows": [{ "correctionIncrements": -1 }],
            "predictions": []
        }));
        let line = line(&failed, &InformationMessages::default());

        assert_eq!(texts(&line), ["reset measurements"]);
    }

    #[test]
    fn fatal_messages_hide_the_data_and_the_boat() {
        let messages = messages(json!([
            { "severity": "WARNING", "type": "MC_VERSION", "message": "" },
            { "severity": "ERROR", "type": "SOMETHING_NEW", "message": "" },
            { "severity": "WARNING", "type": "PORTAL_LINKING", "message": "" }
        ]));
        let line = line(&triangulated(false), &messages);

        assert_eq!(texts(&line), ["wrong minecraft version", "something new"]);
        assert_eq!(
            line.segments[0].pieces[0],
            Piece::text("wrong minecraft version", Palette::Error)
        );
    }

    #[test]
    fn mismeasure_keeps_the_adjustment() {
        let messages = messages(json!([
            { "severity": "WARNING", "type": "MISMEASURE", "message": "" }
        ]));
        let line = line(&triangulated(false), &messages);

        assert_eq!(texts(&line), ["mismeasure", "-2"]);
    }
}
