pub mod types;
pub mod registry;
pub mod unlock;
pub mod persist;

use std::collections::HashSet;
use chrono::Utc;
use crate::buddy::types::{Buddy, BuddyState};

/// 构建完整的 BuddyState：合并注册表与持久化数据
pub fn build_state() -> BuddyState {
    let persisted = persist::load();
    let all = registry::all_buddies();
    let unlocked_set: HashSet<String> = persisted.unlocked.iter().cloned().collect();

    let roster: Vec<Buddy> = all
        .into_iter()
        .map(|mut b| {
            if unlocked_set.contains(&b.id) {
                // 已解锁，设置解锁时间（持久化中未单独存储时间，使用当前时间占位）
                if b.unlocked_at.is_none() {
                    b.unlocked_at = Some(Utc::now().timestamp());
                }
            }
            b
        })
        .collect();

    // 确保 current 有效，否则回退到 kris
    let current = if unlocked_set.contains(&persisted.current) {
        persisted.current.clone()
    } else {
        "kris".to_string()
    };

    BuddyState {
        current,
        roster,
        completed_sessions: persisted.completed_sessions,
    }
}

/// 切换当前角色，返回更新后的 BuddyState
pub fn switch_current(id: &str) -> Result<BuddyState, String> {
    let mut persisted = persist::load();
    let unlocked_set: HashSet<String> = persisted.unlocked.iter().cloned().collect();

    if !unlocked_set.contains(id) {
        return Err(format!("[buddy] 角色 {} 尚未解锁", id));
    }
    if registry::find_buddy(id).is_none() {
        return Err(format!("[buddy] 角色 {} 不在注册表中", id));
    }

    persisted.current = id.to_string();
    persist::save(&persisted)?;
    Ok(build_state())
}

/// 记录一次 Stop 事件，检查解锁，返回新解锁角色列表（如有）
pub fn on_session_completed() -> Vec<Buddy> {
    let mut persisted = persist::load();
    persisted.completed_sessions += 1;

    let unlocked_set: HashSet<String> = persisted.unlocked.iter().cloned().collect();
    let new_unlocks = unlock::check_new_unlocks(persisted.completed_sessions, &unlocked_set);

    for b in &new_unlocks {
        persisted.unlocked.push(b.id.clone());
        log::info!("[buddy] 解锁新角色: {} ({})", b.id, persisted.completed_sessions);
    }

    let _ = persist::save(&persisted);
    new_unlocks
}

/// 确保 kris 在首次启动时解锁（幂等）
pub fn ensure_first_unlock() {
    let mut persisted = persist::load();
    if !persisted.unlocked.contains(&"kris".to_string()) {
        let kris = unlock::first_unlock();
        persisted.unlocked.push(kris.id.clone());
        log::info!("[buddy] 首次启动，解锁 kris");
        let _ = persist::save(&persisted);
    }
}
