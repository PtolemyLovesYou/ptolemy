import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu"
import { Button } from "./ui/button"
import { NavLink } from "react-router"

const fallbackFromName = (name: string) => {
    return name.split(" ").map((word) => word[0].toUpperCase()).join("").slice(0, 2)
}

interface ProfileIconProps {
        name: string,
        profilePictureUrl: string,
}

function ProfileIcon({ name, profilePictureUrl }: ProfileIconProps) {
    return (
        <Avatar className="m-4">
            <AvatarImage src={profilePictureUrl} alt={`${name}'s Profile Picture`} />
            <AvatarFallback>{fallbackFromName(name)}</AvatarFallback>
        </Avatar>
    )
}

function ProfileDropdown({ name, profilePictureUrl }: ProfileIconProps) {
    return (
        <DropdownMenu aria-label="Profile and Settings Dropdown">
            <DropdownMenuTrigger asChild>
                <Button variant="ghost" size="icon">
                    <ProfileIcon name={name} profilePictureUrl={profilePictureUrl} />
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>
                    <DropdownMenuLabel>
                        <NavLink to="/profile" end>Profile</NavLink>
                    </DropdownMenuLabel>
                </DropdownMenuItem>
                <DropdownMenuItem>
                    <DropdownMenuLabel>Settings</DropdownMenuLabel>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>
                    <DropdownMenuLabel>Logout</DropdownMenuLabel>
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    )
}

export default ProfileDropdown
