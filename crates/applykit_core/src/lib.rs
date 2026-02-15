pub mod banks;
pub mod classify;
pub mod config;
pub mod determinism;
pub mod diff;
pub mod insights;
pub mod jd;
pub mod messages;
pub mod packet;
pub mod pipeline;
pub mod resume;
pub mod score;
pub mod source_preview;
pub mod storage;
pub mod truth_gate;
pub mod types;

pub use pipeline::{
    generate_packet, read_packet_detail, read_packet_detail_by_job_id, GenerateOptions,
    GenerateResult,
};
pub use source_preview::{
    create_bullet_value, create_skill_value, load_banks_preview, load_templates_preview,
    save_bullet_text_value, save_template_value, set_bullet_approved_value,
    set_skill_approved_value, set_skill_level_value, BanksPreview, CreateBulletInput,
    MutationResponse, TemplateKey, TemplatesPreview,
};
pub use storage::{
    get_job_by_id, init_db, list_jobs, update_job_status, upsert_job_record, JobRecord,
};
pub use types::{Baseline, Track};

#[cfg(test)]
mod tests;
