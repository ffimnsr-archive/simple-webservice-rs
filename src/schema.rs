table! {
    bug_reports (id) {
        id -> Uuid,
        title -> Text,
        organization_affected -> Text,
        content -> Text,
        wallet_address -> Text,
        feedback -> Text,
        status -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
