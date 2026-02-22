import { useMemo, useState } from "react";

interface Props {
  diff: string;
}

type Mode = "inline" | "side";

function parseDiff(diff: string) {
  const lines = diff.split("\n");
  return lines
    .filter((line) => line.trim().length > 0)
    .map((line) => {
      if (line.startsWith("+ ")) {
        return { type: "add" as const, text: line.slice(2) };
      }
      if (line.startsWith("- ")) {
        return { type: "del" as const, text: line.slice(2) };
      }
      if (line.startsWith("  ")) {
        return { type: "same" as const, text: line.slice(2) };
      }
      return { type: "meta" as const, text: line };
    });
}

export function DiffViewer({ diff }: Props) {
  const [mode, setMode] = useState<Mode>("inline");
  const parsed = useMemo(() => parseDiff(diff), [diff]);

  return (
    <section className="card stack-sm">
      <div className="row between">
        <h3>Diff Viewer</h3>
        <div className="row">
          <button className={`btn ${mode === "inline" ? "btn-primary" : ""}`} onClick={() => setMode("inline")}>
            Inline
          </button>
          <button className={`btn ${mode === "side" ? "btn-primary" : ""}`} onClick={() => setMode("side")}>
            Side-by-side
          </button>
        </div>
      </div>

      {mode === "inline" ? (
        <pre className="diff-block">
          {parsed.map((row, idx) => (
            <div key={idx} className={`diff-row diff-${row.type}`}>
              {row.type === "add" ? "+ " : row.type === "del" ? "- " : "  "}
              {row.text}
            </div>
          ))}
        </pre>
      ) : (
        <div className="diff-side">
          <div>
            <strong>Removed / Context</strong>
            <pre className="diff-block">
              {parsed
                .filter((r) => r.type === "del" || r.type === "same")
                .map((row, idx) => (
                  <div key={idx} className={`diff-row diff-${row.type}`}>
                    {row.text}
                  </div>
                ))}
            </pre>
          </div>
          <div>
            <strong>Added / Context</strong>
            <pre className="diff-block">
              {parsed
                .filter((r) => r.type === "add" || r.type === "same")
                .map((row, idx) => (
                  <div key={idx} className={`diff-row diff-${row.type}`}>
                    {row.text}
                  </div>
                ))}
            </pre>
          </div>
        </div>
      )}
    </section>
  );
}
