import type { ReactElement } from "react";

interface Props {
  markdown: string;
}

function renderParagraph(line: string, key: string) {
  return <p key={key}>{line}</p>;
}

export function MarkdownViewer({ markdown }: Props) {
  const lines = markdown.replace(/\r\n/g, "\n").replace(/\r/g, "\n").split("\n");
  const nodes: ReactElement[] = [];
  let bulletBuffer: string[] = [];

  const flushBullets = () => {
    if (bulletBuffer.length === 0) {
      return;
    }
    const keyBase = `ul-${nodes.length}`;
    nodes.push(
      <ul key={keyBase}>
        {bulletBuffer.map((entry, idx) => (
          <li key={`${keyBase}-${idx}`}>{entry}</li>
        ))}
      </ul>
    );
    bulletBuffer = [];
  };

  lines.forEach((raw, idx) => {
    const line = raw.trimEnd();
    if (!line.trim()) {
      flushBullets();
      return;
    }

    if (line.startsWith("- ")) {
      bulletBuffer.push(line.slice(2).trim());
      return;
    }

    flushBullets();
    if (line.startsWith("### ")) {
      nodes.push(<h4 key={`h4-${idx}`}>{line.slice(4)}</h4>);
      return;
    }
    if (line.startsWith("## ")) {
      nodes.push(<h3 key={`h3-${idx}`}>{line.slice(3)}</h3>);
      return;
    }
    if (line.startsWith("# ")) {
      nodes.push(<h2 key={`h2-${idx}`}>{line.slice(2)}</h2>);
      return;
    }
    if (line.startsWith("```")) {
      nodes.push(<code key={`code-${idx}`}>{line}</code>);
      return;
    }
    nodes.push(renderParagraph(line, `p-${idx}`));
  });

  flushBullets();

  return <div className="markdown-viewer">{nodes}</div>;
}
