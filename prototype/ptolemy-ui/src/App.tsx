import { Routes, Route } from 'react-router';
import './index.css';
import { Menu, ExternalLinks } from './components/menu';
import Home from './pages/Home';
import IDE from './pages/Ide';
import Login from './pages/Login';
import ProfileDropdown from './components/profile-icon';
import Profile from './pages/Profile';
import Events from './pages/Events';
import { useAuth } from './auth/provider';
import { ModeToggle } from './components/theme/toggle';
import { Toaster } from './components/ui/sonner';

function App() {
  const { token } = useAuth();

  if (!token) {
    return <Login />;
  }

  return (
    <>
      <div className='header-container'>
        <Menu />
        <div className='flex justify-end gap-5'>
          <ExternalLinks />
          <ProfileDropdown
            name='José'
            profilePictureUrl='https://github.com/shadcn.png/'
          />
          <ModeToggle />
        </div>
      </div>
      <main className='p-10'>
        <Routes>
          <Route path='/' element={<Home />} />
          <Route path='/events' element={<Events />} />
          <Route path='/ide' element={<IDE />} />
          <Route path='/settings' element={<div>Settings</div>} />
          <Route path='/profile' element={<Profile />} />
        </Routes>
      </main>
      <Toaster />
    </>
  );
}

export default App;
