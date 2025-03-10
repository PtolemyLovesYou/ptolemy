import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from './ui/button';
import { NavLink } from 'react-router';
import { User, LogOut, Settings } from 'lucide-react';
import { useAuth } from '@/auth/provider';

const fallbackFromName = (name: string) => {
  return name
    .split(' ')
    .map((word) => word[0].toUpperCase())
    .join('')
    .slice(0, 2);
};

interface ProfileIconProps {
  name: string;
  profilePictureUrl: string;
}

function ProfileIcon({ name, profilePictureUrl }: ProfileIconProps) {
  return (
    <Avatar className='m-4'>
      <AvatarImage src={profilePictureUrl} alt={`${name}'s Profile Picture`} />
      <AvatarFallback>{fallbackFromName(name)}</AvatarFallback>
    </Avatar>
  );
}

function ProfileDropdown({ name, profilePictureUrl }: ProfileIconProps) {
  const { logout } = useAuth();
  return (
    <DropdownMenu aria-label='Profile and Settings Dropdown'>
      <DropdownMenuTrigger asChild>
        <Button variant='ghost' size='icon'>
          <ProfileIcon name={name} profilePictureUrl={profilePictureUrl} />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem asChild>
          <DropdownMenuLabel asChild>
            <NavLink to='/profile' end>
              <User />
              Profile
            </NavLink>
          </DropdownMenuLabel>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem asChild>
          <DropdownMenuLabel onClick={logout}>
            <LogOut />
            Log out
          </DropdownMenuLabel>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default ProfileDropdown;
