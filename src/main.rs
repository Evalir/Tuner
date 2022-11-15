const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const NOTES_DIM: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

#[derive(Default)]
enum Tuning {
    #[default]
    StandardTuning,
    DropTuning,
}

fn get_note_index(note: &str) -> usize {
    NOTES
        .iter()
        .position(|&x| x == note)
        .expect("Note not found")
}

fn get_notes(root: &str, tuning: &Tuning) -> Vec<String> {
    let mut notes = vec![root.to_string()];

    for x in (1..6).rev() {
        match (x, tuning) {
            (5, Tuning::DropTuning) => {
                notes.push(NOTES[((get_note_index(notes.last().unwrap()) + 7) % 12)].to_string())
            }
            (2, _) => {
                notes.push(NOTES[((get_note_index(notes.last().unwrap()) + 4) % 12)].to_string())
            }
            _ => notes.push(NOTES[((get_note_index(notes.last().unwrap()) + 5) % 12)].to_string()),
        }
    }

    notes
}

fn main() {
    let notes = get_notes("D", &Tuning::DropTuning);
    println!("{:?}", notes);
}
