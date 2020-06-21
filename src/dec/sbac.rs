use super::bsr::*;

use crate::api::*;
use crate::com::*;

/*****************************************************************************
 * SBAC structure
 *****************************************************************************/
#[derive(Default)]
pub(crate) struct EvcdSbac {
    pub(crate) range: u32,
    pub(crate) value: u32,
    pub(crate) ctx: EvcSbacCtx,
}

impl EvcdSbac {
    pub(crate) fn reset(&mut self, bs: &mut EvcdBsr, slice_type: SliceType, slice_qp: u8) {
        /* Initialization of the internal variables */
        self.range = 16384;
        self.value = 0;
        for i in 0..14 {
            let t0 = bs.read1(Some("t0"));
            self.value = ((self.value << 1) | t0) & 0xFFFF;
        }

        let sbac_ctx = &mut self.ctx;

        /* Initialization of the context models */
        for i in 0..NUM_CTX_ALF_CTB_FLAG {
            sbac_ctx.alf_ctb_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_SPLIT_CU_FLAG {
            sbac_ctx.split_cu_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CC_RUN {
            sbac_ctx.run[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CC_LAST {
            sbac_ctx.last[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CC_LEVEL {
            sbac_ctx.level[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CBF_LUMA {
            sbac_ctx.cbf_luma[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CBF_CB {
            sbac_ctx.cbf_cb[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CBF_CR {
            sbac_ctx.cbf_cr[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_CBF_ALL {
            sbac_ctx.cbf_all[i] = PROB_INIT;
        }
        //for i in 0..NUM_CTX_SIG_COEFF_FLAG {
        //    sbac_ctx.sig_coeff_flag[i] = PROB_INIT;
        //}
        for i in 0..NUM_CTX_GTX {
            sbac_ctx.coeff_abs_level_greaterAB_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_LAST_SIG_COEFF {
            sbac_ctx.last_sig_coeff_x_prefix[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_LAST_SIG_COEFF {
            sbac_ctx.last_sig_coeff_y_prefix[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_PRED_MODE {
            sbac_ctx.pred_mode[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MODE_CONS {
            sbac_ctx.mode_cons[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_DIRECT_MODE_FLAG {
            sbac_ctx.direct_mode_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MERGE_MODE_FLAG {
            sbac_ctx.merge_mode_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_INTER_PRED_IDC {
            sbac_ctx.inter_dir[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_INTRA_PRED_MODE {
            sbac_ctx.intra_dir[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_INTRA_LUMA_PRED_MPM_FLAG {
            sbac_ctx.intra_luma_pred_mpm_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_INTRA_LUMA_PRED_MPM_IDX {
            sbac_ctx.intra_luma_pred_mpm_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_INTRA_CHROMA_PRED_MODE {
            sbac_ctx.intra_chroma_pred_mode[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MMVD_FLAG {
            sbac_ctx.mmvd_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MMVD_MERGE_IDX {
            sbac_ctx.mmvd_merge_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MMVD_DIST_IDX {
            sbac_ctx.mmvd_distance_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MMVD_DIRECTION_IDX {
            sbac_ctx.mmvd_direction_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MMVD_GROUP_IDX {
            sbac_ctx.mmvd_group_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MERGE_IDX {
            sbac_ctx.merge_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MVP_IDX {
            sbac_ctx.mvp_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AMVR_IDX {
            sbac_ctx.mvr_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_BI_PRED_IDX {
            sbac_ctx.bi_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_MVD {
            sbac_ctx.mvd[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_REF_IDX {
            sbac_ctx.refi[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_BTT_SPLIT_FLAG {
            sbac_ctx.btt_split_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_BTT_SPLIT_DIR {
            sbac_ctx.btt_split_dir[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_BTT_SPLIT_TYPE {
            sbac_ctx.btt_split_type[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_SUCO_FLAG {
            sbac_ctx.suco_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_DELTA_QP {
            sbac_ctx.delta_qp[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AFFINE_FLAG {
            sbac_ctx.affine_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AFFINE_MODE {
            sbac_ctx.affine_mode[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AFFINE_MRG {
            sbac_ctx.affine_mrg[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AFFINE_MVP_IDX {
            sbac_ctx.affine_mvp_idx[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_AFFINE_MVD_FLAG {
            sbac_ctx.affine_mvd_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_SKIP_FLAG {
            sbac_ctx.skip_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_IBC_FLAG {
            sbac_ctx.ibc_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_ATS_MODE_FLAG {
            sbac_ctx.ats_mode[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_ATS_INTER_FLAG {
            sbac_ctx.ats_cu_inter_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_ATS_INTER_QUAD_FLAG {
            sbac_ctx.ats_cu_inter_quad_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_ATS_INTER_HOR_FLAG {
            sbac_ctx.ats_cu_inter_hor_flag[i] = PROB_INIT;
        }
        for i in 0..NUM_CTX_ATS_INTER_POS_FLAG {
            sbac_ctx.ats_cu_inter_pos_flag[i] = PROB_INIT;
        }
    }

    pub(crate) fn evcd_sbac_decode_bin(
        &mut self,
        bs: &mut EvcdBsr,
        model: &mut SBAC_CTX_MODEL,
    ) -> u32 {
        let mut state: u16 = (*model) >> 1;
        let mut mps: u16 = (*model) & 1;

        let mut lps: u32 = (state as u32 * self.range) >> 9;
        lps = if lps < 437 { 437 } else { lps };

        let mut bin: u32 = mps as u32;

        self.range -= lps;

        //#if TRACE_BIN
        EVC_TRACE_COUNTER(&mut bs.tracer);
        EVC_TRACE_STR(&mut bs.tracer, "model ");
        EVC_TRACE_INT(&mut bs.tracer, *model as isize);
        EVC_TRACE_STR(&mut bs.tracer, "range ");
        EVC_TRACE_INT(&mut bs.tracer, self.range as isize);
        EVC_TRACE_STR(&mut bs.tracer, "lps ");
        EVC_TRACE_INT(&mut bs.tracer, lps as isize);
        EVC_TRACE_STR(&mut bs.tracer, "\n");
        //#endif

        if self.value >= self.range {
            bin = 1 - mps as u32;
            self.value -= self.range;
            self.range = lps;

            state = state + ((512 - state + 16) >> 5);
            if state > 256 {
                mps = 1 - mps;
                state = 512 - state;
            }
            *model = (state << 1) + mps;
        } else {
            bin = mps as u32;
            state = state - ((state + 16) >> 5);
            *model = (state << 1) + mps;
        }

        while self.range < 8192 {
            self.range <<= 1;

            let t0 = bs.read1(Some("t0"));
            self.value = ((self.value << 1) | t0) & 0xFFFF;
        }

        bin
    }
    pub(crate) fn decode_bin_trm(&mut self, bs: &mut EvcdBsr) -> u32 {
        self.range -= 1;
        if self.value >= self.range {
            while !bs.EVC_BSR_IS_BYTE_ALIGN() {
                let t0 = bs.read1(Some("t0"));
                assert_eq!(t0, 0);
            }
            1
        } else {
            while self.range < 8192 {
                self.range <<= 1;
                let t0 = bs.read1(Some("t0"));
                self.value = ((self.value << 1) | t0) & 0xFFFF;
            }
            0
        }
    }
}
