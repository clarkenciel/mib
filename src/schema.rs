table! {
    notes (id) {
        id -> Int4,
        note_id -> Uuid,
        content -> Text,
        publication_date -> Timestamptz,
        initial_location -> Point,
        current_location -> Point,
    }
}
