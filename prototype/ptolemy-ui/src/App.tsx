import { Routes, Route } from 'react-router'
import './index.css'
import {Menu, ExternalLinks} from './components/menu'
import Home from './pages/Home'
import IDE from './pages/Ide'
import Login from './pages/Login'
import ProfileDropdown from './components/profile-icon'
import { useEffect, useState } from 'react'
import { AUTH_TOKEN_KEY } from './constants'
import Profile from './pages/Profile'
import Events from './pages/Events'

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  // TODO - do we need this state?
  useEffect(() => {
    const token = localStorage.getItem(AUTH_TOKEN_KEY);
    if (token) {
      // TODO - fetch user details from graphQL
      setIsAuthenticated(true);
    }
  }, []);

  if (!isAuthenticated) {
    return <Login />;
  }

  return (
    <>
      <div className="header-container">
        <Menu />
        <div className="flex justify-end gap-5">
          <ExternalLinks />
          <ProfileDropdown name="José" profilePictureUrl="https://github.com/shadcn.png/" />
          </div>
      </div>
      <main className="p-10">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/events" element={<Events />} />
          <Route path="/ide" element={<IDE />} />
          <Route path="/settings" element={<div>Settings</div>} />
          <Route path="/profile" element={<Profile />} />
        </Routes>
      </main>
    </>
  )
}

export default App
