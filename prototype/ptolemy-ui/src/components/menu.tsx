import { NavLink } from 'react-router'
import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuIndicator,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
    NavigationMenuViewport,
} from "@/components/ui/navigation-menu"
import ptolemyLogo from '/logomark_lime.svg'
import Link from "./Link"



interface MenuItemProps{
    name: string,
    href: string,
}

function MenuItem({ name, href }: MenuItemProps) {
    return (
        <NavigationMenuItem>
            <Link name={name} href={href} />
        </NavigationMenuItem>
    )
}

function Logo() {
    return (
      <div>
        <NavLink to="/" end>
            <img src={ptolemyLogo} className="logo" alt="Ptolemy logo" />
        </NavLink>
      </div>
    )
}

function Menu() {
return (<NavigationMenu>
    <NavigationMenuList>
    <NavigationMenuItem><Logo /></NavigationMenuItem>
        <MenuItem name="Events" href="/events" />
        <MenuItem name="IDE" href="/ide" />
        <MenuItem name="Settings" href="/settings" />
    </NavigationMenuList>
</NavigationMenu>)
}

export default Menu
