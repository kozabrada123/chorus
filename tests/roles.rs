mod common;

use chorus::types::{self, RoleCreateModifySchema};

#[tokio::test]
async fn create_and_get_roles() {
    let mut bundle = common::setup().await;
    let permissions = types::PermissionFlags::CONNECT | types::PermissionFlags::MANAGE_EVENTS;
    let permissions = Some(permissions.to_string());
    let role_create_schema: types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions,
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let guild_id = bundle.guild.id.clone().to_string();
    let role = types::RoleObject::create(&mut bundle.user, &guild_id, role_create_schema)
        .await
        .unwrap();

    let expected = types::RoleObject::get_all(&mut bundle.user, &guild_id)
        .await
        .unwrap()
        .unwrap()
        .iter()
        .nth(2)
        .unwrap()
        .clone();

    assert_eq!(role, expected);
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_singular_role() {
    let mut bundle = common::setup().await;
    let guild_id = &bundle.guild.id.to_string();
    let role_id = &bundle.role.id.to_string();
    let role = bundle.role.clone();
    let same_role = chorus::types::RoleObject::get(&mut bundle.user, guild_id, role_id)
        .await
        .unwrap();
    assert_eq!(role, same_role);
    common::teardown(bundle).await
}