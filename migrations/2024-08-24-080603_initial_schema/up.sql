CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE FUNCTION update_updated_at_col() RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END
        $$ LANGUAGE plpgsql;;

CREATE TABLE cabinets (
    name VARCHAR NOT NULL, 
    id BIGSERIAL NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TRIGGER update_cabinets_updated_at_trigger
            BEFORE UPDATE ON cabinets
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_cabinets_uuid ON cabinets (uuid);

CREATE TABLE components (
    code VARCHAR NOT NULL, 
    description VARCHAR, 
    datasheet_url VARCHAR, 
    id BIGSERIAL NOT NULL, 
    search_text VARCHAR GENERATED ALWAYS AS (lower(coalesce(code, '') || coalesce(description, ''))) STORED NOT NULL,
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TRIGGER update_components_updated_at_trigger
            BEFORE UPDATE ON components
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_components_uuid ON components (uuid);
CREATE INDEX idx_components_trigrams ON components USING gin (search_text gin_trgm_ops);

CREATE TABLE drawers (
    label VARCHAR NOT NULL, 
    cabinet_id BIGINT NOT NULL, 
    id BIGSERIAL NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id), 
    FOREIGN KEY(cabinet_id) REFERENCES cabinets (id)
);

CREATE TRIGGER update_drawers_updated_at_trigger
            BEFORE UPDATE ON drawers
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_drawers_uuid ON drawers (uuid);
CREATE INDEX idx_drawers_cabinet ON drawers (cabinet_id);

CREATE TABLE collections (
    count INTEGER NOT NULL, 
    drawer_id BIGINT NOT NULL, 
    component_id BIGINT NOT NULL, 
    id BIGSERIAL NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id), 
    CHECK (count >= 0), 
    FOREIGN KEY(component_id) REFERENCES components (id), 
    FOREIGN KEY(drawer_id) REFERENCES drawers (id)
);

CREATE TRIGGER update_collections_updated_at_trigger
            BEFORE UPDATE ON collections
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_collections_uuid ON collections (uuid);
CREATE INDEX idx_collections_component ON collections (component_id);
CREATE INDEX idx_collections_drawer ON collections (drawer_id);

CREATE TABLE users (
    id BIGSERIAL NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    email VARCHAR, 
    email_verified BOOLEAN DEFAULT 'f' NOT NULL, 
    display_name VARCHAR NOT NULL, 
    avatar_url VARCHAR, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TRIGGER update_users_updated_at_trigger
            BEFORE UPDATE ON users
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_users_uuid ON users (uuid);

CREATE TABLE access_tokens (
    token VARCHAR NOT NULL, 
    user_id BIGINT NOT NULL, 
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (token), 
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE TRIGGER update_access_tokens_updated_at_trigger
            BEFORE UPDATE ON access_tokens
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_access_tokens_expires_at ON access_tokens (expires_at);

CREATE TABLE federated_user_credentials (
    id BIGSERIAL NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    subject VARCHAR NOT NULL, 
    audience VARCHAR NOT NULL, 
    issuer VARCHAR NOT NULL, 
    user_id BIGINT NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id), 
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE TRIGGER update_federated_user_credentials_updated_at_trigger
            BEFORE UPDATE ON federated_user_credentials
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_federated_user_credentials_uuid ON federated_user_credentials (uuid);
CREATE INDEX idx_federated_user_credentials_user ON federated_user_credentials (user_id);
CREATE UNIQUE INDEX idx_federated_user_credentials_subject_audience_issuer ON federated_user_credentials (subject, audience, issuer);

CREATE TABLE refresh_tokens (
    token VARCHAR NOT NULL, 
    user_id BIGINT NOT NULL, 
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL, 
    used_at TIMESTAMP WITH TIME ZONE, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (token), 
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE TRIGGER update_refresh_tokens_updated_at_trigger
            BEFORE UPDATE ON refresh_tokens
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens (expires_at);

CREATE TABLE federated_user_credential_uses (
    claims JSONB DEFAULT json_build_object() NOT NULL, 
    id BIGSERIAL NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TRIGGER update_federated_user_credential_uses_updated_at_trigger
            BEFORE UPDATE ON federated_user_credential_uses
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_federated_user_credential_uses_claims ON federated_user_credential_uses USING gin (claims);

CREATE TABLE permissions (
    id VARCHAR NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TABLE roles (
    id VARCHAR NOT NULL, 
    uuid UUID DEFAULT gen_random_uuid() NOT NULL, 
    PRIMARY KEY (id)
);

CREATE TABLE role_permission_bindings (
    role_id VARCHAR NOT NULL, 
    permission_id VARCHAR NOT NULL, 
    PRIMARY KEY (role_id, permission_id), 
    FOREIGN KEY(permission_id) REFERENCES permissions (id), 
    FOREIGN KEY(role_id) REFERENCES roles (id)
);

INSERT INTO roles (id) VALUES ('role/cabinetOwner');
INSERT INTO roles (id) VALUES ('role/cabinetViewer');
INSERT INTO permissions (id) VALUES ('cabinet.read');
INSERT INTO permissions (id) VALUES ('cabinet.write');
INSERT INTO role_permission_bindings (role_id, permission_id) VALUES ('role/cabinetOwner', 'cabinet.read');
INSERT INTO role_permission_bindings (role_id, permission_id) VALUES ('role/cabinetOwner', 'cabinet.write');
INSERT INTO role_permission_bindings (role_id, permission_id) VALUES ('role/cabinetViewer', 'cabinet.read');

CREATE TABLE user_role_bindings (
    user_id BIGINT NOT NULL, 
    role_id VARCHAR NOT NULL, 
    target VARCHAR NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL, 
    PRIMARY KEY (user_id, role_id), 
    FOREIGN KEY(role_id) REFERENCES roles (id), 
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE TRIGGER update_user_role_bindings_updated_at_trigger
            BEFORE UPDATE ON user_role_bindings
            FOR EACH ROW EXECUTE PROCEDURE update_updated_at_col()
        ;;

CREATE INDEX idx_use_role_binding_target ON user_role_bindings (target);
