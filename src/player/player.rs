pub struct Player {
    name: String,
    action_state: Option<ActionState>,
}

impl Player {
    pub fn collect_artefact(&mut self) -> bool {
        unimplemented!()
    }
}

impl CardTransferer for Player;