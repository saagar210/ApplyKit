export interface Toast {
  id: string;
  message: string;
  tone: "info" | "success" | "error";
}

interface Props {
  toasts: Toast[];
}

export function ToastHost({ toasts }: Props) {
  return (
    <div className="toast-host" aria-live="polite" aria-label="notifications">
      {toasts.map((toast) => (
        <div key={toast.id} className={`toast toast-${toast.tone}`}>
          {toast.message}
        </div>
      ))}
    </div>
  );
}
