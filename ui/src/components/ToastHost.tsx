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
    <div className="toast-host" role="region" aria-label="notifications">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`toast toast-${toast.tone}`}
          role={toast.tone === "error" ? "alert" : "status"}
          aria-live={toast.tone === "error" ? "assertive" : "polite"}
        >
          {toast.message}
        </div>
      ))}
    </div>
  );
}
