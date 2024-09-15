import React from 'react'

interface ModalProps {
  select: boolean
  unselect: () => void
  children: React.ReactNode
}

export default function Modal({ select, unselect, children }: ModalProps) {
  return (
    <div
      onClick={unselect}
      className={`fixed inset-0 z-[9999] flex justify-center items-center transition-colors
        ${select ? 'visible bg-black/50' : 'invisible'}
        `}
    >
      <div
        className={`transition-all ${
          select ? 'scale-100 opacity-100' : 'scale-125 opacity-0'
        }`}
        onClick={(e) => e.stopPropagation()}
      >
        {children}
      </div>
    </div>
  )
}
