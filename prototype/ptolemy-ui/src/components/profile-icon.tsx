import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"

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

export default ProfileIcon
