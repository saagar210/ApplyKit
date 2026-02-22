import { type ChangeEventHandler, useState } from "react";
import type { GenerateRequest } from "../lib/types";

interface Props {
  busy: boolean;
  onGenerate: (request: GenerateRequest) => Promise<void>;
}

export function NewJob({ busy, onGenerate }: Props) {
  const [company, setCompany] = useState("");
  const [role, setRole] = useState("");
  const [source, setSource] = useState("manual");
  const [baseline, setBaseline] = useState<"1pg" | "2pg">("1pg");
  const [jdText, setJdText] = useState("");
  const [importFileName, setImportFileName] = useState<string | null>(null);

  const normalizeInput = (value: string) => {
    return value.replace(/\r\n/g, "\n").replace(/\r/g, "\n").trimEnd();
  };

  const readTextFromFile = async (file: File): Promise<string> => {
    if (typeof file.text === "function") {
      return file.text();
    }
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onerror = () => reject(new Error("Failed to read file"));
      reader.onload = () => resolve(String(reader.result ?? ""));
      reader.readAsText(file);
    });
  };

  const onImportFile: ChangeEventHandler<HTMLInputElement> = async (event) => {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }
    const text = await readTextFromFile(file);
    setJdText(normalizeInput(text));
    setImportFileName(file.name);
  };

  return (
    <section className="stack-lg">
      <div className="card">
        <h2>New Job</h2>
        <p className="subtle">Paste the JD and generate a deterministic packet.</p>
      </div>

      <div className="card form-grid">
        <label>
          Company
          <input value={company} onChange={(e) => setCompany(e.target.value)} placeholder="Acme" />
        </label>
        <label>
          Role title
          <input
            value={role}
            onChange={(e) => setRole(e.target.value)}
            placeholder="Senior Support Engineer"
          />
        </label>
        <label>
          Source
          <input value={source} onChange={(e) => setSource(e.target.value)} placeholder="LinkedIn" />
        </label>
        <label>
          Baseline
          <select value={baseline} onChange={(e) => setBaseline(e.target.value as "1pg" | "2pg")}>
            <option value="1pg">1pg</option>
            <option value="2pg">2pg</option>
          </select>
        </label>
        <label className="span-2">
          JD
          <div className="row between wrap">
            <span className="subtle">{importFileName ? `Imported: ${importFileName}` : "Paste or import JD text"}</span>
            <input
              aria-label="Import JD file"
              type="file"
              accept=".txt,.md,text/plain,text/markdown"
              onChange={onImportFile}
            />
          </div>
          <textarea
            value={jdText}
            onChange={(e) => setJdText(normalizeInput(e.target.value))}
            rows={14}
            placeholder="Paste job description here"
          />
        </label>
        <div className="span-2 row end">
          <button
            className="btn btn-primary"
            disabled={busy || !company || !role || !jdText}
            onClick={() => onGenerate({ company, role, source, baseline, jdText })}
          >
            {busy ? "Generating..." : "Generate Packet (⌘Enter)"}
          </button>
        </div>
      </div>
    </section>
  );
}
