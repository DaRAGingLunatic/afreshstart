// Bevy Events:
// Events are used to send data between systems.
// Events are just rust structs.
// They can be empty or contain data.
// We send events using an EventWriter<T>
// We receive events using an EventReader<T>
// Lets say we have a GameOver Event.
// System A can send out that event using an EventWriter<GameOver>.
// One or more other systems B, C, D, ... can read that event using an EventReader<GameOver>
// Depending on how your systems are run, you can have either one of the following behaviours:
// 1. System A runs before System B.
// * System A sends an Event, System B receives the event in the same frame.
// 2. System B runs before System A.
// * System A sends an Event, System B receives the Event in the following frame.
// Events only exist from the time they are sent in a frame, to the end of the next frame.

pub struct GameOver {
    pub score: u32,
}
