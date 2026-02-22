use anyhow::{bail, Context};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

const MIGRATIONS: &[(i64, &str)] = &[
    (
        1,
        "
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            company TEXT NOT NULL,
            role TEXT NOT NULL,
            source TEXT NOT NULL,
            baseline TEXT NOT NULL,
            jd_text TEXT NOT NULL,
            jd_hash TEXT NOT NULL,
            track TEXT,
            fit_total INTEGER,
            status TEXT NOT NULL DEFAULT 'new',
            next_action TEXT,
            notes TEXT,
            output_dir TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS job_events (
            id TEXT PRIMARY KEY,
            job_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        ",
    ),
    (
        2,
        "
        CREATE INDEX IF NOT EXISTS idx_jobs_updated_at ON jobs(updated_at DESC);
        CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
        ",
    ),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRecord {
    pub id: String,
    pub company: String,
    pub role: String,
    pub source: String,
    pub baseline: String,
    pub track: Option<String>,
    pub fit_total: Option<i64>,
    pub status: String,
    pub next_action: Option<String>,
    pub notes: Option<String>,
    pub output_dir: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpsertJobRecordInput<'a> {
    pub id: &'a str,
    pub company: &'a str,
    pub role: &'a str,
    pub source: &'a str,
    pub baseline: &'a str,
    pub jd_text: &'a str,
    pub jd_hash: &'a str,
    pub track: Option<&'a str>,
    pub fit_total: Option<i64>,
    pub output_dir: Option<&'a str>,
}

pub fn init_db(db_path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let conn =
        Connection::open(db_path).with_context(|| format!("opening {}", db_path.display()))?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );
        ",
    )
    .context("creating schema_migrations table")?;

    for (version, sql) in MIGRATIONS {
        let exists = conn.query_row(
            "SELECT COUNT(*) FROM schema_migrations WHERE version = ?1",
            params![version],
            |row| row.get::<_, i64>(0),
        )?;
        if exists > 0 {
            continue;
        }

        let tx = conn.unchecked_transaction().context("starting migration transaction")?;
        tx.execute_batch(sql).with_context(|| format!("applying migration {version}"))?;
        tx.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            params![version, Utc::now().to_rfc3339()],
        )
        .with_context(|| format!("recording migration {version}"))?;
        tx.commit().context("committing migration")?;
    }

    Ok(())
}

pub fn upsert_job_record(db_path: &Path, input: UpsertJobRecordInput<'_>) -> anyhow::Result<()> {
    init_db(db_path)?;
    let conn =
        Connection::open(db_path).with_context(|| format!("opening {}", db_path.display()))?;
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "
        INSERT INTO jobs (id, company, role, source, baseline, jd_text, jd_hash, track, fit_total, status, output_dir, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'new', ?10, ?11, ?11)
        ON CONFLICT(id) DO UPDATE SET
            company=excluded.company,
            role=excluded.role,
            source=excluded.source,
            baseline=excluded.baseline,
            jd_text=excluded.jd_text,
            jd_hash=excluded.jd_hash,
            track=excluded.track,
            fit_total=excluded.fit_total,
            output_dir=excluded.output_dir,
            updated_at=excluded.updated_at
        ",
        params![
            input.id,
            input.company,
            input.role,
            input.source,
            input.baseline,
            input.jd_text,
            input.jd_hash,
            input.track,
            input.fit_total,
            input.output_dir,
            now
        ],
    )
    .context("upserting jobs row")?;

    Ok(())
}

pub fn list_jobs(db_path: &Path) -> anyhow::Result<Vec<JobRecord>> {
    if !db_path.exists() {
        return Ok(Vec::new());
    }
    let conn =
        Connection::open(db_path).with_context(|| format!("opening {}", db_path.display()))?;
    let mut stmt = conn.prepare(
        "
        SELECT id, company, role, source, baseline, track, fit_total, status, next_action, notes, output_dir, created_at, updated_at
        FROM jobs
        ORDER BY updated_at DESC, id ASC
        ",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(JobRecord {
            id: row.get(0)?,
            company: row.get(1)?,
            role: row.get(2)?,
            source: row.get(3)?,
            baseline: row.get(4)?,
            track: row.get(5)?,
            fit_total: row.get(6)?,
            status: row.get(7)?,
            next_action: row.get(8)?,
            notes: row.get(9)?,
            output_dir: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn get_job_by_id(db_path: &Path, id: &str) -> anyhow::Result<Option<JobRecord>> {
    if !db_path.exists() {
        return Ok(None);
    }
    let conn =
        Connection::open(db_path).with_context(|| format!("opening {}", db_path.display()))?;
    let mut stmt = conn.prepare(
        "
        SELECT id, company, role, source, baseline, track, fit_total, status, next_action, notes, output_dir, created_at, updated_at
        FROM jobs
        WHERE id = ?1
        ",
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        return Ok(Some(JobRecord {
            id: row.get(0)?,
            company: row.get(1)?,
            role: row.get(2)?,
            source: row.get(3)?,
            baseline: row.get(4)?,
            track: row.get(5)?,
            fit_total: row.get(6)?,
            status: row.get(7)?,
            next_action: row.get(8)?,
            notes: row.get(9)?,
            output_dir: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        }));
    }
    Ok(None)
}

pub fn update_job_status(
    db_path: &Path,
    id: &str,
    status: &str,
    next_action: Option<&str>,
    notes: Option<&str>,
) -> anyhow::Result<()> {
    let status_lower = status.to_ascii_lowercase();
    let allowed = ["new", "applied", "reply", "interview", "closed"];
    if !allowed.iter().any(|s| *s == status_lower) {
        bail!("invalid status: {status}");
    }

    init_db(db_path)?;
    let conn =
        Connection::open(db_path).with_context(|| format!("opening {}", db_path.display()))?;
    let now = Utc::now().to_rfc3339();
    let updated = conn
        .execute(
            "
        UPDATE jobs
        SET status = ?2,
            next_action = ?3,
            notes = ?4,
            updated_at = ?5
        WHERE id = ?1
        ",
            params![id, status_lower, next_action, notes, now],
        )
        .context("updating job status")?;
    if updated == 0 {
        bail!("job not found: {id}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_apply_and_version_rows_exist() {
        let dir = tempfile::tempdir().expect("temp");
        let db_path = dir.path().join("applykit.db");
        init_db(&db_path).expect("init");
        init_db(&db_path).expect("re-init");

        let conn = Connection::open(&db_path).expect("open");
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| row.get(0))
            .expect("count");
        assert!(count >= 2);
    }

    #[test]
    fn upsert_and_update_status_round_trip() {
        let dir = tempfile::tempdir().expect("temp");
        let db_path = dir.path().join("applykit.db");
        upsert_job_record(
            &db_path,
            UpsertJobRecordInput {
                id: "job-1",
                company: "Acme",
                role: "Role",
                source: "manual",
                baseline: "1pg",
                jd_text: "jd",
                jd_hash: "hash",
                track: Some("Support/Ops Core"),
                fit_total: Some(60),
                output_dir: Some("/tmp/packet"),
            },
        )
        .expect("upsert");

        update_job_status(&db_path, "job-1", "reply", Some("send follow-up"), Some("note"))
            .expect("update");

        let jobs = list_jobs(&db_path).expect("list");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].status, "reply");
        assert_eq!(jobs[0].next_action.as_deref(), Some("send follow-up"));
        assert_eq!(jobs[0].notes.as_deref(), Some("note"));
    }

    #[test]
    fn update_status_fails_for_unknown_job() {
        let dir = tempfile::tempdir().expect("temp");
        let db_path = dir.path().join("applykit.db");
        upsert_job_record(
            &db_path,
            UpsertJobRecordInput {
                id: "job-1",
                company: "Acme",
                role: "Role",
                source: "manual",
                baseline: "1pg",
                jd_text: "jd",
                jd_hash: "hash",
                track: Some("Support/Ops Core"),
                fit_total: Some(60),
                output_dir: Some("/tmp/packet"),
            },
        )
        .expect("upsert");

        let err = update_job_status(&db_path, "missing", "reply", None, None).expect_err("missing");
        assert!(err.to_string().contains("job not found"));
    }

    #[test]
    fn update_status_rejects_invalid_value() {
        let dir = tempfile::tempdir().expect("temp");
        let db_path = dir.path().join("applykit.db");
        upsert_job_record(
            &db_path,
            UpsertJobRecordInput {
                id: "job-1",
                company: "Acme",
                role: "Role",
                source: "manual",
                baseline: "1pg",
                jd_text: "jd",
                jd_hash: "hash",
                track: Some("Support/Ops Core"),
                fit_total: Some(60),
                output_dir: Some("/tmp/packet"),
            },
        )
        .expect("upsert");

        let err = update_job_status(&db_path, "job-1", "foo", None, None).expect_err("invalid");
        assert!(err.to_string().contains("invalid status"));
    }
}
