import { Command } from "cmdk";
import { useMemo, useState } from "react";

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onNavigate: (view: string) => void;
}

const entries = [
  { value: "dashboard", label: "Go to Dashboard" },
  { value: "new-job", label: "Go to New Job" },
  { value: "job-review", label: "Go to Job Review" },
  { value: "banks", label: "Go to Banks" },
  { value: "templates", label: "Go to Templates" },
  { value: "settings", label: "Go to Settings" }
];

export function CommandPalette({ open, onOpenChange, onNavigate }: Props) {
  const [search, setSearch] = useState("");
  const filtered = useMemo(
    () => entries.filter((e) => e.label.toLowerCase().includes(search.toLowerCase())),
    [search]
  );

  if (!open) {
    return null;
  }

  return (
    <div className="palette-overlay" onClick={() => onOpenChange(false)}>
      <Command className="palette" onClick={(e) => e.stopPropagation()}>
        <Command.Input
          placeholder="Type a command..."
          value={search}
          onValueChange={setSearch}
          className="palette-input"
        />
        <Command.List>
          <Command.Empty>No results.</Command.Empty>
          {filtered.map((entry) => (
            <Command.Item
              key={entry.value}
              value={entry.value}
              onSelect={(value) => {
                onNavigate(value);
                onOpenChange(false);
              }}
              className="palette-item"
            >
              {entry.label}
            </Command.Item>
          ))}
        </Command.List>
      </Command>
    </div>
  );
}
