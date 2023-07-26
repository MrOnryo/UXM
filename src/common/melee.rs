use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use crate::globals::STATUS_KIND_INTERRUPT;
use crate::globals::PREV_STATUS_KIND;
use crate::globals::STICK_X;
use smash2::app::work_ids::fighter::rockman::status::special_n::STICK_X as OtherSTICK_X;
use smash2::app::work_ids::fighter::rosetta::status::special_hi_common::STICK_X as OtherOtherSTICK_X;
use smash2::app::work_ids::fighter::sheik::status::special_hi::STICK_X as OtherOtherOtherSTICK_X;
use crate::globals::GUARD_CONT_UNIQ;
use crate::globals::PAD_FLAG;
use smash2::app::work_ids::weapon::rosetta_tico::status::common::PAD_FLAG as OtherPAD_FLAG;
use crate::globals::CMD_CAT1;
use crate::globals::CMD_CAT2;
use crate::globals::CMD_CAT3;
use crate::globals::SITUATION_KIND;
use smash2::app::work_ids::fighter::metaknight::status::special_lw_attack::SITUATION_KIND as OtherSITUATION_KIND;
use smash2::app::work_ids::fighter::pickel::instance::SITUATION_KIND as OtherOtherSITUATION_KIND;
use smash2::app::work_ids::fighter::rosetta::status::special_hi_common::SITUATION_KIND as OtherOtherOtherSITUATION_KIND;

/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
/*SHIELD STATUSES*/
//Sub Guard Cont Pre
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont_pre)]
unsafe fn status_sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    }
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
    ];
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

//Sub Guard Cont
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
unsafe fn status_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont_stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = cont_stick_x * lr;
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let turn_run_stick_x_threshold = stick_x * lr <= turn_run_stick_x;
    let check_guard_hold = fighter.check_guard_hold().get_bool();
    let is_shield_stop = fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_STATUS_KIND_GUARD_ON && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN;
    if fighter.global_table[GUARD_CONT_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return true.into();
    }
    if !check_guard_hold {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
        /* START OF NEW ADDITION */
        //Allows platform drops out of shield
        if GroundModule::is_passable_ground(fighter.module_accessor)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
        && (fighter.global_table[CMD_CAT2].get_i32() & (*FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
        /* END OF NEW ADDITION */
    }
    if is_shield_stop {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
        && turn_run_stick_x_threshold
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
    }
    if !fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                return true.into();
            }
        }
        if !check_guard_hold {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return true.into();
            }
        }
        false.into()
    }
    else {
        true.into()
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_sub_guard_cont_pre,
            status_sub_guard_cont
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}