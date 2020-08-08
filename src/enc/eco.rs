use super::bsw::*;
use super::sbac::*;
use super::util::*;
use crate::api::*;
use crate::def::*;
use crate::tbl::*;
use crate::util::*;

pub(crate) fn evce_eco_nalu(bs: &mut EvceBsw, nalu: &EvcNalu) {
    bs.write(nalu.nal_unit_size, 32, None);
    bs.write(nalu.forbidden_zero_bit as u32, 1, None);
    bs.write(nalu.nal_unit_type as u32 + 1, 6, None);
    bs.write(nalu.nuh_temporal_id as u32, 3, None);
    bs.write(nalu.nuh_reserved_zero_5bits as u32, 5, None);
    bs.write(nalu.nuh_extension_flag as u32, 1, None);
}

pub(crate) fn evce_eco_tile_end_flag(bs: &mut EvceBsw, sbac: &mut EvceSbac, flag: u32) {
    sbac.encode_bin_trm(bs, flag);
}

pub(crate) fn evce_eco_split_mode(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    cud: u16,
    cup: u16,
    cuw: u16,
    cuh: u16,
    lcu_s: u16,
    split_mode_buf: &LcuSplitMode,
) {
    let mut split_mode = SplitMode::NO_SPLIT;
    let mut ctx = 0;
    let order_idx = if cuw >= cuh { 0 } else { 1 };

    let mut split_allow = vec![false; MAX_SPLIT_NUM];

    if cuw < 8 && cuh < 8 {
        return;
    }

    //evc_assert(evce_check_luma(c, core));
    split_mode = evc_get_split_mode(cud, cup, cuw, cuh, lcu_s, split_mode_buf);

    sbac.encode_bin(
        bs,
        &mut sbac_ctx.split_cu_flag[0],
        if split_mode != SplitMode::NO_SPLIT {
            1
        } else {
            0
        },
    ); /* split_cu_flag */

    /*EVC_TRACE_COUNTER;
    EVC_TRACE_STR("x pos ");
    EVC_TRACE_INT(core->x_pel + ((cup % (c->max_cuwh >> MIN_CU_LOG2)) << MIN_CU_LOG2));
    EVC_TRACE_STR("y pos ");
    EVC_TRACE_INT(core->y_pel + ((cup / (c->max_cuwh >> MIN_CU_LOG2)) << MIN_CU_LOG2));
    EVC_TRACE_STR("width ");
    EVC_TRACE_INT(cuw);
    EVC_TRACE_STR("height ");
    EVC_TRACE_INT(cuh);
    EVC_TRACE_STR("depth ");
    EVC_TRACE_INT(cud);
    EVC_TRACE_STR("split mode ");
    EVC_TRACE_INT(split_mode);
    EVC_TRACE_STR("\n");*/
}

pub(crate) fn evce_eco_intra_dir_b(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    ipm: u8,
    mpm: &[u8],
) {
    sbac.write_unary_sym(bs, &mut sbac_ctx.intra_dir, mpm[ipm as usize] as u32, 2);
    /* EVC_TRACE_COUNTER;
    #if TRACE_ADDITIONAL_FLAGS
        EVC_TRACE_STR("mpm list: ");
        for (int i = 0; i < IPD_CNT_B; i++)
        {
            EVC_TRACE_INT(mpm[i]);
        }
    #endif
        EVC_TRACE_STR("ipm Y ");
        EVC_TRACE_INT(ipm);
        EVC_TRACE_STR("\n");*/
}

pub(crate) fn evce_eco_pred_mode(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    pred_mode: PredMode,
    ctx: usize,
) {
    sbac.encode_bin(
        bs,
        &mut sbac_ctx.pred_mode[ctx],
        if pred_mode == PredMode::MODE_INTRA {
            1
        } else {
            0
        },
    );
    /*EVC_TRACE_COUNTER;
    EVC_TRACE_STR("pred mode ");
    EVC_TRACE_INT(pred_mode == MODE_INTRA ? MODE_INTRA : MODE_INTER);
    EVC_TRACE_STR("\n");

    return EVC_OK;*/
}

pub(crate) fn evce_eco_cbf(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    cbf_y: u16,
    cbf_u: u16,
    cbf_v: u16,
    pred_mode: PredMode,
    b_no_cbf: bool,
    cbf_all: u16,
    run: &[bool],
    tree_cons: &TREE_CONS,
) {
    /* code allcbf */
    if pred_mode != PredMode::MODE_INTRA && !evc_check_only_intra(tree_cons) {
        if b_no_cbf {
            assert!(cbf_all != 0);
        } else if (run[Y_C] as u8 + run[U_C] as u8 + run[V_C] as u8) == 3 {
            // not count bits of root_cbf when checking each component

            if cbf_all == 0 {
                sbac.encode_bin(bs, &mut sbac_ctx.cbf_all[0], 0);

                //EVC_TRACE_COUNTER;
                //EVC_TRACE_STR("all_cbf ");
                //EVC_TRACE_INT(0);
                //EVC_TRACE_STR("\n");

                return;
            } else {
                sbac.encode_bin(bs, &mut sbac_ctx.cbf_all[0], 1);

                //EVC_TRACE_COUNTER;
                //EVC_TRACE_STR("all_cbf ");
                //EVC_TRACE_INT(1);
                //EVC_TRACE_STR("\n");
            }
        }

        if run[U_C] {
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_cb[0], cbf_u as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf U ");
            //EVC_TRACE_INT(cbf_u);
            //EVC_TRACE_STR("\n");
        }
        if run[V_C] {
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_cr[0], cbf_v as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf V ");
            //EVC_TRACE_INT(cbf_v);
            //EVC_TRACE_STR("\n");
        }

        if run[Y_C] && (cbf_u + cbf_v) != 0 {
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_luma[0], cbf_y as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf Y ");
            //EVC_TRACE_INT(cbf_y);
            //EVC_TRACE_STR("\n");
        }
    } else {
        if run[U_C] {
            assert!(evc_check_chroma(tree_cons));
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_cb[0], cbf_u as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf U ");
            //EVC_TRACE_INT(cbf_u);
            //EVC_TRACE_STR("\n");
        }
        if run[V_C] {
            assert!(evc_check_chroma(tree_cons));
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_cr[0], cbf_v as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf V ");
            //EVC_TRACE_INT(cbf_v);
            //EVC_TRACE_STR("\n");
        }
        if run[Y_C] {
            assert!(evc_check_luma(tree_cons));
            sbac.encode_bin(bs, &mut sbac_ctx.cbf_luma[0], cbf_y as u32);
            //EVC_TRACE_COUNTER;
            //EVC_TRACE_STR("cbf Y ");
            //EVC_TRACE_INT(cbf_y);
            //EVC_TRACE_STR("\n");
        }
    }
}

pub(crate) fn evce_eco_dqp(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    ref_qp: u8,
    cur_qp: u8,
) {
    let dqp = cur_qp as i8 - ref_qp as i8;
    let abs_dqp = dqp.abs() as u8;

    sbac.write_unary_sym(
        bs,
        &mut sbac_ctx.delta_qp,
        abs_dqp as u32,
        NUM_CTX_DELTA_QP as u32,
    );

    if abs_dqp > 0 {
        let sign = if dqp > 0 { 0 } else { 1 };
        sbac.encode_bin_ep(bs, sign);
    }

    //EVC_TRACE_COUNTER;
    //EVC_TRACE_STR("dqp ");
    //EVC_TRACE_INT(dqp);
    //EVC_TRACE_STR("\n");
}

pub(crate) fn evce_eco_run_length_cc(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    coef: &[i16],
    log2_w: u8,
    log2_h: u8,
    mut num_sig: u16,
    ch_type: usize,
) {
    let mut ctx_last = 0;
    let scanp = &evc_scan_tbl[log2_w as usize - 1];
    let num_coeff = (1 << (log2_w + log2_h)) as usize;
    let mut run = 0;
    let mut prev_level = 6;

    for scan_pos in 0..num_coeff {
        let coef_cur = coef[scanp[scan_pos] as usize];
        if coef_cur != 0 {
            let level = coef_cur.abs() as u32;
            let sign = if coef_cur > 0 { 0 } else { 1 };
            let t0 = if ch_type == Y_C { 0 } else { 2 };

            /* Run coding */
            sbac.write_unary_sym(bs, &mut sbac_ctx.run[t0..], run, 2);

            /* Level coding */
            sbac.write_unary_sym(bs, &mut sbac_ctx.level[t0..], level - 1, 2);

            /* Sign coding */
            sbac.encode_bin_ep(bs, sign);

            if scan_pos == num_coeff - 1 {
                break;
            }

            run = 0;
            prev_level = level;
            num_sig -= 1;

            /* Last flag coding */
            let last_flag = num_sig == 0;
            ctx_last = if ch_type == Y_C { 0 } else { 1 };
            sbac.encode_bin(bs, &mut sbac_ctx.last[ctx_last], last_flag as u32);

            if last_flag {
                break;
            }
        } else {
            run += 1;
        }
    }

    /*#if ENC_DEC_TRACE
        EVC_TRACE_STR("coef luma ");
        for (scan_pos = 0; scan_pos < num_coeff; scan_pos++)
        {
            EVC_TRACE_INT(coef[scan_pos]);
        }
        EVC_TRACE_STR("\n");
    #endif*/
}

pub(crate) fn evce_eco_xcoef(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    coef: &[i16],
    log2_w: u8,
    log2_h: u8,
    num_sig: u16,
    ch_type: usize,
) {
    evce_eco_run_length_cc(bs, sbac, sbac_ctx, coef, log2_w, log2_h, num_sig, ch_type);

    /*#if TRACE_COEFFS
        int cuw = 1 << log2_w;
        int cuh = 1 << log2_h;
        EVC_TRACE_COUNTER;
        EVC_TRACE_STR("Coef for ");
        EVC_TRACE_INT(ch_type);
        EVC_TRACE_STR(": ");
        for (int i = 0; i < (cuw * cuh); ++i)
        {
            if (i != 0)
                EVC_TRACE_STR(", ");
            EVC_TRACE_INT(coef[i]);
        }
        EVC_TRACE_STR("\n");
    #endif*/
}

pub(crate) fn coef_rect_to_series(
    coef_dst: &mut CUBuffer<i16>,
    coef_src: &Vec<Vec<i16>>,
    log2_max_cuwh: u8,
    mut x: u16,
    mut y: u16,
    mut cuw: u16,
    mut cuh: u16,
    tree_cons: &TREE_CONS,
) {
    let max_cuwh = 1u16 << log2_max_cuwh;

    let mut sidx = ((x & (max_cuwh - 1)) + ((y & (max_cuwh - 1)) << log2_max_cuwh)) as usize;
    let mut didx = 0;

    if evc_check_luma(tree_cons) {
        for j in 0..cuh as usize {
            for i in 0..cuw as usize {
                coef_dst.data[Y_C][didx] = coef_src[Y_C][sidx + i];
                didx += 1;
            }
            sidx += max_cuwh as usize;
        }
    }

    if evc_check_chroma(tree_cons) {
        x >>= 1;
        y >>= 1;
        cuw >>= 1;
        cuh >>= 1;

        sidx = ((x & ((max_cuwh >> 1) - 1)) + ((y & ((max_cuwh >> 1) - 1)) << (log2_max_cuwh - 1)))
            as usize;
        didx = 0;

        for j in 0..cuh as usize {
            for i in 0..cuw as usize {
                coef_dst.data[U_C][didx] = coef_src[U_C][sidx + i];
                coef_dst.data[V_C][didx] = coef_src[V_C][sidx + i];
                didx += 1;
            }
            sidx += (max_cuwh >> 1) as usize;
        }
    }
}

pub(crate) fn evce_eco_skip_flag(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    flag: u32,
    ctx: usize,
) {
    sbac.encode_bin(bs, &mut sbac_ctx.skip_flag[ctx], flag);

    /*    EVC_TRACE_COUNTER;
    EVC_TRACE_STR("skip flag ");
    EVC_TRACE_INT(flag);
    EVC_TRACE_STR("ctx ");
    EVC_TRACE_INT(ctx);
    EVC_TRACE_STR("\n");*/
}

pub(crate) fn evce_eco_mvp_idx(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    mvp_idx: u32,
) {
    sbac.write_truncate_unary_sym(bs, &mut sbac_ctx.mvp_idx, mvp_idx, 3, 4);

    /*EVC_TRACE_COUNTER;
    EVC_TRACE_STR("mvp idx ");
    EVC_TRACE_INT(mvp_idx);
    EVC_TRACE_STR("\n");*/
}

pub(crate) fn evce_eco_direct_mode_flag(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    direct_mode_flag: u32,
) {
    sbac.encode_bin(bs, &mut sbac_ctx.direct_mode_flag[0], direct_mode_flag);

    /* EVC_TRACE_COUNTER;
    EVC_TRACE_STR("direct_mode_flag ");
    EVC_TRACE_INT(direct_mode_flag ? PRED_DIR : 0);
    EVC_TRACE_STR("\n");*/
}

pub(crate) fn evce_eco_inter_pred_idc(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    refi: &[i8],
    slice_type: SliceType,
) {
    if REFI_IS_VALID(refi[REFP_0]) && REFI_IS_VALID(refi[REFP_1]) {
        /* PRED_BI */
        assert!(check_bi_applicability(slice_type));
        sbac.encode_bin(bs, &mut sbac_ctx.inter_dir[0], 0);
    } else {
        if check_bi_applicability(slice_type) {
            sbac.encode_bin(bs, &mut sbac_ctx.inter_dir[0], 1);
        }

        if REFI_IS_VALID(refi[REFP_0]) {
            /* PRED_L0 */
            sbac.encode_bin(bs, &mut sbac_ctx.inter_dir[1], 0);
        } else
        /* PRED_L1 */
        {
            sbac.encode_bin(bs, &mut sbac_ctx.inter_dir[1], 1);
        }
    }
    /*
    #if ENC_DEC_TRACE
        EVC_TRACE_COUNTER;
        EVC_TRACE_STR("inter dir ");
        EVC_TRACE_INT(REFI_IS_VALID(refi[REFP_0]) && REFI_IS_VALID(refi[REFP_1])? PRED_BI : (REFI_IS_VALID(refi[REFP_0]) ? PRED_L0 : PRED_L1));
        EVC_TRACE_STR("\n");
    #endif*/
}

pub(crate) fn evce_eco_refi(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    num_refp: u8,
    refi: i8,
) {
    if num_refp > 1 {
        if refi == 0 {
            sbac.encode_bin(bs, &mut sbac_ctx.refi[0], 0);
        } else {
            sbac.encode_bin(bs, &mut sbac_ctx.refi[0], 1);
            if num_refp > 2 {
                for i in 2..num_refp {
                    let bin = if i as i8 == (refi + 1) { 0 } else { 1 };
                    if i == 2 {
                        sbac.encode_bin(bs, &mut sbac_ctx.refi[1], bin);
                    } else {
                        sbac.encode_bin_ep(bs, bin);
                    }
                    if bin == 0 {
                        break;
                    }
                }
            }
        }
    }
}

fn evce_eco_abs_mvd(bs: &mut EvceBsw, sbac: &mut EvceSbac, model: &mut SBAC_CTX_MODEL, sym: u32) {
    let val = sym;

    let mut nn = ((val + 1) >> 1);
    let mut len_i = 0;
    while len_i < 16 && nn != 0 {
        nn >>= 1;
        len_i += 1;
    }

    let info = val + 1 - (1 << len_i);
    let code = (1 << len_i) | ((info) & ((1 << len_i) - 1));

    let len_c = (len_i << 1) + 1;

    for i in 0..len_c {
        let bin = (code >> (len_c - 1 - i)) & 0x01;
        if i <= 1 {
            sbac.encode_bin(bs, model, bin); /* use one context model for two bins */
        } else {
            sbac.encode_bin_ep(bs, bin);
        }
    }
}

pub(crate) fn evce_eco_mvd(
    bs: &mut EvceBsw,
    sbac: &mut EvceSbac,
    sbac_ctx: &mut EvcSbacCtx,
    mvd: &[i16],
) {
    let mut t0 = 0;

    let mut mv = mvd[MV_X];
    if mvd[MV_X] < 0 {
        t0 = 1;
        mv = -mvd[MV_X];
    }
    evce_eco_abs_mvd(bs, sbac, &mut sbac_ctx.mvd[0], mv as u32);

    if mv != 0 {
        sbac.encode_bin_ep(bs, t0);
    }

    t0 = 0;
    mv = mvd[MV_Y];
    if mvd[MV_Y] < 0 {
        t0 = 1;
        mv = -mvd[MV_Y];
    }

    evce_eco_abs_mvd(bs, sbac, &mut sbac_ctx.mvd[0], mv as u32);

    if mv != 0 {
        sbac.encode_bin_ep(bs, t0);
    }

    /*EVC_TRACE_COUNTER;
    EVC_TRACE_STR("mvd x ");
    EVC_TRACE_INT(mvd[MV_X]);
    EVC_TRACE_STR("mvd y ");
    EVC_TRACE_INT(mvd[MV_Y]);
    EVC_TRACE_STR("\n");*/
}