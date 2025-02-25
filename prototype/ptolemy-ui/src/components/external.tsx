import { Link } from 'react-router'
import {
    NavigationMenu,

    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu"

interface MenuItemProps{
    name: string,
    href: string,
}

function MenuItem({ name, href }: MenuItemProps) {
    return (
        <NavigationMenuItem>
            <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
                <Link to={href}>
                        {name}
                </Link>
            </NavigationMenuLink>
        </NavigationMenuItem>
    )
}

const ExternalLinks: React.FC = () => {
    return (
<NavigationMenu>
  <NavigationMenuList>
                <MenuItem name="Feedback" href="mailto:raz@gmail.com" />
                <MenuItem name="Docs" href={import.meta.env.VITE_PTOLEMY_DOCS} />

  </NavigationMenuList>
</NavigationMenu>
    )
  }

export default ExternalLinks
