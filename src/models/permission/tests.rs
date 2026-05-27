use super::*;
use crate::models::HasId;

#[test]
fn test_api_permission() {
    let permission = APIPermission {
        path: "/guilds/123/members/456".to_string(),
        method: "GET".to_string(),
        desc: "Get guild member".to_string(),
        auth_status: 1,
    };

    assert_eq!(permission.path, "/guilds/123/members/456");
    assert_eq!(permission.method, "GET");
    assert_eq!(permission.desc, "Get guild member");
    assert_eq!(permission.auth_status, 1);
}

#[test]
fn test_api_permission_unauthorized() {
    let permission = APIPermission {
        path: "/guilds/123/roles".to_string(),
        method: "POST".to_string(),
        desc: "Create guild role".to_string(),
        auth_status: 0,
    };

    assert_eq!(permission.auth_status, 0);
}

#[test]
fn test_api_permission_unknown_status() {
    let permission = APIPermission {
        path: "/guilds/123/channels".to_string(),
        method: "GET".to_string(),
        desc: String::new(),
        auth_status: 2,
    };

    assert_eq!(permission.auth_status, 2);
}

#[test]
fn api_permission_uses_required_zero_value_fields() {
    let permission: APIPermission = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(permission.path, "");
    assert_eq!(permission.method, "");
    assert_eq!(permission.desc, "");
    assert_eq!(permission.auth_status, 0);

    let value = serde_json::to_value(&permission).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn api_permissions_omits_empty_list() {
    let permissions = APIPermissions::default();
    let value = serde_json::to_value(&permissions).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn api_permissions_keep_official_json_shape() {
    let permissions = APIPermissions {
        api_list: vec![APIPermission {
            path: "/guilds/{guild_id}/members/{user_id}".to_string(),
            method: "GET".to_string(),
            desc: "Get member".to_string(),
            auth_status: 1,
        }],
    };
    let value = serde_json::to_value(&permissions).unwrap();

    assert!(value.get("api_list").is_none());
    assert_eq!(
        value["apis"][0]["path"],
        "/guilds/{guild_id}/members/{user_id}"
    );
    assert_eq!(value["apis"][0]["method"], "GET");
    assert_eq!(value["apis"][0]["desc"], "Get member");
    assert_eq!(value["apis"][0]["auth_status"], 1);
}

#[test]
fn test_api_permission_demand_identify() {
    let identify = APIPermissionDemandIdentify {
        path: "/guilds/{guild_id}/members".to_string(),
        method: "GET".to_string(),
    };
    assert_eq!(identify.path, "/guilds/{guild_id}/members");
    assert_eq!(identify.method, "GET");
    assert_eq!(format!("{}", identify), "GET /guilds/{guild_id}/members");
}

#[test]
fn test_api_permission_demand_identify_presets() {
    let guild_members = APIPermissionDemandIdentify::guild_members();
    assert_eq!(guild_members.path, "/guilds/{guild_id}/members/{user_id}");
    assert_eq!(guild_members.method, "GET");

    let post_messages = APIPermissionDemandIdentify::post_messages();
    assert_eq!(post_messages.path, "/channels/{channel_id}/messages");
    assert_eq!(post_messages.method, "POST");
}

#[test]
fn test_api_permission_demand() {
    let identify = APIPermissionDemandIdentify::guild_members();
    let demand = APIPermissionDemand {
        guild_id: "guild123".to_string(),
        channel_id: "channel456".to_string(),
        api_identify: Some(identify),
        title: String::new(),
        desc: "Need access to get guild member information".to_string(),
    };

    assert_eq!(demand.guild_id, "guild123");
    assert_eq!(demand.channel_id, "channel456");
    let identify = demand.api_identify.as_ref().unwrap();
    assert_eq!(identify.path, "/guilds/{guild_id}/members/{user_id}");
    assert_eq!(identify.method, "GET");
    assert_eq!(demand.desc, "Need access to get guild member information");
    assert_eq!(demand.title, "");
}

#[test]
fn test_api_permission_demand_with_title() {
    let identify = APIPermissionDemandIdentify::post_messages();
    let demand = APIPermissionDemand {
        guild_id: "guild123".to_string(),
        channel_id: "channel456".to_string(),
        api_identify: Some(identify),
        title: "Message Posting Permission".to_string(),
        desc: "Need to send automated messages".to_string(),
    };

    assert_eq!(demand.title, "Message Posting Permission");
    assert_eq!(demand.id(), Some(&"guild123".to_string()));
}

#[test]
fn api_permission_demand_uses_zero_values() {
    let demand: APIPermissionDemand = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(demand.guild_id, "");
    assert_eq!(demand.channel_id, "");
    assert!(demand.api_identify.is_none());
    assert_eq!(demand.title, "");
    assert_eq!(demand.desc, "");

    let value = serde_json::to_value(&demand).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn api_permission_demand_to_create_keeps_required_fields_when_zero() {
    // ChannelID and Desc do NOT have omitempty in the QQ Bot Open API,
    // so they must always be present in the JSON body.
    let demand = APIPermissionDemandToCreate::default();
    let value = serde_json::to_value(&demand).unwrap();

    assert_eq!(value["channel_id"], "");
    assert_eq!(value["desc"], "");
    assert!(value.get("api_identify").is_none());
}

#[test]
fn api_permission_demand_to_create_keeps_official_json_shape() {
    let demand = APIPermissionDemandToCreate::new(
        "channel-1",
        APIPermissionDemandIdentify::post_messages(),
        "Need to send messages",
    );
    let value = serde_json::to_value(&demand).unwrap();

    assert_eq!(value["channel_id"], "channel-1");
    assert_eq!(
        value["api_identify"]["path"],
        "/channels/{channel_id}/messages"
    );
    assert_eq!(value["api_identify"]["method"], "POST");
    assert_eq!(value["desc"], "Need to send messages");
}

#[test]
fn test_api_permission_demand_display() {
    let identify = APIPermissionDemandIdentify::guild_channels();
    let demand = APIPermissionDemand {
        guild_id: "guild999".to_string(),
        channel_id: "channel888".to_string(),
        api_identify: Some(identify),
        title: String::new(),
        desc: "This is a very long description that should be truncated when displayed".to_string(),
    };

    let display = format!("{}", demand);
    assert!(display.contains("guild999"));
    assert!(display.contains("GET /guilds/{guild_id}/channels"));
    // Should be truncated to 50 characters
    assert!(display.len() < 200);
}
