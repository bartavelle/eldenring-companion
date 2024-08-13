use packed_struct::prelude::*;

// TODO: fix bools

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct EQUIP_PARAM_WEAPON_ST {
    // only support unicode
    pub behavior_variation_id: i32, // 行動バリエーションID - 攻撃時に参照する行動パラメータIDを決定するときに使う
    pub sort_id: i32,               // ソートID - ソートID(-1:集めない)
    pub wandering_equip_id: u32,    // 徘徊装備ID - 徘徊ゴースト用の差し替え装備ID.
    pub weight: f32,                // 重量[kg] - 重量[kg].
    pub weapon_weight_rate: f32,    // 装備重量比率 - 装備重量比率
    pub fix_price: i32,             // 修理価格 - 修理基本価格
    pub basic_price: i32,           // 基本価格 - 基本価格
    pub sell_value: i32,            // 販売価格 - 販売価格
    pub correct_strength: f32,      // 筋力補正 - キャラパラ補正値.
    pub correct_agility: f32,       // 俊敏補正 - キャラパラ補正値.
    pub correct_magic: f32,         // 魔力補正 - キャラパラ補正値.
    pub correct_faith: f32,         // 信仰補正 - キャラパラ補正値.
    pub phys_guard_cut_rate: f32,   // ガード時物理攻撃カット率 - ガード時のダメージカット率を各攻撃ごとに設定
    pub mag_guard_cut_rate: f32,    // ガード時魔法攻撃カット率 - ガード攻撃でない場合は、0を入れる
    pub fire_guard_cut_rate: f32,   // ガード時炎攻撃力カット率 - 炎攻撃をどれだけカットするか？
    pub thun_guard_cut_rate: f32,   // ガード時電撃攻撃力カット率 - 電撃攻撃をどれだけカットするか？
    pub sp_effect_behavior_id0: i32, // 特殊効果行動ID0 - 武器に特殊効果を追加するときに登録する
    pub sp_effect_behavior_id1: i32, // 特殊効果行動ID1 - 武器に特殊効果を追加するときに登録する
    pub sp_effect_behavior_id2: i32, // 特殊効果行動ID2 - 武器に特殊効果を追加するときに登録する
    pub resident_sp_effect_id: i32, // 常駐特殊効果ID - 常駐特殊効果ID0
    pub resident_sp_effect_id1: i32, // 常駐特殊効果ID1 - 常駐特殊効果ID1
    pub resident_sp_effect_id2: i32, // 常駐特殊効果ID2 - 常駐特殊効果ID2
    pub material_set_id: i32,       // 素材ID - 武器強化に必要な素材パラメータID
    pub origin_equip_wep: i32,      // 派生元 - この武器の強化元武器ID
    pub origin_equip_wep1: i32,     // 派生元 強化+1 - この武器の強化元武器ID1
    pub origin_equip_wep2: i32,     // 派生元 強化+2 - この武器の強化元武器ID2
    pub origin_equip_wep3: i32,     // 派生元 強化+3 - この武器の強化元武器ID3
    pub origin_equip_wep4: i32,     // 派生元 強化+4 - この武器の強化元武器ID4
    pub origin_equip_wep5: i32,     // 派生元 強化+5 - この武器の強化元武器ID5
    pub origin_equip_wep6: i32,     // 派生元 強化+6 - この武器の強化元武器ID6
    pub origin_equip_wep7: i32,     // 派生元 強化+7 - この武器の強化元武器ID7
    pub origin_equip_wep8: i32,     // 派生元 強化+8 - この武器の強化元武器ID8
    pub origin_equip_wep9: i32,     // 派生元 強化+9 - この武器の強化元武器ID9
    pub origin_equip_wep10: i32,    // 派生元 強化+10 - この武器の強化元武器ID10
    pub origin_equip_wep11: i32,    // 派生元 強化+11 - この武器の強化元武器ID11
    pub origin_equip_wep12: i32,    // 派生元 強化+12 - この武器の強化元武器ID12
    pub origin_equip_wep13: i32,    // 派生元 強化+13 - この武器の強化元武器ID13
    pub origin_equip_wep14: i32,    // 派生元 強化+14 - この武器の強化元武器ID14
    pub origin_equip_wep15: i32,    // 派生元 強化+15 - この武器の強化元武器ID15
    pub anti_demon_damage_rate: f32, // 対デーモンダメージ倍率 - 対デーモン用のダメージ倍率
    pub ant_saint_damage_rate: f32, // 対神聖ダメージ倍率 - 対神聖弱点用ダメージ倍率
    pub ant_weak_adamage_rate: f32, // 対弱点Aダメージ倍率 - 対弱点A用ダメージ倍率
    pub ant_weak_bdamage_rate: f32, // 対弱点Bダメージ倍率 - 対弱点B用ダメージ倍率
    pub vagrant_item_lot_id: i32,   // ベイグラント時アイテム抽選ID - -1：ベイグラントなし 0：抽選なし 1～：抽選あり
    pub vagrant_bonus_ene_drop_item_lot_id: i32, // ベイグラントボーナス敵ドロップアイテム抽選ID - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub vagrant_item_ene_drop_item_lot_id: i32, // ベイグラントアイテム敵ドロップアイテム抽選ID - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub equip_model_id: u16,                    // 装備モデル番号 - 装備モデルの番号.
    pub icon_id: u16,                           // アイコンID - メニューアイコンID.
    pub durability: u16,                        // 耐久度 - 初期耐久度.
    pub durability_max: u16,                    // 耐久度最大値 - 新品耐久度.
    pub attack_throw_escape: u16,               // 投げ抜け攻撃力基本値 - 投げ抜け攻撃力の基本値
    pub parry_damage_life: i16, // パリィ発生時間[frame] - パリィダメージの寿命を制限する。TimeActで設定されている以上には持続しない。
    pub attack_base_physics: u16, // 物理攻撃力基本値 - 敵のＨＰにダメージを与える物理属性攻撃の基本値
    pub attack_base_magic: u16, // 魔法攻撃力基本値 - 敵のＨＰにダメージを与える魔法属性攻撃の基本値
    pub attack_base_fire: u16,  // 炎攻撃力基本値 - 敵のＨＰにダメージを与える炎属性攻撃の基本値
    pub attack_base_thunder: u16, // 電撃攻撃力基本値 - 敵のＨＰにダメージを与える電撃属性攻撃の基本値
    pub attack_base_stamina: u16, // スタミナ攻撃力 - 敵へのスタミナ攻撃力
    pub sa_weapon_damage: u16,  // SA武器攻撃力 - スーパーアーマー基本攻撃力
    pub sa_durability: i16,     // SA耐久値 - 攻撃モーション中に使われる追加SA耐久値
    pub guard_angle: i16,       // ガード範囲[deg] - 武器のガード時の防御発生範囲角度
    pub stamina_guard_def: i16, // ガード時スタミナ防御力 - ガード成功時に、敵のスタミナ攻撃に対する防御力
    pub reinforce_type_id: i16, // 強化タイプID - 強化タイプID
    pub trophy_s_grade_id: i16, // トロフィーＳグレードID - トロフィーシステムに関係あるか？
    pub trophy_seq_id: i16,     // トロフィーSEQ番号 - トロフィーのSEQ番号（１３～２９）
    pub throw_atk_rate: i16,    // 投げ攻撃力倍率 - 投げの攻撃力倍率
    pub bow_dist_rate: i16,     // 弓飛距離補正[％] - 飛距離を伸ばすアップ％
    pub equip_model_category: u8, // 装備モデル種別 - 装備モデルの種別.
    pub equip_model_gender: u8, // 装備モデル性別 - 装備モデルの性別.
    pub weapon_category: u8,    // 武器カテゴリ - 武器のカテゴリ.
    pub wepmotion_category: u8, // 武器モーションカテゴリ - 武器モーションのカテゴリ.
    pub guardmotion_category: u8, // ガードモーションカテゴリ - ガードモーションのカテゴリ
    pub atk_material: u8,       // 攻撃材質 - 攻撃パラから使用される攻撃材質
    pub def_material: u8,       // 防御材質 - 攻撃パラから使用される防御材質
    pub def_sfx_material: u8,   // 防御SFX材質 - 攻撃パラから使用される防御SFX材質
    pub correct_type: u8,       // 補正タイプ - 一次パラメータによる補正グラフのタイプを決める
    pub sp_attribute: u8,       // 特殊属性 - 武器の特殊属性値
    pub sp_atkcategory: u8,     // 特殊攻撃カテゴリ - 特殊攻撃カテゴリ（50～199まで可能)
    pub wepmotion_one_hand_id: u8, // 武器モーション片手ID - 片手装備時の基本モーションID.
    pub wepmotion_both_hand_id: u8, // 武器モーション両手ID - 両手装備時の基本モーションID.
    pub proper_strength: u8,    // 装備適正筋力 - 装備適正値.
    pub proper_agility: u8,     // 装備適正俊敏 - 装備適正値.
    pub proper_magic: u8,       // 装備適正魔力 - 装備適正値.
    pub proper_faith: u8,       // 装備適正信仰 - 装備適正値.
    pub over_strength: u8,      // 筋力オーバー開始値 - 筋力オーバー開始値
    pub attack_base_parry: u8,  // パリィ攻撃基本値 - 敵のパリィをやぶるための基本値
    pub defense_base_parry: u8, // パリィ防御値 - パリィ判定時に、パリィになるかガードになるかの判定に利用
    pub guard_base_repel: u8,   // はじき防御力基本値 - ガード敵を攻撃した時に、はじかれるかどうかの判定に利用
    pub attack_base_repel: u8,  // はじき攻撃力基本値 - 敵の攻撃をガードしたときに、はじけるかどうかの判定に利用
    pub guard_cut_cancel_rate: i8, // ガードカット無効化倍率 - 相手のガードカットを無効化させる倍率。-100で完全無効。100で相手の防御効果倍増。
    pub guard_level: i8,           // ガードレベル - ガードしたとき、敵の攻撃をどのガードモーションで受けるか？を決める
    pub slash_guard_cut_rate: i8,  // 斬撃攻撃カット率 - 攻撃タイプを見て、斬撃属性のダメージを何％カットするか？を指定
    pub blow_guard_cut_rate: i8,   // 打撃攻撃カット率 - 攻撃タイプを見て、打撃属性のダメージを何％カットするか？を指定
    pub thrust_guard_cut_rate: i8, // 刺突攻撃カット率 - 攻撃タイプを見て、刺突属性のダメージを何％カットするか？を指定
    pub poison_guard_resist: i8,   // 毒耐性カット率 - 毒にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub disease_guard_resist: i8, // 疫病攻撃カット率 - 疫病にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub blood_guard_resist: i8, // 出血攻撃カット率 - 出血にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub curse_guard_resist: i8, // 呪攻撃カット率 - 呪いにする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub is_durability_divergence: u8, // 耐久度で分岐するか - 魔法使用武器対応：耐久度によるモーション分岐

    pub right_hand_equipable: bool, // 右手装備 - 右手装備可能か.
    pub left_hand_equipable: bool,  // 左手装備 - 左手装備可能か.
    pub both_hand_equipable: bool,  // 両手装備 - 両手装備可能か.
    pub arrow_slot_equipable: bool, // 弓矢弾装備 - 弓用矢弾装備可能か.
    pub bolt_slot_equipable: bool,  // 弩矢弾装備 - 弩用矢弾装備可能か.
    pub enable_guard: bool,         // ガード可能 - 左手装備時L1でガード
    pub enable_parry: bool,         // パリィ可能 - 左手装備時L2でパリィ
    pub enable_magic: bool,         // 魔法可能 - 攻撃時に魔法発動

    pub enable_sorcery: bool,        // 呪術可能 - 攻撃時に呪術発動
    pub enable_miracle: bool,        // 奇蹟可能 - 攻撃時に奇蹟発動
    pub enable_vow_magic: bool,      // 誓約魔法可能 - 攻撃時に誓約魔法発動
    pub is_normal_attack_type: bool, // 通常 - メニュー表示用攻撃タイプ。通常か
    pub is_blow_attack_type: bool,   // 打撃 - メニュー表示用攻撃タイプ。打撃か
    pub is_slash_attack_type: bool,  // 斬撃 - メニュー表示用攻撃タイプ。斬撃か
    pub is_thrust_attack_type: bool, // 刺突 - メニュー表示用攻撃タイプ。刺突か
    pub is_enhance: bool,            // エンチャント可能か？ - 松脂などで、強化可能か？

    pub is_luck_correct: bool,           // 運補正あるか - 運による攻撃力補正があるか
    pub is_custom: bool, // 強化できるか？ - 強化ショップで強化対象リストに並ぶ(仕様変更で削除するかも？)
    pub disable_base_change_reset: bool, // 転職リセット禁止か - 転職リセット禁止か
    pub disable_repair: bool, // 修理禁止か - 修理禁止か
    pub is_dark_hand: bool, // ダークハンドか - ダークハンドか
    pub simple_model_for_dlc: bool, // DLC用シンプルモデルありか - ＤＬＣ用シンプルモデルが存在しているか
    pub lantern_wep: bool, // ランタン武器 - ランタン武器か
    pub is_versus_ghost_wep: bool, // 対霊武器 - 対霊武器か

    pub base_change_category: Integer<u8, packed_bits::Bits<6>>, // 武器転職カテゴリ - 武器転職カテゴリ
    pub is_dragon_slayer: bool,                                  // 竜狩りか - 竜狩り武器か
    pub is_deposit: bool,                                        // 預けれるか - 倉庫に預けれるか

    pub disable_multi_drop_share: bool, // マルチドロップ共有禁止か - マルチドロップ共有禁止か
    pub pad_x: [bool; 7],               // パディング -

    pub pad_0: [u8; 1],              // パディング -
    pub old_sort_id: i16,            // 旧ソートID - 旧ソートID(-1:集めない)
    pub level_sync_correct_i_d: i16, // レベルの同期ID - レベルの同期ID
    pub pad_1: [u8; 6],              // パディング -
}
