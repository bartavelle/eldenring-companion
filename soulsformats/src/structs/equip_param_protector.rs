use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian="lsb", bit_numbering="msb0")]
pub struct EQUIP_PARAM_PROTECTOR_ST {
    pub disable_param_reserve1: Integer<u8, packed_bits::Bits::<7>>, // パッケージ出力用リザーブ1 - パッケージ出力用リザーブ1
    pub disable_paramn_t: bool, // NT版出力から外すか - ○をつけたパラメータをNT版パッケージでは除外します
    pub disable_param_reserve2: [u8;3], // パッケージ出力用リザーブ2 - パッケージ出力用リザーブ2
    pub sort_id: i32, // ソートID - ソートID(プログラム内で強化レベルを加味しているので s32 では７桁が限界)
    pub wandering_equip_id: u32, // 徘徊装備ID - 徘徊ゴースト用の差し替え装備ID.
    pub resist_sleep: u16, // 睡眠耐性 - 睡眠状態異常へのかかりにくさ
    pub resist_madness: u16, // 発狂耐性 - 発狂状態異常へのかかりにくさ
    pub sa_durability: f32, // SA耐久値 - スーパーアーマー耐久力
    pub toughness_correct_rate: f32, // 強靭度 補正倍率 - 強靭度の基本値を補正する倍率です
    pub fix_price: i32, // 修理価格 - 修理基本価格
    pub basic_price: i32, // 基本価格 - 基本価格
    pub sell_value: i32, // 売却価格 - 販売価格
    pub weight: f32, // 重量[kg] - 重量[kg].
    pub resident_sp_effect_id: i32, // 常駐特殊効果ID1 - 常駐特殊効果ID1
    pub resident_sp_effect_id2: i32, // 常駐特殊効果ID2 - 常駐特殊効果ID2
    pub resident_sp_effect_id3: i32, // 常駐特殊効果ID3 - 常駐特殊効果ID3
    pub material_set_id: i32, // 素材ID - 武器強化に必要な素材パラメータID
    pub parts_damage_rate: f32, // 部位ダメージ率 - 部位ダメージ率
    pub corect_s_a_recover: f32, // SA回復時間補正値 - スーパーアーマー回復時間の補正値
    pub origin_equip_pro: i32, // 派生元 - この防具の強化元防具ID
    pub origin_equip_pro1: i32, // 派生元 強化+1 - この防具の強化元防具ID1
    pub origin_equip_pro2: i32, // 派生元 強化+2 - この防具の強化元防具ID2
    pub origin_equip_pro3: i32, // 派生元 強化+3 - この防具の強化元防具ID3
    pub origin_equip_pro4: i32, // 派生元 強化+4 - この防具の強化元防具ID4
    pub origin_equip_pro5: i32, // 派生元 強化+5 - この防具の強化元防具ID5
    pub origin_equip_pro6: i32, // 派生元 強化+6 - この防具の強化元防具ID6
    pub origin_equip_pro7: i32, // 派生元 強化+7 - この防具の強化元防具ID7
    pub origin_equip_pro8: i32, // 派生元 強化+8 - この防具の強化元防具ID8
    pub origin_equip_pro9: i32, // 派生元 強化+9 - この防具の強化元防具ID9
    pub origin_equip_pro10: i32, // 派生元 強化+10 - この防具の強化元防具ID10
    pub origin_equip_pro11: i32, // 派生元 強化+11 - この防具の強化元防具ID11
    pub origin_equip_pro12: i32, // 派生元 強化+12 - この防具の強化元防具ID12
    pub origin_equip_pro13: i32, // 派生元 強化+13 - この防具の強化元防具ID13
    pub origin_equip_pro14: i32, // 派生元 強化+14 - この防具の強化元防具ID14
    pub origin_equip_pro15: i32, // 派生元 強化+15 - この防具の強化元防具ID15
    pub face_scale_mscale_x: f32, // 男横顔拡大スケール - 
    pub face_scale_mscale_z: f32, // 男前顔拡大スケール - 
    pub face_scale_mmax_x: f32, // 男横顔拡大最大倍率 - 
    pub face_scale_mmax_z: f32, // 男前顔拡大最大倍率 - 
    pub face_scale_fscale_x: f32, // 女横顔拡大スケール - 
    pub face_scale_fscale_z: f32, // 女前顔拡大スケール - 
    pub face_scale_fmax_x: f32, // 女横顔拡大最大倍率 - 
    pub face_scale_fmax_z: f32, // 女前顔拡大最大倍率 - 
    pub qwc_id: i32, // QWCID - QWCのパラメタID
    pub equip_model_id: u16, // 装備モデル番号 - 装備モデルの番号.
    pub icon_id_m: u16, // 男用アイコンID - 男用メニューアイコンID.
    pub icon_id_f: u16, // 女用アイコンID - 女用メニューアイコンID.
    pub knock_back: u16, // ノックバックカット率 - ノックバックの減少値.
    pub knockback_bounce_rate: u16, // ノックバック反発率 - ノックバックの反発率.
    pub durability: u16, // 耐久度 - 初期耐久度.
    pub durability_max: u16, // 耐久度最大値 - 新品耐久度.
    pub pad03: [u8;2], // pad - 
    pub def_flick_power: u16, // はじき防御力 - 敵の攻撃のはじき返し判定に利用.
    pub defense_physics: u16, // 物理防御力 - 物理攻撃のダメージ防御.
    pub defense_magic: u16, // 魔法防御力 - 魔法攻撃のダメージ防御.
    pub defense_fire: u16, // 炎防御力 - 炎攻撃のダメージ防御.
    pub defense_thunder: u16, // 電撃防御力 - 電撃攻撃のダメージ防御.
    pub defense_slash: i16, // 斬撃防御力 - 攻撃タイプを見て、斬撃属性のときは、防御力を減少させる
    pub defense_blow: i16, // 打撃防御力 - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
    pub defense_thrust: i16, // 刺突防御力 - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
    pub resist_poison: u16, // 毒耐性 - 毒状態異常へのかかりにくさ
    pub resist_disease: u16, // 疫病耐性 - 疫病状態異常へのかかりにくさ
    pub resist_blood: u16, // 出血耐性 - 出血状態異常へのかかりにくさ
    pub resist_curse: u16, // 呪耐性 - 呪い状態異常へのかかりにくさ
    pub reinforce_type_id: i16, // 強化タイプID - 強化タイプID
    pub trophy_s_grade_id: i16, // トロフィー - トロフィーシステムに関係あるか？
    pub shop_lv: i16, // ショップレベル - お店で販売できるレベル
    pub knockback_param_id: u8, // ノックバックパラメータID - ノックバックで使用するパラメータのID
    pub flick_damage_cut_rate: u8, // はじき時ダメージ減衰率[%] - はじき時のダメージ減衰率に使用
    pub equip_model_category: u8, // 装備モデル種別 - 装備モデルの種別.
    pub equip_model_gender: u8, // 装備モデル性別 - 装備モデルの性別.
    pub protector_category: u8, // 防具カテゴリ - 防具のカテゴリ.
    pub rarity: u8, // レア度 - アイテム取得ログで使うレア度
    pub sort_group_id: u8, // ソートアイテム種別ID - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
    pub parts_dmg_type: u8, // 部位ダメージ適用攻撃 - 部位ダメージ判定を行う攻撃タイプを設定
    pub pad04: [u8;2], // パディング - 

    pub pad06: bool, // パディング - 
    pub is_skip_weak_damage_anim: bool, // 弱点アニメをスキップするか - 弱点ダメージアニメ再生をスキップするかどうか。アニメを再生しないだけで「部位ダメージ率」「防御材質」は弱点として扱われます。
    pub use_face_scale: bool, // 顔スケールを使用するか - 顔スケールを使用するか
    pub leg_equip: bool, // 脚装備 - 脚装備か.
    pub arm_equip: bool, // 腕装備 - 腕装備か.
    pub body_equip: bool, // 胴装備 - 胴装備か.
    pub head_equip: bool, // 頭装備 - 頭装備か.
    pub is_deposit: bool, // 預けれるか - 倉庫に預けれるか

    pub defense_material_variation_valueweak: u8, // 弱点防御材質バリエーション値 - 弱点防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
    pub auto_foot_effect_decal_base_id2: i16, // フットデカール識別子2 - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
    pub auto_foot_effect_decal_base_id3: i16, // フットデカール識別子3 - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
    pub defense_material_variation_value: u8, // 防御材質バリエーション値 - 防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473

    pub pad: bool, // パディング - 
    pub show_dialog_cond_type: Integer<u8, packed_bits::Bits::<2>>, // 取得ダイアログ表示条件 - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
    pub show_log_cond_type: bool, // 取得ログ表示条件 - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
    pub simple_model_for_dlc: bool, // DLC用シンプルモデルありか - ＤＬＣ用シンプルモデルが存在しているか
    pub disable_multi_drop_share: bool, // マルチドロップ共有禁止か - マルチドロップ共有禁止か
    pub is_drop: bool, // その場に置けるか - アイテムをその場に置けるか？TRUE=置ける
    pub is_discard: bool, // 捨てれるか - アイテムを捨てれるか？TRUE=捨てれる

    pub neutral_damage_cut_rate: f32, // 無属性ダメージ倍率 - 無属性ダメージ倍率
    pub slash_damage_cut_rate: f32, // 斬撃ダメージ倍率 - 斬撃ダメージ倍率
    pub blow_damage_cut_rate: f32, // 打撃ダメージ倍率 - 打撃ダメージ倍率
    pub thrust_damage_cut_rate: f32, // 刺突ダメージ倍率 - 刺突ダメージ倍率
    pub magic_damage_cut_rate: f32, // 魔法ダメージ倍率 - 魔法ダメージ倍率
    pub fire_damage_cut_rate: f32, // 火炎ダメージ倍率 - 火炎ダメージ倍率
    pub thunder_damage_cut_rate: f32, // 電撃ダメージ倍率 - 電撃ダメージ倍率
    pub defense_material_sfx1: u16, // 防御材質1【SFX】 - 移動/防御時のSFX用.1
    pub defense_material_sfxweak1: u16, // 弱点防御材質1【SFX】 - 弱点部位ダメージ時のSFX用1
    pub defense_material1: u16, // 防御材質1【SE】 - 移動/防御時のSE用.1
    pub defense_materialweak1: u16, // 弱点防御材質1【SE】 - 弱点部位ダメージ時のSE用1
    pub defense_material_sfx2: u16, // 防御材質2【SFX】 - 移動/防御時のSFX用.2
    pub defense_material_sfxweak2: u16, // 弱点防御材質2【SFX】 - 弱点部位ダメージ時のSFX用2
    pub foot_material_se: u16, // 足装備材質【SE】 - 足装備SE用材質。足装備のみ参照される。(【GR】SEQ10061) 「139:なし」の場合は防御材質1【SE】が参照される
    pub defense_materialweak2: u16, // 弱点防御材質2【SE】 - 弱点部位ダメージ時のSE用2
    pub auto_foot_effect_decal_base_id1: i32, // フットデカール識別子1 - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
    pub toughness_damage_cut_rate: f32, // 強靭度 被ダメージ倍率 - 強靭度版カット率
    pub toughness_recover_correction: f32, // 強靭度 回復時間補正値 - 強靭度の回復時間用の補正値
    pub dark_damage_cut_rate: f32, // 闇ダメージ倍率 - 闇ダメージ倍率
    pub defense_dark: u16, // 闇防御力 - 闇攻撃のダメージ防御.

    pub invisible_flag48: bool, // PAD_元_#48#非表示 - 元_#48#非表示
    pub invisible_flag49: bool, // PAD_元_#49#非表示 - 元_#49#非表示
    pub invisible_flag50: bool, // PAD_元_#50#非表示 - 元_#50#非表示
    pub invisible_flag51: bool, // PAD_元_#51#非表示 - 元_#51#非表示
    pub invisible_flag52: bool, // PAD_元_#52#非表示 - 元_#52#非表示
    pub invisible_flag53: bool, // PAD_元_#53#非表示 - 元_#53#非表示
    pub invisible_flag54: bool, // PAD_元_#54#非表示 - 元_#54#非表示
    pub invisible_flag55: bool, // PAD_元_#55#非表示 - 元_#55#非表示

    pub invisible_flag56: bool, // PAD_元_#56#非表示 - 元_#56#非表示
    pub invisible_flag57: bool, // PAD_元_#57#非表示 - 元_#57#非表示
    pub invisible_flag58: bool, // PAD_元_#58#非表示 - 元_#58#非表示
    pub invisible_flag59: bool, // PAD_元_#59#非表示 - 元_#59#非表示
    pub invisible_flag60: bool, // PAD_元_#60#非表示 - 元_#60#非表示
    pub invisible_flag61: bool, // PAD_元_#61#非表示 - 元_#61#非表示
    pub invisible_flag62: bool, // PAD_元_#62#非表示 - 元_#62#非表示
    pub invisible_flag63: bool, // PAD_元_#63#非表示 - 元_#63#非表示

    pub invisible_flag64: bool, // PAD_元_#64#非表示 - 元_#64#非表示
    pub invisible_flag65: bool, // PAD_元_#65#非表示 - 元_#65#非表示
    pub invisible_flag66: bool, // PAD_元_#66#非表示 - 元_#66#非表示
    pub invisible_flag67: bool, // PAD_元_#67#非表示 - 元_#67#非表示
    pub invisible_flag68: bool, // PAD_元_#68#非表示 - 元_#68#非表示
    pub invisible_flag69: bool, // PAD_元_#69#非表示 - 元_#69#非表示
    pub invisible_flag70: bool, // PAD_元_#70#非表示 - 元_#70#非表示
    pub invisible_flag71: bool, // PAD_元_#71#非表示 - 元_#71#非表示

    pub invisible_flag72: bool, // PAD_元_#72#非表示 - 元_#72#非表示
    pub invisible_flag73: bool, // PAD_元_#73#非表示 - 元_#73#非表示
    pub invisible_flag74: bool, // PAD_元_#74#非表示 - 元_#74#非表示
    pub invisible_flag75: bool, // PAD_元_#75#非表示 - 元_#75#非表示
    pub invisible_flag76: bool, // PAD_元_#76#非表示 - 元_#76#非表示
    pub invisible_flag77: bool, // PAD_元_#77#非表示 - 元_#77#非表示
    pub invisible_flag78: bool, // PAD_元_#78#非表示 - 元_#78#非表示
    pub invisible_flag79: bool, // PAD_元_#79#非表示 - 元_#79#非表示

    pub invisible_flag80: bool, // PAD_元_#80#非表示 - 元_#80#非表示
    pub padbit: Integer<u8, packed_bits::Bits::<7>>, // パディング - 

    pub posture_control_id: u8, // 姿勢制御ID(胴) - 姿勢制御ID(胴)
    pub pad2: [u8;4], // pad - 
    pub sale_value: i32, // 販売価格 - 販売価格
    pub resist_freeze: u16, // 冷気耐性 - 冷気状態異常へのかかりにくさ
    pub invisible_flagsex_ver00: u8, // #00#非表示(男女指定) - 前髪の先
    pub invisible_flagsex_ver01: u8, // #01#非表示(男女指定) - 前髪の根元
    pub invisible_flagsex_ver02: u8, // #02#非表示(男女指定) - もみあげ
    pub invisible_flagsex_ver03: u8, // #03#非表示(男女指定) - 頭頂部
    pub invisible_flagsex_ver04: u8, // #04#非表示(男女指定) - 頭頂部
    pub invisible_flagsex_ver05: u8, // #05#非表示(男女指定) - 後ろ髪
    pub invisible_flagsex_ver06: u8, // #06#非表示(男女指定) - 後ろ髪の先
    pub invisible_flagsex_ver07: u8, // #07#非表示(男女指定) - 
    pub invisible_flagsex_ver08: u8, // #08#非表示(男女指定) - 
    pub invisible_flagsex_ver09: u8, // #09#非表示(男女指定) - 
    pub invisible_flagsex_ver10: u8, // #10#非表示(男女指定) - 襟
    pub invisible_flagsex_ver11: u8, // #11#非表示(男女指定) - 襟回り
    pub invisible_flagsex_ver12: u8, // #12#非表示(男女指定) - 
    pub invisible_flagsex_ver13: u8, // #13#非表示(男女指定) - 
    pub invisible_flagsex_ver14: u8, // #14#非表示(男女指定) - 
    pub invisible_flagsex_ver15: u8, // #15#非表示(男女指定) - 頭巾の裾
    pub invisible_flagsex_ver16: u8, // #16#非表示(男女指定) - 
    pub invisible_flagsex_ver17: u8, // #17#非表示(男女指定) - 
    pub invisible_flagsex_ver18: u8, // #18#非表示(男女指定) - 
    pub invisible_flagsex_ver19: u8, // #19#非表示(男女指定) - 
    pub invisible_flagsex_ver20: u8, // #20#非表示(男女指定) - 袖A
    pub invisible_flagsex_ver21: u8, // #21#非表示(男女指定) - 袖B
    pub invisible_flagsex_ver22: u8, // #22#非表示(男女指定) - 
    pub invisible_flagsex_ver23: u8, // #23#非表示(男女指定) - 
    pub invisible_flagsex_ver24: u8, // #24#非表示(男女指定) - 
    pub invisible_flagsex_ver25: u8, // #25#非表示(男女指定) - 腕
    pub invisible_flagsex_ver26: u8, // #26#非表示(男女指定) - 
    pub invisible_flagsex_ver27: u8, // #27#非表示(男女指定) - 
    pub invisible_flagsex_ver28: u8, // #28#非表示(男女指定) - 
    pub invisible_flagsex_ver29: u8, // #29#非表示(男女指定) - 
    pub invisible_flagsex_ver30: u8, // #30#非表示(男女指定) - ベルト
    pub invisible_flagsex_ver31: u8, // #31#非表示(男女指定) - 
    pub invisible_flagsex_ver32: u8, // #32#非表示(男女指定) - 
    pub invisible_flagsex_ver33: u8, // #33#非表示(男女指定) - 
    pub invisible_flagsex_ver34: u8, // #34#非表示(男女指定) - 
    pub invisible_flagsex_ver35: u8, // #35#非表示(男女指定) - 
    pub invisible_flagsex_ver36: u8, // #36#非表示(男女指定) - 
    pub invisible_flagsex_ver37: u8, // #37#非表示(男女指定) - 
    pub invisible_flagsex_ver38: u8, // #38#非表示(男女指定) - 
    pub invisible_flagsex_ver39: u8, // #39#非表示(男女指定) - 
    pub invisible_flagsex_ver40: u8, // #40#非表示(男女指定) - 
    pub invisible_flagsex_ver41: u8, // #41#非表示(男女指定) - 
    pub invisible_flagsex_ver42: u8, // #42#非表示(男女指定) - 
    pub invisible_flagsex_ver43: u8, // #43#非表示(男女指定) - 
    pub invisible_flagsex_ver44: u8, // #44#非表示(男女指定) - 
    pub invisible_flagsex_ver45: u8, // #45#非表示(男女指定) - 
    pub invisible_flagsex_ver46: u8, // #46#非表示(男女指定) - 
    pub invisible_flagsex_ver47: u8, // #47#非表示(男女指定) - 
    pub invisible_flagsex_ver48: u8, // #48#非表示(男女指定) - 
    pub invisible_flagsex_ver49: u8, // #49#非表示(男女指定) - 
    pub invisible_flagsex_ver50: u8, // #50#非表示(男女指定) - 
    pub invisible_flagsex_ver51: u8, // #51#非表示(男女指定) - 
    pub invisible_flagsex_ver52: u8, // #52#非表示(男女指定) - 
    pub invisible_flagsex_ver53: u8, // #53#非表示(男女指定) - 
    pub invisible_flagsex_ver54: u8, // #54#非表示(男女指定) - 
    pub invisible_flagsex_ver55: u8, // #55#非表示(男女指定) - 
    pub invisible_flagsex_ver56: u8, // #56#非表示(男女指定) - 
    pub invisible_flagsex_ver57: u8, // #57#非表示(男女指定) - 
    pub invisible_flagsex_ver58: u8, // #58#非表示(男女指定) - 
    pub invisible_flagsex_ver59: u8, // #59#非表示(男女指定) - 
    pub invisible_flagsex_ver60: u8, // #60#非表示(男女指定) - 
    pub invisible_flagsex_ver61: u8, // #61#非表示(男女指定) - 
    pub invisible_flagsex_ver62: u8, // #62#非表示(男女指定) - 
    pub invisible_flagsex_ver63: u8, // #63#非表示(男女指定) - 
    pub invisible_flagsex_ver64: u8, // #64#非表示(男女指定) - 
    pub invisible_flagsex_ver65: u8, // #65#非表示(男女指定) - 
    pub invisible_flagsex_ver66: u8, // #66#非表示(男女指定) - 
    pub invisible_flagsex_ver67: u8, // #67#非表示(男女指定) - 
    pub invisible_flagsex_ver68: u8, // #68#非表示(男女指定) - 
    pub invisible_flagsex_ver69: u8, // #69#非表示(男女指定) - 
    pub invisible_flagsex_ver70: u8, // #70#非表示(男女指定) - 
    pub invisible_flagsex_ver71: u8, // #71#非表示(男女指定) - 
    pub invisible_flagsex_ver72: u8, // #72#非表示(男女指定) - 
    pub invisible_flagsex_ver73: u8, // #73#非表示(男女指定) - 
    pub invisible_flagsex_ver74: u8, // #74#非表示(男女指定) - 
    pub invisible_flagsex_ver75: u8, // #75#非表示(男女指定) - 
    pub invisible_flagsex_ver76: u8, // #76#非表示(男女指定) - 
    pub invisible_flagsex_ver77: u8, // #77#非表示(男女指定) - 
    pub invisible_flagsex_ver78: u8, // #78#非表示(男女指定) - 
    pub invisible_flagsex_ver79: u8, // #79#非表示(男女指定) - 
    pub invisible_flagsex_ver80: u8, // #80#非表示(男女指定) - 
    pub invisible_flagsex_ver81: u8, // #81#非表示(男女指定) - 
    pub invisible_flagsex_ver82: u8, // #82#非表示(男女指定) - 
    pub invisible_flagsex_ver83: u8, // #83#非表示(男女指定) - 
    pub invisible_flagsex_ver84: u8, // #84#非表示(男女指定) - 
    pub invisible_flagsex_ver85: u8, // #85#非表示(男女指定) - 
    pub invisible_flagsex_ver86: u8, // #86#非表示(男女指定) - 
    pub invisible_flagsex_ver87: u8, // #87#非表示(男女指定) - 
    pub invisible_flagsex_ver88: u8, // #88#非表示(男女指定) - 
    pub invisible_flagsex_ver89: u8, // #89#非表示(男女指定) - 
    pub invisible_flagsex_ver90: u8, // #90#非表示(男女指定) - 
    pub invisible_flagsex_ver91: u8, // #91#非表示(男女指定) - 
    pub invisible_flagsex_ver92: u8, // #92#非表示(男女指定) - 
    pub invisible_flagsex_ver93: u8, // #93#非表示(男女指定) - 
    pub invisible_flagsex_ver94: u8, // #94#非表示(男女指定) - 
    pub invisible_flagsex_ver95: u8, // #95#非表示(男女指定) - 
    pub pad404: [u8;14], // パディング - パディング
}
