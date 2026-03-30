use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        d1202,
        common::{
            self,
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                GamingMode, LowBatteryPrompt, button_configuration::ButtonStatusCollection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1202StateUpdate {
    pub tws_status: common::structures::TwsStatus,
    pub dual_battery: common::structures::DualBattery,
    pub dual_firmware_version: common::structures::DualFirmwareVersion,
    pub serial_number: common::structures::SerialNumber,
    pub equalizer_configuration: common::structures::CommonEqualizerConfiguration<1, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: common::structures::AmbientSoundModeCycle,
    pub sound_modes: crate::devices::soundcore::a3959::structures::SoundModes,
    pub touch_tone: common::structures::TouchTone,
    pub auto_power_off: common::structures::AutoPowerOff,
    pub low_battery_prompt: LowBatteryPrompt,
    pub gaming_mode: GamingMode,
}

impl Default for D1202StateUpdate {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: d1202::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            auto_power_off: Default::default(),
            low_battery_prompt: Default::default(),
            gaming_mode: Default::default(),
        }
    }
}

impl FromPacketBody for D1202StateUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1202 state update packet",
            map(
                (
                    common::structures::TwsStatus::take,
                    take(4usize),
                    common::structures::FirmwareVersion::take,
                    common::structures::FirmwareVersion::take,
                    common::structures::SerialNumber::take,
                    take(6usize),
                    common::structures::CommonEqualizerConfiguration::take,
                    take(10usize),
                    take(1usize),
                    ButtonStatusCollection::take::<E, 8>(
                        d1202::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    common::structures::AmbientSoundModeCycle::take,
                    crate::devices::soundcore::a3959::structures::SoundModes::take,
                    take(1usize),
                    common::structures::TouchTone::take,
                    take(2usize),
                    common::structures::AutoPowerOff::take,
                    LowBatteryPrompt::take,
                    GamingMode::take,
                ),
                |(
                    tws_status,
                    raw_battery,
                    fw1,
                    fw2,
                    serial_number,
                    _firmware_extra,
                    equalizer_configuration,
                    _unknown1,
                    _unknown2,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown3,
                    touch_tone,
                    _unknown4,
                    auto_power_off,
                    low_battery_prompt,
                    gaming_mode,
                )| {
                    let dual_battery = common::structures::DualBattery {
                        left: common::structures::SingleBattery {
                            level: common::structures::BatteryLevel(raw_battery[0]),
                            is_charging: if raw_battery[2] == 1 { common::structures::IsBatteryCharging::Yes } else { common::structures::IsBatteryCharging::No },
                        },
                        right: common::structures::SingleBattery {
                            level: common::structures::BatteryLevel(raw_battery[1]),
                            is_charging: if raw_battery[3] == 1 { common::structures::IsBatteryCharging::Yes } else { common::structures::IsBatteryCharging::No },
                        },
                    };

                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version: common::structures::DualFirmwareVersion::Both {
                            left: fw1,
                            right: fw2,
                        },
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        touch_tone,
                        auto_power_off,
                        low_battery_prompt,
                        gaming_mode,
                    }
                },
            ),
        )
        .parse(input)
    }
}

impl ToPacket for D1202StateUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        Vec::new() 
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<d1202::state::D1202State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<d1202::state::D1202State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: D1202StateUpdate = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<d1202::state::D1202State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::VerboseError;

    #[test]
    fn test_parse_p31i_packet() {
        let packet_body = vec![
            1, 1, 5, 9, 0, 0, 48, 50, 46, 56, 51, 48, 50, 46, 56, 51, 49, 50, 48, 50, 51, 52, 48, 57, 67, 57, 65, 69, 65, 70, 56, 52, 48, 50, 46, 56, 51, 8, 254, 254, 151, 154, 162, 166, 167, 168, 168, 165, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 10, 246, 246, 98, 99, 241, 240, 68, 79, 55, 2, 81, 0, 1, 0, 0, 0, 1, 49, 1, 1, 1, 1, 1, 0, 1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 255, 255, 255, 255, 255
        ];
        let result = D1202StateUpdate::take::<VerboseError<&[u8]>>(&packet_body);
        assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
        let (_, update) = result.unwrap();
        assert_eq!(update.tws_status.connected, true);
        assert_eq!(update.dual_battery.left.level.0, 5);
        assert_eq!(update.dual_battery.right.level.0, 9);
        assert_eq!(update.serial_number.as_str(), "12023409C9AEAF84");
    }
}
