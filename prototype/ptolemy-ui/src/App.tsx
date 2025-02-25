import { Routes, Route } from 'react-router'
import './index.css'
import Menu from './components/menu'
import Home from './Home'
import IDE from './Ide'
import ExternalLinks from './components/external'
import ProfileIcon from './components/profile-icon'

function App() {
  return (
    <>
      <div className="header-container">
        <Menu />
        <div className="flex align-right">
          <ExternalLinks />
          <ProfileIcon name="José" profilePictureUrl="https://github.com/shadcn.png/" />
        </div>
      </div>
      <main className="p-10">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/events" element={<div>Events</div>} />
          <Route path="/ide" element={<IDE />} />
          <Route path="/settings" element={<div>Settings</div>} />
        </Routes>
      </main>
    </>
  )
}

export default App
