use std::collections::HashMap;

use crate::{
    api::connection::RfcommServiceSelectionStrategy,
    devices::soundcore::{
        self,
        common::{
            device::{SoundcoreDeviceConfig, fetch_state_from_state_update_packet},
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
                },
                equalizer,
            },
            packet,
            packet::outbound::{RequestState, ToPacket},
            structures::button_configuration::{
                ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
            },
        },
        d1202::{packets::inbound::D1202StateUpdate, state::D1202State},
    },
};

pub mod modules;
pub mod packets;
pub mod state;
pub mod structures;

soundcore_device!(
    D1202State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, D1202State, D1202StateUpdate>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3959_sound_modes();

        builder
            .equalizer_with_drc_tws_no_wait(equalizer::common_settings())
            .await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();

        builder.reset_button_configuration::<D1202StateUpdate>(RequestState::default().to_packet());

        builder.touch_tone();
        builder.tws_status();
        builder.low_battery_prompt();
        builder.gaming_mode();

        builder.dual_battery(10);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            D1202StateUpdate::default().to_packet(),
        )])
    },
    SoundcoreDeviceConfig {
        checksum_kind: packet::ChecksumKind::Suffix,
        rfcomm_service_selection_strategy: RfcommServiceSelectionStrategy::Dynamic(
            |service_uuids| {
                service_uuids
                    .into_iter()
                    .find(soundcore::is_soundcore_vendor_rfcomm_uuid)
                    .unwrap_or(soundcore::RFCOMM_UUID)
            },
        ),
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftTriplePress,
            Button::RightTriplePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };
