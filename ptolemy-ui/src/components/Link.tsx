import { NavLink, Link as ExtLink } from 'react-router';
import { ExternalLinkIcon } from 'lucide-react';
import {
  NavigationMenuLink,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu';

interface LinkProps {
  href: string;
  name: string;
  props?: object;
}

export const InternalLink = ({ href, name, ...props }: LinkProps) => {
  return (
    <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
      <NavLink to={href} end {...props}>
        {name}
      </NavLink>
    </NavigationMenuLink>
  );
};

export const ExternalLink = ({ href, name, ...props }: LinkProps) => {
  return (
    <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
      <a href={href} {...props} className='flex flex-row align-middle'>
        {name} <ExternalLinkIcon />
      </a>
    </NavigationMenuLink>
  );
};
