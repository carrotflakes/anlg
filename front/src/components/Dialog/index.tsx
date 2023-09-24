import { useEffect, useRef } from "react";

import styles from "./index.module.scss";

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
    <dialog className={styles.Dialog} onClick={onClick} ref={ref}>
      {children}
    </dialog>
  );
}
