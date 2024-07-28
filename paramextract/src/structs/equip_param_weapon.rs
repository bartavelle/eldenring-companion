use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian="lsb", bit_numbering="msb0")]
pub struct EQUIP_PARAM_WEAPON_ST {
    pub disable_paramn_t: bool, // NT版出力から外すか - ○をつけたパラメータをNT版パッケージでは除外します
    pub disable_param_reserve1: Integer<u8, packed_bits::Bits::<7>>, // パッケージ出力用リザーブ1 - パッケージ出力用リザーブ1
    pub disable_param_reserve2: [u8;3], // パッケージ出力用リザーブ2 - パッケージ出力用リザーブ2
    pub behavior_variation_id: i32, // 行動バリエーションID - 攻撃時に参照する行動パラメータIDを決定するときに使う
    pub sort_id: i32, // ソートID - ソートID(-1:集めない)(プログラム内で強化レベルを加味するため s32 では７桁が限界)
    pub wandering_equip_id: u32, // 徘徊装備ID - 徘徊ゴースト用の差し替え装備ID.
    pub weight: f32, // 重量[kg] - 重量[kg].
    pub weapon_weight_rate: f32, // 装備重量比率 - 装備重量比率
    pub fix_price: i32, // 修理価格 - 修理基本価格
    pub reinforce_price: i32, // 強化価格 - 強化価格
    pub sell_value: i32, // 売却価格 - 販売価格
    pub correct_strength: f32, // 筋力補正 - キャラパラ補正値.
    pub correct_agility: f32, // 俊敏補正 - キャラパラ補正値.
    pub correct_magic: f32, // 魔力補正 - キャラパラ補正値.
    pub correct_faith: f32, // 信仰補正 - キャラパラ補正値.
    pub phys_guard_cut_rate: f32, // ガード時物理攻撃カット率 - ガード時のダメージカット率を各攻撃ごとに設定
    pub mag_guard_cut_rate: f32, // ガード時魔法攻撃カット率 - ガード攻撃でない場合は、0を入れる
    pub fire_guard_cut_rate: f32, // ガード時炎攻撃力カット率 - 炎攻撃をどれだけカットするか？
    pub thun_guard_cut_rate: f32, // ガード時電撃攻撃力カット率 - 電撃攻撃をどれだけカットするか？
    pub sp_effect_behavior_id0: i32, // 攻撃ヒット時特殊効果ID0 - 武器に特殊効果を追加するときに登録する
    pub sp_effect_behavior_id1: i32, // 攻撃ヒット時特殊効果ID1 - 武器に特殊効果を追加するときに登録する
    pub sp_effect_behavior_id2: i32, // 攻撃ヒット時特殊効果ID2 - 武器に特殊効果を追加するときに登録する
    pub resident_sp_effect_id: i32, // 常駐特殊効果ID - 常駐特殊効果ID0
    pub resident_sp_effect_id1: i32, // 常駐特殊効果ID1 - 常駐特殊効果ID1
    pub resident_sp_effect_id2: i32, // 常駐特殊効果ID2 - 常駐特殊効果ID2
    pub material_set_id: i32, // 素材ID - 武器強化に必要な素材パラメータID
    pub origin_equip_wep: i32, // 派生元 - この武器の強化元武器ID
    pub origin_equip_wep1: i32, // 派生元 強化+1 - この武器の強化元武器ID1
    pub origin_equip_wep2: i32, // 派生元 強化+2 - この武器の強化元武器ID2
    pub origin_equip_wep3: i32, // 派生元 強化+3 - この武器の強化元武器ID3
    pub origin_equip_wep4: i32, // 派生元 強化+4 - この武器の強化元武器ID4
    pub origin_equip_wep5: i32, // 派生元 強化+5 - この武器の強化元武器ID5
    pub origin_equip_wep6: i32, // 派生元 強化+6 - この武器の強化元武器ID6
    pub origin_equip_wep7: i32, // 派生元 強化+7 - この武器の強化元武器ID7
    pub origin_equip_wep8: i32, // 派生元 強化+8 - この武器の強化元武器ID8
    pub origin_equip_wep9: i32, // 派生元 強化+9 - この武器の強化元武器ID9
    pub origin_equip_wep10: i32, // 派生元 強化+10 - この武器の強化元武器ID10
    pub origin_equip_wep11: i32, // 派生元 強化+11 - この武器の強化元武器ID11
    pub origin_equip_wep12: i32, // 派生元 強化+12 - この武器の強化元武器ID12
    pub origin_equip_wep13: i32, // 派生元 強化+13 - この武器の強化元武器ID13
    pub origin_equip_wep14: i32, // 派生元 強化+14 - この武器の強化元武器ID14
    pub origin_equip_wep15: i32, // 派生元 強化+15 - この武器の強化元武器ID15
    pub weak_adamage_rate: f32, // 特攻Aダメージ倍率 - 特攻A用のダメージ倍率
    pub weak_bdamage_rate: f32, // 特攻Bダメージ倍率 - 特攻B用のダメージ倍率
    pub weak_cdamage_rate: f32, // 特攻Cダメージ倍率 - 特攻C用のダメージ倍率
    pub weak_ddamage_rate: f32, // 特攻Dダメージ倍率 - 特攻D用のダメージ倍率
    pub sleep_guard_resistmax_correct: f32, // 睡眠耐性カット率_最大補正値 - 睡眠に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub madness_guard_resistmax_correct: f32, // 発狂耐性カット率_最大補正値 - 発狂に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub sa_weapon_damage: f32, // SA武器攻撃力 - スーパーアーマー基本攻撃力
    pub equip_model_id: u16, // 装備モデル番号 - 装備モデルの番号.
    pub icon_id: u16, // アイコンID - メニューアイコンID.
    pub durability: u16, // 耐久度 - 初期耐久度.
    pub durability_max: u16, // 耐久度最大値 - 新品耐久度.
    pub attack_throw_escape: u16, // 投げ抜け攻撃力基本値 - 投げ抜け攻撃力の基本値
    pub parry_damage_life: i16, // パリィ発生時間[frame] - パリィダメージの寿命を制限する。TimeActで設定されている以上には持続しない。
    pub attack_base_physics: u16, // 物理攻撃力基本値 - 敵のＨＰにダメージを与える物理属性攻撃の基本値
    pub attack_base_magic: u16, // 魔法攻撃力基本値 - 敵のＨＰにダメージを与える魔法属性攻撃の基本値
    pub attack_base_fire: u16, // 炎攻撃力基本値 - 敵のＨＰにダメージを与える炎属性攻撃の基本値
    pub attack_base_thunder: u16, // 電撃攻撃力基本値 - 敵のＨＰにダメージを与える電撃属性攻撃の基本値
    pub attack_base_stamina: u16, // スタミナ攻撃力 - 敵へのスタミナ攻撃力
    pub guard_angle: i16, // ガード範囲[deg] - 武器のガード時の防御発生範囲角度
    pub sa_durability: f32, // SA耐久値 - 攻撃モーション中に使われる追加SA耐久値
    pub stamina_guard_def: i16, // ガード時スタミナ防御力 - ガード成功時に、敵のスタミナ攻撃に対する防御力
    pub reinforce_type_id: i16, // 強化タイプID - 強化タイプID
    pub trophy_s_grade_id: i16, // トロフィーＳグレードID - トロフィーシステムに関係あるか？
    pub trophy_seq_id: i16, // トロフィーSEQ番号 - トロフィーのSEQ番号（１３～２９）
    pub throw_atk_rate: i16, // 投げ攻撃力倍率 - 投げの攻撃力倍率
    pub bow_dist_rate: i16, // 弓飛距離補正[％] - 飛距離を伸ばすアップ％
    pub equip_model_category: u8, // 装備モデル種別 - 装備モデルの種別.
    pub equip_model_gender: u8, // 装備モデル性別 - 装備モデルの性別.
    pub weapon_category: u8, // 武器カテゴリ - 武器のカテゴリ.
    pub wepmotion_category: u8, // 武器モーションカテゴリ - 武器モーションのカテゴリ.
    pub guardmotion_category: u8, // ガードモーションカテゴリ - ガードモーションのカテゴリ
    pub atk_material: u8, // 攻撃材質 - 攻撃パラから使用される攻撃材質
    pub def_se_material1: u16, // 防御SE材質1 - 攻撃パラから使用される防御SE材質1
    pub correct_typephysics: u8, // 補正タイプ（物理攻撃力） - 一次パラメータによる物理攻撃力の補正グラフのタイプを決める
    pub sp_attribute: u8, // 特殊属性 - 武器の特殊属性値
    pub sp_atkcategory: u16, // 特殊攻撃カテゴリ - 特殊攻撃カテゴリ（50～999まで可能)
    pub wepmotion_one_hand_id: u8, // 武器モーション片手ID - 片手装備時の基本モーションID.
    pub wepmotion_both_hand_id: u8, // 武器モーション両手ID - 両手装備時の基本モーションID.
    pub proper_strength: u8, // 装備適正筋力 - 装備適正値.
    pub proper_agility: u8, // 装備適正俊敏 - 装備適正値.
    pub proper_magic: u8, // 装備適正魔力 - 装備適正値.
    pub proper_faith: u8, // 装備適正信仰 - 装備適正値.
    pub over_strength: u8, // 筋力オーバー開始値 - 筋力オーバー開始値
    pub attack_base_parry: u8, // パリィ攻撃基本値 - 敵のパリィをやぶるための基本値
    pub defense_base_parry: u8, // パリィ防御値 - パリィ判定時に、パリィになるかガードになるかの判定に利用
    pub guard_base_repel: u8, // はじき防御力基本値 - 敵の攻撃をガードしたときに、はじけるかどうかの判定に利用
    pub attack_base_repel: u8, // はじき攻撃力基本値 - ガード敵を攻撃した時に、はじかれるかどうかの判定に利用
    pub guard_cut_cancel_rate: i8, // ガードカット無効化倍率 - 相手のガードカットを無効化させる倍率。-100で完全無効。100で相手の防御効果倍増。
    pub guard_level: i8, // ガードレベル - ガードしたとき、敵の攻撃をどのガードモーションで受けるか？を決める
    pub slash_guard_cut_rate: i8, // 斬撃攻撃カット率 - 攻撃タイプを見て、斬撃属性のダメージを何％カットするか？を指定
    pub blow_guard_cut_rate: i8, // 打撃攻撃カット率 - 攻撃タイプを見て、打撃属性のダメージを何％カットするか？を指定
    pub thrust_guard_cut_rate: i8, // 刺突攻撃カット率 - 攻撃タイプを見て、刺突属性のダメージを何％カットするか？を指定
    pub poison_guard_resist: i8, // 毒耐性カット率 - 毒にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub disease_guard_resist: i8, // 疫病攻撃カット率 - 疫病にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub blood_guard_resist: i8, // 出血攻撃カット率 - 出血にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub curse_guard_resist: i8, // 呪攻撃カット率 - 呪いにする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub atk_attribute: u8, // 物理属性1 - 物理属性1
    pub right_hand_equipable: bool, // 右手装備 - 右手装備可能か.
    pub left_hand_equipable: bool, // 左手装備 - 左手装備可能か.
    pub both_hand_equipable: bool, // 両手装備 - 両手装備可能か.
    pub arrow_slot_equipable: bool, // 弓矢弾装備 - 弓用矢弾装備可能か.
    pub bolt_slot_equipable: bool, // 弩矢弾装備 - 弩用矢弾装備可能か.
    pub enable_guard: bool, // ガード可能 - 左手装備時L1でガード
    pub enable_parry: bool, // パリィ可能 - 左手装備時L2でパリィ
    pub enable_magic: bool, // 魔法可能 - 攻撃時に魔法発動
    pub enable_sorcery: bool, // 呪術可能 - 攻撃時に呪術発動
    pub enable_miracle: bool, // 奇蹟可能 - 攻撃時に奇蹟発動
    pub enable_vow_magic: bool, // 誓約魔法可能 - 攻撃時に誓約魔法発動
    pub is_normal_attack_type: bool, // 通常 - メニュー表示用攻撃タイプ。通常か
    pub is_blow_attack_type: bool, // 打撃 - メニュー表示用攻撃タイプ。打撃か
    pub is_slash_attack_type: bool, // 斬撃 - メニュー表示用攻撃タイプ。斬撃か
    pub is_thrust_attack_type: bool, // 刺突 - メニュー表示用攻撃タイプ。刺突か
    pub is_enhance: bool, // エンチャント可能か？ - 松脂などで、強化可能か？
    pub is_hero_point_correct: bool, // 人間性補正あるか - 人間性による攻撃力補正があるか
    pub is_custom: bool, // 強化できるか？ - 強化ショップで強化対象リストに並ぶ(仕様変更で削除するかも？)
    pub disable_base_change_reset: bool, // 転職リセット禁止か - 転職リセット禁止か
    pub disable_repair: bool, // 修理禁止か - 修理禁止か
    pub is_dark_hand: bool, // ダークハンドか - ダークハンドか
    pub simple_model_for_dlc: bool, // DLC用シンプルモデルありか - ＤＬＣ用シンプルモデルが存在しているか
    pub lantern_wep: bool, // ランタン武器 - ランタン武器か
    pub is_versus_ghost_wep: bool, // 対霊武器 - NPCパラの「霊体か」が○の相手に攻撃を当たるようになります。また、攻撃パラの「霊体攻撃か」が○の攻撃をガードできるようになります。
    pub base_change_category: Integer<u8, packed_bits::Bits::<6>>, // 武器転職カテゴリ - 武器転職カテゴリ。属性アイコン表示に使用します。
    pub is_dragon_slayer: bool, // 竜狩りか - 竜狩り武器か
    pub is_deposit: bool, // 預けれるか - 倉庫に預けれるか
    pub disable_multi_drop_share: bool, // マルチドロップ共有禁止か - マルチドロップ共有禁止か
    pub is_discard: bool, // 捨てれるか - アイテムを捨てれるか？TRUE=捨てれる
    pub is_drop: bool, // その場に置けるか - アイテムをその場に置けるか？TRUE=置ける
    pub show_log_cond_type: bool, // 取得ログ表示条件 - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
    pub enable_throw: bool, // 投げ可能 - 投げ可能な武器かどうか
    pub show_dialog_cond_type: Integer<u8, packed_bits::Bits::<2>>, // 取得ダイアログ表示条件 - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
    pub disable_gem_attr: bool, // 魔石属性変更禁止か - 魔石属性変更禁止か
    pub def_sfx_material1: u16, // 防御SFX材質1 - 攻撃パラから使用される防御SFX材質1
    pub wep_collidable_type0: u8, // 武器コライダブル設定 - 武器のコライダブル設定
    pub wep_collidable_type1: u8, // 武器1コライダブル設定 - 武器1のコライダブル設定
    pub posture_control_idright: u8, // 姿勢制御ID(右手) - 姿勢制御ID(右手)
    pub posture_control_idleft: u8, // 姿勢制御ID(左手) - 姿勢制御ID(左手)
    pub trace_sfx_id0: i32, // 剣閃SfxID_０ - 剣閃SfxID_０(-1無効)
    pub trace_dmy_id_head0: i32, // 根元剣閃ダミポリID_０ - 剣閃根元ダミポリID_０(-1無効)
    pub trace_dmy_id_tail0: i32, // 剣先剣閃ダミポリID_０ - 剣閃剣先ダミポリID_０
    pub trace_sfx_id1: i32, // 剣閃SfxID_１ - 剣閃SfxID_１(-1無効)
    pub trace_dmy_id_head1: i32, // 根元剣閃ダミポリID_１ - 剣閃根元ダミポリID_１(-1無効)
    pub trace_dmy_id_tail1: i32, // 剣先剣閃ダミポリID_１ - 剣閃剣先ダミポリID_１
    pub trace_sfx_id2: i32, // 剣閃SfxID_２ - 剣閃SfxID_２(-1無効)
    pub trace_dmy_id_head2: i32, // 根元剣閃ダミポリID_２ - 剣閃根元ダミポリID_２(-1無効)
    pub trace_dmy_id_tail2: i32, // 剣先剣閃ダミポリID_２ - 剣閃剣先ダミポリID_２
    pub trace_sfx_id3: i32, // 剣閃SfxID_３ - 剣閃SfxID_３(-1無効)
    pub trace_dmy_id_head3: i32, // 根元剣閃ダミポリID_３ - 剣閃根元ダミポリID_３(-1無効)
    pub trace_dmy_id_tail3: i32, // 剣先剣閃ダミポリID_３ - 剣閃剣先ダミポリID_３
    pub trace_sfx_id4: i32, // 剣閃SfxID_４ - 剣閃SfxID_４(-1無効)
    pub trace_dmy_id_head4: i32, // 根元剣閃ダミポリID_４ - 剣閃根元ダミポリID_４(-1無効)
    pub trace_dmy_id_tail4: i32, // 剣先剣閃ダミポリID_４ - 剣閃剣先ダミポリID_４
    pub trace_sfx_id5: i32, // 剣閃SfxID_５ - 剣閃SfxID_５(-1無効)
    pub trace_dmy_id_head5: i32, // 根元剣閃ダミポリID_５ - 剣閃根元ダミポリID_５(-1無効)
    pub trace_dmy_id_tail5: i32, // 剣先剣閃ダミポリID_５ - 剣閃剣先ダミポリID_５
    pub trace_sfx_id6: i32, // 剣閃SfxID_６ - 剣閃SfxID_６(-1無効)
    pub trace_dmy_id_head6: i32, // 根元剣閃ダミポリID_６ - 剣閃根元ダミポリID_６(-1無効)
    pub trace_dmy_id_tail6: i32, // 剣先剣閃ダミポリID_６ - 剣閃剣先ダミポリID_６
    pub trace_sfx_id7: i32, // 剣閃SfxID_７ - 剣閃SfxID_７(-1無効)
    pub trace_dmy_id_head7: i32, // 根元剣閃ダミポリID_７ - 剣閃根元ダミポリID_７(-1無効)
    pub trace_dmy_id_tail7: i32, // 剣先剣閃ダミポリID_７ - 剣閃剣先ダミポリID_７
    pub def_sfx_material2: u16, // 防御SFX材質2 - 攻撃パラから使用される防御SFX材質2
    pub def_se_material2: u16, // 防御SE材質2 - 攻撃パラから使用される防御SE材質2
    pub absorp_param_id: i32, // 吸着位置Id - 武器吸着位置パラメータのId。この値により武器が吸着する位置を決定する(-1：旧ソースコード直書きの値を参照する)
    pub toughness_correct_rate: f32, // 強靭度 補正倍率 - 強靭度の基本値を補正する倍率です
    pub is_valid_toughprot_s_a_dmg: bool, // 防具SAダメージ倍率が初期値でも有効か？ - 防具SAが初期値でも強靭度計算が行われるかどうか。詳細は強靭度仕様書.xlsxを確認してください
    pub is_dual_blade: bool, // 双剣か - この武器は双剣か。
    pub is_auto_equip: bool, // 自動装填可能か - 矢・ボルトのみ有効。新しくこの武器を拾っ時に対象装備スロットが空の場合に自動で装備するかどうか
    pub is_enable_emergency_step: bool, // 緊急回避可能か - 緊急回避可能な武器かどうか。ビヘイビアスクリプトに渡す。
    pub invisible_on_remo: bool, // カットシーン中非表示か - カットシーン中非表示か
    pub pad2: Integer<u8, packed_bits::Bits::<3>>, // pad - 
    pub correct_typemagic: u8, // 補正タイプ（魔法攻撃力） - 一次パラメータによる魔法攻撃力の補正グラフのタイプを決める
    pub correct_typefire: u8, // 補正タイプ（炎攻撃力） - 一次パラメータによる炎攻撃力の補正グラフのタイプを決める
    pub correct_typethunder: u8, // 補正タイプ（雷攻撃力） - 一次パラメータによる雷攻撃力の補正グラフのタイプを決める
    pub weak_edamage_rate: f32, // 特攻Eダメージ倍率 - 特攻E用のダメージ倍率
    pub weak_fdamage_rate: f32, // 特攻Fダメージ倍率 - 特攻F用のダメージ倍率
    pub dark_guard_cut_rate: f32, // ガード時闇攻撃力カット率 - 闇攻撃をどれだけカットするか？
    pub attack_base_dark: u16, // 闇攻撃力基本値 - 敵のＨＰにダメージを与える闇属性攻撃の基本値
    pub correct_typedark: u8, // 補正タイプ（闇攻撃力） - 一次パラメータによる闇攻撃力の補正グラフのタイプを決める
    pub correct_typepoison: u8, // 補正タイプ（毒攻撃力） - 一次パラメータによる毒攻撃力の補正グラフのタイプを決める
    pub sort_group_id: u8, // ソートアイテム種別ID - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
    pub atk_attribute2: u8, // 物理属性2 - 物理属性2
    pub sleep_guard_resist: i8, // 睡眠攻撃カット率 - 睡眠にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub madness_guard_resist: i8, // 発狂攻撃カット率 - 発狂にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub correct_typeblood: u8, // 補正タイプ（出血攻撃力） - 一次パラメータによる出血攻撃力の補正グラフのタイプを決める
    pub proper_luck: u8, // 装備適正運 - 装備適正値.
    pub freeze_guard_resist: i8, // 冷気攻撃カット率 - 冷気にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
    pub auto_replenish_type: u8, // 自動補充タイプ - 自動補充する/しないの可否およびデフォルト設定をコントロールします
    pub sword_arts_param_id: i32, // アーツパラメータID - アーツパラメータのID
    pub correct_luck: f32, // 運補正 - キャラパラ補正値.
    pub arrow_bolt_equip_id: u32, // 矢筒(弾倉)表示モデル用装備ID - 矢筒(弾倉)表示モデルの装備品番号。弓の場合は矢筒、弩の場合は弾倉として表示する。
    pub derivation_level_type: u8, // 還元時レベル設定 - 武器を還元・派生させるときに強化レベルをどう設定するかの種別
    pub enchant_sfx_size: u8, // エンチャントSfxサイズ - エンチャントSfxIdにオフセットする値
    pub wep_type: u16, // 武器種別 - 武器種別。テキストと、魔石の紐付けに使われる（※テキスト以外にも使われるようになった）
    pub phys_guard_cut_ratemax_correct: f32, // ガード時物理攻撃カット率_最大補正値 - ガード時のダメージ物理カット率の補正値の最大値
    pub mag_guard_cut_ratemax_correct: f32, // ガード時魔法攻撃カット率_最大補正値 - ガード時のダメージ魔法カット率の補正値の最大値
    pub fire_guard_cut_ratemax_correct: f32, // ガード時炎攻撃力カット率_最大補正値 - ガード時のダメージ炎カット率の補正値の最大値
    pub thun_guard_cut_ratemax_correct: f32, // ガード時電撃攻撃力カット率_最大補正値 - ガード時のダメージ電撃カット率の補正値の最大値
    pub dark_guard_cut_ratemax_correct: f32, // ガード時闇攻撃力カット率_最大補正値 - ガード時のダメージ闇カット率の補正値の最大値
    pub poison_guard_resistmax_correct: f32, // 毒耐性カット率_最大補正値 - 毒に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub disease_guard_resistmax_correct: f32, // 疫病耐性カット率_最大補正値 - 疫病に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub blood_guard_resistmax_correct: f32, // 出血耐性カット率_最大補正値 - 出血に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub curse_guard_resistmax_correct: f32, // 呪耐性カット率_最大補正値 - 呪いに対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub freeze_guard_resistmax_correct: f32, // 冷気耐性カット率_最大補正値 - 冷気に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
    pub stamina_guard_defmax_correct: f32, // ガード時スタミナ防御力_最大補正値 - ガード成功時に、敵のスタミナ攻撃に対する防御力の補正値の最大値
    pub resident_sfx_id_1: i32, // 常駐SfxId１ - 常駐SfxId1
    pub resident_sfx_id_2: i32, // 常駐SfxId２ - 常駐SfxId2
    pub resident_sfx_id_3: i32, // 常駐SfxId３ - 常駐SfxId3
    pub resident_sfx_id_4: i32, // 常駐SfxId４ - 常駐SfxId4
    pub resident_sfxdmy_id_1: i32, // 常駐SfxダミポリId１ - 常駐SfxダミポリId１
    pub resident_sfxdmy_id_2: i32, // 常駐SfxダミポリId２ - 常駐SfxダミポリId２
    pub resident_sfxdmy_id_3: i32, // 常駐SfxダミポリId３ - 常駐SfxダミポリId３
    pub resident_sfxdmy_id_4: i32, // 常駐SfxダミポリId４ - 常駐SfxダミポリId４
    pub stamina_consumption_rate: f32, // スタミナ消費量倍率 - スタミナ消費量倍率
    pub vs_player_dmg_correct_ratephysics: f32, // 対プレイヤー 物理ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratemagic: f32, // 対プレイヤー 魔法ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratefire: f32, // 対プレイヤー 炎ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratethunder: f32, // 対プレイヤー 雷ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratedark: f32, // 対プレイヤー 闇ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratepoison: f32, // 対プレイヤー 毒ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_rateblood: f32, // 対プレイヤー 出血ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratefreeze: f32, // 対プレイヤー 冷気ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub attainment_wep_status_str: i32, // 武器能力解放ステータス値：筋力 - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
    pub attainment_wep_status_dex: i32, // 武器能力解放ステータス値：技量 - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
    pub attainment_wep_status_mag: i32, // 武器能力解放ステータス値：理力 - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
    pub attainment_wep_status_fai: i32, // 武器能力解放ステータス値：信仰 - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
    pub attainment_wep_status_luc: i32, // 武器能力解放ステータス値：運 - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
    pub attack_element_correct_id: i32, // 攻撃属性補正ID - 攻撃属性を補正するパラメータのID
    pub sale_value: i32, // 販売価格 - 販売価格
    pub reinforce_shop_category: u8, // 強化ショップカテゴリ - 強化ショップカテゴリ
    pub max_arrow_quantity: u8, // 矢の最大所持数 - 矢の最大所持数
    pub resident_sfx_1is_visible_for_hang: bool, // 常駐SFX1納刀時表示するか - 「常駐SFX1納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID1」に設定されているSFXを非表示にする
    pub resident_sfx_2is_visible_for_hang: bool, // 常駐SFX2納刀時表示するか - 「常駐SFX2納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID2」に設定されているSFXを非表示にする
    pub resident_sfx_3is_visible_for_hang: bool, // 常駐SFX3納刀時表示するか - 「常駐SFX3納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID3」に設定されているSFXを非表示にする
    pub resident_sfx_4is_visible_for_hang: bool, // 常駐SFX4納刀時表示するか - 「常駐SFX4納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID4」に設定されているSFXを非表示にする
    pub is_soul_param_id_change_model0: bool, // モデル_0 ソウルパラムID差し替え可能か - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
    pub is_soul_param_id_change_model1: bool, // モデル_1 ソウルパラムID差し替え可能か - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
    pub is_soul_param_id_change_model2: bool, // モデル_2 ソウルパラムID差し替え可能か - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
    pub is_soul_param_id_change_model3: bool, // モデル_3 ソウルパラムID差し替え可能か - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
    pub wep_se_id_offset: i8, // 武器SEIDオフセット値 - SEIDのオフセット値 
    pub base_change_price: i32, // 進化価格 - 進化価格
    pub level_sync_correct_id: i16, // レベルシンク補正ID - レベルシンク補正ID
    pub correct_typesleep: u8, // 補正タイプ（睡眠攻撃力） - 一次パラメータによる睡眠攻撃力の補正グラフのタイプを決める
    pub correct_typemadness: u8, // 補正タイプ（発狂攻撃力） - 一次パラメータによる発狂攻撃力の補正グラフのタイプを決める
    pub rarity: u8, // レア度 - アイテム取得ログで使うレア度
    pub gem_mount_type: u8, // 魔石装着可能か - 魔石装着可能か
    pub wep_regain_hp: u16, // 武器リゲイン量 - 武器リゲイン量
    pub sp_effect_msg_id0: i32, // 効果テキストID00 - 効果テキストID00(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
    pub sp_effect_msg_id1: i32, // 効果テキストID01 - 効果テキストID01(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
    pub sp_effect_msg_id2: i32, // 効果テキストID02 - 効果テキストID02(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
    pub origin_equip_wep16: i32, // 派生元 強化+16 - この武器の強化元武器ID16
    pub origin_equip_wep17: i32, // 派生元 強化+17 - この武器の強化元武器ID17
    pub origin_equip_wep18: i32, // 派生元 強化+18 - この武器の強化元武器ID18
    pub origin_equip_wep19: i32, // 派生元 強化+19 - この武器の強化元武器ID19
    pub origin_equip_wep20: i32, // 派生元 強化+20 - この武器の強化元武器ID20
    pub origin_equip_wep21: i32, // 派生元 強化+21 - この武器の強化元武器ID21
    pub origin_equip_wep22: i32, // 派生元 強化+22 - この武器の強化元武器ID22
    pub origin_equip_wep23: i32, // 派生元 強化+23 - この武器の強化元武器ID23
    pub origin_equip_wep24: i32, // 派生元 強化+24 - この武器の強化元武器ID24
    pub origin_equip_wep25: i32, // 派生元 強化+25 - この武器の強化元武器ID25
    pub vs_player_dmg_correct_ratesleep: f32, // 対プレイヤー 睡眠ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratemadness: f32, // 対プレイヤー 発狂ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub sa_guard_cut_rate: f32, // ガード時SA攻撃カット率 - ガード成功時のSAダメージのカット率
    pub def_material_variation_value: u8, // 防御材質バリエーション値 - ガード時に使用される防御材質と組み合わせてダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
    pub sp_attribute_variation_value: u8, // 特殊属性バリエーション値 - 武器の特殊属性と組み合わせて状態異常SFX,SEなどにバリエーションを持たせるために使用する値です。SEQ16473
    pub stealth_atk_rate: i16, // ステルス攻撃力倍率 - ステルス攻撃力倍率
    pub vs_player_dmg_correct_ratedisease: f32, // 対プレイヤー 疫病ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub vs_player_dmg_correct_ratecurse: f32, // 対プレイヤー 呪ダメージ補正倍率 - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
    pub pad: [u8;8], //  - pad
}
