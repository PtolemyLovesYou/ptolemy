import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { cn } from '@/lib/utils';
import { UserApiKey } from '@/graphql/types';
import { CREATE_USER_API_KEY, GET_USER_PROFILE, UPDATE_USER } from '@/graphql/queries';
import { useMutation, useQuery } from '@apollo/client';
import { Slider } from '@/components/ui/slider';
import { toast } from 'sonner';

interface APIKeysProps {
  apiKeys: UserApiKey[];
}

function APIKeys({ apiKeys }: APIKeysProps) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>ID</TableHead>
          <TableHead>Name</TableHead>
          <TableHead>Key Preview</TableHead>
          <TableHead>Expires</TableHead>
          <TableHead>Delete?</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {apiKeys.map((row, i) => (
          <TableRow key={i}>
            {['id', 'name', 'keyPreview', 'expiresAt'].map((col, j) => (
              <TableCell key={j}>
                {row[col as keyof UserApiKey].toString()}
              </TableCell>
            ))}
            <TableCell>
              <Checkbox id={`delete-${row.id}`} />
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

const apiKeyFormSchema = z
  .object({
    name: z.string().max(256),
    durationDays: z.number().int().gte(1).lte(365).default(30),
  })
  .required();

interface CAPIKeyVarProps {
  variables: {
    name: string;
    durationDays: number;
  };
}

interface CreateAPIKeyFormProps {
  createUserApiKey: ({ variables }: CAPIKeyVarProps) => void;
}

function CreateAPIKeyForm({ createUserApiKey }: CreateAPIKeyFormProps) {
  const createKeyForm = useForm<z.infer<typeof apiKeyFormSchema>>({
    resolver: zodResolver(apiKeyFormSchema),
  });

  async function onSubmit(values: z.infer<typeof apiKeyFormSchema>) {
    const { name, durationDays } = values;
    createUserApiKey({ variables: { name, durationDays } });
  }

  return (
    <Form {...createKeyForm}>
      <form
        onSubmit={createKeyForm.handleSubmit(onSubmit)}
        className='space-y-8'
      >
        <FormField
          control={createKeyForm.control}
          name='name'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormControl>
                <Input placeholder='name' autoComplete='off' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={createKeyForm.control}
          name='durationDays'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Duration (Days)</FormLabel>
              <FormControl>
                <span className='flex gap-2'>
                  <Slider
                    defaultValue={[30]}
                    max={365}
                    min={1}
                    step={1}
                    className={cn('w-[60%]')}
                    name={field.name}
                    onValueChange={(v) => field.onChange(v[0])}
                  />{' '}
                  {field.value}
                </span>
              </FormControl>
              <FormDescription>When should the API Key expire?</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <DialogClose>
          <Button type='submit'>Create</Button>
        </DialogClose>
      </form>
    </Form>
  );
}


const UserStatus = ['ACTIVE', 'SUSPENDED'] as const;

// For now, can only edit username
const profileFormSchema = z.object({
  id: z.string(),
  username: z.string(),
  status: z.enum(UserStatus),
  displayName: z.string(),
  isAdmin: z.boolean(),
  isSysadmin: z.boolean(),
}).partial();

type Workspace = {
  id: string;
  name: string;
}

interface ProfileFormProps {
  profile: {
    id: string;
    username: string;
    status: typeof UserStatus[number];
    displayName: string;
    isAdmin: boolean;
    isSysadmin: boolean;
    workspaces: Workspace[];
  }
}

const ProfileForm = ({ profile }: ProfileFormProps) => {
  const [updateUser, _userData] = useMutation(UPDATE_USER, {
    refetchQueries: [GET_USER_PROFILE, 'Me'],
    onCompleted: () => {
      toast.success('Profile updated!');
    },
  });
  const form = useForm<z.infer<typeof profileFormSchema>>({
    resolver: zodResolver(profileFormSchema),
    defaultValues: {
      ...profile,
    }
  });

  async function onSubmit(data: z.infer<typeof profileFormSchema>) {
    const { displayName } = data;
    updateUser({ variables: { userId: profile.id, displayName } });
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="id"
          render={({ field }) => (
            <FormItem>
              <FormLabel htmlFor="id">ID</FormLabel>
              <FormControl>
                <Input placeholder="(empty)" {...field} disabled />
              </FormControl>
              <FormMessage />
            </FormItem>
          )} />
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel htmlFor="username">Username</FormLabel>
              <FormControl>
                <Input placeholder="(empty)" {...field} disabled />
              </FormControl>
              <FormMessage />
            </FormItem>
          )} />
                <FormField
          control={form.control}
          name="displayName"
          render={({ field }) => (
            <FormItem>
              <FormLabel htmlFor="displayName">Display Name</FormLabel>
              <FormControl>
                <Input placeholder="(empty)" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )} />
        <FormField
          control={form.control}
          name="isSysadmin"
          render={({ field }) => (
            <FormItem className="flex flex-row">
              <FormControl>
                <Checkbox checked={field.value} onCheckedChange={field.onChange} disabled />
              </FormControl>
              <FormLabel htmlFor="isSysadmin">Is System Admin?</FormLabel>

              <FormMessage />
            </FormItem>
          )} />
        <FormField
          control={form.control}
          name="isAdmin"
          render={({ field }) => (
            <FormItem className="flex flex-row">
              <FormControl>
                <Checkbox checked={field.value} onCheckedChange={field.onChange} disabled />
              </FormControl>
              <FormLabel htmlFor="isAdmin">Is Admin?</FormLabel>

              <FormMessage />
            </FormItem>
          )} />
        <div className='grid max-w-sm gap-1.5'>
        <Label htmlFor="workspaces">Workspaces</Label>
        <Input id="workspaces" value={String(profile.workspaces)} placeholder="(empty)" disabled />
        </div>
        <Button type="submit">Save</Button>
      </form>
    </Form>
  )
}


const Profile: React.FC = () => {
  const { loading, error, data } = useQuery(GET_USER_PROFILE);
  const [createUserApiKey, _data] = useMutation(CREATE_USER_API_KEY, {
    refetchQueries: [GET_USER_PROFILE, 'Me'],
    onCompleted: (data) => {
      const {
        user: {
          createUserApiKey: { apiKey },
        },
      } = data;
      toast.success('API Key created!', {
        duration: 1000 * 60,
        description: <pre>{apiKey.apiKey}</pre>,
      });
    },
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  const {
    me: { userApiKeys, __typename: _, ...profile },
  } = data;

  return (
    <div className='grid gap-5'>
      <h1>Profile</h1>
      <ProfileForm profile={profile} />

      <h2>API Keys</h2>
      {userApiKeys.length ? (
        <APIKeys apiKeys={userApiKeys} />
      ) : (
        <p>No API Keys available</p>
      )}
      <div>
        <Dialog>
          <DialogTrigger asChild>
            <Button>Create API Key</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create User API Key</DialogTitle>
              <DialogDescription asChild>
                <CreateAPIKeyForm createUserApiKey={createUserApiKey} />
              </DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
};

export default Profile;
