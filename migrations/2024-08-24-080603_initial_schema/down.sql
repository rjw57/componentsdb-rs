DROP TRIGGER update_user_role_bindings_updated_at_trigger
            ON user_role_bindings;;

DROP INDEX idx_use_role_binding_target;
DROP TABLE user_role_bindings;

DELETE FROM role_permission_bindings
  WHERE
    role_permission_bindings.role_id = 'role/cabinetOwner'
    AND role_permission_bindings.permission_id IN ('cabinet.read', 'cabinet.write');
DELETE FROM role_permission_bindings
  WHERE
    role_permission_bindings.role_id = 'role/cabinetViewer'
    AND role_permission_bindings.permission_id IN ('cabinet.read');
DELETE FROM permissions WHERE permissions.id IN ('cabinet.read', 'cabinet.write');
DELETE FROM roles WHERE roles.id IN ('role/cabinetOwner', 'role/cabinetViewer');

DROP TABLE role_permission_bindings;
DROP TABLE roles;
DROP TABLE permissions;

DROP INDEX idx_federated_user_credential_uses_claims;
DROP TRIGGER update_federated_user_credential_uses_updated_at_trigger
            ON federated_user_credential_uses;;
DROP TABLE federated_user_credential_uses;

DROP INDEX idx_refresh_tokens_expires_at;
DROP TRIGGER update_refresh_tokens_updated_at_trigger ON refresh_tokens;;
DROP TABLE refresh_tokens;

DROP TRIGGER update_federated_user_credentials_updated_at_trigger
            ON federated_user_credentials;;

DROP INDEX idx_federated_user_credentials_subject_audience_issuer;
DROP INDEX idx_federated_user_credentials_uuid;
DROP INDEX idx_federated_user_credentials_user;
DROP TABLE federated_user_credentials;

DROP INDEX idx_access_tokens_expires_at;
DROP TRIGGER update_access_tokens_updated_at_trigger ON access_tokens;;
DROP TABLE access_tokens;

DROP INDEX idx_users_uuid;
DROP TRIGGER update_users_updated_at_trigger ON users;;
DROP TABLE users;

DROP INDEX idx_collections_drawer;
DROP INDEX idx_collections_component;
DROP INDEX idx_collections_uuid;
DROP TRIGGER update_collections_updated_at_trigger ON collections;;
DROP TABLE collections;


DROP INDEX idx_drawers_cabinet;
DROP INDEX idx_drawers_uuid;
DROP TRIGGER update_drawers_updated_at_trigger ON drawers;;
DROP TABLE drawers;

DROP INDEX idx_components_trigrams;
DROP INDEX idx_components_uuid;
DROP TRIGGER update_components_updated_at_trigger ON components;;
DROP TABLE components;

DROP INDEX idx_cabinets_uuid;
DROP TRIGGER update_cabinets_updated_at_trigger ON cabinets;;
DROP TABLE cabinets;

DROP FUNCTION update_updated_at_col() CASCADE;;

DROP EXTENSION IF EXISTS pg_trgm;
