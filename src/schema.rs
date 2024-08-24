// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (token) {
        token -> Varchar,
        user_id -> Int8,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    cabinets (id) {
        name -> Varchar,
        id -> Int8,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    collections (id) {
        count -> Int4,
        drawer_id -> Int8,
        component_id -> Int8,
        id -> Int8,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    components (id) {
        code -> Varchar,
        description -> Nullable<Varchar>,
        datasheet_url -> Nullable<Varchar>,
        id -> Int8,
        search_text -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    drawers (id) {
        label -> Varchar,
        cabinet_id -> Int8,
        id -> Int8,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    federated_user_credential_uses (id) {
        claims -> Jsonb,
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    federated_user_credentials (id) {
        id -> Int8,
        uuid -> Uuid,
        subject -> Varchar,
        audience -> Varchar,
        issuer -> Varchar,
        user_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (id) {
        id -> Varchar,
        uuid -> Uuid,
    }
}

diesel::table! {
    refresh_tokens (token) {
        token -> Varchar,
        user_id -> Int8,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_permission_bindings (role_id, permission_id) {
        role_id -> Varchar,
        permission_id -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Varchar,
        uuid -> Uuid,
    }
}

diesel::table! {
    user_role_bindings (user_id, role_id) {
        user_id -> Int8,
        role_id -> Varchar,
        target -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        uuid -> Uuid,
        email -> Nullable<Varchar>,
        email_verified -> Bool,
        display_name -> Varchar,
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(collections -> components (component_id));
diesel::joinable!(collections -> drawers (drawer_id));
diesel::joinable!(drawers -> cabinets (cabinet_id));
diesel::joinable!(federated_user_credentials -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(role_permission_bindings -> permissions (permission_id));
diesel::joinable!(role_permission_bindings -> roles (role_id));
diesel::joinable!(user_role_bindings -> roles (role_id));
diesel::joinable!(user_role_bindings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    cabinets,
    collections,
    components,
    drawers,
    federated_user_credential_uses,
    federated_user_credentials,
    permissions,
    refresh_tokens,
    role_permission_bindings,
    roles,
    user_role_bindings,
    users,
);
