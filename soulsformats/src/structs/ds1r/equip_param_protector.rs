use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct EQUIP_PARAM_PROTECTOR_ST {
    // only support unicode
    pub sort_id: i32,                            // ソートID - ソートID
    pub wandering_equip_id: u32,                 // 徘徊装備ID - 徘徊ゴースト用の差し替え装備ID.
    pub vagrant_item_lot_id: i32, // ベイグラント時アイテム抽選ID - -1：ベイグラントなし 0：抽選なし 1～：抽選あり
    pub vagrant_bonus_ene_drop_item_lot_id: i32, // ベイグラントボーナス敵ドロップアイテム抽選ID - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub vagrant_item_ene_drop_item_lot_id: i32, // ベイグラントアイテム敵ドロップアイテム抽選ID - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub fix_price: i32,                         // 修理価格 - 修理基本価格
    pub basic_price: i32,                       // 基本価格 - 基本価格
    pub sell_value: i32,                        // 販売価格 - 販売価格
    pub weight: f32,                            // 重量[kg] - 重量[kg].
    pub resident_sp_effect_id: i32,             // 常駐特殊効果ID1 - 常駐特殊効果ID1
    pub resident_sp_effect_id2: i32,            // 常駐特殊効果ID2 - 常駐特殊効果ID2
    pub resident_sp_effect_id3: i32,            // 常駐特殊効果ID3 - 常駐特殊効果ID3
    pub material_set_id: i32,                   // 素材ID - 武器強化に必要な素材パラメータID
    pub parts_damage_rate: f32,                 // 部位ダメージ率 - 部位ダメージ率
    pub corect_s_a_recover: f32,                // SA回復時間補正値 - スーパーアーマー回復時間の補正値
    pub origin_equip_pro: i32,                  // 派生元 - この防具の強化元防具ID
    pub origin_equip_pro1: i32,                 // 派生元 強化+1 - この防具の強化元防具ID1
    pub origin_equip_pro2: i32,                 // 派生元 強化+2 - この防具の強化元防具ID2
    pub origin_equip_pro3: i32,                 // 派生元 強化+3 - この防具の強化元防具ID3
    pub origin_equip_pro4: i32,                 // 派生元 強化+4 - この防具の強化元防具ID4
    pub origin_equip_pro5: i32,                 // 派生元 強化+5 - この防具の強化元防具ID5
    pub origin_equip_pro6: i32,                 // 派生元 強化+6 - この防具の強化元防具ID6
    pub origin_equip_pro7: i32,                 // 派生元 強化+7 - この防具の強化元防具ID7
    pub origin_equip_pro8: i32,                 // 派生元 強化+8 - この防具の強化元防具ID8
    pub origin_equip_pro9: i32,                 // 派生元 強化+9 - この防具の強化元防具ID9
    pub origin_equip_pro10: i32,                // 派生元 強化+10 - この防具の強化元防具ID10
    pub origin_equip_pro11: i32,                // 派生元 強化+11 - この防具の強化元防具ID11
    pub origin_equip_pro12: i32,                // 派生元 強化+12 - この防具の強化元防具ID12
    pub origin_equip_pro13: i32,                // 派生元 強化+13 - この防具の強化元防具ID13
    pub origin_equip_pro14: i32,                // 派生元 強化+14 - この防具の強化元防具ID14
    pub origin_equip_pro15: i32,                // 派生元 強化+15 - この防具の強化元防具ID15
    pub face_scale_mscale_x: f32,               // 男横顔拡大スケール -
    pub face_scale_mscale_z: f32,               // 男前顔拡大スケール -
    pub face_scale_mmax_x: f32,                 // 男横顔拡大最大倍率 -
    pub face_scale_mmax_z: f32,                 // 男前顔拡大最大倍率 -
    pub face_scale_fscale_x: f32,               // 女横顔拡大スケール -
    pub face_scale_fscale_z: f32,               // 女前顔拡大スケール -
    pub face_scale_fmax_x: f32,                 // 女横顔拡大最大倍率 -
    pub face_scale_fmax_z: f32,                 // 女前顔拡大最大倍率 -
    pub qwc_id: i32,                            // QWCID - QWCのパラメタID
    pub equip_model_id: u16,                    // 装備モデル番号 - 装備モデルの番号.
    pub icon_id_m: u16,                         // 男用アイコンID - 男用メニューアイコンID.
    pub icon_id_f: u16,                         // 女用アイコンID - 女用メニューアイコンID.
    pub knock_back: u16,                        // ノックバックカット率 - ノックバックの減少値.
    pub knockback_bounce_rate: u16,             // ノックバック反発率 - ノックバックの反発率.
    pub durability: u16,                        // 耐久度 - 初期耐久度.
    pub durability_max: u16,                    // 耐久度最大値 - 新品耐久度.
    pub sa_durability: i16,                     // SA耐久値 - スーパーアーマー耐久力
    pub def_flick_power: u16,                   // はじき防御力 - 敵の攻撃のはじき返し判定に利用.
    pub defense_physics: u16,                   // 物理防御力 - 物理攻撃のダメージ防御.
    pub defense_magic: u16,                     // 魔法防御力 - 魔法攻撃のダメージ防御.
    pub defense_fire: u16,                      // 炎防御力 - 炎攻撃のダメージ防御.
    pub defense_thunder: u16,                   // 電撃防御力 - 電撃攻撃のダメージ防御.
    pub defense_slash: i16,                     // 斬撃防御力 - 攻撃タイプを見て、斬撃属性のときは、防御力を減少させる
    pub defense_blow: i16,                      // 打撃防御力 - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
    pub defense_thrust: i16,                    // 刺突防御力 - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
    pub resist_poison: u16,                     // 毒耐性 - 毒状態異常へのかかりにくさ
    pub resist_disease: u16,                    // 疫病耐性 - 疫病状態異常へのかかりにくさ
    pub resist_blood: u16,                      // 出血耐性 - 出血状態異常へのかかりにくさ
    pub resist_curse: u16,                      // 呪耐性 - 呪い状態異常へのかかりにくさ
    pub reinforce_type_id: i16,                 // 強化タイプID - 強化タイプID
    pub trophy_s_grade_id: i16,                 // トロフィー - トロフィーシステムに関係あるか？
    pub shop_lv: i16,                           // ショップレベル - お店で販売できるレベル
    pub knockback_param_id: u8,                 // ノックバックパラメータID - ノックバックで使用するパラメータのID
    pub flick_damage_cut_rate: u8,              // はじき時ダメージ減衰率[%] - はじき時のダメージ減衰率に使用
    pub equip_model_category: u8,               // 装備モデル種別 - 装備モデルの種別.
    pub equip_model_gender: u8,                 // 装備モデル性別 - 装備モデルの性別.
    pub protector_category: u8,                 // 防具カテゴリ - 防具のカテゴリ.
    pub defense_material: u8,                   // 防御材質【SE】 - 移動/防御時のSE用.
    pub defense_material_sfx: u8,               // 防御材質【SFX】 - 移動/防御時のSFX用.
    pub parts_dmg_type: u8,                     // 部位ダメージ適用攻撃 - 部位ダメージ判定を行う攻撃タイプを設定
    pub defense_materialweak: u8,               // 弱点防御材質【SE】 - 弱点部位ダメージ時のSE用
    pub defense_material_sfxweak: u8,           // 弱点防御材質【SFX】 - 弱点部位ダメージ時のSFX用

    pub invisible_flag01: bool, // #01#非表示 - 前髪の根元
    pub invisible_flag00: bool, // #00#非表示 - 前髪の先
    pub use_face_scale: bool,   // 顔スケールを使用するか - 顔スケールを使用するか
    pub leg_equip: bool,        // 脚装備 - 脚装備か.
    pub arm_equip: bool,        // 腕装備 - 腕装備か.
    pub body_equip: bool,       // 胴装備 - 胴装備か.
    pub head_equip: bool,       // 頭装備 - 頭装備か.
    pub is_deposit: bool,       // 預けれるか - 倉庫に預けれるか

    // TODO: fix bools after that
    pub invisible_flag02: bool, // #02#非表示 - もみあげ
    pub invisible_flag03: bool, // #03#非表示 - 頭頂部
    pub invisible_flag04: bool, // #04#非表示 - 頭頂部
    pub invisible_flag05: bool, // #05#非表示 - 後ろ髪
    pub invisible_flag06: bool, // #06#非表示 - 後ろ髪の先
    pub invisible_flag07: bool, // #07#非表示 -
    pub invisible_flag08: bool, // #08#非表示 -
    pub invisible_flag09: bool, // #09#非表示 -

    pub invisible_flag10: bool, // #10#非表示 - 襟
    pub invisible_flag11: bool, // #11#非表示 - 襟回り
    pub invisible_flag12: bool, // #12#非表示 -
    pub invisible_flag13: bool, // #13#非表示 -
    pub invisible_flag14: bool, // #14#非表示 -
    pub invisible_flag15: bool, // #15#非表示 - 頭巾の裾
    pub invisible_flag16: bool, // #16#非表示 -
    pub invisible_flag17: bool, // #17#非表示 -

    pub invisible_flag18: bool, // #18#非表示 -
    pub invisible_flag19: bool, // #19#非表示 -
    pub invisible_flag20: bool, // #20#非表示 - 袖A
    pub invisible_flag21: bool, // #21#非表示 - 袖B
    pub invisible_flag22: bool, // #22#非表示 -
    pub invisible_flag23: bool, // #23#非表示 -
    pub invisible_flag24: bool, // #24#非表示 -
    pub invisible_flag25: bool, // #25#非表示 - 腕

    pub invisible_flag26: bool, // #26#非表示 -
    pub invisible_flag27: bool, // #27#非表示 -
    pub invisible_flag28: bool, // #28#非表示 -
    pub invisible_flag29: bool, // #29#非表示 -
    pub invisible_flag30: bool, // #30#非表示 - ベルト
    pub invisible_flag31: bool, // #31#非表示 -
    pub invisible_flag32: bool, // #32#非表示 -
    pub invisible_flag33: bool, // #33#非表示 -

    pub invisible_flag34: bool, // #34#非表示 -
    pub invisible_flag35: bool, // #35#非表示 -
    pub invisible_flag36: bool, // #36#非表示 -
    pub invisible_flag37: bool, // #37#非表示 -
    pub invisible_flag38: bool, // #38#非表示 -
    pub invisible_flag39: bool, // #39#非表示 -
    pub invisible_flag40: bool, // #40#非表示 -
    pub invisible_flag41: bool, // #41#非表示 -

    pub invisible_flag42: bool,         // #42#非表示 -
    pub invisible_flag43: bool,         // #43#非表示 -
    pub invisible_flag44: bool,         // #44#非表示 -
    pub invisible_flag45: bool,         // #45#非表示 -
    pub invisible_flag46: bool,         // #46#非表示 -
    pub invisible_flag47: bool,         // #47#非表示 -
    pub disable_multi_drop_share: bool, // マルチドロップ共有禁止か - マルチドロップ共有禁止か
    pub simple_model_for_dlc: bool,     // DLC用シンプルモデルありか - ＤＬＣ用シンプルモデルが存在しているか

    pub pad_0: [u8; 1],   // パディング -
    pub old_sort_id: i16, // 旧ソートID - 旧ソートID(-1:集めない)
    pub pad_1: [u8; 6],   // パディング -
}
