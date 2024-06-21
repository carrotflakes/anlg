import { useEffect, useRef } from "react";

export function Dialog({
  children,
  onClose,
}: {
  children?: React.ReactNode;
  onClose?: () => void;
}) {
  const ref = useRef<HTMLDialogElement>(null!);

  useEffect(() => {
    ref.current.showModal();
  }, []);

  const onClick = (e: React.MouseEvent<HTMLDialogElement, MouseEvent>) => {
    const rect = ref.current.getBoundingClientRect();
    const clickedInDialog =
      rect.top <= e.clientY &&
      e.clientY <= rect.top + rect.height &&
      rect.left <= e.clientX &&
      e.clientX <= rect.left + rect.width;
    if (!clickedInDialog) onClose?.();
  };

  return (
    <dialog className={"min-w-[10rem] min-h-10 rounded-lg bg-white p-4"} onClick={onClick} ref={ref}>
      {children}
    </dialog>
  );
}
