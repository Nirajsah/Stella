import React from 'react'
import Modal from '../Modal'

const Settings = () => {
  return (
    <div className="w-[800px] h-[600px] bg-white rounded-lg p-4">Settings</div>
  )
}

export const LeftSideMenu = () => {
  const [showSettings, setShowSettings] = React.useState(true)
  return (
    <div className="border w-full h-full border-black">
      <Modal
        select={showSettings}
        unselect={() => setShowSettings(!showSettings)}
      >
        <Settings />
      </Modal>
      <div>
        <button>Settings</button>
      </div>
    </div>
  )
}
