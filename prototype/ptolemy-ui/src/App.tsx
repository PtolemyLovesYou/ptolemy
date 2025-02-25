import { Routes, Route } from 'react-router'
import './App.css'
import Menu from './components/menu'
import Home from './Home'
function App() {
  return (
    <>
      <Menu />
      <main>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/events" element={<div>Events</div>} />
        <Route path="/ide" element={<div>IDE</div>} />
        <Route path="/settings" element={<div>Settings</div>} />
      </Routes>
        </main>
    </>
  )
}

export default App
