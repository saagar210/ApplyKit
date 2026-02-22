import { useCallback, useEffect, useMemo, useState } from "react";
import { invokeSafe } from "../lib/tauri";
import type { MutationResponse } from "../lib/types";

interface Props {
  onNotify: (message: string, tone: "info" | "success" | "error") => void;
}

interface BanksPreviewResponse {
  bulletBankJson: string;
  skillsBankJson: string;
  bulletCount: number;
  approvedBulletCount: number;
  skillCount: number;
  approvedSkillCount: number;
}

interface BulletRow {
  id: string;
  scope: string;
  approved: boolean;
  claimLevel: string;
  text: string;
}

interface SkillRow {
  name: string;
  approved: boolean;
  level: string;
}

const allowedLevels = ["admin", "operator", "familiar"];

export function Banks({ onNotify }: Props) {
  const [onlyApproved, setOnlyApproved] = useState(true);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [preview, setPreview] = useState<BanksPreviewResponse | null>(null);
  const [editingBulletId, setEditingBulletId] = useState<string | null>(null);
  const [editingBulletText, setEditingBulletText] = useState("");
  const [newSkillName, setNewSkillName] = useState("");
  const [newSkillLevel, setNewSkillLevel] = useState("operator");
  const [newSkillApproved, setNewSkillApproved] = useState(false);
  const [newBulletId, setNewBulletId] = useState("");
  const [newBulletScope, setNewBulletScope] = useState("Custom");
  const [newBulletClaimLevel, setNewBulletClaimLevel] = useState("supported");
  const [newBulletSeniority, setNewBulletSeniority] = useState("mid");
  const [newBulletText, setNewBulletText] = useState("");
  const [newBulletCategory, setNewBulletCategory] = useState("");
  const [newBulletTags, setNewBulletTags] = useState("");
  const [newBulletTools, setNewBulletTools] = useState("");
  const [newBulletApproved, setNewBulletApproved] = useState(false);

  const load = useCallback(async () => {
    const response = await invokeSafe<BanksPreviewResponse>("get_banks_preview_cmd", {});
    setPreview(response);
    setError(null);
  }, []);

  useEffect(() => {
    let mounted = true;
    void (async () => {
      try {
        await load();
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err.message : "Failed to load bank previews");
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    })();
    return () => {
      mounted = false;
    };
  }, [load]);

  const bullets = useMemo(() => {
    if (!preview) {
      return [];
    }
    try {
      const parsed = JSON.parse(preview.bulletBankJson) as {
        bullets: Array<{
          id: string;
          scope: string;
          approved: boolean;
          claim_level: string;
          text: string;
        }>;
      };
      const rows: BulletRow[] = parsed.bullets
        .map((row) => ({
          id: row.id,
          scope: row.scope,
          approved: row.approved,
          claimLevel: row.claim_level,
          text: row.text
        }))
        .sort((a, b) => a.id.localeCompare(b.id));
      return rows.filter((row) => (onlyApproved ? row.approved : true));
    } catch {
      return [];
    }
  }, [preview, onlyApproved]);

  const skills = useMemo(() => {
    if (!preview) {
      return [];
    }
    try {
      const parsed = JSON.parse(preview.skillsBankJson) as {
        skills: Record<string, { approved: boolean; level: string }>;
      };
      const rows: SkillRow[] = Object.entries(parsed.skills).map(([name, meta]) => ({
        name,
        approved: meta.approved,
        level: meta.level
      }));
      rows.sort((a, b) => a.name.localeCompare(b.name));
      return rows.filter((row) => (onlyApproved ? row.approved : true));
    } catch {
      return [];
    }
  }, [preview, onlyApproved]);

  const runMutation = useCallback(
    async (command: string, input: object) => {
      setSaving(true);
      try {
        const response = await invokeSafe<MutationResponse>(command, { input });
        onNotify(response.message, response.ok ? "success" : "error");
        await load();
        return true;
      } catch (err) {
        const message = err instanceof Error ? err.message : "Failed to update banks";
        onNotify(message, "error");
        return false;
      } finally {
        setSaving(false);
      }
    },
    [load, onNotify]
  );

  const activeBullet = editingBulletId ? bullets.find((row) => row.id === editingBulletId) : undefined;

  return (
    <section className="stack-lg">
      <section className="card row between">
        <div>
          <h2>Banks Editor</h2>
          <p className="subtle">Edit approval state and levels with local-first atomic saves.</p>
        </div>
        <label className="row">
          <input
            type="checkbox"
            checked={onlyApproved}
            onChange={(e) => setOnlyApproved(e.target.checked)}
          />
          Only approved entries
        </label>
      </section>

      {loading ? <section className="card">Loading banks preview...</section> : null}
      {error ? <section className="card">{error}</section> : null}

      {preview ? (
        <section className="card row wrap">
          <span className="chip">bullets: {preview.bulletCount}</span>
          <span className="chip">approved bullets: {preview.approvedBulletCount}</span>
          <span className="chip">skills: {preview.skillCount}</span>
          <span className="chip">approved skills: {preview.approvedSkillCount}</span>
        </section>
      ) : null}

      {preview ? (
        <section className="card stack-sm">
          <h3>Add Skill</h3>
          <div className="form-grid">
            <label>
              Name
              <input
                value={newSkillName}
                onChange={(e) => setNewSkillName(e.target.value)}
                placeholder="ExampleTool"
              />
            </label>
            <label>
              Level
              <select value={newSkillLevel} onChange={(e) => setNewSkillLevel(e.target.value)}>
                {allowedLevels.map((level) => (
                  <option value={level} key={level}>
                    {level}
                  </option>
                ))}
              </select>
            </label>
            <label className="row">
              <input
                type="checkbox"
                checked={newSkillApproved}
                onChange={(e) => setNewSkillApproved(e.target.checked)}
              />
              Approved
            </label>
            <div className="row end">
              <button
                className="btn btn-primary"
                disabled={saving || !newSkillName.trim()}
                onClick={async () => {
                  const ok = await runMutation("create_skill_cmd", {
                    name: newSkillName,
                    level: newSkillLevel,
                    approved: newSkillApproved
                  });
                  if (ok) {
                    setNewSkillName("");
                    setNewSkillLevel("operator");
                    setNewSkillApproved(false);
                  }
                }}
              >
                Add Skill
              </button>
            </div>
          </div>
        </section>
      ) : null}

      {preview ? (
        <section className="card stack-sm">
          <h3>Add Bullet</h3>
          <div className="form-grid">
            <label>
              ID
              <input
                value={newBulletId}
                onChange={(e) => setNewBulletId(e.target.value)}
                placeholder="custom_bullet_001"
              />
            </label>
            <label>
              Scope
              <input value={newBulletScope} onChange={(e) => setNewBulletScope(e.target.value)} />
            </label>
            <label>
              Claim level
              <select value={newBulletClaimLevel} onChange={(e) => setNewBulletClaimLevel(e.target.value)}>
                <option value="owned">owned</option>
                <option value="led">led</option>
                <option value="partnered">partnered</option>
                <option value="supported">supported</option>
              </select>
            </label>
            <label>
              Seniority
              <input
                value={newBulletSeniority}
                onChange={(e) => setNewBulletSeniority(e.target.value)}
                placeholder="mid"
              />
            </label>
            <label className="span-2">
              Text
              <textarea
                value={newBulletText}
                rows={3}
                onChange={(e) => setNewBulletText(e.target.value)}
                placeholder="Defensible, specific bullet text"
              />
            </label>
            <label>
              Categories (comma-separated)
              <input
                value={newBulletCategory}
                onChange={(e) => setNewBulletCategory(e.target.value)}
                placeholder="ops, support"
              />
            </label>
            <label>
              Tags (comma-separated)
              <input
                value={newBulletTags}
                onChange={(e) => setNewBulletTags(e.target.value)}
                placeholder="incident, automation"
              />
            </label>
            <label className="span-2">
              Tools (comma-separated)
              <input
                value={newBulletTools}
                onChange={(e) => setNewBulletTools(e.target.value)}
                placeholder="Okta, Jira"
              />
            </label>
            <label className="row">
              <input
                type="checkbox"
                checked={newBulletApproved}
                onChange={(e) => setNewBulletApproved(e.target.checked)}
              />
              Approved
            </label>
            <div className="row end">
              <button
                className="btn btn-primary"
                disabled={saving || !newBulletId.trim() || !newBulletText.trim()}
                onClick={async () => {
                  const split = (value: string) =>
                    value
                      .split(",")
                      .map((item) => item.trim())
                      .filter((item) => item.length > 0);
                  const ok = await runMutation("create_bullet_cmd", {
                    id: newBulletId,
                    scope: newBulletScope,
                    claimLevel: newBulletClaimLevel,
                    text: newBulletText,
                    seniority: newBulletSeniority,
                    category: split(newBulletCategory),
                    tags: split(newBulletTags),
                    tools: split(newBulletTools),
                    approved: newBulletApproved
                  });
                  if (ok) {
                    setNewBulletId("");
                    setNewBulletScope("Custom");
                    setNewBulletClaimLevel("supported");
                    setNewBulletSeniority("mid");
                    setNewBulletText("");
                    setNewBulletCategory("");
                    setNewBulletTags("");
                    setNewBulletTools("");
                    setNewBulletApproved(false);
                  }
                }}
              >
                Add Bullet
              </button>
            </div>
          </div>
        </section>
      ) : null}

      {preview ? (
        <section className="card stack-sm">
          <h3>BulletBank</h3>
          <p className="subtle">Safe edits for existing rows.</p>
          <table className="table">
            <thead>
              <tr>
                <th>ID</th>
                <th>Scope</th>
                <th>Claim</th>
                <th>Approved</th>
                <th>Text</th>
                <th>Edit</th>
              </tr>
            </thead>
            <tbody>
              {bullets.slice(0, 40).map((row) => (
                <tr key={row.id}>
                  <td>{row.id}</td>
                  <td>{row.scope}</td>
                  <td>{row.claimLevel}</td>
                  <td>
                    <input
                      type="checkbox"
                      checked={row.approved}
                      disabled={saving}
                      onChange={(e) =>
                        void runMutation("set_bullet_approved_cmd", { id: row.id, approved: e.target.checked })
                      }
                    />
                  </td>
                  <td>{row.text}</td>
                  <td>
                    <button
                      className="btn"
                      onClick={() => {
                        setEditingBulletId(row.id);
                        setEditingBulletText(row.text);
                      }}
                    >
                      Edit
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </section>
      ) : null}

      {preview ? (
        <section className="card stack-sm">
          <h3>SkillsBank</h3>
          <p className="subtle">Update approval and normalized level.</p>
          <table className="table">
            <thead>
              <tr>
                <th>Skill</th>
                <th>Level</th>
                <th>Approved</th>
              </tr>
            </thead>
            <tbody>
              {skills.map((row) => {
                const options = allowedLevels.includes(row.level.toLowerCase())
                  ? allowedLevels
                  : [row.level, ...allowedLevels];
                return (
                  <tr key={row.name}>
                    <td>{row.name}</td>
                    <td>
                      <select
                        value={row.level}
                        disabled={saving}
                        onChange={(e) =>
                          void runMutation("set_skill_level_cmd", { name: row.name, level: e.target.value })
                        }
                      >
                        {options.map((level) => (
                          <option key={`${row.name}-${level}`} value={level}>
                            {level}
                          </option>
                        ))}
                      </select>
                    </td>
                    <td>
                      <input
                        type="checkbox"
                        checked={row.approved}
                        disabled={saving}
                        onChange={(e) =>
                          void runMutation("set_skill_approved_cmd", {
                            name: row.name,
                            approved: e.target.checked
                          })
                        }
                      />
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </section>
      ) : null}

      {activeBullet ? (
        <section className="card stack-sm">
          <h3>Edit Bullet Text</h3>
          <p className="subtle">{activeBullet.id}</p>
          <textarea
            value={editingBulletText}
            rows={5}
            onChange={(e) => setEditingBulletText(e.target.value)}
          />
          <div className="row end">
            <button className="btn" onClick={() => setEditingBulletId(null)}>
              Cancel
            </button>
            <button
              className="btn btn-primary"
              disabled={saving}
              onClick={async () => {
                const ok = await runMutation("save_bullet_text_cmd", {
                  id: activeBullet.id,
                  text: editingBulletText
                });
                if (ok) {
                  setEditingBulletId(null);
                }
              }}
            >
              Save Bullet Text
            </button>
          </div>
        </section>
      ) : null}
    </section>
  );
}
