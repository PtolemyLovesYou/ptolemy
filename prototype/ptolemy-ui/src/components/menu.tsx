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
import { InternalLink, ExternalLink } from "./Link"



interface MenuItemProps{
    name: string,
    href: string,
    isExternal?: boolean
}

function MenuItem({ name, href, isExternal = false }: MenuItemProps) {
    const Link = isExternal ? ExternalLink : InternalLink
    return (
        <NavigationMenuItem className="inline-block">
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

export function Menu() {
return (<NavigationMenu aria-label="Main Menu">
    <NavigationMenuList>
    <NavigationMenuItem><Logo /></NavigationMenuItem>
        <MenuItem name="Events" href="/events" />
        <MenuItem name="IDE" href="/ide" />
        <MenuItem name="Settings" href="/settings" />
    </NavigationMenuList>
</NavigationMenu>)
}

export const ExternalLinks: React.FC = () => {
    return (
<NavigationMenu aria-label="External Links">
  <NavigationMenuList>
                <MenuItem name="Feedback" href="mailto:raz@gmail.com" isExternal />
                <MenuItem name="Docs" href={import.meta.env.VITE_PTOLEMY_DOCS} isExternal />
  </NavigationMenuList>
</NavigationMenu>
    )
  }
