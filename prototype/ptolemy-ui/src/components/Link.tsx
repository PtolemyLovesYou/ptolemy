import { NavLink } from "react-router"
import {
    NavigationMenuLink,
    navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu"

interface LinkProps {
    href: string
    name: string
    props?: object
}

const Link = ({ href, name, ...props }: LinkProps) => {
    return (
        <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
            <NavLink to={href} end {...props}>
                {name}
            </NavLink>
        </NavigationMenuLink>
    );
}

export default Link
