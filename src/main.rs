const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const NOTES_DIM: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

enum Tuning {
    StandardTuning,
    DropTuning,
}

fn get_note_index(note: &str) -> usize {
    NOTES
        .iter()
        .position(|&x| x == note)
        .expect("Note not found")
}

fn get_notes(root: &str) -> Vec<String> {
    let mut notes = vec![root.to_string()];

    // 5th string
    notes.push(NOTES[((get_note_index(root) + 5) % 12)].to_string());
    // 4th string
    notes.push(NOTES[((get_note_index(notes.get(1).unwrap()) + 5) % 12)].to_string());
    // 3rd string
    notes.push(NOTES[((get_note_index(notes.get(2).unwrap()) + 5) % 12)].to_string());
    // 2nd string (it's offset by one semitone)
    notes.push(NOTES[((get_note_index(notes.get(3).unwrap()) + 4) % 12)].to_string());
    // 1st string
    notes.push(NOTES[((get_note_index(notes.get(4).unwrap()) + 5) % 12)].to_string());

    notes
}

fn main() {
    let notes = get_notes("C");
    println!("{:?}", notes);
}
