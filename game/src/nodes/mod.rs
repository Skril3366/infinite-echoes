use audio_effect::AudioEffect;
use note_effect::NoteEffect;
use note_effect::NoteEffectType;
use note_effect::Scale;
use note_generator::NoteGenerator;
use oscillator::Oscillator;

use crate::nodes::audio_effect::DistortionCurve;
use crate::render::widgets::card_widget::CardType;

pub mod audio_effect;
pub mod audio_graph;
pub mod note_effect;
pub mod note_generator;
pub mod oscillator;

#[derive(PartialEq, Clone)]
pub enum AudioNode {
    NoteGenerator(NoteGenerator),
    NoteEffect(NoteEffect),
    Oscillator(Oscillator),
    AudioEffect(AudioEffect),
}

impl AudioNode {
    pub fn as_note_effect(&self) -> Option<&NoteEffect> {
        match self {
            AudioNode::NoteEffect(effect) => Some(effect),
            _ => None,
        }
    }
}

impl AudioNode {
    pub fn as_note_generator(&self) -> Option<&NoteGenerator> {
        match self {
            AudioNode::NoteGenerator(note_generator) => Some(note_generator),
            _ => None,
        }
    }

    pub fn from_card(card: &CardType) -> Self {
        match card {
            CardType::NoteGenerator(Some(note_name)) => {
                Self::NoteGenerator(NoteGenerator::from_note_name(*note_name))
            }
            CardType::NoteGenerator(None) => {
                Self::NoteGenerator(NoteGenerator::empty())
            }
            CardType::Oscillator(wave) => Self::Oscillator(Oscillator::new(*wave)),
            CardType::Filter(filter_type) => {
                Self::AudioEffect(AudioEffect::new_filter(*filter_type, 1000.0, 1.0, 0.0))
            }
            CardType::Distortion => {
                Self::AudioEffect(AudioEffect::new_distortion(0.03, DistortionCurve::SoftClip))
            }
            CardType::Reverb => Self::AudioEffect(AudioEffect::new_reverb(2.0, 0.5, 0.5)),
            CardType::ChordInScale(root, scale_type) => Self::NoteEffect(NoteEffect::new(
                NoteEffectType::Scale(Scale::new(*root, *scale_type)),
            )),
            CardType::ChangeLen(amount) => {
                Self::NoteEffect(NoteEffect::new(NoteEffectType::ChangeLen(*amount)))
            }
            CardType::BlankNoteEffect => Self::NoteEffect(NoteEffect::new(NoteEffectType::Blank)),
        }
    }

    pub fn as_type(&self) -> AudioNodeType {
        match self {
            AudioNode::NoteGenerator(_) => AudioNodeType::NoteGenerator,
            AudioNode::NoteEffect(_) => AudioNodeType::NoteEffect,
            AudioNode::Oscillator(_) => AudioNodeType::Oscillator,
            AudioNode::AudioEffect(_) => AudioNodeType::AudioEffect,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AudioNodeType {
    NoteGenerator,
    NoteEffect,
    Oscillator,
    AudioEffect,
}

impl AudioNodeType {
    pub fn can_put_between_strict(
        &self,
        before: &Option<AudioNodeType>,
        after: &Option<AudioNodeType>,
    ) -> bool {
        match (before, after) {
            (None, None) => true,
            (None, Some(after)) => self.allowed_after(after),
            (Some(before), None) => self.allowed_before(before),
            (Some(before), Some(after)) => self.allowed_before(before) && self.allowed_after(after),
        }
    }

    pub fn can_put_between_loose(
        &self,
        before: &Option<AudioNodeType>,
        after: &Option<AudioNodeType>,
    ) -> bool {
        match (before, after) {
            (None, None) => true,
            (None, Some(after)) => self.allowed_after_loose(after),
            (Some(before), None) => self.allowed_before_loose(before),
            (Some(before), Some(after)) => {
                self.allowed_before_loose(before) && self.allowed_after_loose(after)
            }
        }
    }

    fn allowed_before(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::NoteEffect => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::Oscillator, AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    fn allowed_before_loose(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::NoteEffect => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    fn allowed_after(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                ]
            }
            AudioNodeType::NoteEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                ]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::AudioEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    fn allowed_after_loose(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
            AudioNodeType::NoteEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::AudioEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }
}
