use rlua::UserData;

/// A wrapper around UserData so that StaticToLua could be automatically implemented without
/// conflicts
pub trait StaticUserData: UserData {}
