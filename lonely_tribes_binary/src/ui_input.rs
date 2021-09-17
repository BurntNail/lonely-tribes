use crate::states::states_util::load_font;
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::VirtualKeyCode,
    ui::{Anchor, LineMode, UiText, UiTransform},
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct UiTextInput {
    pub ent: Entity,
    text: String,
}

impl UiTextInput {
    #[allow(dead_code)]
    pub fn new(
        default_text: String,
        world: &mut World,
        font: &str,
        size: f32,
        ui_trans: UiTransform,
    ) -> Self {
        let ui_txt = UiText::new(
            load_font(world, font),
            default_text.clone(),
            [1.0; 4],
            size,
            LineMode::Wrap,
            Anchor::Middle,
        );
        let ent = world.create_entity().with(ui_trans).with(ui_txt).build();

        Self {
            text: default_text,
            ent,
        }
    }
    #[allow(dead_code)]
    pub fn handle_input(&mut self, input: VirtualKeyCode, world: &mut World) {
        let inp = format!("{:?}", input).to_lowercase();
        if "abcdefghijklmnopqrstuvwxyz1234567890".contains(&inp) {
            self.text.push_str(&inp);

            if let Some(txt) = world.write_storage::<UiText>().get_mut(self.ent) {
                txt.text = self.text.clone();
            }
        }
    }
    #[allow(dead_code)]
    pub fn get(&self) -> &str {
        self.text.trim()
    }
}
