import * as Dialog from "@radix-ui/react-dialog";
import { Command } from "cmdk";
import { useEffect, useMemo, useRef, useState } from "react";

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
  const inputRef = useRef<HTMLInputElement | null>(null);
  const previousActiveRef = useRef<HTMLElement | null>(null);

  const filtered = useMemo(
    () => entries.filter((e) => e.label.toLowerCase().includes(search.toLowerCase())),
    [search]
  );

  useEffect(() => {
    if (!open) {
      return;
    }

    previousActiveRef.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    window.setTimeout(() => {
      inputRef.current?.focus();
    }, 0);
  }, [open]);

  return (
    <Dialog.Root
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) {
          setSearch("");
          window.setTimeout(() => {
            previousActiveRef.current?.focus();
          }, 0);
        }
        onOpenChange(nextOpen);
      }}
    >
      <Dialog.Portal>
        <Dialog.Overlay className="palette-overlay" />
        <Dialog.Content
          className="palette"
          aria-label="Command palette"
          onOpenAutoFocus={(event) => event.preventDefault()}
        >
          <Dialog.Title className="sr-only">Command palette</Dialog.Title>
          <Dialog.Description className="sr-only">
            Type to find a command, then press Enter to navigate.
          </Dialog.Description>
          <Command>
            <Command.Input
              ref={inputRef}
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
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
