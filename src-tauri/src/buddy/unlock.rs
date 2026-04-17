use crate::buddy::registry::all_buddies;
use crate::buddy::types::{Buddy, Rarity};

/// 返回 completed_sessions 数量下应解锁的角色列表（不含已解锁）。
/// `already_unlocked`：已解锁的角色 ID 集合
pub fn check_new_unlocks(
    completed_sessions: u32,
    already_unlocked: &std::collections::HashSet<String>,
) -> Vec<Buddy> {
    all_buddies()
        .into_iter()
        .filter(|b| {
            !already_unlocked.contains(&b.id)
                && completed_sessions >= b.rarity.unlock_threshold()
        })
        .collect()
}

/// 首次启动时解锁 "Kris"（Common）
pub fn first_unlock() -> Buddy {
    crate::buddy::registry::find_buddy("kris")
        .expect("kris 必须在注册表中")
}

/// 判断某角色是否应该在当前 completed_sessions 下解锁
pub fn should_unlock(buddy: &Buddy, completed_sessions: u32) -> bool {
    completed_sessions >= buddy.rarity.unlock_threshold()
}

#[cfg(test)]
mod unlock_test {
    use super::*;
    use std::collections::HashSet;

    /// 构造只解锁了 kris 的初始集合
    fn kris_unlocked() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert("kris".to_string());
        set
    }

    #[test]
    fn test_first_unlock_is_kris() {
        let kris = first_unlock();
        assert_eq!(kris.id, "kris");
        assert_eq!(kris.rarity, Rarity::Common);
    }

    #[test]
    fn test_common_threshold_is_1() {
        assert_eq!(Rarity::Common.unlock_threshold(), 1);
    }

    #[test]
    fn test_rare_threshold_is_10() {
        assert_eq!(Rarity::Rare.unlock_threshold(), 10);
    }

    #[test]
    fn test_epic_threshold_is_50() {
        assert_eq!(Rarity::Epic.unlock_threshold(), 50);
    }

    #[test]
    fn test_legendary_threshold_is_100() {
        assert_eq!(Rarity::Legendary.unlock_threshold(), 100);
    }

    #[test]
    fn test_no_unlock_at_zero_sessions_except_kris_already_held() {
        // 0 会话：Common 阈值 1，所以无法解锁任何人
        let new = check_new_unlocks(0, &HashSet::new());
        assert!(
            new.iter().all(|b| b.rarity != Rarity::Common),
            "0 次会话不应解锁 Common（阈值 1）"
        );
    }

    #[test]
    fn test_common_unlocked_at_1_session() {
        let new = check_new_unlocks(1, &HashSet::new());
        // 至少有 Common 角色被解锁（包括 kris）
        assert!(
            new.iter().any(|b| b.rarity == Rarity::Common),
            "1 次会话应能解锁 Common 角色"
        );
    }

    #[test]
    fn test_rare_unlocked_at_10_sessions() {
        let new = check_new_unlocks(10, &kris_unlocked());
        assert!(
            new.iter().any(|b| b.rarity == Rarity::Rare),
            "10 次会话应能解锁 Rare 角色"
        );
    }

    #[test]
    fn test_epic_unlocked_at_50_sessions() {
        let already: HashSet<String> = all_buddies()
            .iter()
            .filter(|b| b.rarity == Rarity::Common || b.rarity == Rarity::Rare)
            .map(|b| b.id.clone())
            .collect();
        let new = check_new_unlocks(50, &already);
        assert!(
            new.iter().any(|b| b.rarity == Rarity::Epic),
            "50 次会话应能解锁 Epic 角色"
        );
    }

    #[test]
    fn test_legendary_unlocked_at_100_sessions() {
        let already: HashSet<String> = all_buddies()
            .iter()
            .filter(|b| {
                b.rarity == Rarity::Common
                    || b.rarity == Rarity::Rare
                    || b.rarity == Rarity::Epic
            })
            .map(|b| b.id.clone())
            .collect();
        let new = check_new_unlocks(100, &already);
        assert!(
            new.iter().any(|b| b.rarity == Rarity::Legendary),
            "100 次会话应能解锁 Legendary 角色"
        );
    }

    #[test]
    fn test_already_unlocked_not_returned() {
        // 全部标为已解锁，应返回空
        let all_ids: HashSet<String> = all_buddies().iter().map(|b| b.id.clone()).collect();
        let new = check_new_unlocks(100, &all_ids);
        assert!(new.is_empty(), "全部已解锁时不应返回新角色");
    }

    #[test]
    fn test_should_unlock_common_at_1() {
        let kris = crate::buddy::registry::find_buddy("kris").unwrap();
        assert!(should_unlock(&kris, 1));
        assert!(!should_unlock(&kris, 0));
    }
}
