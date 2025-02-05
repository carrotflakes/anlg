import React from 'react';

interface ButtonProps {
  onClick: () => void;
  disabled?: boolean;
  selected?: boolean;
  children: React.ReactNode;
}

export function Button({ onClick, disabled, selected, children }: ButtonProps) {
  return (
    <button
      className={`px-2 py-1 rounded cursor-pointer ${selected ? 'bg-blue-500 text-white' : 'bg-gray-100'}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
