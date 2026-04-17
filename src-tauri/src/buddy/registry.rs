use crate::buddy::types::{Buddy, BuddyStats, Rarity};

/// 构建所有角色的静态注册表（不含解锁状态）。
/// 返回顺序：Common → Rare → Epic → Legendary
pub fn all_buddies() -> Vec<Buddy> {
    vec![
        // ── Common ──────────────────────────────────────────────────────
        Buddy {
            id: "kris".to_string(),
            name: "Kris".to_string(),
            display_name: "Kris".to_string(),
            rarity: Rarity::Common,
            ascii_art: vec![
                " (•‿•) \n /|=|\\  \n  / \\  ".to_string(),
                " (^‿^) \n /|=|\\  \n  / \\  ".to_string(),
            ],
            description: "你忠诚的初始伙伴。总是第一个出现，从不抱怨。".to_string(),
            stats: BuddyStats { debug: 40, patience: 80, chaos: 20, wisdom: 50, sneak: 30 },
            unlocked_at: None,
        },
        Buddy {
            id: "byte".to_string(),
            name: "Byte".to_string(),
            display_name: "Byte 咬一口".to_string(),
            rarity: Rarity::Common,
            ascii_art: vec![
                " [0101] \n |  o  | \n |_/ \\_| ".to_string(),
                " [1010] \n |  o  | \n |_\\ /_| ".to_string(),
            ],
            description: "喜欢在 0 和 1 之间横跳，偶尔也能解出 bug。".to_string(),
            stats: BuddyStats { debug: 60, patience: 40, chaos: 50, wisdom: 35, sneak: 45 },
            unlocked_at: None,
        },
        Buddy {
            id: "pixel_pete".to_string(),
            name: "PixelPete".to_string(),
            display_name: "像素皮特".to_string(),
            rarity: Rarity::Common,
            ascii_art: vec![
                "  [##]   \n #(o.o)# \n  /#-#\\  ".to_string(),
            ],
            description: "8-bit 时代走出来的小家伙，擅长用像素解释一切。".to_string(),
            stats: BuddyStats { debug: 45, patience: 55, chaos: 30, wisdom: 60, sneak: 25 },
            unlocked_at: None,
        },

        // ── Rare ────────────────────────────────────────────────────────
        Buddy {
            id: "neon_cat".to_string(),
            name: "NeonCat".to_string(),
            display_name: "霓虹猫".to_string(),
            rarity: Rarity::Rare,
            ascii_art: vec![
                "~*~*~*~*~\n/\\ ≖‿≖ /\\\n~rainbow~ ".to_string(),
                "~*~*~*~*~\n/\\ ^‿^ /\\\n*rainbow* ".to_string(),
            ],
            description: "在代码海洋里乘彩虹冲浪，带来意想不到的灵感。".to_string(),
            stats: BuddyStats { debug: 55, patience: 45, chaos: 75, wisdom: 40, sneak: 70 },
            unlocked_at: None,
        },
        Buddy {
            id: "patient_penguin".to_string(),
            name: "PatientPenguin".to_string(),
            display_name: "耐心企鹅".to_string(),
            rarity: Rarity::Rare,
            ascii_art: vec![
                "  _~_   \n (o_o)  \n  )-(   \n (___) ".to_string(),
                "  _~_   \n (-_-)  \n  )-(   \n (___) ".to_string(),
            ],
            description: "天生无限耐心，等再长的编译都不皱一下眉头。".to_string(),
            stats: BuddyStats { debug: 50, patience: 98, chaos: 10, wisdom: 70, sneak: 20 },
            unlocked_at: None,
        },
        Buddy {
            id: "stack_fox".to_string(),
            name: "StackFox".to_string(),
            display_name: "堆栈狐".to_string(),
            rarity: Rarity::Rare,
            ascii_art: vec![
                " /\\_/\\  \n( >.< ) \n  )-(   \n (___)  ".to_string(),
                " /\\_/\\  \n( ^.^ ) \n  )-(   \n (___)  ".to_string(),
            ],
            description: "天生嗅觉灵敏，stack overflow 难不倒它。".to_string(),
            stats: BuddyStats { debug: 75, patience: 60, chaos: 35, wisdom: 65, sneak: 55 },
            unlocked_at: None,
        },

        // ── Epic ────────────────────────────────────────────────────────
        Buddy {
            id: "debug_dragon".to_string(),
            name: "DebugDragon".to_string(),
            display_name: "调试龙".to_string(),
            rarity: Rarity::Epic,
            ascii_art: vec![
                " <>=<>=<> \n  (O_O)  \n  //|\\\\  \n  //|\\\\  ".to_string(),
                " <>~<>~<> \n  (@_@)  \n  //|\\\\  \n  //|\\\\  ".to_string(),
            ],
            description: "喷出的不是火焰，而是精准的 breakpoint。".to_string(),
            stats: BuddyStats { debug: 95, patience: 65, chaos: 60, wisdom: 80, sneak: 40 },
            unlocked_at: None,
        },
        Buddy {
            id: "octopus".to_string(),
            name: "Octopus".to_string(),
            display_name: "章鱼客".to_string(),
            rarity: Rarity::Epic,
            ascii_art: vec![
                "  _____  \n ( o o ) \n  \\___/  \n~8 arms~".to_string(),
                "  _____  \n ( * * ) \n  \\___/  \n~8 arms~".to_string(),
            ],
            description: "八条手臂同时运行八个任务，并发大师。".to_string(),
            stats: BuddyStats { debug: 70, patience: 50, chaos: 80, wisdom: 75, sneak: 90 },
            unlocked_at: None,
        },

        // ── Legendary ───────────────────────────────────────────────────
        Buddy {
            id: "void_walker".to_string(),
            name: "VoidWalker".to_string(),
            display_name: "虚空行者".to_string(),
            rarity: Rarity::Legendary,
            ascii_art: vec![
                "**.*.*.**\n*( ◉_◉)*\n*.\\___/.*\n**.*.*.**".to_string(),
                ".*.*.*.*.\n*( ◉‿◉)*\n*.\\___/.*\n.*.*.*.*. ".to_string(),
            ],
            description: "来自代码虚空的存在，看穿一切 null pointer。".to_string(),
            stats: BuddyStats { debug: 99, patience: 90, chaos: 95, wisdom: 99, sneak: 99 },
            unlocked_at: None,
        },
        Buddy {
            id: "quantum_quokka".to_string(),
            name: "QuantumQuokka".to_string(),
            display_name: "量子袋熊".to_string(),
            rarity: Rarity::Legendary,
            ascii_art: vec![
                "  ~Q~Q~  \n (^‿^)Q \n  |_Q_|  \n /_Q_Q_\\ ".to_string(),
                "  ~Q~Q~  \n (Q‿Q)  \n  |_Q_|  \n \\Q_Q_Q/ ".to_string(),
            ],
            description: "同时存在于所有分支，叠加态的调试专家。".to_string(),
            stats: BuddyStats { debug: 97, patience: 95, chaos: 88, wisdom: 96, sneak: 85 },
            unlocked_at: None,
        },
    ]
}

/// 按 ID 查找角色（不含解锁状态）
pub fn find_buddy(id: &str) -> Option<Buddy> {
    all_buddies().into_iter().find(|b| b.id == id)
}

/// 所有角色数量
pub fn buddy_count() -> usize {
    all_buddies().len()
}

#[cfg(test)]
mod registry_test {
    use super::*;
    use crate::buddy::types::Rarity;

    #[test]
    fn test_at_least_10_buddies() {
        assert!(
            buddy_count() >= 10,
            "注册表至少需要 10 个角色，当前 {} 个",
            buddy_count()
        );
    }

    #[test]
    fn test_common_at_least_3() {
        let count = all_buddies().iter().filter(|b| b.rarity == Rarity::Common).count();
        assert!(count >= 3, "Common 至少 3 个，当前 {} 个", count);
    }

    #[test]
    fn test_rare_at_least_3() {
        let count = all_buddies().iter().filter(|b| b.rarity == Rarity::Rare).count();
        assert!(count >= 3, "Rare 至少 3 个，当前 {} 个", count);
    }

    #[test]
    fn test_epic_at_least_2() {
        let count = all_buddies().iter().filter(|b| b.rarity == Rarity::Epic).count();
        assert!(count >= 2, "Epic 至少 2 个，当前 {} 个", count);
    }

    #[test]
    fn test_legendary_at_least_2() {
        let count = all_buddies().iter().filter(|b| b.rarity == Rarity::Legendary).count();
        assert!(count >= 2, "Legendary 至少 2 个，当前 {} 个", count);
    }

    #[test]
    fn test_all_ids_unique() {
        let buddies = all_buddies();
        let mut ids = std::collections::HashSet::new();
        for b in &buddies {
            assert!(ids.insert(&b.id), "角色 ID 重复: {}", b.id);
        }
    }

    #[test]
    fn test_kris_is_first_common() {
        let buddies = all_buddies();
        let kris = buddies.iter().find(|b| b.id == "kris");
        assert!(kris.is_some(), "必须存在 kris");
        assert_eq!(kris.unwrap().rarity, Rarity::Common, "Kris 必须是 Common");
    }

    #[test]
    fn test_all_ascii_art_non_empty() {
        for b in all_buddies() {
            assert!(!b.ascii_art.is_empty(), "角色 {} ASCII 艺术不能为空", b.id);
        }
    }

    #[test]
    fn test_stats_in_range() {
        for b in all_buddies() {
            let s = &b.stats;
            // 所有属性范围 0-100（u8 已保证 0-255，我们额外验证上限）
            assert!(s.debug <= 100, "{} debug 超范围", b.id);
            assert!(s.patience <= 100, "{} patience 超范围", b.id);
            assert!(s.chaos <= 100, "{} chaos 超范围", b.id);
            assert!(s.wisdom <= 100, "{} wisdom 超范围", b.id);
            assert!(s.sneak <= 100, "{} sneak 超范围", b.id);
        }
    }

    #[test]
    fn test_find_buddy_by_id() {
        let kris = find_buddy("kris");
        assert!(kris.is_some());
        assert_eq!(kris.unwrap().name, "Kris");

        let none = find_buddy("nonexistent_id");
        assert!(none.is_none());
    }

    #[test]
    fn test_descriptions_non_empty() {
        for b in all_buddies() {
            assert!(!b.description.is_empty(), "角色 {} 描述不能为空", b.id);
        }
    }
}
