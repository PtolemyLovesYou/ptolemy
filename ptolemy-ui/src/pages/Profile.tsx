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
import { CREATE_USER_API_KEY, GET_USER_PROFILE } from '@/graphql/queries';
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

const formSchema = z
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
  const createKeyForm = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
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

  const mapToInput = ([key, value]: [string, unknown]) => {
    return (
      <div key={key} className='grid max-w-sm gap-1.5'>
        <Label htmlFor={key.toLowerCase()}>{key}</Label>
        <Input
          type='text'
          id={key.toLowerCase()}
          placeholder='(empty)'
          value={String(value)}
          disabled
        />
      </div>
    );
  };

  return (
    <div className='grid gap-5'>
      <h1>Profile</h1>
      {Object.entries(profile).map(mapToInput)}

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
