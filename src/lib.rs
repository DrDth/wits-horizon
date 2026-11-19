//! WITS (What is this structure?) for Horizon: `/wits` lists the registry names of the structures
//! whose start's box holds the block the player stands in, `/witsop <dimension> <location>` the
//! ones at any position in any dimension, for operators.
//!
//! A port of TelepathicGrunt's WITS (MIT), from its 1.21 sources (1.3.1). The structure read is
//! `LevelRef::structures_at`, vanilla's `StructureManager.startsForStructure` for the chunk
//! followed by the start box test the original runs.
use mod_api::text::{TextClick, TextStyle};
use mod_api::{
    Command, CommandArgumentValue, CommandSource, Flow, Hooks, LevelRef, RegisterCommandsEvent,
    RunModCommandEvent, ServerHost, Text, TextHover,
};

#[derive(Default)]
pub struct Wits;

/// `ComponentUtils.copyOnClickText`: the text in green, copied on a click, inserted on a
/// shift-click, with "Click to Copy to Clipboard" on hover, in square brackets.
fn copy_on_click(text: &str) -> Text {
    let mut style = TextStyle::default();
    style.color = Some("green".to_owned());
    style.click = Some(TextClick::CopyToClipboard(text.to_owned()));
    style.hover = Some(TextHover::Text(Box::new(Text::translate(
        "chat.copy.click",
        Vec::new(),
    ))));
    style.insertion = Some(text.to_owned());
    Text::translate(
        "chat.square_brackets",
        vec![Text::literal(text).with_style(style)],
    )
}

/// `WITSCommand.listStructuresAtSpot`: the structures at `pos` of `level`, or that there are
/// none, as the command's feedback.
fn list_structures_at_spot(
    host: &mut dyn ServerHost,
    level: LevelRef,
    pos: [i32; 3],
    caller_position: bool,
) {
    // The original loads the chunk (`startsForStructure` asks for it at STRUCTURE_REFERENCES);
    // a mod's reads load none, so an unloaded spot answers as `/setblock` does there.
    let Some(structures) = level.structures_at(host, pos) else {
        let mut red = TextStyle::default();
        red.color = Some("red".to_owned());
        let line = Text::translate("argument.pos.unloaded", Vec::new()).with_style(red);
        let _ = host.commands().feedback(line);
        return;
    };
    let line = if structures.is_empty() {
        Text::literal(if caller_position {
            "There's no structures at your location."
        } else {
            "There's no structures at the location."
        })
    } else {
        let mut line = if caller_position {
            Text::literal("").append(Text::literal("Structure(s) at your location:"))
        } else {
            let [x, y, z] = pos.map(|c| Text::literal(c.to_string()));
            Text::literal("")
                .append(Text::literal("Structure(s) at "))
                .append(Text::translate("chat.coordinates", vec![x, y, z]))
                .append(Text::literal(":"))
        };
        for found in &structures {
            // The original's "§r\n - §6": its codes style nothing after them in that literal,
            // and the name that follows keeps its own green.
            line = line
                .append(Text::literal("\n - "))
                .append(copy_on_click(&found.structure));
        }
        line
    };
    let _ = host.commands().feedback(line);
}

#[mod_api::horizon_mod]
impl Hooks for Wits {
    fn register_commands(&self, _: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        event.commands.extend([
            Command::new("wits"),
            Command::op("witsop <dimension:dimension> <location:vec3>", 2),
        ]);
    }

    fn run_mod_command(
        &self,
        host: &mut dyn ServerHost,
        event: &mut RunModCommandEvent<'_>,
    ) -> Flow {
        let command = event.command;
        match command.literals.first().map(String::as_str) {
            Some("wits") => {
                // The level the command runs in; a player's own block, else the origin.
                let Some(level) = command
                    .dimension
                    .id(host)
                    .and_then(|id| host.level(&id))
                else {
                    return Flow::Cancel;
                };
                let pos = match event.source {
                    CommandSource::Player { player, .. } => player
                        .entity()
                        .block_position(host)
                        .unwrap_or([0, 0, 0]),
                    _ => [0, 0, 0],
                };
                list_structures_at_spot(host, level, pos, true);
            }
            Some("witsop") => {
                let (
                    Some(CommandArgumentValue::Dimension(level)),
                    Some(CommandArgumentValue::Vec3(location)),
                ) = (command.argument("dimension"), command.argument("location"))
                else {
                    return Flow::Continue;
                };
                // `Coordinates.getBlockPos`: the block the resolved position is in.
                let pos = location.map(|c| c.floor() as i32);
                list_structures_at_spot(host, *level, pos, false);
            }
            _ => return Flow::Continue,
        }
        Flow::Cancel
    }
}
