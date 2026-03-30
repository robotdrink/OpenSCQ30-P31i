use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    d1202,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, DualBattery, DualFirmwareVersion, GamingMode,
            LowBatteryPrompt, SerialNumber, TwsStatus, TouchTone,
            button_configuration::ButtonStatusCollection, EqualizerConfiguration,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1202State {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<1, 10, -120, 134, 1>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: crate::devices::soundcore::a3959::structures::SoundModes,
    pub touch_tone: TouchTone,
    pub auto_power_off: AutoPowerOff,
    pub low_battery_prompt: LowBatteryPrompt,
    pub gaming_mode: GamingMode,
    pub button_reset_pending: ResetButtonConfigurationPending,
}

impl Default for D1202State {
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
            button_reset_pending: Default::default(),
        }
    }
}

impl From<super::packets::inbound::D1202StateUpdate> for D1202State {
    fn from(packet: super::packets::inbound::D1202StateUpdate) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            touch_tone: packet.touch_tone,
            auto_power_off: packet.auto_power_off,
            low_battery_prompt: packet.low_battery_prompt,
            gaming_mode: packet.gaming_mode,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}
